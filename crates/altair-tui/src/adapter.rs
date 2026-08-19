//! The conformance adapter: a hidden mode of this same binary.
//!
//! `conformance/scenarios.md` observes two boundaries, the person's and the
//! instance's. This module is the person's, reduced to a line of JSON in and a
//! line of JSON out, so that the suite specified in Wave 1.5 can drive the
//! client the same way it will drive the Kotlin one.
//!
//! # Why this is a mode and not a second binary
//!
//! A separate adapter would be a second implementation of local acceptance,
//! and the suite would then be judging something shaped like the client rather
//! than the client. Everything below goes through exactly the same store and
//! the same sender the person's screens will.
//!
//! # What this mode owes that the product does not
//!
//! **The label.** A person's way of returning to something is not an
//! identifier, and the harness needs one anyway, so it names each capture and
//! expects to be able to open it again by that name. That mapping is this
//! module's obligation and lives in this module's own table, on this module's
//! own connection, so that nothing in [`crate::device`] carries a concept the
//! product does not have.
//!
//! **The mapping is written first.** Acceptance is shown after the store's
//! commit, so writing the name before that commit means that at the instant
//! the person is told, both are durable. Written afterwards, a kill in between
//! would leave a capture nobody could name.

use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use altair_conformance::client::{Action, CaptureContent, Event, Reply, Signal, SignalKind};
use altair_conformance::process::env_var;
use rusqlite::{Connection, OptionalExtension, params};

use crate::capture::Captured;
use crate::device::Store;
use crate::sender::Sender;
use crate::signals::{self, Signals};
use crate::wire;

/// The argument that selects this mode.
pub const FLAG: &str = "--conformance-adapter";

/// Run the adapter until stdin ends.
///
/// # Panics
///
/// Panics when the environment the harness is supposed to supply is absent,
/// which is a broken harness rather than a conformance verdict.
pub async fn run() {
    let state_dir = PathBuf::from(require(env_var::STATE_DIR));
    let instance_url = require(env_var::INSTANCE);
    let token = require(env_var::TOKEN);

    let signals = Arc::new(Signals::new());

    // A store that will not open is not a reason not to run: the client still
    // has to answer, and what it has to answer is a refusal that states the
    // condition. Nothing sends, because nothing was accepted.
    let store = Store::open(&state_dir).map_err(|error| error.to_string());
    if store.is_ok() {
        match Sender::new(&state_dir, &instance_url, token, Arc::clone(&signals)) {
            Ok(sender) => {
                tokio::spawn(sender.run());
            }
            Err(error) => eprintln!("altair: no sender: {error}"),
        }
    }

    let mut surface = Surface {
        labels: Labels::open(&state_dir).ok(),
        store,
        signals,
    };

    // Blocking, on its own thread, because the channel is lockstep and because
    // nothing on this path may wait on anything the sender is doing.
    tokio::task::spawn_blocking(move || surface.serve())
        .await
        .expect("the adapter's surface thread");
}

fn require(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("the harness must set {name}"))
}

struct Surface {
    store: Result<Store, String>,
    labels: Option<Labels>,
    signals: Arc<Signals>,
}

impl Surface {
    fn serve(&mut self) {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        for line in stdin.lock().lines() {
            let Ok(line) = line else { return };
            if line.trim().is_empty() {
                continue;
            }
            let reply = match serde_json::from_str::<Action>(&line) {
                Ok(action) => self.answer(&action),
                Err(error) => Reply::Error {
                    message: format!("unreadable action: {error}"),
                },
            };
            let Ok(encoded) = serde_json::to_string(&reply) else {
                return;
            };
            if writeln!(stdout, "{encoded}").is_err() || stdout.flush().is_err() {
                return;
            }
        }
    }

    fn answer(&mut self, action: &Action) -> Reply {
        match action {
            Action::Capture { label, content } => self.capture(label, content),
            Action::Edit { label, title } => self.edit(label, title),
            Action::Bind { member_id } => {
                if let Ok(store) = self.store.as_mut() {
                    let _ = store.bind(member_id);
                }
                Reply::Nothing
            }
            Action::Open { label } => self.open(label),
            Action::Showing => Reply::Showing {
                signals: self.signals.showing().iter().map(as_signal).collect(),
            },
            Action::DrainEvents => Reply::Events {
                events: self.signals.drain_events().iter().map(as_event).collect(),
            },
        }
    }

    fn capture(&mut self, label: &str, content: &CaptureContent) -> Reply {
        let (store, labels) = match (self.store.as_mut(), self.labels.as_ref()) {
            (Ok(store), Some(labels)) => (store, labels),
            // The one local condition that reaches the guarantee rather than
            // delaying it, stated at the moment of the attempt.
            (Err(condition), _) => {
                return Reply::Refused {
                    condition: condition.clone(),
                };
            }
            (Ok(_), None) => {
                return Reply::Refused {
                    condition: "local storage cannot be written to".to_string(),
                };
            }
        };

        let composed = wire::compose_creation(&as_captured(content));
        if labels.remember(label, &composed.entity_id).is_err() {
            return Reply::Refused {
                condition: "local storage cannot be written to".to_string(),
            };
        }
        let body = composed
            .body
            .as_ref()
            .map(|(id, bytes)| (id.as_slice(), bytes.as_slice()));
        match store.accept_creation(
            &composed.entity_id,
            &composed.content,
            body,
            &composed.intent,
        ) {
            Ok(()) => Reply::Accepted {
                label: label.to_string(),
            },
            Err(error) => Reply::Refused {
                condition: error.to_string(),
            },
        }
    }

    fn edit(&mut self, label: &str, title: &str) -> Reply {
        let (Ok(store), Some(labels)) = (self.store.as_mut(), self.labels.as_ref()) else {
            return Reply::Refused {
                condition: "local storage cannot be written to".to_string(),
            };
        };
        let Ok(Some(entity_id)) = labels.entity(label) else {
            return Reply::Nothing;
        };
        let (patch, intent) = wire::compose_title_edit(&entity_id, title);
        match store.accept_edit(&entity_id, &patch, &intent) {
            Ok(()) => Reply::Accepted {
                label: label.to_string(),
            },
            Err(error) => Reply::Refused {
                condition: error.to_string(),
            },
        }
    }

    fn open(&mut self, label: &str) -> Reply {
        let absent = Reply::Entity {
            label: label.to_string(),
            present: false,
            entity_id: Vec::new(),
            title: None,
            body: None,
            bytes: None,
        };
        let (Ok(store), Some(labels)) = (self.store.as_ref(), self.labels.as_ref()) else {
            return absent;
        };
        let Ok(Some(entity_id)) = labels.entity(label) else {
            return absent;
        };
        // `held` follows a recreation on its own, so the name the harness gave
        // keeps working across one without this module knowing it happened.
        let Ok(Some(held)) = store.held(&entity_id) else {
            return absent;
        };
        Reply::Entity {
            label: label.to_string(),
            present: true,
            entity_id: held.entity_id.clone(),
            title: held.title().map(str::to_string),
            body: held.body().map(str::to_string),
            bytes: held.bytes.clone(),
        }
    }
}

/// The harness's names for things, and nothing else.
struct Labels {
    connection: Connection,
}

impl Labels {
    fn open(state_dir: &Path) -> rusqlite::Result<Self> {
        let connection = Connection::open(state_dir.join(crate::device::FILE))?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS adapter_label (
               label     TEXT PRIMARY KEY,
               entity_id BLOB NOT NULL
             );",
        )?;
        Ok(Self { connection })
    }

    fn remember(&self, label: &str, entity_id: &[u8]) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT OR REPLACE INTO adapter_label (label, entity_id) VALUES (?1, ?2)",
            params![label, entity_id],
        )?;
        Ok(())
    }

    fn entity(&self, label: &str) -> rusqlite::Result<Option<Vec<u8>>> {
        self.connection
            .query_row(
                "SELECT entity_id FROM adapter_label WHERE label = ?1",
                params![label],
                |row| row.get(0),
            )
            .optional()
    }
}

fn as_captured(content: &CaptureContent) -> Captured {
    match content {
        CaptureContent::Bare => Captured::Bare,
        CaptureContent::Note { title, body } => Captured::Note {
            title: title.clone(),
            body: body.clone(),
        },
        CaptureContent::File { media_type, bytes } => Captured::File {
            media_type: media_type.clone(),
            bytes: bytes.clone(),
        },
    }
}

fn as_signal(signal: &signals::Signal) -> Signal {
    Signal {
        kind: match signal.kind {
            signals::Kind::Fault => SignalKind::Fault,
            signals::Kind::Other => SignalKind::Other,
        },
        text: signal.text.clone(),
        quantity: signal.quantity,
    }
}

fn as_event(event: &signals::Event) -> Event {
    match event {
        signals::Event::Raised(text) => Event::SignalRaised { text: text.clone() },
        signals::Event::Cleared(text) => Event::SignalCleared { text: text.clone() },
    }
}

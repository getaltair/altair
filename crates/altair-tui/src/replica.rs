//! Keeping a copy of what the household holds.
//!
//! # Why there is a replica at all
//!
//! **The instance serves no way to read an entity by identity and no way to
//! list what a container holds.** `Query` is the literal arm: its predicate
//! matches nothing when the text is empty, so it cannot enumerate, and it
//! answers with entities and never relations. Every screen that is not search —
//! the captures list, the ladder, a detail with its backlinks — is assembled on
//! the device from what the change stream delivered. That is not a performance
//! decision and there is no other way to build one.
//!
//! # What this loop is allowed to conclude
//!
//! A change is a statement of what is, not a delta, so taking one twice is
//! harmless and taking them out of order is not possible. That is what lets the
//! position be recorded in the same commit as the changes it covers without any
//! further care: a kill costs a repeat.
//!
//! Being told the held position is past the horizon is an outcome and not an
//! error. The answer is to rebuild, which is always available and always
//! correct, and which costs time rather than data.
//!
//! # What it is not
//!
//! It never widens what the person can see. The audience predicate lives inside
//! the instance's candidate query and nowhere else; this holds what a member was
//! already sent and can only narrow it further.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use altair_proto::v1;
use tonic::transport::Channel;

use crate::device::Store;
use crate::signals::Signals;
use crate::wire::{self, Condition};

/// How long to wait before asking again once there is nothing more waiting.
///
/// **Never reported.** How long ago the client last heard is not a condition
/// and does not reach the person.
const POLL: Duration = Duration::from_millis(500);

/// A call that has not answered in this long is an unanswered call, which is a
/// wait.
const PATIENCE: Duration = Duration::from_secs(30);

pub struct Replica {
    store: Store,
    client: v1::altair_client::AltairClient<Channel>,
    token: String,
    signals: Arc<Signals>,
}

impl Replica {
    /// Point a replica at an instance. Nothing connects here.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be opened or the instance's address cannot
    /// be read as one.
    pub fn new(
        state_dir: &Path,
        instance_url: &str,
        token: String,
        signals: Arc<Signals>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let store = Store::reopen(state_dir)?;
        let channel =
            tonic::transport::Endpoint::from_shared(instance_url.to_string())?.connect_lazy();
        Ok(Self {
            store,
            client: v1::altair_client::AltairClient::new(channel),
            token,
            signals,
        })
    }

    /// Run until the process ends.
    pub async fn run(mut self) {
        loop {
            if !self.pass().await {
                tokio::time::sleep(POLL).await;
            }
        }
    }

    /// Answers whether there is more waiting right now.
    async fn pass(&mut self) -> bool {
        let Ok(since) = self.store.position() else {
            return false;
        };
        let request = self.authorised(v1::ChangesRequest {
            since: Some(v1::Position { value: since }),
        });
        let answer = match tokio::time::timeout(PATIENCE, self.client.changes(request)).await {
            Err(_elapsed) => return self.hold(Condition::Wait),
            Ok(Err(status)) => return self.hold(wire::classify(&status)),
            Ok(Ok(response)) => response.into_inner(),
        };
        self.signals.unrecognised(false);

        match answer.outcome {
            Some(v1::changes_response::Outcome::Changes(set)) => {
                let position = set.position.map_or(since, |position| position.value);
                if self.store.take_changes(&set.changes, position).is_err() {
                    return false;
                }
                set.more
            }
            // Past the horizon. Throw the copy away and read the whole stream
            // again; nothing local is touched, because an unsent capture is not
            // the instance's to have an opinion about.
            Some(v1::changes_response::Outcome::Unanswerable(_)) => self.store.rebuild().is_ok(),
            None => false,
        }
    }

    fn hold(&self, condition: Condition) -> bool {
        self.signals.unrecognised(condition == Condition::Fault);
        false
    }

    fn authorised<T>(&self, message: T) -> tonic::Request<T> {
        let mut request = tonic::Request::new(message);
        if let Ok(value) = format!("Bearer {}", self.token).parse() {
            request.metadata_mut().insert("authorization", value);
        }
        request
    }
}

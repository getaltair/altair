//! One scenario's world: a fake instance, a client under test, and the two
//! directories that make a process kill and a device restart different things.
//!
//! Everything a scenario is allowed to do goes through here, which is how the
//! two-boundary rule is kept: there is no method on [`World`] that reads the
//! client's storage or its queue.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::client::{
    Action, CaptureContent, ClientEnvironment, ClientProcess, ClientUnderTest, Event, Label,
    Offers, Reply, Signal,
};
use crate::instance::FakeInstance;

/// How long the harness waits for something that should happen.
pub const SETTLE: Duration = Duration::from_secs(3);

/// How long the harness watches to prove something does not happen. Long
/// enough to cover several retry cycles of any sane client.
pub const QUIET: Duration = Duration::from_millis(1500);

pub struct World {
    pub instance: FakeInstance,
    under_test: Arc<dyn ClientUnderTest>,
    root: PathBuf,
    environment: ClientEnvironment,
    client: Option<Box<dyn ClientProcess>>,
}

impl World {
    /// Start a fake instance and prepare a client's environment. Nothing is
    /// launched until [`World::launch`].
    ///
    /// # Panics
    ///
    /// Panics if the temporary directories cannot be made.
    pub async fn new(under_test: Arc<dyn ClientUnderTest>) -> Self {
        let instance = FakeInstance::start().await;
        let root =
            std::env::temp_dir().join(format!("altair-conformance-{}", uuid::Uuid::new_v4()));
        let state_dir = root.join("state");
        let runtime_dir = root.join("runtime");
        std::fs::create_dir_all(&state_dir).expect("make state dir");
        std::fs::create_dir_all(&runtime_dir).expect("make runtime dir");
        let environment = ClientEnvironment {
            instance_url: instance.url(),
            state_dir,
            runtime_dir,
            token: format!("conformance-{}", uuid::Uuid::new_v4()),
        };
        Self {
            instance,
            under_test,
            root,
            environment,
            client: None,
        }
    }

    #[must_use]
    pub fn offers(&self) -> Offers {
        self.under_test.offers()
    }

    #[must_use]
    pub fn client_name(&self) -> &str {
        self.under_test.name()
    }

    /// Start the client. Panics if it cannot be started, which is a harness
    /// fault rather than a verdict.
    ///
    /// # Panics
    ///
    /// Panics if the client process cannot be launched.
    pub fn launch(&mut self) {
        assert!(
            self.client.is_none(),
            "the client under test is already running"
        );
        let client = self
            .under_test
            .launch(&self.environment)
            .unwrap_or_else(|error| panic!("could not launch the client under test: {error}"));
        self.client = Some(client);
    }

    /// Kill the client without warning. `SIGKILL`; no flush, no cleanup.
    pub fn kill(&mut self) {
        if let Some(mut client) = self.client.take() {
            client.kill();
        }
    }

    /// B1: the process dies and comes back. Both directories survive.
    pub fn restart_process(&mut self) {
        self.kill();
        self.launch();
    }

    /// B2: the device restarts. Durable storage survives; everything
    /// ephemeral does not, so the runtime directory is wiped.
    ///
    /// # Panics
    ///
    /// Panics if the runtime directory cannot be recreated.
    pub fn restart_device(&mut self) {
        self.kill();
        let runtime = self.environment.runtime_dir.clone();
        let _ = std::fs::remove_dir_all(&runtime);
        std::fs::create_dir_all(&runtime).expect("recreate runtime dir");
        self.launch();
    }

    /// A4 and F5: local storage the client cannot write to.
    ///
    /// **Answers whether the harness could impose the condition**, rather than
    /// assuming it can. On unix it is a mode bit. On Windows there is no mode
    /// bit and the equivalent is a deny entry in the directory's access control
    /// list, which this harness does not yet write — so it says no there and
    /// the two scenarios that need it skip, which is a visible line in the
    /// ledger rather than a silent pass.
    ///
    /// # Panics
    ///
    /// Panics if the permissions cannot be changed on a platform that has
    /// them, which is a broken harness rather than a verdict.
    #[must_use]
    pub fn make_state_unwritable(&self) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(
                &self.environment.state_dir,
                std::fs::Permissions::from_mode(0o500),
            )
            .expect("make the state dir unwritable");
            true
        }
        #[cfg(not(unix))]
        {
            false
        }
    }

    #[cfg(unix)]
    fn make_state_writable(path: &Path) {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700));
    }

    /// Do one thing at the person's surface.
    ///
    /// # Panics
    ///
    /// Panics if the client cannot be reached at all.
    pub fn act(&mut self, action: &Action) -> Reply {
        let client = self
            .client
            .as_mut()
            .expect("the client under test is not running");
        client
            .act(action)
            .unwrap_or_else(|error| panic!("could not reach the client under test: {error}"))
    }

    /// Do one thing, and say how long the person waited for the answer.
    pub fn act_timed(&mut self, action: &Action) -> (Reply, Duration) {
        let started = Instant::now();
        let reply = self.act(action);
        (reply, started.elapsed())
    }

    /// Capture a note whose title is the label, which is how the harness
    /// recognises it again at the instance boundary.
    pub fn capture(&mut self, label: &str) -> Reply {
        self.act(&Action::Capture {
            label: label.to_string(),
            content: CaptureContent::Note {
                title: label.to_string(),
                body: String::new(),
            },
        })
    }

    pub fn capture_content(&mut self, label: &str, content: CaptureContent) -> Reply {
        self.act(&Action::Capture {
            label: label.to_string(),
            content,
        })
    }

    pub fn edit(&mut self, label: &str, title: &str) -> Reply {
        self.act(&Action::Edit {
            label: label.to_string(),
            title: title.to_string(),
        })
    }

    pub fn open(&mut self, label: &str) -> Reply {
        self.act(&Action::Open {
            label: label.to_string(),
        })
    }

    pub fn bind(&mut self, member_id: &[u8]) -> Reply {
        self.act(&Action::Bind {
            member_id: member_id.to_vec(),
        })
    }

    /// Everything standing in front of the person right now.
    ///
    /// # Panics
    ///
    /// Panics if the client answers something other than a showing.
    pub fn showing(&mut self) -> Vec<Signal> {
        match self.act(&Action::Showing) {
            Reply::Showing { signals } => signals,
            other => panic!("asked what the client is showing and it said {other:?}"),
        }
    }

    /// Person facing events since the last drain.
    ///
    /// # Panics
    ///
    /// Panics if the client answers something other than events.
    pub fn drain_events(&mut self) -> Vec<Event> {
        match self.act(&Action::DrainEvents) {
            Reply::Events { events } => events,
            other => panic!("asked the client for events and it said {other:?}"),
        }
    }

    /// Assert the client accepted `label`, and say so in the failure when it
    /// did not. Almost every scenario opens with this, which is why the stub
    /// client fails all of them for the same honest reason.
    ///
    /// # Panics
    ///
    /// Panics when the client did not show acceptance.
    pub fn accept(&mut self, label: &str) {
        let reply = self.capture(label);
        assert_accepted(&reply, label);
    }
}

/// Assert one reply is acceptance of `label`.
///
/// # Panics
///
/// Panics when it is not.
pub fn assert_accepted(reply: &Reply, label: &str) {
    match reply {
        Reply::Accepted { label: shown } => assert_eq!(
            shown, label,
            "the client accepted something, but named it {shown} rather than {label}"
        ),
        other => panic!(
            "the client was expected to show {label} as accepted and showed {other:?} instead"
        ),
    }
}

/// Pull an entity reply apart, or fail saying what came instead.
///
/// # Panics
///
/// Panics when the reply is not an entity.
#[must_use]
pub fn expect_entity(reply: &Reply) -> Seen {
    match reply {
        Reply::Entity {
            label,
            present,
            entity_id,
            title,
            body,
            bytes,
        } => Seen {
            label: label.clone(),
            present: *present,
            entity_id: entity_id.clone(),
            title: title.clone(),
            body: body.clone(),
            bytes: bytes.clone(),
        },
        other => panic!("the client was asked to show an entity and answered {other:?}"),
    }
}

/// What the person sees when they return to something.
#[derive(Debug, Clone)]
pub struct Seen {
    pub label: Label,
    pub present: bool,
    pub entity_id: Vec<u8>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub bytes: Option<Vec<u8>>,
}

impl Drop for World {
    fn drop(&mut self) {
        self.kill();
        #[cfg(unix)]
        Self::make_state_writable(&self.environment.state_dir);
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

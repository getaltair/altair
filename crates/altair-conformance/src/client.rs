//! The client under test, as the harness is allowed to see it.
//!
//! `conformance/scenarios.md` observes two boundaries and no others: what the
//! person sees, and what reaches the instance. This module is the first of
//! them. Nothing here reaches into the client's storage, its queue, or its
//! internals — every type below names something a person could point at.
//!
//! Wave 4.1 supplies the real implementation of [`ClientUnderTest`]. In Wave 1
//! the only implementor is [`crate::null::NullClient`], which does nothing,
//! which is why every scenario fails.
//!
//! # The shape, and why it is this shape
//!
//! A client under test is a **separate operating system process**. It has to
//! be: sections B and F kill it without warning, and a `Drop` impl or a
//! cooperative shutdown would not test what those scenarios test.
//!
//! The harness drives that process over one newline delimited JSON channel:
//! an [`Action`] in on stdin, a [`Reply`] out on stdout, one line each, in
//! lockstep. That is deliberately the most boring protocol available, because
//! DR-006 requires this harness to exist a second time in Kotlin and a
//! transcription must not become a reinterpretation.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// What a client offers above the capture floor.
///
/// The substrate puts the floor at creation and makes everything above it a
/// platform decision, so scenarios that edit, or that capture files, or that
/// capture before binding, apply only where the client offers those. A client
/// that declines them is conforming and skips them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offers {
    /// A5's floor is always offered; this names A3's case: capture before the
    /// device has ever been bound to a household.
    pub unbound_capture: bool,
    /// Editing an entity while the instance is unreachable. Sections C, E and
    /// G lean on it.
    pub offline_edit: bool,
    /// Capturing a file, meaning bytes that travel through `PutBody`.
    pub file_capture: bool,
}

impl Offers {
    /// Everything above the floor. What the Wave 4.1 terminal client is
    /// expected to offer.
    #[must_use]
    pub const fn everything() -> Self {
        Self {
            unbound_capture: true,
            offline_edit: true,
            file_capture: true,
        }
    }

    /// The floor and nothing else. A conforming client, with more skips.
    #[must_use]
    pub const fn capture_only() -> Self {
        Self {
            unbound_capture: false,
            offline_edit: false,
            file_capture: false,
        }
    }
}

/// Everything a launched client is told about the world.
///
/// Two directories, and the difference between them is load bearing. B1 kills
/// a process; B2 restarts a device. A device restart additionally loses
/// everything ephemeral, so the harness models it by wiping `runtime_dir` and
/// keeping `state_dir`. A client that keeps durable state in `runtime_dir`
/// passes B1 and fails B2, which is the distinction B2 exists to draw.
#[derive(Debug, Clone)]
pub struct ClientEnvironment {
    /// Where the fake instance answers, as a URL.
    pub instance_url: String,
    /// Durable storage. Survives a process kill and a device restart.
    pub state_dir: PathBuf,
    /// Ephemeral storage. Survives a process kill; wiped by a device restart.
    pub runtime_dir: PathBuf,
    /// The token the client should present. Opaque; the fake instance never
    /// verifies a signature, it only records what it was handed.
    pub token: String,
}

/// A harness chosen name for one thing the person captured.
///
/// The person's own way of returning to something is not a UUID, and the
/// harness needs one anyway, so the client under test is asked to remember the
/// label it was given alongside the capture. That is an obligation of the
/// adapter the client exposes for testing, not of the product.
pub type Label = String;

/// What the person captures.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CaptureContent {
    /// A5: an identity, a type, and a timestamp, and nothing else.
    Bare,
    /// A note. `title` doubles as the correlation handle at the instance
    /// boundary, because a title is a thing that crosses the wire.
    Note { title: String, body: String },
    /// A file, meaning bytes that must reach `PutBody` before the intent that
    /// names them.
    File { media_type: String, bytes: Vec<u8> },
}

/// Something the harness asks the person's surface to do, or to show.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Action {
    /// The person captures.
    Capture {
        label: Label,
        content: CaptureContent,
    },
    /// The person edits something they captured, giving it a new title.
    Edit { label: Label, title: String },
    /// The device is bound to a household as this member. A3's second half.
    Bind { member_id: Vec<u8> },
    /// The person returns to something they captured. Answers what they see.
    Open { label: Label },
    /// Everything the client is currently showing the person and has not
    /// withdrawn. Section F is almost entirely assertions over this.
    Showing,
    /// Person facing events since the last drain. F9 needs to know that a
    /// standing fault did not repeat, which the standing surface cannot say.
    DrainEvents,
}

/// One thing the client is currently showing the person.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Signal {
    pub kind: SignalKind,
    /// Whatever the platform's surface puts in front of the person. A terminal
    /// and an Android client word it differently and both conform; showing
    /// nothing is what is not permitted.
    pub text: String,
    /// A number the signal states. F7 permits exactly one number here — how
    /// many the instance refused — and forbids every other.
    pub quantity: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SignalKind {
    /// A condition that will not clear by the ordinary path continuing to run.
    Fault,
    /// Anything else the client puts in front of the person. Section F mostly
    /// asserts that this list is empty.
    Other,
}

/// A person facing event, as distinct from a standing signal.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum Event {
    SignalRaised { text: String },
    SignalCleared { text: String },
}

/// What the client answers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "reply", rename_all = "snake_case")]
pub enum Reply {
    /// The capture is theirs. Nothing about the network or credentials
    /// participated in this, and it is durable already.
    Accepted { label: Label },
    /// The client would not take it, and says why, at the moment of the
    /// attempt. A4 and F5.
    Refused { condition: String },
    /// The action produced nothing the person can see.
    Nothing,
    /// What the person sees when they return to something.
    Entity {
        label: Label,
        present: bool,
        /// The entity's public identity. Not internal state: identity is
        /// client assigned, crosses the wire, and G3 is about it changing.
        #[serde(default)]
        entity_id: Vec<u8>,
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        body: Option<String>,
        #[serde(default)]
        bytes: Option<Vec<u8>>,
    },
    /// Answer to [`Action::Showing`].
    Showing { signals: Vec<Signal> },
    /// Answer to [`Action::DrainEvents`].
    Events { events: Vec<Event> },
    /// The adapter itself broke. Never a conforming answer to anything.
    Error { message: String },
}

/// Something went wrong reaching the client, as opposed to the client
/// answering badly. Always a harness fault, never a conformance verdict.
#[derive(Debug)]
pub enum ClientError {
    /// The client did not answer within the harness's patience.
    Silent,
    /// The process is gone.
    Gone,
    /// The channel carried something that is not a [`Reply`].
    Unreadable(String),
    Io(std::io::Error),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Silent => write!(f, "the client under test did not answer in time"),
            Self::Gone => write!(f, "the client under test is not running"),
            Self::Unreadable(s) => {
                write!(f, "the client under test said something unreadable: {s}")
            }
            Self::Io(e) => write!(f, "i/o with the client under test: {e}"),
        }
    }
}

impl std::error::Error for ClientError {}

/// How the harness starts a client. Implemented once per client, and swapped
/// in one place — `client_under_test()` in `tests/scenarios.rs`.
pub trait ClientUnderTest: Send + Sync {
    /// A name for this client in test output.
    fn name(&self) -> &str;

    /// What this client offers above the capture floor. Scenarios that need
    /// something it does not offer are skipped, not failed.
    fn offers(&self) -> Offers;

    /// Start the client as its own operating system process.
    ///
    /// Called more than once per scenario with the same [`ClientEnvironment`]:
    /// that is what a restart is.
    fn launch(
        &self,
        environment: &ClientEnvironment,
    ) -> Result<Box<dyn ClientProcess>, ClientError>;
}

/// A running client under test.
pub trait ClientProcess: Send {
    /// Do one thing at the person's surface and read the one answer. Blocks
    /// until the client answers or the harness runs out of patience, which is
    /// itself observable: E1 times this.
    fn act(&mut self, action: &Action) -> Result<Reply, ClientError>;

    /// Kill the process without warning. `SIGKILL`, no cleanup, no chance to
    /// flush. Idempotent.
    fn kill(&mut self);
}

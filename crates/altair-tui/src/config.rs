//! Where the client is told about the world.
//!
//! **Environment first, and no configuration file yet.** A file is a thing to
//! design — where it lives on three platforms, what happens when it is
//! malformed, whether it can be reloaded — and none of that is decided by
//! anything above this wave. What is needed to run is small enough to say in
//! four variables, and a file can be laid over these later without changing
//! what reads them.

use std::path::PathBuf;

use crate::ui::glyphs::Set;
use crate::ui::theme::Mode;

/// Everything the client is told before it draws anything.
#[derive(Debug, Clone)]
pub struct Config {
    /// Where this device's own store lives. Durable: it holds the outbox.
    pub state_dir: PathBuf,
    /// Where the household's instance answers.
    pub instance: Option<String>,
    /// Night or daylight.
    pub mode: Mode,
    /// The design's glyphs, or the set for terminals that render Ambiguous
    /// characters wide.
    pub glyphs: Set,
}

impl Config {
    /// Read the environment.
    ///
    /// **Nothing here fails.** A client that would not start because a variable
    /// was misspelled is a client that cannot be used to capture, and capture
    /// is the thing that must never be expensive. Every setting has an answer
    /// it falls back to and none of them is a question.
    #[must_use]
    pub fn from_environment() -> Self {
        Self {
            state_dir: state_dir(),
            instance: var("ALTAIR_INSTANCE"),
            mode: match var("ALTAIR_THEME").as_deref() {
                Some("daylight") => Mode::Daylight,
                _ => Mode::Starfield,
            },
            glyphs: match var("ALTAIR_GLYPHS").as_deref() {
                // For a terminal that renders `◆` and its family two cells
                // wide. See `crate::ui::glyphs`.
                Some("narrow") => Set::NarrowSafe,
                _ => Set::Signature,
            },
        }
    }
}

fn var(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

/// Where durable state goes on this platform.
///
/// **Durable is the word that matters.** This directory holds the outbox and
/// the compositions, so it is the state directory rather than a cache or a
/// runtime one: a device restart takes those away, and it must not take a
/// person's unsent captures with them.
fn state_dir() -> PathBuf {
    if let Some(given) = var("ALTAIR_STATE_DIR") {
        return PathBuf::from(given);
    }
    #[cfg(windows)]
    {
        if let Some(local) = var("LOCALAPPDATA") {
            return PathBuf::from(local).join("altair");
        }
    }
    if let Some(state) = var("XDG_STATE_HOME") {
        return PathBuf::from(state).join("altair");
    }
    if let Some(home) = var("HOME") {
        return PathBuf::from(home).join(".local/state/altair");
    }
    PathBuf::from("altair-state")
}

//! Handing composition to the person's own editor, and keeping the file.
//!
//! # Why the client does not have an editor
//!
//! A body is markdown the person wrote, and they already have somewhere they
//! write markdown. Building a second editor inside a terminal client means
//! reimplementing everything they have configured and getting it slightly
//! wrong. So composition is a handoff: this client owns a file, `$EDITOR`
//! writes into it, and this client reads it back.
//!
//! # What the client owes, given that
//!
//! **The file is durable from the moment it exists.** It goes in the state
//! directory rather than the runtime one, so a crash of the editor, of this
//! client, or of the machine leaves the person's words on disk. Composition is
//! the one place a person's work exists outside the store, and losing it
//! because a process died would be losing something they had already typed.
//!
//! **A composition left behind is offered back.** [`Editor::abandoned`] is what
//! the client asks after a crash; nothing is cleaned up on the way past.
//!
//! **An editor that is not there is a fault, not a wait.** It will not start
//! working because time passed, so it is said plainly at the moment of the
//! attempt rather than swallowed into silence.

use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Where compositions live, inside the durable directory.
const COMPOSITIONS: &str = "compositions";

/// One thing being written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composition {
    path: PathBuf,
}

impl Composition {
    /// The file the editor is pointed at, and that the client keeps.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// What can go wrong handing over.
#[derive(Debug)]
pub enum Handoff {
    /// Nothing said which editor to use. A fault: waiting will not produce
    /// one.
    NoEditorConfigured,
    /// Something said which editor to use and it could not be started —
    /// usually because it is not installed. Also a fault, and it names what it
    /// tried so the person can see the typo.
    CannotStart {
        program: String,
        why: io::Error,
    },
    /// The editor ran and said it failed.
    EditorFailed {
        program: String,
        status: String,
    },
    Io(io::Error),
}

impl std::fmt::Display for Handoff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoEditorConfigured => write!(
                f,
                "no editor is configured; set EDITOR or VISUAL to the one you use"
            ),
            Self::CannotStart { program, why } => {
                write!(f, "{program} could not be started: {why}")
            }
            Self::EditorFailed { program, status } => {
                write!(f, "{program} ended badly: {status}")
            }
            Self::Io(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Handoff {}

impl From<io::Error> for Handoff {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

pub struct Editor {
    directory: PathBuf,
}

impl Editor {
    /// Compositions are kept under `state_dir`, which survives a device
    /// restart. The runtime directory does not, and a person's unfinished
    /// paragraph is not ephemeral.
    #[must_use]
    pub fn new(state_dir: &Path) -> Self {
        Self {
            directory: state_dir.join(COMPOSITIONS),
        }
    }

    /// Which editor the person uses. `VISUAL` first, then `EDITOR`, which is
    /// the order every other program on the machine uses.
    #[must_use]
    pub fn configured() -> Option<String> {
        for name in ["VISUAL", "EDITOR"] {
            if let Ok(value) = std::env::var(name)
                && !value.trim().is_empty()
            {
                return Some(value);
            }
        }
        None
    }

    /// Start composing, seeded with whatever is already there.
    ///
    /// **The file is on disk before this returns**, which is what makes the
    /// promise in the help surface true.
    ///
    /// # Errors
    ///
    /// Fails when the file cannot be written.
    pub fn begin(&self, seed: &str) -> io::Result<Composition> {
        std::fs::create_dir_all(&self.directory)?;
        let path = self.directory.join(format!("{}.md", uuid::Uuid::new_v4()));
        std::fs::write(&path, seed)?;
        Ok(Composition { path })
    }

    /// Hand the file to the person's editor and wait for them.
    ///
    /// # Errors
    ///
    /// Fails when no editor is configured, when the configured one cannot be
    /// started, or when it ends badly. All three are faults.
    pub fn edit(&self, composition: &Composition) -> Result<String, Handoff> {
        let configured = Self::configured().ok_or(Handoff::NoEditorConfigured)?;
        self.edit_with(&configured, composition)
    }

    /// Hand the file to a named editor.
    ///
    /// **The seam is here rather than around `std::env`** so that what happens
    /// when an editor is missing can be checked without a test reaching into
    /// the process's environment and changing it under every other test in the
    /// binary.
    ///
    /// # Errors
    ///
    /// Fails when the editor cannot be started or ends badly. Both are faults.
    pub fn edit_with(
        &self,
        configured: &str,
        composition: &Composition,
    ) -> Result<String, Handoff> {
        // A configured editor is often a program and its arguments — `code
        // --wait`, `emacsclient -nw`. Splitting on whitespace is what every
        // other program on the machine does with this variable.
        let mut parts = configured.split_whitespace();
        let program = parts.next().ok_or(Handoff::NoEditorConfigured)?.to_string();
        let status = Command::new(&program)
            .args(parts)
            .arg(&composition.path)
            .status()
            .map_err(|why| Handoff::CannotStart {
                program: program.clone(),
                why,
            })?;
        if !status.success() {
            return Err(Handoff::EditorFailed {
                program,
                status: status.to_string(),
            });
        }
        Ok(self.read(composition)?)
    }

    /// What is in the file now.
    ///
    /// # Errors
    ///
    /// Fails when the file cannot be read.
    pub fn read(&self, composition: &Composition) -> io::Result<String> {
        std::fs::read_to_string(&composition.path)
    }

    /// Compositions still on disk: the ones something died in the middle of.
    ///
    /// **Offered back rather than tidied away.** A file here is a person's
    /// words that never reached the store, which makes it the most valuable
    /// thing this client is holding.
    ///
    /// # Errors
    ///
    /// Fails when the directory cannot be read for a reason other than its not
    /// being there.
    pub fn abandoned(&self) -> io::Result<Vec<Composition>> {
        let entries = match std::fs::read_dir(&self.directory) {
            Ok(entries) => entries,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(error),
        };
        let mut out: Vec<Composition> = entries
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|extension| extension == "md"))
            .map(|path| Composition { path })
            .collect();
        out.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(out)
    }

    /// The words reached the store, so the file is no longer the only copy.
    ///
    /// # Errors
    ///
    /// Fails when the file cannot be removed.
    pub fn finish(&self, composition: &Composition) -> io::Result<()> {
        match std::fs::remove_file(&composition.path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }
}

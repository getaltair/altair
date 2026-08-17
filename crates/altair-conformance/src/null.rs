//! The Wave 1 client under test: one that does nothing.
//!
//! This is why the suite is red, and it is red for the right reason. The null
//! client is a real process, launched the same way the terminal client will be,
//! speaking the same channel. It answers every action honestly with "nothing
//! happened", so each scenario runs its steps to completion and then fails on
//! the behaviour it is actually about.
//!
//! It declares [`Offers::everything`] on purpose. It offers nothing at all, but
//! the client this suite exists for — the Wave 4.1 terminal client — offers
//! editing and file capture, and a stub that declared otherwise would skip
//! nineteen scenarios rather than fail them. The skip machinery is exercised
//! separately, in `tests/skips.rs`.

use std::path::PathBuf;

use crate::client::{ClientEnvironment, ClientError, ClientProcess, ClientUnderTest, Offers};
use crate::process::ChildClient;

pub struct NullClient {
    program: PathBuf,
    offers: Offers,
}

impl NullClient {
    /// Point at the built `altair-null-client` binary. Tests pass
    /// `env!("CARGO_BIN_EXE_altair-null-client")`.
    #[must_use]
    pub fn at(program: impl Into<PathBuf>) -> Self {
        Self {
            program: program.into(),
            offers: Offers::everything(),
        }
    }

    /// Narrow what this client claims to offer. Used to prove that a
    /// capture-only client skips rather than fails.
    #[must_use]
    pub fn offering(mut self, offers: Offers) -> Self {
        self.offers = offers;
        self
    }
}

impl ClientUnderTest for NullClient {
    fn name(&self) -> &str {
        "null client (Wave 1 placeholder; implements nothing)"
    }

    fn offers(&self) -> Offers {
        self.offers
    }

    fn launch(
        &self,
        environment: &ClientEnvironment,
    ) -> Result<Box<dyn ClientProcess>, ClientError> {
        let child = ChildClient::spawn(&self.program, &[], environment)?;
        Ok(Box::new(child))
    }
}

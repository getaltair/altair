//! The outbox conformance suite: `conformance/scenarios.md`, made executable.
//!
//! # This suite is expected to fail. Do not delete it.
//!
//! The outbox does not exist yet. Wave 4.1 writes it; Wave 1.5 wrote the
//! scenarios that judge it, because DR-006's obligation is that they exist
//! before the *second* implementation and they may as well exist before the
//! first. Every scenario here runs to completion against a stub client that
//! implements nothing, and fails on the behaviour it is about. That gradient —
//! scenarios turning green one at a time as Wave 4.1 lands behaviour — is the
//! whole value of the suite, and it is destroyed by anything that makes a
//! scenario pass without an outbox behind it.
//!
//! So: the scenarios are gated behind the `run-conformance` feature, off by
//! default, and `mise run conformance` is how you look at the red. See
//! `README.md` beside this file.
//!
//! # What is observed
//!
//! Two boundaries and no others. [`client`] is the person's; [`instance`] is
//! the instance's. Nothing here reads the client's storage, its queue, or any
//! internal state, because `conformance/scenarios.md` says in as many words
//! that those are not conformance concerns.

pub mod client;
pub mod instance;
pub mod null;
pub mod process;
pub mod world;

/// How a scenario ended, other than by failing. A failure is a panic, so
/// libtest reports it the way it reports every other failing test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The client did what the scenario requires.
    Passed,
    /// The scenario does not apply to this client, and says why. The substrate
    /// puts everything above the capture floor at the client's discretion, so
    /// a skip is a conforming answer and not a quiet pass.
    Skipped(String),
}

/// Record how a scenario ended.
///
/// Prints, so `--nocapture` shows skips plainly, and appends to the file named
/// by `ALTAIR_CONFORMANCE_LEDGER` when that is set, so a skip is mechanically
/// distinguishable from a pass rather than only visible to a reader.
pub fn report(id: &str, test_name: &str, outcome: &Outcome) {
    let line = match outcome {
        Outcome::Passed => format!("{id}\tPASSED\t{test_name}"),
        Outcome::Skipped(why) => format!("{id}\tSKIPPED\t{test_name}\t{why}"),
    };
    println!("[conformance] {line}");
    if let Ok(path) = std::env::var("ALTAIR_CONFORMANCE_LEDGER") {
        use std::io::Write;
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            let _ = writeln!(file, "{line}");
        }
    }
}

/// Declare one scenario.
///
/// The literal is the scenario's id in `conformance/scenarios.md`, and
/// `tests/coverage.rs` reads these literals back out of the source to prove
/// that every scenario in the document has exactly one test and no test names
/// a scenario the document does not have.
#[macro_export]
macro_rules! scenario {
    ($id:literal, $name:ident, $body:block) => {
        #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
        async fn $name() {
            let outcome: $crate::Outcome = async move $body.await;
            $crate::report($id, stringify!($name), &outcome);
        }
    };
}

/// End a scenario as skipped unless the client offers what it needs.
#[macro_export]
macro_rules! skip_unless {
    ($condition:expr, $why:expr) => {
        if !$condition {
            return $crate::Outcome::Skipped(($why).to_string());
        }
    };
}

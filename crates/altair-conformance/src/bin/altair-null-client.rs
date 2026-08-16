//! A client under test that implements nothing.
//!
//! It reads actions and answers, truthfully, that nothing happened: no capture
//! is accepted, no entity is present, nothing is shown to the person. It never
//! opens a connection to the instance and never writes to its state directory.
//!
//! That is the Wave 1 deliverable. Every scenario in `conformance/scenarios.md`
//! drives this process through its steps and fails on the step that asks for
//! outbox behaviour. Wave 4.1 replaces it, one behaviour at a time, and the
//! suite goes green in the same order.

use std::io::{BufRead, Write};

use altair_conformance::client::{Action, Reply};

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { return };
        if line.trim().is_empty() {
            continue;
        }
        let reply = match serde_json::from_str::<Action>(&line) {
            Ok(action) => answer(&action),
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

fn answer(action: &Action) -> Reply {
    match action {
        // Nothing is captured, so nothing is accepted and nothing is refused
        // either: the person is simply shown nothing.
        Action::Capture { .. } | Action::Edit { .. } | Action::Bind { .. } => Reply::Nothing,
        Action::Open { label } => Reply::Entity {
            label: label.clone(),
            present: false,
            entity_id: Vec::new(),
            title: None,
            body: None,
            bytes: None,
        },
        Action::Showing => Reply::Showing {
            signals: Vec::new(),
        },
        Action::DrainEvents => Reply::Events { events: Vec::new() },
    }
}

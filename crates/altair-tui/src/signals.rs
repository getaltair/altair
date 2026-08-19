//! What the client puts in front of the person, and what it deliberately does
//! not.
//!
//! # The rule this module exists to keep
//!
//! **Waiting is silent; faults signal.** An unreachable instance and an
//! expired session are the same wait and neither is ever shown. A refusal and
//! a condition the client cannot classify are faults and are always shown.
//!
//! **Exactly one number exists anywhere in this client, and it is how many the
//! instance refused.** No queue depth, no badge, no counter that rises while
//! the person is away. Forty things waiting and forty things waiting for three
//! weeks look identical, because they are.
//!
//! # Why signals are computed rather than accumulated
//!
//! A signal is a statement about a condition that holds now, so it is derived
//! from the conditions each time they change. Nothing appends. That is what
//! makes a standing fault not escalate, not repeat, and not age: after three
//! weeks the person finds one current statement, because there was only ever
//! one and nothing counted the interval.

use std::sync::Mutex;

/// One thing the client is showing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signal {
    pub kind: Kind,
    pub text: String,
    /// The one permitted number. Never a backlog.
    pub quantity: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Fault,
    Other,
}

/// A transition, as distinct from a standing statement. What repeats is
/// visible here and nowhere else, which is why the conformance suite asks for
/// it separately.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Raised(String),
    Cleared(String),
}

#[derive(Default)]
struct State {
    /// How many intents the instance has refused.
    refusals: u64,
    /// Whether the last thing the instance said was something this client
    /// cannot classify as self clearing.
    unrecognised: bool,
    standing: Vec<Signal>,
    events: Vec<Event>,
}

/// Everything the person can be shown. One of these per running client.
#[derive(Default)]
pub struct Signals {
    state: Mutex<State>,
}

impl Signals {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// How many the instance refused. Setting this to zero clears the signal.
    pub fn refusals(&self, count: u64) {
        self.update(|state| state.refusals = count);
    }

    /// Whether an unrecognised condition stands right now. It clears the
    /// moment the ordinary path succeeds, with nothing for the person to
    /// acknowledge or dismiss.
    pub fn unrecognised(&self, present: bool) {
        self.update(|state| state.unrecognised = present);
    }

    /// Everything standing in front of the person.
    #[must_use]
    pub fn showing(&self) -> Vec<Signal> {
        self.state.lock().expect("signals").standing.clone()
    }

    /// Transitions since the last drain.
    #[must_use]
    pub fn drain_events(&self) -> Vec<Event> {
        std::mem::take(&mut self.state.lock().expect("signals").events)
    }

    fn update(&self, change: impl FnOnce(&mut State)) {
        let mut state = self.state.lock().expect("signals");
        change(&mut state);
        let next = render(&state);
        if next == state.standing {
            return;
        }
        let mut transitions: Vec<Event> = state
            .standing
            .iter()
            .filter(|signal| !next.contains(signal))
            .map(|gone| Event::Cleared(gone.text.clone()))
            .collect();
        transitions.extend(
            next.iter()
                .filter(|signal| !state.standing.contains(signal))
                .map(|fresh| Event::Raised(fresh.text.clone())),
        );
        state.events.append(&mut transitions);
        state.standing = next;
    }
}

/// The statements, in a fixed order so that an unchanged condition produces an
/// unchanged answer.
fn render(state: &State) -> Vec<Signal> {
    let mut signals = Vec::new();
    if state.unrecognised {
        signals.push(Signal {
            kind: Kind::Fault,
            // Says what happened and nothing about why. The instance's detail
            // is free text for a log and is never repeated to a person, so a
            // refusal cannot reveal whether something is absent or invisible.
            text: "the instance answered with something this client does not understand"
                .to_string(),
            quantity: None,
        });
    }
    if state.refusals > 0 {
        signals.push(Signal {
            kind: Kind::Fault,
            text: if state.refusals == 1 {
                "the instance would not take one capture".to_string()
            } else {
                format!("the instance would not take {} captures", state.refusals)
            },
            quantity: Some(state.refusals),
        });
    }
    signals
}

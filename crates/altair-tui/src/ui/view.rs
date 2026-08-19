//! What a screen is handed, as plain data.
//!
//! **Rendering takes a view and never a store.** Everything below is composed
//! before anything is drawn, which is what lets every screen be checked as a
//! snapshot without a database, an instance, or a running client — and what
//! keeps a screen from quietly acquiring the ability to ask a question mid-draw.
//!
//! The types are the vocabulary of the design rather than of the wire: a row
//! has a *when*, a *what* and a *note*, because that is what the person reads.
//! Turning an `Entity` into one of these is [`crate::ui::compose`]'s job.

use super::glyphs::State;

/// One line in a list.
#[derive(Debug, Clone, Default)]
pub struct Row {
    /// The left column: a time, a date, or nothing.
    pub when: String,
    /// What the person called it.
    pub what: String,
    /// Its type, in the person's words rather than the wire's.
    pub kind: String,
    /// One quiet thing on the right — how it got here, what it is waiting on.
    pub note: String,
    /// Where the person is.
    pub selected: bool,
    /// Whether it is stepped back: older, or already dealt with.
    pub past: bool,
}

/// A run of rows under a section label.
#[derive(Debug, Clone, Default)]
pub struct Group {
    pub label: String,
    pub rows: Vec<Row>,
    /// Whether the label is a quieter rule than the one that opened the screen.
    pub quiet: bool,
}

/// One rung of the guidance ladder.
#[derive(Debug, Clone)]
pub struct Rung {
    pub depth: usize,
    pub state: State,
    pub what: String,
    pub kind: String,
    pub note: String,
    pub selected: bool,
    /// Whether this rung is waiting on something above it. The one place a
    /// ladder row carries the irreversible colour, because being blocked is
    /// not something the person can clear from here.
    pub blocked: bool,
}

/// One thing under a location, or one location under another.
#[derive(Debug, Clone)]
pub struct Kept {
    pub depth: usize,
    pub what: String,
    /// What the person last said is there, in their own units. Never computed
    /// from history.
    pub amount: String,
    /// What live relations have committed of it, where anything has.
    pub committed: String,
    pub selected: bool,
}

/// A relation, from whichever end the person is standing at.
#[derive(Debug, Clone)]
pub struct Link {
    /// The type, or nothing where it is untyped. An untyped relation is
    /// untyped, not possibly typed.
    pub kind: String,
    /// Which way it runs, as read from here.
    pub outward: bool,
    pub what: String,
    /// Where it was formed, where it was formed at a phrase.
    pub anchor: String,
}

/// What retrieval was able to do, and what it could not.
///
/// **It degrades per stage and says so; it does not fail.** A missing arm is a
/// sentence, not an error, and the client shows it as one.
#[derive(Debug, Clone, Default)]
pub struct Answered {
    pub complete: bool,
    /// What was not run, in the person's words.
    pub missing: String,
}

/// Both retained values of one part two people wrote at once.
///
/// **Not a failure and not a question.** The write applied, both values were
/// kept, and the entity is readable, editable and relatable throughout. There
/// is nothing here for the person to resolve before continuing.
#[derive(Debug, Clone)]
pub struct Retained {
    pub part: String,
    pub mine: String,
    pub theirs: String,
    pub theirs_member: String,
}

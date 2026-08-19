//! The glyph table, and the hazard it exists to contain.
//!
//! # Why there are two sets
//!
//! Eleven of the locked design's glyphs are East-Asian-Width **Ambiguous**:
//! `◇ ◈ ◆ · — → ↑ ↓ ← ⇧ ▏`. Terminals disagree about whether an Ambiguous
//! character occupies one cell or two, and there is no way to ask which answer
//! this terminal will give — the escape sequence that reports it is answered
//! unreliably, and a wrong answer is worse than a fixed one because it is
//! wrong everywhere at once rather than in one place.
//!
//! So: the design's glyphs are the default, a narrow-safe set sits behind
//! configuration for terminals where the default looks wrong, and **nothing is
//! ever aligned by measuring a glyph.**
//!
//! `✦ ▸ ▾ › ↵ ⌫` are Neutral and safe, so both sets share them. Changing them
//! would damage a settled design over a hazard they do not have.
//!
//! # The rule that actually prevents drift
//!
//! **Every glyph is drawn into a cell whose width the layout declares.** The
//! width is a property of the column, not of what goes in it, so a terminal
//! that renders a diamond wide spends the cell's own padding rather than every
//! column to its right. A layout that asked how wide `◆` is would be correct on
//! one terminal and skewed on another; there is no such question anywhere here,
//! and `tests/glyphs.rs` checks that both sets fit the cells they are drawn
//! into.
//!
//! > **Rejected: substituting globally.** It damages a settled design over a
//! > hazard most terminals do not have.
//! >
//! > **Rejected: measuring at startup.** Terminals answer that question
//! > unreliably, and a wrong answer is worse than a fixed cell.

/// How wide the column holding a state glyph is. Two, so that a terminal
/// rendering an Ambiguous diamond as two cells fills this one exactly instead
/// of pushing everything after it along.
pub const STATE_CELL: u16 = 2;

/// How wide an inline marker's column is, for the same reason.
pub const MARKER_CELL: u16 = 2;

/// Which set is in use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Set {
    /// The locked design's own glyphs. The default.
    Signature,
    /// For terminals that render Ambiguous characters as two cells. Every
    /// substitution is a character no terminal disagrees about.
    NarrowSafe,
}

/// Every glyph the client draws.
#[derive(Debug, Clone, Copy)]
pub struct Glyphs {
    pub set: Set,

    /// The mark. Neutral, so it is the same in both sets — it is the
    /// signature, and there is nothing to gain by substituting it.
    pub mark: &'static str,

    /// The state family: waiting, working, worked.
    pub waiting: &'static str,
    pub working: &'static str,
    pub worked: &'static str,

    /// A closed and an open disclosure. Both Neutral.
    pub closed: &'static str,
    pub open: &'static str,

    /// A step in a breadcrumb. Neutral.
    pub crumb: &'static str,

    /// Between two pieces of meta on one line.
    pub between: &'static str,
    /// A relation, pointing.
    pub toward: &'static str,

    /// The four directions, as they appear on a keycap.
    pub up: &'static str,
    pub down: &'static str,
    pub left: &'static str,
    pub right: &'static str,

    /// Keycaps that are characters. Both Neutral.
    pub enter: &'static str,
    pub backspace: &'static str,

    /// The hairline that holds an indent. Never a glyph tree.
    pub hairline: &'static str,
}

impl Glyphs {
    #[must_use]
    pub const fn of(set: Set) -> Self {
        match set {
            Set::Signature => Self::signature(),
            Set::NarrowSafe => Self::narrow_safe(),
        }
    }

    /// The design's own.
    #[must_use]
    pub const fn signature() -> Self {
        Self {
            set: Set::Signature,
            mark: "✦",
            waiting: "◇",
            working: "◈",
            worked: "◆",
            closed: "▸",
            open: "▾",
            crumb: "›",
            between: "·",
            toward: "→",
            up: "↑",
            down: "↓",
            left: "←",
            right: "→",
            enter: "↵",
            backspace: "⌫",
            hairline: "▏",
        }
    }

    /// For a terminal that renders Ambiguous characters wide.
    ///
    /// The state family stays a family — three marks that escalate — and every
    /// one of them is a character every terminal agrees is one cell. It loses
    /// something; a column that drifts loses more, and the state is written out
    /// in words beside the glyph on every screen that shows one.
    #[must_use]
    pub const fn narrow_safe() -> Self {
        Self {
            set: Set::NarrowSafe,
            // Neutral in both sets. Substituting the signature would be giving
            // up the thing this design is for, over a hazard it does not have.
            mark: "✦",
            waiting: "-",
            working: ">",
            worked: "#",
            closed: "▸",
            open: "▾",
            crumb: "›",
            between: ".",
            toward: ">",
            up: "^",
            down: "v",
            left: "<",
            right: ">",
            enter: "↵",
            backspace: "⌫",
            hairline: "|",
        }
    }

    /// The glyph for one guidance state.
    #[must_use]
    pub const fn state(&self, state: State) -> &'static str {
        match state {
            State::Waiting => self.waiting,
            State::Working => self.working,
            State::Worked => self.worked,
        }
    }

    /// Every glyph in this set, for the check that they fit their cells.
    #[must_use]
    pub const fn all(&self) -> [&'static str; 16] {
        [
            self.mark,
            self.waiting,
            self.working,
            self.worked,
            self.closed,
            self.open,
            self.crumb,
            self.between,
            self.toward,
            self.up,
            self.down,
            self.left,
            self.right,
            self.enter,
            self.backspace,
            self.hairline,
        ]
    }
}

impl Default for Glyphs {
    fn default() -> Self {
        Self::signature()
    }
}

/// The three states a piece of guidance can be in. One upward movement, and
/// the words are what the person reads; the glyph reinforces them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Waiting,
    Working,
    Worked,
}

impl State {
    /// What the person is shown in words. **Always shown**, which is what lets
    /// the narrow-safe set be plain without losing the meaning.
    #[must_use]
    pub const fn word(self) -> &'static str {
        match self {
            Self::Waiting => "waiting",
            Self::Working => "working",
            Self::Worked => "worked",
        }
    }
}

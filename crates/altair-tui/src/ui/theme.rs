//! One house style in two modes.
//!
//! **The same markup in both**, with a token swap between them. Nothing draws
//! a colour literal and nothing branches on which theme is in use: a widget
//! asks for the role it means — the thing wanting attention, an identity, a
//! rule — and gets whichever colour that role has here.
//!
//! # The colour discipline, which is narrower than a palette
//!
//! - **Amber** is only ever the one thing wanting attention.
//! - **Blue** is identity and relations, and nothing else.
//! - **Red** is only irreversible or refused.
//! - **Green** is only worked, or captured.
//!
//! A role used for a second purpose is how a palette stops meaning anything,
//! so the tokens below are named for what they say rather than for what they
//! look like.
//!
//! # What is deliberately absent
//!
//! No solarized, and nothing that reads as somebody else's editor. The
//! client's whole visual argument is that a terminal need not look like a
//! server.

use ratatui::style::Color;

/// Which mode the person is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Night. Altair is a blue-white star, so starlight blue carries identity
    /// and amber carries attention. The default.
    Starfield,
    /// The same system in daylight, on warm paper.
    Daylight,
}

/// Every colour the client is allowed to use, by what it means.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub mode: Mode,

    /// The ground everything sits on.
    pub ground: Color,
    /// Ordinary text.
    pub body: Color,
    /// Text being emphasised: a title, the selected row's own words.
    pub bright: Color,
    /// Text stepped back once.
    pub mid: Color,
    /// Text stepped back twice: meta, units, timestamps.
    pub dim: Color,
    /// Text stepped back as far as it goes.
    pub faint: Color,

    /// A section rule.
    pub rule: Color,
    /// A rule inside a section, quieter than the one that opened it.
    pub rule_faint: Color,
    /// The row the person is on.
    pub selected: Color,

    /// Identity and relations. Never anything else.
    pub identity: Color,
    /// The one thing wanting attention. Never anything else.
    pub attention: Color,
    /// Worked, or captured. Never anything else.
    pub worked: Color,
    /// Irreversible, or refused. Never anything else.
    pub refused: Color,

    /// The head of a table.
    pub table_head: Color,
    /// A keycap's fill and its border.
    pub keycap: Color,
    pub keycap_border: Color,
    /// An inset sheet: a dialog, an overlay.
    pub sheet: Color,
}

impl Theme {
    #[must_use]
    pub fn of(mode: Mode) -> Self {
        match mode {
            Mode::Starfield => Self::starfield(),
            Mode::Daylight => Self::daylight(),
        }
    }

    /// Night, and the default.
    #[must_use]
    pub const fn starfield() -> Self {
        Self {
            mode: Mode::Starfield,
            ground: Color::Rgb(0x0c, 0x0f, 0x18),
            body: Color::Rgb(0xc6, 0xcf, 0xdf),
            bright: Color::Rgb(0xf0, 0xf4, 0xfb),
            mid: Color::Rgb(0x99, 0xa3, 0xb8),
            dim: Color::Rgb(0x65, 0x6f, 0x85),
            faint: Color::Rgb(0x4d, 0x56, 0x6b),
            rule: Color::Rgb(0x1e, 0x25, 0x34),
            rule_faint: Color::Rgb(0x16, 0x1d, 0x2a),
            selected: Color::Rgb(0x16, 0x1d, 0x2c),
            identity: Color::Rgb(0x7f, 0xb2, 0xf0),
            attention: Color::Rgb(0xd9, 0xa4, 0x5f),
            worked: Color::Rgb(0x7f, 0xc0, 0xa2),
            refused: Color::Rgb(0xd9, 0x85, 0x70),
            table_head: Color::Rgb(0x14, 0x1a, 0x27),
            keycap: Color::Rgb(0x1a, 0x21, 0x30),
            keycap_border: Color::Rgb(0x26, 0x2f, 0x42),
            sheet: Color::Rgb(0x11, 0x17, 0x25),
        }
    }

    /// Daylight, on warm paper. The same markup; only these values differ.
    #[must_use]
    pub const fn daylight() -> Self {
        Self {
            mode: Mode::Daylight,
            ground: Color::Rgb(0xf4, 0xf1, 0xe9),
            body: Color::Rgb(0x3f, 0x3b, 0x33),
            bright: Color::Rgb(0x20, 0x1e, 0x19),
            mid: Color::Rgb(0x6d, 0x68, 0x5c),
            dim: Color::Rgb(0x8d, 0x88, 0x78),
            faint: Color::Rgb(0xa3, 0x9d, 0x8c),
            rule: Color::Rgb(0xdd, 0xd6, 0xc6),
            rule_faint: Color::Rgb(0xe4, 0xdd, 0xcd),
            selected: Color::Rgb(0xe8, 0xe3, 0xd5),
            identity: Color::Rgb(0x3c, 0x5f, 0x9c),
            attention: Color::Rgb(0xa8, 0x56, 0x2f),
            worked: Color::Rgb(0x4d, 0x75, 0x48),
            refused: Color::Rgb(0xa3, 0x32, 0x24),
            table_head: Color::Rgb(0xea, 0xe5, 0xd8),
            keycap: Color::Rgb(0xea, 0xe5, 0xd8),
            keycap_border: Color::Rgb(0xd3, 0xcc, 0xbc),
            sheet: Color::Rgb(0xec, 0xe7, 0xda),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::starfield()
    }
}

//! Drawing a list, once, for every screen that draws one.
//!
//! **The selected row is a fill across the whole width**, not a marker in a
//! gutter: the design holds attention with ground rather than with a character,
//! which is also why nothing here draws a cursor glyph.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

use crate::ui::chrome::Shell;

/// How wide the left column of a list is: a time, or a date.
pub const WHEN: usize = 8;
/// How wide the type column is.
pub const KIND: usize = 12;
/// How wide a guidance state's word is.
pub const STATE: usize = 9;
/// How wide the one quiet thing on the right is.
///
/// **Fixed, like every other column on a row.** These are set widths and the
/// title takes what is left, so the right-hand columns line up down the screen.
/// Sizing them from their own contents — which the first version of this did —
/// gives every row a different set of column positions and turns a list into a
/// staircase.
pub const NOTE: usize = 14;

/// A cursor down the rows of a screen.
pub struct Rows<'a> {
    pub shell: &'a Shell,
    pub area: Rect,
    pub buffer: &'a mut Buffer,
    y: u16,
}

impl<'a> Rows<'a> {
    pub fn new(shell: &'a Shell, area: Rect, buffer: &'a mut Buffer) -> Self {
        let y = area.top();
        Self {
            shell,
            area,
            buffer,
            y,
        }
    }

    #[must_use]
    pub fn room(&self) -> bool {
        self.y < self.area.bottom()
    }

    /// The line the next thing will be drawn on.
    #[must_use]
    pub fn line(&self) -> Rect {
        Rect {
            y: self.y,
            height: 1,
            ..self.area
        }
    }

    pub fn skip(&mut self) {
        self.y += 1;
    }

    /// A section rule, then move on.
    pub fn section(&mut self, label: &str, quiet: bool) {
        if !self.room() {
            return;
        }
        if quiet {
            self.shell.subsection(label, self.line(), self.buffer);
        } else {
            self.shell.section(label, self.line(), self.buffer);
        }
        self.y += 1;
    }

    /// Fill this line with `ground`, so a selected row reads as a band rather
    /// than as coloured text.
    fn band(&mut self, ground: Color) {
        let line = self.line();
        for x in line.left()..line.right() {
            if let Some(cell) = self.buffer.cell_mut((x, line.y)) {
                cell.set_char(' ').set_bg(ground);
            }
        }
    }

    /// Draw one line of spans over a band.
    pub fn draw(&mut self, ground: Color, spans: Vec<Span<'static>>) {
        if !self.room() {
            return;
        }
        self.band(ground);
        let line = self.line();
        Line::from(spans).render(line, self.buffer);
        self.y += 1;
    }
}

/// Cut a string to fit, without pretending it was not cut.
///
/// The marker is three full stops rather than an ellipsis: `…` is
/// East-Asian-Width Ambiguous, and a column that ends in a character terminals
/// disagree about is the one place truncation could make the drift it exists
/// to prevent.
#[must_use]
pub fn clip(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        return text.to_string();
    }
    if width <= 3 {
        return text.chars().take(width).collect();
    }
    let mut out: String = text.chars().take(width - 3).collect();
    out.push_str("...");
    out
}

/// Pad to exactly `width`, cutting where it must.
///
/// **One column of the width is a gutter and is never written into.** A title
/// that filled its column exactly would touch the one after it, and "the
/// shroud lip is 3mm p...note" is a row nobody can read — the truncation
/// marker and the next column's first word become one string.
#[must_use]
pub fn cell(text: &str, width: usize) -> String {
    let room = width.saturating_sub(1);
    let clipped = clip(text, room);
    let short = width.saturating_sub(clipped.chars().count());
    format!("{clipped}{}", " ".repeat(short))
}

/// The three inks a row is drawn in, given what it is.
#[must_use]
pub fn inks(shell: &Shell, selected: bool, past: bool) -> (Color, Color, Color) {
    let t = &shell.theme;
    if past {
        (t.faint, t.mid, t.faint)
    } else if selected {
        (t.mid, t.bright, t.mid)
    } else {
        (t.dim, t.body, t.dim)
    }
}

/// Bold, but only where the person is.
#[must_use]
pub fn weight(selected: bool) -> Modifier {
    if selected {
        Modifier::BOLD
    } else {
        Modifier::empty()
    }
}

//! Create and update: the floor, the deliberate version of it, an amount, and
//! the handoff to the person's own editor.
//!
//! **Every one of these is finished the moment the store commits.** Nothing
//! here waits on a network, nothing here can be refused for a reason that is
//! not local, and nothing here shows the person a spinner: acceptance is the
//! answer to the act, not to a round trip.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Span;

use super::rows::{Rows, cell};
use crate::ui::chrome::Shell;

/// The floor: an identity, a type, a timestamp, and whatever the person typed.
///
/// **No field is required on this path.** The screen does not ask for a title,
/// does not ask where it goes, and cannot refuse for the want of either. That
/// is the whole point of it: the cheapest thing the client can do is the thing
/// it must never make expensive.
pub fn capture(shell: &Shell, typed: &str, area: Rect, buffer: &mut Buffer) {
    let t = shell.theme;
    let mut list = Rows::new(shell, area, buffer);
    list.section("capture", false);
    list.skip();
    list.draw(
        t.ground,
        vec![
            Span::styled("> ", Style::new().fg(t.identity)),
            Span::styled(typed.to_string(), Style::new().fg(t.bright)),
            // A block cursor, drawn as ground on ink, which is the one place a
            // terminal's own inversion is the right answer.
            Span::styled(" ", Style::new().bg(t.body)),
        ],
    );
    list.skip();
    list.draw(
        t.ground,
        vec![Span::styled(
            "it is yours as soon as you press return; nothing else is needed".to_string(),
            Style::new().fg(t.dim),
        )],
    );
}

/// One labelled field on a deliberate form.
pub struct Field {
    pub about: &'static str,
    pub value: String,
    /// Whether the person is in this one.
    pub here: bool,
    /// What this field means when it is left empty. **Every field on a
    /// deliberate form says this**, because a form the person cannot skip is a
    /// form that turns a capture into a chore.
    pub if_empty: &'static str,
}

/// The deliberate version: the same act, with the things a person sometimes
/// wants to say at the same time.
///
/// **Nothing on it is required.** Each field says what it means to leave it
/// alone, so skipping is a choice the screen makes visible rather than a risk.
pub fn deliberate(shell: &Shell, what: &str, fields: &[Field], area: Rect, buffer: &mut Buffer) {
    let t = shell.theme;
    let mut list = Rows::new(shell, area, buffer);
    list.section(what, false);
    list.skip();
    for field in fields {
        let ink = if field.here { t.bright } else { t.body };
        list.draw(
            if field.here { t.selected } else { t.ground },
            vec![
                Span::styled(cell(field.about, 16), Style::new().fg(t.dim)),
                Span::styled(
                    if field.value.is_empty() {
                        field.if_empty.to_string()
                    } else {
                        field.value.clone()
                    },
                    Style::new()
                        .fg(if field.value.is_empty() { t.faint } else { ink })
                        .add_modifier(if field.here {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        }),
                ),
            ],
        );
    }
}

/// Saying what is actually there.
///
/// **An assertion replaces, and never adjusts.** The person says what is in
/// the cupboard; nothing is inferred from what was taken, and the difference
/// between what they said and what is committed elsewhere is shown rather than
/// quietly reconciled — negative availability is information about a household,
/// not an error to be clamped away.
pub fn assert_amount(
    shell: &Shell,
    what: &str,
    was: &str,
    typed: &str,
    committed: &str,
    area: Rect,
    buffer: &mut Buffer,
) {
    let t = shell.theme;
    let mut list = Rows::new(shell, area, buffer);
    list.section("what is actually there", false);
    list.skip();
    list.draw(
        t.ground,
        vec![Span::styled(
            what.to_string(),
            Style::new().fg(t.bright).add_modifier(Modifier::BOLD),
        )],
    );
    list.skip();
    list.draw(
        t.ground,
        vec![
            Span::styled(cell("you last said", 16), Style::new().fg(t.dim)),
            Span::styled(was.to_string(), Style::new().fg(t.mid)),
        ],
    );
    list.draw(
        t.selected,
        vec![
            Span::styled(cell("now", 16), Style::new().fg(t.dim)),
            Span::styled(typed.to_string(), Style::new().fg(t.bright)),
            Span::styled(" ", Style::new().bg(t.body)),
        ],
    );
    if !committed.is_empty() {
        list.skip();
        list.draw(
            t.ground,
            vec![
                Span::styled(cell("committed", 16), Style::new().fg(t.dim)),
                Span::styled(committed.to_string(), Style::new().fg(t.identity)),
            ],
        );
    }
}

/// Composing, which happens somewhere else.
///
/// **The client is not on screen while this is true**, which is why there is a
/// screen for it at all: the person needs to know where their words went and
/// that they are safe if the editor dies.
pub fn composing(shell: &Shell, editor: &str, file: &str, area: Rect, buffer: &mut Buffer) {
    let t = shell.theme;
    let mut list = Rows::new(shell, area, buffer);
    list.section("writing", false);
    list.skip();
    list.draw(
        t.ground,
        vec![
            Span::styled(cell("in", 10), Style::new().fg(t.dim)),
            Span::styled(editor.to_string(), Style::new().fg(t.bright)),
        ],
    );
    list.draw(
        t.ground,
        vec![
            Span::styled(cell("the file", 10), Style::new().fg(t.dim)),
            Span::styled(file.to_string(), Style::new().fg(t.identity)),
        ],
    );
    list.skip();
    list.draw(
        t.ground,
        vec![Span::styled(
            "the file is ours and stays on disk; if anything dies in there it \
             is offered back next time"
                .to_string(),
            Style::new().fg(t.mid),
        )],
    );
}

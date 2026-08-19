//! When it goes sideways.
//!
//! # The rule the whole group exists to keep
//!
//! **A wait is silent and a fault signals.** An instance nobody can reach and a
//! session that has run out are the same wait, and neither of them has a screen
//! here — not because one was left out, but because there is nothing to say.
//! What is here is the small set of things that will not clear by the ordinary
//! path continuing to run.
//!
//! And **exactly one number exists anywhere in this client**: how many the
//! instance refused. No depth, no badge, and nothing that counts what happened
//! while the person was away.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Span;

use super::rows::{Rows, cell};
use crate::ui::chrome::Shell;
use crate::ui::view::{Retained, Row};

/// What the instance would not take.
///
/// **The count is the one number this client states**, and it is a fact about
/// what is held rather than about what is waiting. Nothing was dropped: every
/// row here is still the person's, and still exactly where they left it.
pub fn faults(shell: &Shell, refused: &[Row], area: Rect, buffer: &mut Buffer) {
    let t = shell.theme;
    let mut list = Rows::new(shell, area, buffer);
    let count = refused.len();
    list.draw(
        t.ground,
        vec![Span::styled(
            if count == 1 {
                "the instance would not take one capture".to_string()
            } else {
                format!("the instance would not take {count} captures")
            },
            Style::new().fg(t.refused).add_modifier(Modifier::BOLD),
        )],
    );
    list.draw(
        t.ground,
        vec![Span::styled(
            "they are still here, exactly as you left them, and nothing is \
             being retried on its own"
                .to_string(),
            Style::new().fg(t.mid),
        )],
    );
    list.skip();
    list.section("what it would not take", false);
    for row in refused {
        list.draw(
            t.ground,
            vec![
                Span::styled(cell(&row.when, 10), Style::new().fg(t.dim)),
                Span::styled(
                    cell(&row.what, usize::from(area.width).saturating_sub(24)),
                    Style::new().fg(t.body),
                ),
                // Every refusal reads the same, whatever the instance said in
                // its detail. Nothing distinguishes something absent from
                // something this member cannot see, so nothing here tries.
                Span::styled("not available".to_string(), Style::new().fg(t.refused)),
            ],
        );
    }
}

/// Removing, which can be undone.
///
/// **The ordinary affordance, and the screen says so.** Everything named in one
/// removal is one act, and that grouping is kept, so restoring any one of them
/// can bring back the rest.
pub fn confirm_removal(
    shell: &Shell,
    what: &str,
    alongside: &[&str],
    area: Rect,
    buffer: &mut Buffer,
) {
    let t = shell.theme;
    let mut list = Rows::new(shell, area, buffer);
    list.draw(
        t.ground,
        vec![
            Span::styled("remove ".to_string(), Style::new().fg(t.body)),
            Span::styled(
                what.to_string(),
                Style::new().fg(t.bright).add_modifier(Modifier::BOLD),
            ),
        ],
    );
    if !alongside.is_empty() {
        list.skip();
        list.draw(
            t.ground,
            vec![Span::styled(
                "and what it holds, as one act:".to_string(),
                Style::new().fg(t.mid),
            )],
        );
        for held in alongside {
            list.draw(
                t.ground,
                vec![
                    Span::raw("  "),
                    Span::styled((*held).to_string(), Style::new().fg(t.body)),
                ],
            );
        }
    }
    list.skip();
    list.draw(
        t.ground,
        vec![Span::styled(
            "you can bring this back, and bringing back any of it can bring \
             back the rest"
                .to_string(),
            // Amber, not red: this wants an answer and it is not irreversible.
            Style::new().fg(t.attention),
        )],
    );
}

/// Erasing, which cannot.
///
/// **Red, a different word, and the difference said out loud.** The one thing
/// on this screen that matters is that the person understands which of the two
/// acts they are performing.
pub fn confirm_erasure(shell: &Shell, what: &str, area: Rect, buffer: &mut Buffer) {
    let t = shell.theme;
    let mut list = Rows::new(shell, area, buffer);
    list.draw(
        t.ground,
        vec![
            Span::styled("erase ".to_string(), Style::new().fg(t.refused)),
            Span::styled(
                what.to_string(),
                Style::new().fg(t.bright).add_modifier(Modifier::BOLD),
            ),
        ],
    );
    list.skip();
    list.draw(
        t.ground,
        vec![Span::styled(
            "for everyone, and for good. this is not removing: nothing brings \
             it back, and nothing keeps a copy."
                .to_string(),
            Style::new().fg(t.refused),
        )],
    );
    list.skip();
    list.draw(
        t.ground,
        vec![Span::styled(
            "if you only want it out of your way, remove it instead".to_string(),
            Style::new().fg(t.mid),
        )],
    );
}

/// Two people wrote the same part, and both values were kept.
///
/// **Not a failure and not a question.** The write applied, nothing was sent
/// back, and the entity is readable, editable and relatable throughout. What
/// this screen offers is a look, not a resolution the person must perform
/// before they can carry on.
pub fn conflict(shell: &Shell, what: &str, parts: &[Retained], area: Rect, buffer: &mut Buffer) {
    let t = shell.theme;
    let mut list = Rows::new(shell, area, buffer);
    list.draw(
        t.ground,
        vec![Span::styled(
            what.to_string(),
            Style::new().fg(t.bright).add_modifier(Modifier::BOLD),
        )],
    );
    list.draw(
        t.ground,
        vec![Span::styled(
            "you and somebody else wrote this at the same time. both are kept.".to_string(),
            Style::new().fg(t.mid),
        )],
    );
    list.skip();
    for part in parts {
        list.section(&part.part, true);
        list.draw(
            t.ground,
            vec![
                Span::styled(cell("yours", 12), Style::new().fg(t.dim)),
                Span::styled(part.mine.clone(), Style::new().fg(t.body)),
            ],
        );
        list.draw(
            t.ground,
            vec![
                Span::styled(cell(&part.theirs_member, 12), Style::new().fg(t.dim)),
                Span::styled(part.theirs.clone(), Style::new().fg(t.body)),
            ],
        );
        list.skip();
    }
}

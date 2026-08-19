//! Browse and find: the four ways in, and the one place everything meets.
//!
//! **Only one of these asks the instance anything.** It serves no way to read
//! an entity by identity and no way to list what a container holds, so the
//! captures list, the ladder, the location tree and a detail are all drawn from
//! what the change stream delivered with this device's unsent writes over it.
//! [`search`] is the exception, and what it asks is the literal arm.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};

use super::rows::{KIND, NOTE, Rows, STATE, WHEN, cell, inks, weight};
use crate::ui::chrome::Shell;
use crate::ui::glyphs::State;
use crate::ui::view::{Answered, Group, Kept, Link, Row, Rung};

/// How wide the column saying where a relation was formed is.
const ANCHOR: usize = 30;

/// What the person touched, most recent first, in days.
///
/// **The right-hand column is what is coming, not what is queued.** Nothing on
/// this screen counts what has not reached the instance; there is no number
/// here that rises while the person is away.
pub fn captures(shell: &Shell, groups: &[Group], coming: &[Row], area: Rect, buffer: &mut Buffer) {
    let split = area.width.saturating_sub(38).max(area.width / 2);
    let left = Rect {
        width: split,
        ..area
    };
    let right = Rect {
        x: area.x + split + 2,
        width: area.width.saturating_sub(split + 2),
        ..area
    };

    let mut list = Rows::new(shell, left, buffer);
    for group in groups {
        list.section(&group.label, group.quiet);
        for row in &group.rows {
            row_of(&mut list, row);
        }
        list.skip();
    }

    let mut side = Rows::new(shell, right, buffer);
    side.section("coming forward", false);
    for row in coming {
        let (when, what, _) = inks(shell, row.selected, row.past);
        // Amber here and only here: the thing that wants attention is what is
        // due, and nothing else on this screen competes with it.
        let when_ink = if row.note == "start" {
            shell.theme.attention
        } else {
            when
        };
        side.draw(
            shell.theme.ground,
            vec![
                Span::styled(cell(&row.when, 9), Style::new().fg(when_ink)),
                Span::styled(row.what.clone(), Style::new().fg(what)),
            ],
        );
        side.draw(
            shell.theme.ground,
            vec![
                Span::raw(" ".repeat(9)),
                Span::styled(row.kind.clone(), Style::new().fg(shell.theme.faint)),
            ],
        );
    }
}

fn row_of(list: &mut Rows<'_>, row: &Row) {
    let shell = list.shell;
    let t = shell.theme;
    let ground = if row.selected { t.selected } else { t.ground };
    let (when, what, kind) = inks(shell, row.selected, row.past);
    let used = 2 + WHEN + KIND + NOTE;
    let room = usize::from(list.area.width).saturating_sub(used);
    list.draw(
        ground,
        vec![
            Span::styled(
                cell(
                    if row.selected {
                        shell.glyphs.closed
                    } else {
                        ""
                    },
                    2,
                ),
                Style::new().fg(t.identity),
            ),
            Span::styled(cell(&row.when, WHEN), Style::new().fg(when)),
            Span::styled(
                cell(&row.what, room),
                Style::new().fg(what).add_modifier(weight(row.selected)),
            ),
            Span::styled(cell(&row.kind, KIND), Style::new().fg(kind)),
            Span::styled(cell(&row.note, NOTE), Style::new().fg(kind)),
        ],
    );
}

/// The guidance ladder: campaign, arc, quest, held apart by hairlines.
///
/// **Never a glyph tree.** The indent is a rule, which is the design's whole
/// answer to nesting, and the state is a word beside a mark rather than a mark
/// alone.
pub fn ladder(shell: &Shell, rungs: &[Rung], area: Rect, buffer: &mut Buffer) {
    let t = shell.theme;
    let mut list = Rows::new(shell, area, buffer);
    for rung in rungs {
        let ground = if rung.selected { t.selected } else { t.ground };
        let (_, what, kind) = inks(shell, rung.selected, rung.state == State::Worked);
        let mark = match rung.state {
            State::Waiting => t.faint,
            State::Working => t.attention,
            State::Worked => t.worked,
        };
        let mut spans = list.shell.indent(rung.depth).spans;
        let indent: usize = spans.iter().map(|s| s.content.chars().count()).sum();
        spans.push(Span::styled(
            cell(shell.glyphs.state(rung.state), 2),
            Style::new().fg(mark),
        ));
        let used = indent + 2 + KIND + STATE + NOTE;
        let room = usize::from(area.width).saturating_sub(used);
        spans.push(Span::styled(
            cell(&rung.what, room),
            Style::new().fg(what).add_modifier(weight(rung.selected)),
        ));
        spans.push(Span::styled(cell(&rung.kind, KIND), Style::new().fg(kind)));
        spans.push(Span::styled(
            cell(rung.state.word(), STATE),
            Style::new().fg(mark),
        ));
        spans.push(Span::styled(
            cell(&rung.note, NOTE),
            // Red only where it is genuinely not the person's to clear from
            // here. Everything else on this line is quiet.
            Style::new().fg(if rung.blocked { t.refused } else { t.dim }),
        ));
        list.draw(ground, spans);
    }
}

/// One search, and what retrieval was able to do.
///
/// **The answer says what it could not do rather than failing.** Retrieval
/// degrades per stage; a missing arm is a sentence in the person's words, not
/// an error, and not silence either.
pub fn search(
    shell: &Shell,
    query: &str,
    results: &[Row],
    answered: &Answered,
    area: Rect,
    buffer: &mut Buffer,
) {
    let t = shell.theme;
    let mut list = Rows::new(shell, area, buffer);
    list.draw(
        t.ground,
        vec![
            Span::styled("/ ", Style::new().fg(t.identity)),
            Span::styled(query.to_string(), Style::new().fg(t.bright)),
        ],
    );
    list.skip();
    list.section("what matched", false);
    for row in results {
        row_of(&mut list, row);
    }
    list.skip();
    if !answered.complete {
        // Not a fault. Nothing here will be signalled and nothing is retried:
        // it is a statement about this answer, on this answer.
        list.draw(
            t.ground,
            vec![Span::styled(
                answered.missing.clone(),
                Style::new().fg(t.mid),
            )],
        );
    }
}

/// Where things are: locations inside locations, and what is kept in them.
///
/// **An amount is what the person last said is there.** It changes by an
/// explicit act and by nothing else, and it is never computed from history.
/// What live relations have committed of it is shown beside it rather than
/// subtracted from it, because the difference is information.
pub fn tracking(shell: &Shell, kept: &[Kept], area: Rect, buffer: &mut Buffer) {
    let t = shell.theme;
    let mut list = Rows::new(shell, area, buffer);
    for thing in kept {
        let ground = if thing.selected { t.selected } else { t.ground };
        let (_, what, _) = inks(shell, thing.selected, false);
        let mut spans = list.shell.indent(thing.depth).spans;
        let indent: usize = spans.iter().map(|s| s.content.chars().count()).sum();
        let used = indent + 16 + 18;
        let room = usize::from(area.width).saturating_sub(used);
        spans.push(Span::styled(
            cell(&thing.what, room),
            Style::new().fg(what).add_modifier(weight(thing.selected)),
        ));
        spans.push(Span::styled(
            cell(&thing.amount, 16),
            Style::new().fg(t.body),
        ));
        spans.push(Span::styled(
            thing.committed.clone(),
            Style::new().fg(t.identity),
        ));
        list.draw(ground, spans);
    }
}

/// One entity, with everything joined to it.
///
/// **Relations are read from both ends.** There is one record per connection
/// and no reverse row, so what points here and what this points at are the same
/// lookup asked from different sides — and both are shown, because to a person
/// standing here they are the same kind of fact.
pub fn detail(
    shell: &Shell,
    title: &str,
    body: &[Line<'static>],
    links: &[Link],
    area: Rect,
    buffer: &mut Buffer,
) {
    let t = shell.theme;
    let mut list = Rows::new(shell, area, buffer);
    list.draw(
        t.ground,
        vec![Span::styled(
            title.to_string(),
            Style::new()
                .fg(t.bright)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )],
    );
    list.skip();
    for line in body {
        list.draw(t.ground, line.spans.clone());
    }
    list.skip();
    list.section("joined to", false);
    for link in links {
        let arrow = if link.outward {
            shell.glyphs.toward
        } else {
            shell.glyphs.crumb
        };
        let kind = if link.kind.is_empty() {
            // An untyped relation is untyped, not possibly typed, and saying so
            // is better than leaving a gap a reader fills in themselves.
            "untyped".to_string()
        } else {
            link.kind.clone()
        };
        let used = 2 + 16 + ANCHOR;
        let room = usize::from(area.width).saturating_sub(used);
        list.draw(
            t.ground,
            vec![
                Span::styled(cell(arrow, 2), Style::new().fg(t.identity)),
                Span::styled(cell(&kind, 16), Style::new().fg(t.identity)),
                Span::styled(cell(&link.what, room), Style::new().fg(t.body)),
                Span::styled(cell(&link.anchor, ANCHOR), Style::new().fg(t.faint)),
            ],
        );
    }
}

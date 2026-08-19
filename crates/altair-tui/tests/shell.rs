//! The shell, in both themes, as a snapshot.
//!
//! The mechanism, and why a snapshot here records three things rather than one,
//! is on `tests/snapshot/mod.rs`.

mod snapshot;

use altair_tui::ui::{Modal, State};
use ratatui::layout::Rect;
use snapshot::in_both_themes;

#[test]
fn the_header() {
    in_both_themes("header", 2, |shell, area, buffer| {
        shell.header(
            "guidance · ladder",
            "showing working",
            Rect { height: 1, ..area },
            buffer,
        );
        shell.rule(
            Rect {
                y: 1,
                height: 1,
                ..area
            },
            buffer,
        );
    });
}

#[test]
fn the_section_rules() {
    in_both_themes("sections", 3, |shell, area, buffer| {
        shell.section("what you touched", Rect { height: 1, ..area }, buffer);
        shell.subsection(
            "sunday 16",
            Rect {
                y: 2,
                height: 1,
                ..area
            },
            buffer,
        );
    });
}

#[test]
fn the_status_line_in_every_modal_state() {
    let modals = [
        Modal::Browsing,
        Modal::Writing,
        Modal::Capturing,
        Modal::Asserting,
        Modal::Confirming,
        Modal::Erasing,
        Modal::Faults,
        Modal::Help,
    ];
    in_both_themes("status", modals.len() as u16, |shell, area, buffer| {
        for (index, modal) in modals.iter().enumerate() {
            shell.status(
                *modal,
                &[
                    "rebuild the rack",
                    "airflow and thermals",
                    "wire the t-encoder",
                ],
                "4 of 11",
                Rect {
                    y: u16::try_from(index).expect("a short list"),
                    height: 1,
                    ..area
                },
                buffer,
            );
        }
    });
}

#[test]
fn the_keys() {
    in_both_themes("keys", 1, |shell, area, buffer| {
        shell.keys(
            &[
                ("↑↓", "move"),
                ("↵", "open where it lives"),
                ("c", "capture"),
            ],
            area,
            buffer,
        );
    });
}

#[test]
fn the_hairline_indent() {
    in_both_themes("indent", 4, |shell, area, buffer| {
        use ratatui::text::{Line, Span};
        use ratatui::widgets::Widget;
        for (depth, (state, title)) in [
            (State::Working, "Rebuild the rack"),
            (State::Working, "Airflow and thermals"),
            (State::Worked, "Bench-test the fan controller"),
            (State::Waiting, "Cut the intake shroud"),
        ]
        .iter()
        .enumerate()
        {
            let mut spans = shell.indent(depth).spans;
            spans.push(Span::styled(
                format!("{} ", shell.glyphs.state(*state)),
                ratatui::style::Style::new().fg(match state {
                    State::Waiting => shell.theme.faint,
                    State::Working => shell.theme.attention,
                    State::Worked => shell.theme.worked,
                }),
            ));
            spans.push(Span::styled(
                (*title).to_string(),
                ratatui::style::Style::new().fg(shell.theme.body),
            ));
            spans.push(Span::styled(
                format!("  {}", state.word()),
                ratatui::style::Style::new().fg(shell.theme.dim),
            ));
            Line::from(spans).render(
                Rect {
                    y: u16::try_from(depth).expect("a short list"),
                    height: 1,
                    ..area
                },
                buffer,
            );
        }
    });
}

#[test]
fn an_inset_sheet() {
    in_both_themes("sheet", 6, |shell, area, buffer| {
        shell.sheet(
            "erase for everyone",
            Rect {
                x: 6,
                width: 60,
                ..area
            },
            buffer,
        );
    });
}

#[test]
fn the_help_surface() {
    in_both_themes("help", 40, |shell, area, buffer| {
        shell.help_surface(area, buffer);
    });
}

#[test]
fn a_terminal_with_no_room_in_it_is_drawn_and_not_crashed_into() {
    // Found by running the client rather than by reading it: a pty opened
    // before anything said how big it was reported nothing by nothing, and the
    // first draw subtracted the chrome's four lines from a height of zero.
    // Every snapshot in this file passes a comfortable area, so none of them
    // could have caught it.
    use altair_tui::ui::{Frame, Glyphs, Mode, Set, Shell, Theme};
    use ratatui::buffer::Buffer;

    let shell = Shell::new(Theme::of(Mode::Starfield), Glyphs::of(Set::Signature));
    for (width, height) in [(0, 0), (1, 1), (0, 40), (40, 0), (3, 2), (8, 4), (200, 7)] {
        let area = Rect::new(0, 0, width, height);
        let mut buffer = Buffer::empty(area);
        shell.frame(
            &Frame {
                domain: "captures",
                meta: "monday",
                modal: Modal::Browsing,
                crumbs: &["captures", "something"],
                position: "1 of 2",
                keys: &[("↑↓", "move")],
            },
            area,
            &mut buffer,
            |body, buffer| shell.help_surface(body, buffer),
        );
    }
}

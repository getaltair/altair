//! Every screen, in both themes, as a snapshot.
//!
//! **A screen is a function of a view and nothing else**, so each of these
//! composes its own view and draws it. No store, no instance, no clock — which
//! is what lets thirteen screens be checked in a hundredth of a second and on
//! both platforms.
//!
//! Regenerate with `ALTAIR_UPDATE_SNAPSHOTS=1 cargo test -p altair-tui --test screens`,
//! and read the diff before staging it.

mod snapshot;

use altair_tui::ui::glyphs::State;
use altair_tui::ui::markdown;
use altair_tui::ui::screens::{browse, sideways, write};
use altair_tui::ui::view::{Answered, Group, Kept, Link, Retained, Row, Rung};
use altair_tui::ui::{Frame, Modal};
use snapshot::in_both_themes;

fn row(when: &str, what: &str, kind: &str, note: &str) -> Row {
    Row {
        when: when.into(),
        what: what.into(),
        kind: kind.into(),
        note: note.into(),
        selected: false,
        past: false,
    }
}

// ---------------------------------------------------------------------------
// Browse and find
// ---------------------------------------------------------------------------

#[test]
fn the_captures_list() {
    in_both_themes("captures", 22, |shell, area, buffer| {
        let mut today = Group {
            label: "what you touched".into(),
            quiet: false,
            rows: vec![
                row("21:38", "Fan curve, second pass", "note", "edited"),
                row("20:02", "Dupont jumpers", "item", ""),
                row("18:51", "rack-front.jpg", "file", "bridge"),
                row("14:07", "Swap the UPS battery", "quest", "new"),
            ],
        };
        today.rows[0].selected = true;
        let mut older = Group {
            label: "sunday 16".into(),
            quiet: true,
            rows: vec![
                row("23:11", "the shroud lip is 3mm proud", "note", "bridge"),
                row("15:58", "Measure idle draw", "quest", "working"),
            ],
        };
        for row in &mut older.rows {
            row.past = true;
        }
        let coming = vec![
            row("today", "Wire the T-Encoder", "start", "start"),
            row("wed 19", "Label everything", "every wed", ""),
        ];
        shell.frame(
            &Frame {
                domain: "captures",
                meta: "monday 17 august",
                modal: Modal::Browsing,
                crumbs: &["captures", "fan curve, second pass"],
                position: "",
                keys: &[
                    ("↑↓", "move"),
                    ("↵", "open where it lives"),
                    ("c", "capture"),
                    ("/", "search"),
                ],
            },
            area,
            buffer,
            |body, buffer| browse::captures(shell, &[today, older], &coming, body, buffer),
        );
    });
}

#[test]
fn the_guidance_ladder() {
    in_both_themes("ladder", 16, |shell, area, buffer| {
        let rungs = vec![
            rung(
                0,
                State::Working,
                "Rebuild the rack",
                "campaign",
                "7 quests",
                false,
                false,
            ),
            rung(
                1,
                State::Working,
                "Airflow and thermals",
                "arc",
                "",
                false,
                false,
            ),
            rung(
                2,
                State::Worked,
                "Bench-test the fan controller",
                "quest",
                "aug 12",
                false,
                false,
            ),
            rung(
                2,
                State::Working,
                "Wire the T-Encoder",
                "quest",
                "uses ×4",
                true,
                false,
            ),
            rung(
                2,
                State::Waiting,
                "Cut the intake shroud",
                "quest",
                "blocked above",
                false,
                true,
            ),
            rung(
                1,
                State::Waiting,
                "Cable management",
                "arc",
                "",
                false,
                false,
            ),
        ];
        shell.frame(
            &Frame {
                domain: "guidance · ladder",
                meta: "showing working",
                modal: Modal::Browsing,
                crumbs: &[
                    "rebuild the rack",
                    "airflow and thermals",
                    "wire the t-encoder",
                ],
                position: "4 of 11",
                keys: &[
                    ("↑↓", "move"),
                    ("←→", "fold"),
                    ("space", "next state"),
                    ("o", "new quest"),
                ],
            },
            area,
            buffer,
            |body, buffer| browse::ladder(shell, &rungs, body, buffer),
        );
    });
}

fn rung(
    depth: usize,
    state: State,
    what: &str,
    kind: &str,
    note: &str,
    selected: bool,
    blocked: bool,
) -> Rung {
    Rung {
        depth,
        state,
        what: what.into(),
        kind: kind.into(),
        note: note.into(),
        selected,
        blocked,
    }
}

#[test]
fn a_search() {
    in_both_themes("search", 16, |shell, area, buffer| {
        let results = vec![
            row("aug 17", "Fan curve, second pass", "note", ""),
            row("aug 12", "Bench-test the fan controller", "quest", "worked"),
            row("aug 09", "Shroud measurements", "note", ""),
        ];
        // Wave 5 has not happened, so there is no semantic arm to have run.
        // The answer says which arms it used rather than pretending it used
        // all of them.
        let answered = Answered {
            complete: false,
            missing: "matched on words only; nothing here searched by meaning".into(),
        };
        shell.frame(
            &Frame {
                domain: "search",
                meta: "",
                modal: Modal::Browsing,
                crumbs: &["search"],
                position: "3 found",
                keys: &[
                    ("↑↓", "move"),
                    ("↵", "open where it lives"),
                    ("esc", "back out"),
                ],
            },
            area,
            buffer,
            |body, buffer| browse::search(shell, "fan curve", &results, &answered, body, buffer),
        );
    });
}

#[test]
fn the_location_tree() {
    in_both_themes("tracking", 16, |shell, area, buffer| {
        let kept = vec![
            kept(0, "Workshop", "", "", false),
            kept(1, "Intake shelf", "", "", false),
            kept(2, "M3×8 socket cap", "a handful", "", true),
            kept(2, "Dupont jumpers", "18", "4 committed", false),
            kept(1, "Parts bin", "", "", false),
            kept(2, "Intake filter foam", "2 sheets", "", false),
        ];
        shell.frame(
            &Frame {
                domain: "tracking",
                meta: "workshop",
                modal: Modal::Browsing,
                crumbs: &["workshop", "intake shelf", "m3×8 socket cap"],
                position: "",
                keys: &[("↑↓", "move"), ("a", "say what is there"), ("↵", "detail")],
            },
            area,
            buffer,
            |body, buffer| browse::tracking(shell, &kept, body, buffer),
        );
    });
}

fn kept(depth: usize, what: &str, amount: &str, committed: &str, selected: bool) -> Kept {
    Kept {
        depth,
        what: what.into(),
        amount: amount.into(),
        committed: committed.into(),
        selected,
    }
}

#[test]
fn an_entity_detail() {
    in_both_themes("detail", 18, |shell, area, buffer| {
        let links = vec![
            Link {
                kind: "uses".into(),
                outward: true,
                what: "M3×8 socket cap ×4".into(),
                anchor: "at “four of the short ones”".into(),
            },
            Link {
                kind: String::new(),
                outward: false,
                what: "Fan curve, second pass".into(),
                anchor: String::new(),
            },
        ];
        shell.frame(
            &Frame {
                domain: "guidance · quest",
                meta: "created aug 14",
                modal: Modal::Browsing,
                crumbs: &["rebuild the rack", "wire the t-encoder"],
                position: "",
                keys: &[
                    ("e", "edit in your editor"),
                    ("r", "join to something"),
                    ("esc", "back out"),
                ],
            },
            area,
            buffer,
            |body, buffer| {
                // Drawn through the markdown renderer, as the client draws it.
                let prose = markdown::render(
                    shell,
                    "The header is keyed the other way round from the diagram, so the \
                     ribbon has to come off the **back** of the board.",
                    &[],
                    body.width,
                );
                browse::detail(
                    shell,
                    "Wire the T-Encoder to the header",
                    &prose,
                    &links,
                    body,
                    buffer,
                );
            },
        );
    });
}

// ---------------------------------------------------------------------------
// Create and update
// ---------------------------------------------------------------------------

#[test]
fn a_quick_capture() {
    in_both_themes("capture", 12, |shell, area, buffer| {
        shell.frame(
            &Frame {
                domain: "captures",
                meta: "",
                modal: Modal::Capturing,
                crumbs: &["captures"],
                position: "",
                keys: &[("↵", "keep it"), ("esc", "never mind")],
            },
            area,
            buffer,
            |body, buffer| {
                write::capture(shell, "check whether 30ppi is too coarse", body, buffer);
            },
        );
    });
}

#[test]
fn a_deliberate_create() {
    in_both_themes("deliberate", 14, |shell, area, buffer| {
        let fields = vec![
            write::Field {
                about: "what",
                value: "Intake filter foam".into(),
                here: false,
                if_empty: "nothing, which is fine",
            },
            write::Field {
                about: "how much",
                value: "2 sheets".into(),
                here: true,
                if_empty: "you can say later",
            },
            write::Field {
                about: "where",
                value: String::new(),
                here: false,
                if_empty: "nowhere in particular",
            },
        ];
        shell.frame(
            &Frame {
                domain: "tracking · new item",
                meta: "",
                modal: Modal::Capturing,
                crumbs: &["workshop", "new item"],
                position: "",
                keys: &[("tab", "next"), ("↵", "keep it"), ("esc", "never mind")],
            },
            area,
            buffer,
            |body, buffer| write::deliberate(shell, "a new item", &fields, body, buffer),
        );
    });
}

#[test]
fn asserting_an_amount() {
    in_both_themes("assert", 14, |shell, area, buffer| {
        shell.frame(
            &Frame {
                domain: "tracking",
                meta: "",
                modal: Modal::Asserting,
                crumbs: &["workshop", "intake shelf", "dupont jumpers"],
                position: "",
                keys: &[("↵", "that is what is there"), ("esc", "never mind")],
            },
            area,
            buffer,
            |body, buffer| {
                write::assert_amount(
                    shell,
                    "Dupont jumpers",
                    "18",
                    "14",
                    "4 committed",
                    body,
                    buffer,
                );
            },
        );
    });
}

#[test]
fn composing_somewhere_else() {
    in_both_themes("composing", 12, |shell, area, buffer| {
        shell.frame(
            &Frame {
                domain: "knowledge · note",
                meta: "",
                modal: Modal::Writing,
                crumbs: &["fan curve, second pass"],
                position: "",
                keys: &[("", "waiting for your editor")],
            },
            area,
            buffer,
            |body, buffer| {
                write::composing(
                    shell,
                    "nvim",
                    "~/.local/state/altair/compositions/…md",
                    body,
                    buffer,
                );
            },
        );
    });
}

// ---------------------------------------------------------------------------
// When it goes sideways
// ---------------------------------------------------------------------------

#[test]
fn the_faults() {
    in_both_themes("faults", 14, |shell, area, buffer| {
        let refused = vec![
            row("aug 17", "Swap the UPS battery", "", ""),
            row("aug 16", "Blanking panels, 4U", "", ""),
            row("aug 16", "psu-whine.m4a", "", ""),
        ];
        shell.frame(
            &Frame {
                domain: "faults",
                meta: "",
                modal: Modal::Faults,
                crumbs: &["faults"],
                position: "",
                keys: &[("↵", "look at it"), ("esc", "back out")],
            },
            area,
            buffer,
            |body, buffer| sideways::faults(shell, &refused, body, buffer),
        );
    });
}

#[test]
fn confirming_a_removal() {
    in_both_themes("removal", 14, |shell, area, buffer| {
        shell.frame(
            &Frame {
                domain: "guidance",
                meta: "",
                modal: Modal::Confirming,
                crumbs: &["rebuild the rack", "airflow and thermals"],
                position: "",
                keys: &[("↵", "remove it"), ("esc", "never mind")],
            },
            area,
            buffer,
            |body, buffer| {
                sideways::confirm_removal(
                    shell,
                    "Airflow and thermals",
                    &[
                        "Bench-test the fan controller",
                        "Wire the T-Encoder",
                        "Cut the intake shroud",
                    ],
                    body,
                    buffer,
                );
            },
        );
    });
}

#[test]
fn confirming_an_erasure() {
    in_both_themes("erasure", 14, |shell, area, buffer| {
        shell.frame(
            &Frame {
                domain: "guidance",
                meta: "",
                modal: Modal::Erasing,
                crumbs: &["rebuild the rack", "airflow and thermals"],
                position: "",
                keys: &[("type erase", "to mean it"), ("esc", "never mind")],
            },
            area,
            buffer,
            |body, buffer| sideways::confirm_erasure(shell, "Airflow and thermals", body, buffer),
        );
    });
}

#[test]
fn a_retained_conflict() {
    // Rendered from a fixture. Reaching one for real needs two writers, and the
    // second client is Wave 6 — so this is the screen, checked, without a claim
    // that the path to it has been walked.
    in_both_themes("conflict", 16, |shell, area, buffer| {
        let parts = vec![Retained {
            part: "title".into(),
            mine: "Wire the T-Encoder to the header".into(),
            theirs: "Wire the T-Encoder (header is keyed backwards)".into(),
            theirs_member: "sam".into(),
        }];
        shell.frame(
            &Frame {
                domain: "guidance · quest",
                meta: "",
                modal: Modal::Browsing,
                crumbs: &["rebuild the rack", "wire the t-encoder"],
                position: "",
                keys: &[("↵", "keep reading"), ("e", "write it the way you want it")],
            },
            area,
            buffer,
            |body, buffer| {
                sideways::conflict(
                    shell,
                    "Wire the T-Encoder to the header",
                    &parts,
                    body,
                    buffer,
                );
            },
        );
    });
}

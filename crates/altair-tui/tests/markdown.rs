//! A note's body, drawn.
//!
//! The prose here is deliberately awkward: a heading, a wrapped paragraph with
//! a relation formed inside it, emphasis that straddles a line break, a nested
//! list, a task list, a fence, a quote, a table and a word too long for the
//! column. Each of those is somewhere a renderer quietly gets it wrong.

mod snapshot;

use altair_proto::v1;
use altair_tui::ui::markdown;
use snapshot::in_both_themes_wide;

const BODY: &str = "\
# Fan curve, second pass

The first curve was tuned against an empty rack, which is why it ramps late. \
With the shelves loaded the intake reads 6°C warmer at idle, so the whole \
curve moves left rather than getting *steeper*.

Measured at the top intake sensor, 40 min after boot, using **the short \
loom** and a [borrowed meter](https://example.test/meter).

> The lip is 3mm proud of the shroud. Everything downstream of this assumes
> it is flush, which it is not.

| rpm | intake | exhaust |
| --- | --- | --- |
| 600 | 31 | 38 |
| 1200 | 27 | 34 |

What is left:

- Re-cut the shroud
  - measure twice, the acrylic is expensive
  - deburr the edge
- [ ] order more filter foam
- [x] bench-test the controller

```
fan_curve = [(600, 31), (1200, 27)]
# no highlighting here, on purpose
```

An unbreakable_identifier_that_is_far_too_long_for_any_sensible_column_width ends it.
";

/// The blocks the instance would have sent back, and a relation formed at a
/// phrase inside one of them.
fn formed() -> Vec<std::ops::Range<usize>> {
    // Division is the instance's, so this stands in for it: one block per
    // top-level construct, concatenating to the body exactly.
    let block_id = vec![1_u8; 16];
    let paragraph = BODY
        .find("Measured at")
        .expect("the paragraph is in the body");
    let end = BODY[paragraph..]
        .find("\n\n")
        .map_or(BODY.len(), |at| paragraph + at);
    let blocks = vec![v1::Block {
        block_id: block_id.clone(),
        text: BODY[paragraph..end].to_string(),
    }];
    // The blocks a client holds start at the body's beginning; this one does
    // not, so shift the anchor by where it really sits.
    let relations = vec![v1::Relation {
        relation_id: vec![2_u8; 16],
        content: Some(v1::RelationContent {
            anchor: Some(v1::Anchor {
                entity_id: vec![3_u8; 16],
                block_id,
                phrase: "top intake sensor".to_string(),
            }),
            ..Default::default()
        }),
        ..Default::default()
    }];
    markdown::formed_at(&BODY[paragraph..end], &blocks, &relations)
        .into_iter()
        .map(|range| range.start + paragraph..range.end + paragraph)
        .collect()
}

#[test]
fn a_note_body() {
    let formed = formed();
    in_both_themes_wide("body", 76, 44, |shell, area, buffer| {
        let lines = markdown::render(shell, BODY, &formed, area.width);
        for (index, line) in lines.iter().enumerate() {
            let y = area.top() + u16::try_from(index).unwrap_or(u16::MAX);
            if y >= area.bottom() {
                break;
            }
            ratatui::widgets::Widget::render(
                line.clone(),
                ratatui::layout::Rect {
                    y,
                    height: 1,
                    ..area
                },
                buffer,
            );
        }
    });
}

#[test]
fn a_relation_is_the_only_blue_in_the_prose() {
    use altair_tui::ui::{Glyphs, Mode, Set, Shell, Theme};
    let shell = Shell::new(Theme::of(Mode::Starfield), Glyphs::of(Set::Signature));
    let lines = markdown::render(&shell, BODY, &formed(), 76);

    // Joined, because wrapping splits a run into words and a relation's
    // phrase is allowed to straddle a line break — what matters is which text
    // carries the colour, not how many spans it took.
    let blue: String = lines
        .iter()
        .flat_map(|line| &line.spans)
        .filter(|span| span.style.fg == Some(shell.theme.identity))
        .map(|span| span.content.as_ref())
        .collect();
    assert_eq!(
        blue, "top intake sensor",
        "blue means identity and relations and nothing else. The body contains \
         a markdown link, and if that were drawn blue then seeing blue in \
         prose would stop meaning the person formed a relation there."
    );
}

#[test]
fn the_link_is_still_marked_just_not_in_that_colour() {
    use altair_tui::ui::{Glyphs, Mode, Set, Shell, Theme};
    use ratatui::style::Modifier;
    let shell = Shell::new(Theme::of(Mode::Starfield), Glyphs::of(Set::Signature));
    let lines = markdown::render(&shell, BODY, &formed(), 76);
    let underlined: Vec<String> = lines
        .iter()
        .flat_map(|line| &line.spans)
        .filter(|span| {
            span.style.add_modifier.contains(Modifier::UNDERLINED)
                && span.style.fg != Some(shell.theme.identity)
        })
        .map(|span| span.content.to_string())
        .collect();
    assert!(
        underlined.iter().any(|text| text.contains("borrowed")),
        "the link is drawn as a link, just not as a relation: {underlined:?}"
    );
}

#[test]
fn nothing_runs_past_the_width_it_was_given() {
    use altair_tui::ui::{Glyphs, Mode, Set, Shell, Theme};
    use unicode_width::UnicodeWidthStr;
    let shell = Shell::new(Theme::of(Mode::Starfield), Glyphs::of(Set::Signature));
    for width in [24_u16, 40, 76, 120] {
        let lines = markdown::render(&shell, BODY, &formed(), width);
        for line in &lines {
            let drawn: usize = line.spans.iter().map(|span| span.content.width()).sum();
            assert!(
                drawn <= usize::from(width),
                "a line is {drawn} wide in a {width} column body. A line that \
                 runs off the edge takes whatever is drawn after it too:\n{}",
                line.spans
                    .iter()
                    .map(|s| s.content.as_ref())
                    .collect::<String>()
            );
        }
    }
}

#[test]
fn an_empty_body_draws_nothing() {
    use altair_tui::ui::{Glyphs, Mode, Set, Shell, Theme};
    let shell = Shell::new(Theme::of(Mode::Starfield), Glyphs::of(Set::Signature));
    assert!(markdown::render(&shell, "", &[], 60).is_empty());
    assert!(markdown::render(&shell, "\n\n  \n", &[], 60).is_empty());
}

// ---------------------------------------------------------------------------
// What rendering may not become
// ---------------------------------------------------------------------------

/// The client's send path: what composes an intent, what stores one, and what
/// puts one on the wire.
const SENDING: &[&str] = &[
    "src/wire.rs",
    "src/sender.rs",
    "src/device.rs",
    "src/capture.rs",
    "src/replica.rs",
];

#[test]
fn nothing_on_the_send_path_can_reach_the_renderer() {
    // **Rendering is a way of looking at a body and never a source of one.**
    // DR-001 requires that nothing forecloses export, and a client that sent
    // anything derived from its own parse would have quietly made its renderer
    // authoritative over the markdown the person actually wrote.
    //
    // The other half is division: the instance divides a body into blocks,
    // once, and that is the only implementation of the rule. Parsing here is
    // for display and produces nothing anybody else sees — which is only true
    // while this holds.
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut offenders = Vec::new();
    for path in SENDING {
        let source = std::fs::read_to_string(crate_root.join(path))
            .unwrap_or_else(|_| panic!("{path} is named here and is not there; fix this list"));
        // **Code, not prose.** `capture.rs` says in a doc comment that a body
        // is markdown as the person typed it, which is exactly what that file
        // ought to say — and a scan for the bare word called it an offender.
        // What this is looking for is a path into the module.
        let code: String = source
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        for needle in ["markdown::", "pulldown_cmark"] {
            if code.contains(needle) {
                offenders.push(format!("{path} uses {needle}"));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "{offenders:?}. What a person wrote is what gets sent, byte for byte. \
         Anything on this path that reached the renderer would be sending \
         something the renderer decided."
    );
}

#[test]
fn the_send_path_is_actually_what_this_names() {
    // A list of file names is only a guard while the names are real and while
    // they are still where the sending happens.
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let wire = std::fs::read_to_string(crate_root.join("src/wire.rs")).expect("wire.rs");
    assert!(
        wire.contains("compose_body_edit"),
        "wire.rs is listed as the place a body becomes an intent and no longer \
         seems to be; the list above needs revisiting"
    );
    let sender = std::fs::read_to_string(crate_root.join("src/sender.rs")).expect("sender.rs");
    assert!(
        sender.contains("submit"),
        "sender.rs is listed as the place an intent reaches the instance and no \
         longer seems to be"
    );
}

#[test]
fn nothing_is_marked_when_the_blocks_are_not_this_bodys() {
    // Blocks come from the instance and the body can be something this device
    // has edited since. Offsets computed from stale blocks land in the middle
    // of the wrong words, and an underline in the wrong place is worse than no
    // underline at all.
    let blocks = vec![v1::Block {
        block_id: vec![1_u8; 16],
        text: "the shroud lip is 3mm proud".to_string(),
    }];
    let relations = vec![v1::Relation {
        relation_id: vec![2_u8; 16],
        content: Some(v1::RelationContent {
            anchor: Some(v1::Anchor {
                entity_id: vec![3_u8; 16],
                block_id: vec![1_u8; 16],
                phrase: "shroud lip".to_string(),
            }),
            ..Default::default()
        }),
        ..Default::default()
    }];

    assert_eq!(
        markdown::formed_at("the shroud lip is 3mm proud", &blocks, &relations).len(),
        1,
        "the blocks are this body's, so the phrase is found"
    );
    assert!(
        markdown::formed_at(
            "the shroud lip is 3mm proud, and I have since typed more",
            &blocks,
            &relations
        )
        .is_empty(),
        "and when they are not, nothing is marked"
    );
}

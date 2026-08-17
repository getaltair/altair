//! Division: the boundary rules, and the properties that hold for every body.
//!
//! The plan calls block division "the most testable component in the system and
//! the easiest to get subtly wrong", so this file is weighted accordingly. Each
//! test names the sentence it enforces.

use altaird::body::{BlockKind, divide};
use proptest::prelude::*;

// ---------------------------------------------------------------------------
// Boundary rules, stated one at a time
// ---------------------------------------------------------------------------

fn kinds(body: &str) -> Vec<BlockKind> {
    divide(body).iter().map(|b| b.kind()).collect()
}

fn contents(body: &str) -> Vec<String> {
    divide(body)
        .iter()
        .map(|b| b.content().to_owned())
        .collect()
}

/// "Accepted limitation: a long unbroken stretch of prose is one block."
///
/// This test exists to be *failed by a helpful improvement*. Anyone who makes
/// division split on sentences to let two people merge edits to one paragraph
/// has reintroduced the interleaving the substrate spec rejects by name, and
/// this is where they find out.
#[test]
fn a_long_unbroken_stretch_of_prose_is_one_block() {
    let prose = "Sentence one is here. Sentence two follows it. ".repeat(200);
    let blocks = divide(&prose);
    assert_eq!(blocks.len(), 1, "long prose must not subdivide");
    assert_eq!(blocks[0].kind(), BlockKind::Paragraph);
    assert_eq!(blocks[0].content(), prose.trim_end());
}

/// "Some constructs are atomic and never split, whatever blank lines they
/// contain and however large they are. A fenced code block, a diagram, a
/// table."
#[test]
fn a_fenced_code_block_is_one_block_however_many_blank_lines_it_holds() {
    let body = "before\n\n```rust\nfn a() {}\n\n\nfn b() {}\n\n- not a list item\n\n# not a heading\n```\n\nafter\n";
    assert_eq!(
        kinds(body),
        vec![
            BlockKind::Paragraph,
            BlockKind::FencedCode,
            BlockKind::Paragraph
        ]
    );
    assert!(contents(body)[1].contains("- not a list item"));
}

/// A diagram is a fenced code block, and these documents are full of them.
#[test]
fn a_mermaid_diagram_is_one_block() {
    let body = "```mermaid\nflowchart LR\n    A --- R\n\n    R --- B\n```\n";
    let blocks = divide(body);
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind(), BlockKind::FencedCode);
}

#[test]
fn a_table_is_one_block() {
    let body = "| a | b |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |\n";
    let blocks = divide(body);
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind(), BlockKind::Table);
}

/// A block quote holds quoted material. Splitting it would let half of one
/// quotation merge with half of another, which is the same failure the spec
/// names for a diagram.
#[test]
fn a_block_quote_is_one_block_including_a_list_inside_it() {
    let body = "> a warning\n>\n> - one\n> - two\n";
    let blocks = divide(body);
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind(), BlockKind::BlockQuote);
}

/// "Some constructs contain independently editable parts and split at them. A
/// list item is a block ... two people adding to a shared shopping list is
/// among the most likely concurrent edits in a household, and it has to merge."
#[test]
fn each_list_item_is_its_own_block() {
    let body = "- milk\n- eggs\n- bread\n";
    assert_eq!(contents(body), vec!["- milk", "- eggs", "- bread"]);
    assert_eq!(kinds(body), vec![BlockKind::ListItem; 3]);
}

#[test]
fn task_list_items_split_too() {
    let body = "- [ ] milk\n- [x] eggs\n";
    assert_eq!(contents(body), vec!["- [ ] milk", "- [x] eggs"]);
}

#[test]
fn ordered_list_items_split_too() {
    let body = "1. first\n2. second\n";
    assert_eq!(contents(body), vec!["1. first", "2. second"]);
}

/// A sectioned shopping list is still a shopping list, and two people adding to
/// different sections still have to merge.
#[test]
fn nested_list_items_split_and_keep_their_indentation() {
    let body = "- produce\n  - apples\n  - pears\n- dairy\n  - milk\n";
    assert_eq!(
        contents(body),
        vec![
            "- produce",
            "  - apples",
            "  - pears",
            "- dairy",
            "  - milk"
        ]
    );
}

/// A list item's own content is one block, whatever it holds, until a nested
/// list opens inside it. Content resuming after that nested list has no block
/// it could join without reaching back past entries that are not its own, so it
/// starts one.
#[test]
fn an_item_is_one_block_until_a_nested_list_opens_and_content_after_one_starts_another() {
    let body = "- lead line\n\n  second paragraph of the same entry\n- plain\n";
    assert_eq!(
        contents(body),
        vec![
            "- lead line\n\n  second paragraph of the same entry",
            "- plain"
        ],
        "a multi-paragraph item with no nested list is one block"
    );

    let with_sublist = "- lead line\n  - nested\n\n  resumed content\n";
    assert_eq!(
        contents(with_sublist),
        vec!["- lead line", "  - nested", "  resumed content"]
    );
    assert_eq!(
        kinds(with_sublist),
        vec![
            BlockKind::ListItem,
            BlockKind::ListItem,
            BlockKind::ListItemTail
        ]
    );
}

/// An item holding a fenced code block does not split at it: the fence is part
/// of the entry, and the entry is the independently editable part.
#[test]
fn a_list_item_holding_a_fence_is_still_one_block() {
    let body = "- run this:\n\n  ```sh\n  ls\n\n  pwd\n  ```\n- and this\n";
    let blocks = divide(body);
    assert_eq!(blocks.len(), 2);
    assert!(blocks[0].content().contains("pwd"));
    assert_eq!(blocks[1].content(), "- and this");
}

#[test]
fn headings_thematic_breaks_and_indented_code_are_each_one_block() {
    let body = "# Title\n\ntext\n\n---\n\n    indented code\n    second line\n\n## Next\n";
    assert_eq!(
        kinds(body),
        vec![
            BlockKind::Heading,
            BlockKind::Paragraph,
            BlockKind::ThematicBreak,
            BlockKind::IndentedCode,
            BlockKind::Heading,
        ]
    );
    assert_eq!(
        contents(body)[3],
        "    indented code\n    second line",
        "indentation is what makes it code and must survive"
    );
}

/// A body arrives as whatever the person's editor produced. Windows line
/// endings divide the same way and reassemble unchanged — division does not
/// normalise the text, because normalising it would be an edit nobody made.
#[test]
fn carriage_returns_divide_the_same_way_and_survive_reassembly() {
    let body = "# Title\r\n\r\n- milk\r\n- eggs\r\n\r\nA paragraph.\r\n";
    assert_eq!(
        kinds(body),
        vec![
            BlockKind::Heading,
            BlockKind::ListItem,
            BlockKind::ListItem,
            BlockKind::Paragraph,
        ]
    );
    let rebuilt: String = divide(body).iter().map(|b| b.text()).collect();
    assert_eq!(rebuilt, body);
}

#[test]
fn an_empty_or_blank_body_has_no_blocks() {
    assert!(divide("").is_empty());
    assert!(divide("   \n\n\t\n").is_empty());
}

/// Nothing may foreclose export (DR-001). A body is its blocks in order, so
/// concatenating the stored text has to give the body back byte for byte —
/// including the author's blank lines and whether a list was tight or loose.
#[test]
fn concatenating_the_blocks_reproduces_the_body_exactly() {
    let body = "# Title\n\n\n\nA paragraph.\n\n- tight\n- list\n\n- loose\n\n- list\n\n```\ncode\n```\n\n\n";
    let rebuilt: String = divide(body).iter().map(|b| b.text()).collect();
    assert_eq!(rebuilt, body);
}

/// A boundary is deterministic, so a golden fixture is a legitimate assertion
/// rather than a brittle one. If this changes, the division rule changed, and
/// every body already stored divides differently from the day it was written.
#[test]
fn a_mixed_document_divides_into_exactly_these_boundaries() {
    let body = "Intro paragraph.\n\n- top one\n  - nested a\n  - nested b\n\n  trailing of top one\n- top two\n\n```rust\nfn f() {\n\n}\n```\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n> quote\n>\n> more\n\n    indented code\n\n---\n\n# Heading\n";
    let blocks = divide(body);
    let observed: Vec<(BlockKind, &str)> = blocks.iter().map(|b| (b.kind(), b.content())).collect();
    assert_eq!(
        observed,
        vec![
            (BlockKind::Paragraph, "Intro paragraph."),
            (BlockKind::ListItem, "- top one"),
            (BlockKind::ListItem, "  - nested a"),
            (BlockKind::ListItem, "  - nested b"),
            (BlockKind::ListItemTail, "  trailing of top one"),
            (BlockKind::ListItem, "- top two"),
            (BlockKind::FencedCode, "```rust\nfn f() {\n\n}\n```"),
            (BlockKind::Table, "| a | b |\n|---|---|\n| 1 | 2 |"),
            (BlockKind::BlockQuote, "> quote\n>\n> more"),
            (BlockKind::IndentedCode, "    indented code"),
            (BlockKind::ThematicBreak, "---"),
            (BlockKind::Heading, "# Heading"),
        ]
    );
}

// ---------------------------------------------------------------------------
// Properties
// ---------------------------------------------------------------------------

/// Bodies with markdown structure in them. Arbitrary `String` finds panics;
/// this finds boundary bugs, because an arbitrary string is almost never a
/// fenced code block.
fn fragment() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-z ]{1,40}".prop_map(|s| format!("{}\n", s.trim())),
        "[a-z ]{1,20}".prop_map(|s| format!("# {}\n", s.trim())),
        "[a-z ]{1,20}".prop_map(|s| format!("- {}\n- second\n", s.trim())),
        "[a-z ]{1,20}".prop_map(|s| format!("- outer\n  - {}\n", s.trim())),
        "[a-z \n]{1,40}".prop_map(|s| format!("```\n{s}\n```\n")),
        Just("| a | b |\n|---|---|\n| 1 | 2 |\n".to_owned()),
        Just("> quoted\n>\n> more\n".to_owned()),
        Just("    indented\n".to_owned()),
        Just("---\n".to_owned()),
    ]
}

fn body() -> impl Strategy<Value = String> {
    prop::collection::vec((fragment(), 1usize..4), 0..8).prop_map(|parts| {
        parts
            .into_iter()
            .map(|(text, blanks)| format!("{}{}", text, "\n".repeat(blanks - 1)))
            .collect()
    })
}

proptest! {
    /// **The property test the plan asks for**: the same text always yields the
    /// same boundaries.
    ///
    /// Repeating the call inside one process is a real check, not a tautology:
    /// two `HashMap`s built in the same process have different random states,
    /// so any dependence on hash iteration order shows up here. Nothing else in
    /// division reads state — no clock, no locale, no environment, no
    /// randomness, and no floating point — so this plus review is the whole of
    /// what determinism needs.
    #[test]
    fn the_same_text_always_yields_the_same_boundaries(body in body()) {
        let first: Vec<_> = divide(&body).iter().map(|b| (b.kind(), b.span())).collect();
        let second: Vec<_> = divide(&body).iter().map(|b| (b.kind(), b.span())).collect();
        prop_assert_eq!(first, second);
    }

    /// The same, for text that was never meant to be markdown at all.
    #[test]
    fn arbitrary_text_divides_deterministically_and_without_panicking(body in ".{0,400}") {
        let first: Vec<_> = divide(&body).iter().map(|b| (b.kind(), b.span())).collect();
        let second: Vec<_> = divide(&body).iter().map(|b| (b.kind(), b.span())).collect();
        prop_assert_eq!(first, second);
    }

    /// Spans partition the body: contiguous, in order, covering every byte.
    /// This is what makes "a body is its blocks in order" exactly true.
    #[test]
    fn spans_partition_the_body(body in body()) {
        let blocks = divide(&body);
        if blocks.is_empty() {
            prop_assert!(body.trim().is_empty());
            return Ok(());
        }
        prop_assert_eq!(blocks[0].span().start, 0);
        prop_assert_eq!(blocks[blocks.len() - 1].span().end, body.len());
        for pair in blocks.windows(2) {
            prop_assert_eq!(pair[0].span().end, pair[1].span().start);
            prop_assert!(pair[0].span().start < pair[0].span().end);
        }
        let rebuilt: String = blocks.iter().map(|b| b.text()).collect();
        prop_assert_eq!(rebuilt, body);
    }

    /// The same, for arbitrary text.
    #[test]
    fn arbitrary_text_partitions_too(body in ".{0,400}") {
        let blocks = divide(&body);
        if blocks.is_empty() {
            prop_assert!(body.trim().is_empty());
            return Ok(());
        }
        let rebuilt: String = blocks.iter().map(|b| b.text()).collect();
        prop_assert_eq!(rebuilt, body);
    }

    /// Content is the span with surrounding blank space removed, is never
    /// empty, and never loses the indentation of its first line.
    #[test]
    fn content_is_the_span_without_surrounding_blank_space(body in body()) {
        for block in divide(&body) {
            let span = block.span();
            let content = block.content_span();
            prop_assert!(span.start <= content.start && content.end <= span.end);
            prop_assert!(!block.content().is_empty());
            prop_assert_eq!(block.content(), block.content().trim_end());
            prop_assert!(body[span.start..content.start].trim().is_empty());
            prop_assert!(body[content.end..span.end].trim().is_empty());
        }
    }

    /// Division is a function of the text and nothing else, so identical text
    /// appearing in two different documents divides identically. A body built
    /// by concatenating fragments contains each fragment's own division.
    #[test]
    fn a_fenced_block_never_splits_however_it_is_surrounded(
        prefix in body(),
        inner in "[a-z \n]{0,60}",
    ) {
        let fence = format!("```\n{inner}\n```\n");
        let combined = format!("{prefix}\n{fence}\n");
        let last = divide(&combined)
            .into_iter()
            .rfind(|b| b.kind() == BlockKind::FencedCode);
        prop_assert_eq!(last.map(|b| b.content().to_owned()), Some(fence.trim_end().to_owned()));
    }
}

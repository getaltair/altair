//! The mutation suite: identity surviving rewording, reordering, insertion and
//! deletion around a block.
//!
//! *"A block is recognisable as the same block after the text around it
//! changes, and after edits to its own text."* — `docs/altair-substrate-spec.md`.
//!
//! Every test here mutates a body, reconciles the mutation against the blocks
//! held before it, and asserts which identities came through. Identifiers are
//! never inspected for meaning (DR-007); they are only ever compared for
//! equality with one held from before the mutation.

use std::cell::Cell;

use altaird::body::{Origin, Reconciliation, StoredBlock, divide, reconcile, reconcile_with};
use proptest::prelude::*;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Harness
// ---------------------------------------------------------------------------

/// The blocks an instance would hold after first accepting `body`.
fn store(body: &str) -> Vec<StoredBlock> {
    divide(body)
        .iter()
        .map(|block| StoredBlock {
            id: Uuid::new_v4(),
            text: block.text().to_owned(),
        })
        .collect()
}

/// The identity of the stored block whose content is `content`.
fn id_of(stored: &[StoredBlock], content: &str) -> Uuid {
    let mut found = stored
        .iter()
        .filter(|b| b.text.trim() == content)
        .map(|b| b.id);
    let id = found.next().unwrap_or_else(|| {
        panic!(
            "no stored block with content {content:?}; have {:?}",
            stored.iter().map(|b| b.text.trim()).collect::<Vec<_>>()
        )
    });
    assert!(found.next().is_none(), "content {content:?} is not unique");
    id
}

/// The identity the reconciled body gave to the block whose content is
/// `content`.
fn resolved_id(outcome: &Reconciliation, content: &str) -> Uuid {
    let mut found = outcome
        .blocks
        .iter()
        .filter(|b| b.text.trim() == content)
        .map(|b| b.id);
    let id = found.next().unwrap_or_else(|| {
        panic!(
            "no resolved block with content {content:?}; have {:?}",
            outcome
                .blocks
                .iter()
                .map(|b| b.text.trim())
                .collect::<Vec<_>>()
        )
    });
    assert!(found.next().is_none(), "content {content:?} is not unique");
    id
}

const LIST: &str = "- milk\n- eggs\n- bread\n";

// ---------------------------------------------------------------------------
// The four named mutations
// ---------------------------------------------------------------------------

/// **Rewording.** The named case in the spec: "rewording an entry does not
/// detach a relation anchored to the entry itself".
///
/// This is also the test that rules out hashing the text as an identity.
#[test]
fn identity_survives_rewording_a_block() {
    let stored = store(LIST);
    let eggs = id_of(&stored, "- eggs");

    let outcome = reconcile(&stored, "- milk\n- six eggs\n- bread\n");

    assert_eq!(resolved_id(&outcome, "- six eggs"), eggs);
    assert!(outcome.removed.is_empty());
    let reworded = &outcome.blocks[1];
    assert_eq!(reworded.origin, Origin::Carried);
    assert!(reworded.text_changed);
    assert!(!reworded.moved);
}

/// **Reordering.** This is the test that rules out position as an identity.
#[test]
fn identity_survives_reordering() {
    let stored = store(LIST);
    let (milk, eggs, bread) = (
        id_of(&stored, "- milk"),
        id_of(&stored, "- eggs"),
        id_of(&stored, "- bread"),
    );

    let outcome = reconcile(&stored, "- bread\n- milk\n- eggs\n");

    assert_eq!(
        outcome.blocks.iter().map(|b| b.id).collect::<Vec<_>>(),
        vec![bread, milk, eggs]
    );
    assert!(outcome.removed.is_empty());
    assert!(outcome.blocks.iter().all(|b| b.origin == Origin::Carried));
    assert!(
        outcome.blocks.iter().all(|b| !b.text_changed),
        "reordering changes no block's text"
    );
    assert!(outcome.blocks.iter().all(|b| b.moved));
}

/// **Insertion around a block.**
#[test]
fn identity_survives_insertion_around_a_block() {
    let stored = store(LIST);
    let (milk, eggs, bread) = (
        id_of(&stored, "- milk"),
        id_of(&stored, "- eggs"),
        id_of(&stored, "- bread"),
    );

    let outcome = reconcile(
        &stored,
        "- flour\n- milk\n- eggs\n- salt\n- bread\n- yeast\n",
    );

    assert_eq!(resolved_id(&outcome, "- milk"), milk);
    assert_eq!(resolved_id(&outcome, "- eggs"), eggs);
    assert_eq!(resolved_id(&outcome, "- bread"), bread);
    assert!(outcome.removed.is_empty());
    assert_eq!(
        outcome
            .blocks
            .iter()
            .filter(|b| b.origin == Origin::Created)
            .count(),
        3
    );
}

/// **Deletion around a block.**
#[test]
fn identity_survives_deletion_around_a_block() {
    let stored = store(LIST);
    let (milk, eggs, bread) = (
        id_of(&stored, "- milk"),
        id_of(&stored, "- eggs"),
        id_of(&stored, "- bread"),
    );

    let outcome = reconcile(&stored, "- eggs\n");

    assert_eq!(outcome.blocks.len(), 1);
    assert_eq!(outcome.blocks[0].id, eggs);
    assert_eq!(outcome.blocks[0].origin, Origin::Carried);
    assert_eq!(outcome.removed, vec![milk, bread]);
}

/// The mutations together, which is what a real concurrent edit looks like:
/// somebody reordered the list, reworded one entry, added one, and removed one.
#[test]
fn identity_survives_all_four_mutations_at_once() {
    let stored = store("- milk\n- eggs\n- bread\n- butter\n");
    let (milk, eggs, butter) = (
        id_of(&stored, "- milk"),
        id_of(&stored, "- eggs"),
        id_of(&stored, "- butter"),
    );
    let bread = id_of(&stored, "- bread");

    let outcome = reconcile(&stored, "- butter\n- half a dozen eggs\n- milk\n- tea\n");

    assert_eq!(resolved_id(&outcome, "- butter"), butter);
    assert_eq!(resolved_id(&outcome, "- half a dozen eggs"), eggs);
    assert_eq!(resolved_id(&outcome, "- milk"), milk);
    assert_eq!(outcome.removed, vec![bread]);
    assert_eq!(
        outcome
            .blocks
            .iter()
            .filter(|b| b.origin == Origin::Created)
            .count(),
        1
    );
}

// ---------------------------------------------------------------------------
// Where "this is now a different block" sits
// ---------------------------------------------------------------------------

/// A rewrite that keeps half the text keeps the identity.
#[test]
fn a_modest_rewrite_keeps_the_identity() {
    let body = "The quarterly plan covers the garden and the shed.\n";
    let stored = store(body);
    let original = stored[0].id;

    let outcome = reconcile(
        &stored,
        "The quarterly plan covers the garden and the greenhouse.\n",
    );

    assert_eq!(outcome.blocks[0].id, original);
    assert_eq!(outcome.blocks[0].origin, Origin::Carried);
    assert!(outcome.removed.is_empty());
}

/// A wholesale replacement is a new block, and the old one is reported removed
/// so that a relation anchored to it can lose its anchor and survive.
///
/// This is the deliberate half of the threshold. Carrying an identity onto text
/// nobody derived from the original would move somebody's relation onto words
/// they did not write, silently.
#[test]
fn a_wholesale_replacement_is_a_new_block() {
    let stored = store("The quarterly plan covers the garden and the shed.\n");
    let original = stored[0].id;

    let outcome = reconcile(&stored, "Reminder: the car needs its MOT before Friday.\n");

    assert_eq!(outcome.blocks[0].origin, Origin::Created);
    assert_ne!(outcome.blocks[0].id, original);
    assert_eq!(outcome.removed, vec![original]);
}

/// The documented consequence of matching on text alone: a block replaced
/// wholesale between two unchanged neighbours is still a new block, even though
/// a person reading the diff would call it an edit.
///
/// This is pinned rather than fixed. Reading the neighbours would catch this
/// case, and would also carry an identity onto text with nothing in common with
/// the original — which is the error worth refusing, because it moves somebody's
/// relation silently. Losing an anchor is loud by comparison: the relation is
/// still there, unanchored.
#[test]
fn a_replacement_between_unchanged_neighbours_is_still_a_new_block() {
    let stored =
        store("First about the garden.\n\nSecond about the shed.\n\nThird about the car.\n");
    let middle = stored[1].id;

    let outcome = reconcile(
        &stored,
        "First about the garden.\n\nRing the vet on Tuesday.\n\nThird about the car.\n",
    );

    assert_eq!(outcome.blocks[1].origin, Origin::Created);
    assert_eq!(outcome.removed, vec![middle]);
}

/// The length floor: overlap alone would let a short entry match any long block
/// that starts and ends the way it does.
#[test]
fn a_short_entry_does_not_match_a_far_longer_block() {
    let stored = store("- tea\n");
    let original = stored[0].id;

    let outcome = reconcile(
        &stored,
        "- tea leaves, coffee beans, oat milk, two loaves, a dozen eggs and some tea\n",
    );

    assert_eq!(outcome.blocks[0].origin, Origin::Created);
    assert_eq!(outcome.removed, vec![original]);
}

/// Emptying a body removes every block. Nothing is carried onto nothing.
#[test]
fn emptying_the_body_removes_every_block() {
    let stored = store(LIST);
    let outcome = reconcile(&stored, "");
    assert!(outcome.blocks.is_empty());
    assert_eq!(outcome.removed.len(), 3);
}

/// The first write to an entity has nothing to match against.
#[test]
fn a_first_body_creates_every_block() {
    let outcome = reconcile(&[], LIST);
    assert_eq!(outcome.blocks.len(), 3);
    assert!(outcome.blocks.iter().all(|b| b.origin == Origin::Created));
    assert!(outcome.blocks.iter().all(|b| b.text_changed));
    assert!(outcome.removed.is_empty());
}

/// "A list item is a block **while the list holds together around it**." When
/// the list stops holding together — the markers go and the entries become
/// paragraphs — the entries are still the same entries, and similarity carries
/// them across. Nothing special-cases the transition; it falls out of matching
/// on text.
#[test]
fn identity_survives_the_list_ceasing_to_be_a_list() {
    let stored = store("- collect the parcel\n- book the service\n");
    let (parcel, service) = (
        id_of(&stored, "- collect the parcel"),
        id_of(&stored, "- book the service"),
    );

    let outcome = reconcile(&stored, "collect the parcel\n\nbook the service\n");

    assert_eq!(resolved_id(&outcome, "collect the parcel"), parcel);
    assert_eq!(resolved_id(&outcome, "book the service"), service);
    assert!(outcome.removed.is_empty());
}

/// Two identical entries are not one entry. Each keeps a distinct identity, and
/// the nearer one wins so an edit does not shuffle relations between them.
#[test]
fn duplicate_entries_keep_distinct_identities_and_match_the_nearer_one() {
    let stored = store("- milk\n- eggs\n- milk\n");
    let first = stored[0].id;
    let last = stored[2].id;
    assert_ne!(first, last);

    let outcome = reconcile(&stored, "- milk\n- eggs\n- milk\n- bread\n");

    assert_eq!(outcome.blocks[0].id, first);
    assert_eq!(outcome.blocks[2].id, last);
    assert!(outcome.removed.is_empty());
}

/// Editing one paragraph leaves every other paragraph's identity alone, which
/// is the whole reason division exists: "a relation formed in one paragraph is
/// undisturbed when someone rewrites another".
#[test]
fn rewriting_one_paragraph_disturbs_no_other() {
    let body = "First paragraph about the garden.\n\nSecond paragraph about the shed.\n\nThird paragraph about the car.\n";
    let stored = store(body);
    let first = stored[0].id;
    let third = stored[2].id;

    let outcome = reconcile(
        &stored,
        "First paragraph about the garden.\n\nA completely unrelated statement regarding taxes.\n\nThird paragraph about the car.\n",
    );

    assert_eq!(outcome.blocks[0].id, first);
    assert!(!outcome.blocks[0].text_changed);
    assert_eq!(outcome.blocks[2].id, third);
    assert!(!outcome.blocks[2].text_changed);
}

/// Adding a blank line under a paragraph is not an edit to that paragraph's
/// content. The block's stored text changes, because the body changed and the
/// body has to reassemble byte for byte; its identity and its content do not.
#[test]
fn surrounding_blank_lines_do_not_reword_a_block() {
    let stored = store("one\n\ntwo\n");
    let (first, second) = (stored[0].id, stored[1].id);

    let outcome = reconcile(&stored, "one\n\n\n\ntwo\n");

    assert_eq!(outcome.blocks[0].id, first);
    assert_eq!(outcome.blocks[1].id, second);
    assert!(outcome.removed.is_empty());
}

/// An atomic construct is matched as one thing, so editing a line inside a
/// fenced block keeps the whole block's identity rather than producing a new
/// one for the edited region.
#[test]
fn editing_inside_a_fence_keeps_the_fence_block() {
    let stored = store("```rust\nfn main() {\n    println!(\"one\");\n}\n```\n");
    let original = stored[0].id;

    let outcome = reconcile(
        &stored,
        "```rust\nfn main() {\n    println!(\"two\");\n}\n```\n",
    );

    assert_eq!(outcome.blocks.len(), 1);
    assert_eq!(outcome.blocks[0].id, original);
    assert!(outcome.blocks[0].text_changed);
}

// ---------------------------------------------------------------------------
// Determinism
// ---------------------------------------------------------------------------

/// Matching is deterministic given the same inputs. New identifiers are random
/// by DR-007, so the source of them is supplied here; everything else about the
/// outcome must be identical run to run.
#[test]
fn matching_the_same_pair_twice_gives_the_same_answer() {
    let stored = store("- milk\n- eggs\n- bread\n- butter\n");
    let body = "- butter\n- half a dozen eggs\n- milk\n- tea\n";

    let run = || {
        let counter = Cell::new(0u128);
        reconcile_with(&stored, body, || {
            counter.set(counter.get() + 1);
            Uuid::from_u128(counter.get())
        })
    };

    let first = run();
    let second = run();
    assert_eq!(
        first.blocks.iter().map(|b| b.id).collect::<Vec<_>>(),
        second.blocks.iter().map(|b| b.id).collect::<Vec<_>>()
    );
    assert_eq!(first.removed, second.removed);
}

// ---------------------------------------------------------------------------
// Properties over the mutations
// ---------------------------------------------------------------------------

/// Distinct one-line paragraphs, so that a body's blocks are unambiguous and a
/// mutation can be aimed at exactly one of them.
fn entries() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec(0usize..1000, 2..8).prop_map(|numbers| {
        let mut seen = Vec::new();
        for n in numbers {
            if !seen.contains(&n) {
                seen.push(n);
            }
        }
        seen.into_iter()
            .map(|n| format!("entry number {n} of this note"))
            .collect()
    })
}

fn assemble(entries: &[String]) -> String {
    entries
        .iter()
        .map(|e| format!("{e}\n\n"))
        .collect::<String>()
}

proptest! {
    /// Inserting a block anywhere leaves every existing identity in place and
    /// removes nothing.
    #[test]
    fn insertion_anywhere_preserves_every_identity(
        entries in entries(),
        at in 0usize..16,
    ) {
        let stored = store(&assemble(&entries));
        let before: Vec<Uuid> = stored.iter().map(|b| b.id).collect();

        let mut mutated = entries.clone();
        mutated.insert(at % (entries.len() + 1), "a newly inserted line".to_owned());
        let outcome = reconcile(&stored, &assemble(&mutated));

        let after: Vec<Uuid> = outcome.blocks.iter().map(|b| b.id).collect();
        prop_assert!(outcome.removed.is_empty());
        prop_assert!(before.iter().all(|id| after.contains(id)));
        prop_assert_eq!(after.len(), before.len() + 1);
    }

    /// Deleting one block leaves every other identity in place, and reports
    /// exactly the deleted one as removed.
    #[test]
    fn deletion_preserves_every_surviving_identity(
        entries in entries(),
        at in 0usize..16,
    ) {
        let stored = store(&assemble(&entries));
        let index = at % entries.len();
        let removed_id = stored[index].id;

        let mut mutated = entries.clone();
        mutated.remove(index);
        let outcome = reconcile(&stored, &assemble(&mutated));

        prop_assert_eq!(&outcome.removed, &vec![removed_id]);
        for (position, block) in outcome.blocks.iter().enumerate() {
            let expected = if position < index { position } else { position + 1 };
            prop_assert_eq!(block.id, stored[expected].id);
            prop_assert_eq!(block.origin, Origin::Carried);
        }
    }

    /// Reordering carries every identity with its text, wherever it went.
    #[test]
    fn reordering_carries_every_identity_with_its_text(
        entries in entries(),
        rotate in 1usize..8,
    ) {
        let stored = store(&assemble(&entries));
        let shift = rotate % entries.len().max(1);

        let mut mutated = entries.clone();
        mutated.rotate_left(shift);
        let outcome = reconcile(&stored, &assemble(&mutated));

        prop_assert!(outcome.removed.is_empty());
        for (position, block) in outcome.blocks.iter().enumerate() {
            let origin = (position + shift) % entries.len();
            prop_assert_eq!(block.id, stored[origin].id, "block {} came from {}", position, origin);
            prop_assert!(!block.text_changed);
        }
    }

    /// Rewording one block keeps that block's identity and disturbs no other.
    /// The edit here appends to an entry, which is the shape of a real one: a
    /// person adds a word rather than replacing the line.
    #[test]
    fn rewording_one_block_disturbs_no_other(
        entries in entries(),
        at in 0usize..16,
    ) {
        let stored = store(&assemble(&entries));
        let index = at % entries.len();

        let mut mutated = entries.clone();
        mutated[index] = format!("{} and one more clause", entries[index]);
        let outcome = reconcile(&stored, &assemble(&mutated));

        prop_assert!(outcome.removed.is_empty());
        for (position, block) in outcome.blocks.iter().enumerate() {
            prop_assert_eq!(block.id, stored[position].id);
            prop_assert_eq!(block.origin, Origin::Carried);
            prop_assert_eq!(block.text_changed, position == index);
        }
    }

    /// A body written again unchanged carries every identity, changes nothing,
    /// and moves nothing. Reconciliation must be idempotent or a replayed
    /// intent would churn the store.
    #[test]
    fn rewriting_an_unchanged_body_changes_nothing(entries in entries()) {
        let body = assemble(&entries);
        let stored = store(&body);
        let outcome = reconcile(&stored, &body);

        prop_assert!(outcome.removed.is_empty());
        for (position, block) in outcome.blocks.iter().enumerate() {
            prop_assert_eq!(block.id, stored[position].id);
            prop_assert!(!block.text_changed);
            prop_assert!(!block.moved);
        }
    }

    /// Every incoming block gets exactly one identity, positions are dense and
    /// in order, no identity is used twice, and no identity is both carried and
    /// reported removed.
    #[test]
    fn every_block_gets_exactly_one_identity(entries in entries(), extra in entries()) {
        let stored = store(&assemble(&entries));
        let outcome = reconcile(&stored, &assemble(&extra));

        let mut ids: Vec<Uuid> = outcome.blocks.iter().map(|b| b.id).collect();
        let count = ids.len();
        ids.sort();
        ids.dedup();
        prop_assert_eq!(ids.len(), count, "an identity was used twice");

        for (position, block) in outcome.blocks.iter().enumerate() {
            prop_assert_eq!(block.position, position);
            prop_assert!(!outcome.removed.contains(&block.id));
        }

        let carried = outcome.blocks.iter().filter(|b| b.origin == Origin::Carried).count();
        prop_assert_eq!(carried + outcome.removed.len(), stored.len());
    }
}

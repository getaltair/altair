//! Carrying block identity across a body write.
//!
//! # What this owes
//!
//! *"A block is recognisable as the same block after the text around it
//! changes, and after edits to its own text. This is what anchors and
//! reconciliation both depend on, and the second half is what a block anchor
//! relies on: rewording an entry does not detach a relation anchored to the
//! entry itself."* — `docs/altair-substrate-spec.md`.
//!
//! Two consequences fall straight out of that sentence and rule out the two
//! obvious implementations:
//!
//! - Identity cannot be a hash of the block's text, because the text changes
//!   and the identity must not.
//! - Identity cannot be the block's position, because reordering a list must
//!   not renumber every entry's relations onto their neighbours.
//!
//! # The algorithm
//!
//! Three passes over the blocks the store already holds and the blocks the
//! incoming body divides into. Each pass claims pairs; a claimed block takes no
//! further part.
//!
//! 1. **Exact content.** An incoming block whose content is byte-identical to
//!    an unclaimed stored block's content takes that block's identity. Where
//!    several stored blocks have the same content — two identical shopping list
//!    entries — the one nearest in position wins, and on a tie the earlier one.
//!    This pass alone covers insertion, deletion, and reordering around a
//!    block, which is why it runs first and unconditionally.
//!
//! 2. **Similarity.** Every remaining (stored, incoming) pair is scored over
//!    character trigrams and the pairs at or above the threshold are taken
//!    greedily, best first. This is the pass that survives rewording.
//!
//! 3. **The remainder is new.** An incoming block that matched nothing gets a
//!    fresh UUIDv4. A stored block that matched nothing is reported as removed;
//!    a relation anchored to it loses its anchor and survives, which is the
//!    rule the substrate spec already states for a removed block.
//!
//! # Where "this is now a different block" sits, and why there
//!
//! Two conditions, both integer ratios so that "exactly at the threshold"
//! decides the same way on every machine:
//!
//! - **Overlap ≥ [`SIMILARITY_THRESHOLD`] (1/2).** Of the trigrams of the
//!   *shorter* of the two texts, at least half appear in the longer.
//! - **Length ratio ≥ [`LENGTH_RATIO_FLOOR`] (1/4).** The shorter text is at
//!   least a quarter the length of the longer.
//!
//! The measure is the overlap coefficient — `|A ∩ B| / min(|A|, |B|)` — rather
//! than the more usual Sørensen–Dice, and the length floor is what makes that
//! safe. Dice divides by the *combined* size, so it falls away sharply when one
//! side grows: `- eggs` reworded to `- half a dozen eggs` scores 0.48 under
//! Dice and would become a new block, losing its relations, for an edit that is
//! obviously the same entry with more said about it. Expanding or trimming an
//! entry is the commonest edit there is, so a measure that punishes it is the
//! wrong measure. Overlap ignores the excess; the length floor puts a bound on
//! how much excess is credible. (The floor never rejects a pair Dice would have
//! accepted: below a ratio of 1/4, Dice cannot exceed 0.4.)
//!
//! Trigrams are padded at both ends, so two texts must begin alike *and* end
//! alike to score highly. That is what stops a short block from matching any
//! long block that happens to contain its words.
//!
//! Which side of the threshold to err toward is settled by which error is
//! worse. Refusing a match that should have held costs an anchor: the relation
//! survives without one, which the substrate spec already describes as an
//! ordinary outcome, and the block simply reads as new. Accepting a match that
//! should not have held silently moves somebody's relation onto text they did
//! not write, and nothing tells them. So the thresholds are set to admit the
//! ordinary shapes of an edit and refuse a replacement, and where a case is
//! genuinely ambiguous the answer is "new block".
//!
//! # Failure modes, stated rather than discovered later
//!
//! - **Short, similar entries cross-match.** `- eggs` and `- legs` score
//!   exactly at the threshold and match. Short blocks are where trigrams carry
//!   least information; the positional tiebreak reduces the damage and does not
//!   remove it.
//! - **A wholesale rewrite produces a new block**, and its relations lose their
//!   anchors. Deliberate, per the reasoning above.
//! - **A block split in two** matches at most one of the halves — whichever
//!   scores higher, with ties broken toward the earlier position. The other
//!   half is new. There is no notion of one block becoming two.
//! - **Two blocks merged into one** carries the identity of whichever original
//!   the merged text resembles more. The other is reported removed.
//! - **Structure is not consulted.** Matching sees text only, never position
//!   except as a tiebreak and never the surrounding blocks. A block replaced
//!   wholesale between two unchanged neighbours is a new block, even though a
//!   person reading the diff would call it an edit. Consulting neighbours would
//!   catch that case and would also carry an identity onto text with nothing in
//!   common with the original, which is the error this refuses to make.
//! - **Greedy is not optimal.** A globally better assignment can exist. Greedy
//!   is chosen because it is deterministic, explicable, and stable under small
//!   edits; an optimal assignment is neither cheap nor obviously more correct
//!   when the scores are this close together.
//! - **Very large bodies skip pass 2.** See [`MAX_SIMILARITY_PAIRS`].

use std::collections::BTreeMap;

use uuid::Uuid;

use super::divide::{content_of, divide};

/// The trigram overlap at or above which a rewritten block keeps its identity,
/// as an exact ratio. See the module documentation for why it sits here.
pub const SIMILARITY_THRESHOLD: (u128, u128) = (1, 2);

/// How far apart in length two texts may be and still be one block reworded,
/// as an exact ratio of shorter to longer. Overlap alone would let a short
/// block match an arbitrarily long one; this bounds it.
pub const LENGTH_RATIO_FLOOR: (u128, u128) = (1, 4);

/// Above this many candidate pairs, the similarity pass is skipped and every
/// unmatched incoming block is new.
///
/// The pass is quadratic in unmatched blocks. A body large enough to reach this
/// bound has had almost every one of its thousands of blocks rewritten in a
/// single write, which is a paste of a different document rather than an edit.
/// Skipping is still deterministic — the bound is a count, not a clock.
pub const MAX_SIMILARITY_PAIRS: usize = 250_000;

/// A block as the store holds it, in position order.
///
/// `text` is the verbatim span, which is what a block row holds. Matching
/// reduces it with [`content_of`] so both sides are compared by the same rule.
#[derive(Clone, Debug)]
pub struct StoredBlock {
    pub id: Uuid,
    pub text: String,
}

/// Where a resolved block's identity came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    /// The identity of a block the store already held.
    Carried,
    /// A fresh identity. Nothing in the store resembled this text.
    Created,
}

/// One block of the incoming body, with the identity it will be written under.
#[derive(Clone, Debug)]
pub struct ResolvedBlock {
    pub id: Uuid,
    /// Zero-based, dense, in body order.
    pub position: usize,
    /// The verbatim text to store.
    pub text: String,
    pub origin: Origin,
    /// True where the stored text differs from `text`, or the block is new.
    /// A change set carries the identities of the blocks that changed, so this
    /// is the thing that decides what goes in it.
    pub text_changed: bool,
    /// True where a carried block's position moved. A move is a change to the
    /// body, not to the block's text, and the two are reported separately
    /// because a client comparing its own unsent edit cares about the second.
    pub moved: bool,
}

/// The outcome of matching an incoming body against the blocks already held.
#[derive(Clone, Debug)]
pub struct Reconciliation {
    /// The incoming body's blocks, in order, with identities settled.
    pub blocks: Vec<ResolvedBlock>,
    /// Stored blocks nothing in the incoming body matched, in position order.
    /// Their rows go, and a relation anchored to one loses its anchor and
    /// survives.
    pub removed: Vec<Uuid>,
}

/// Divide `body` and match it against the blocks the store holds.
///
/// `stored` is in position order; its index is the block's position.
pub fn reconcile(stored: &[StoredBlock], body: &str) -> Reconciliation {
    reconcile_with(stored, body, Uuid::new_v4)
}

/// [`reconcile`], with the source of new identifiers supplied.
///
/// Every identifier is a random UUIDv4 generated by whatever brings the thing
/// into existence (DR-007), so the production caller passes [`Uuid::new_v4`].
/// This form exists so a test can assert *which* blocks were created without
/// the assertion depending on randomness. Nothing reads an identifier, here or
/// anywhere: matching looks only at text and position.
pub fn reconcile_with(
    stored: &[StoredBlock],
    body: &str,
    mut new_id: impl FnMut() -> Uuid,
) -> Reconciliation {
    let incoming = divide(body);
    let stored_content: Vec<&str> = stored.iter().map(|b| content_of(&b.text)).collect();
    let incoming_content: Vec<&str> = incoming.iter().map(|b| b.content()).collect();

    // `matched[i]` is the stored index the incoming block at `i` claimed.
    let mut matched: Vec<Option<usize>> = vec![None; incoming.len()];
    let mut claimed: Vec<bool> = vec![false; stored.len()];

    match_exact(
        &stored_content,
        &incoming_content,
        &mut matched,
        &mut claimed,
    );
    match_similar(
        &stored_content,
        &incoming_content,
        &mut matched,
        &mut claimed,
    );

    let blocks = incoming
        .iter()
        .enumerate()
        .map(|(position, block)| match matched[position] {
            Some(from) => ResolvedBlock {
                id: stored[from].id,
                position,
                text: block.text().to_owned(),
                origin: Origin::Carried,
                text_changed: stored[from].text != block.text(),
                moved: from != position,
            },
            None => ResolvedBlock {
                id: new_id(),
                position,
                text: block.text().to_owned(),
                origin: Origin::Created,
                text_changed: true,
                moved: false,
            },
        })
        .collect();

    let removed = stored
        .iter()
        .zip(&claimed)
        .filter(|(_, claimed)| !**claimed)
        .map(|(block, _)| block.id)
        .collect();

    Reconciliation { blocks, removed }
}

/// Pass 1: byte-identical content, nearest position first.
///
/// This is the pass that makes identity survive edits to a block's
/// *neighbours*: inserting, deleting, or reordering around a block leaves its
/// own text untouched, so it matches here regardless of where it moved to.
fn match_exact(
    stored: &[&str],
    incoming: &[&str],
    matched: &mut [Option<usize>],
    claimed: &mut [bool],
) {
    // BTreeMap, not HashMap: the values are read in order and nothing about
    // this function may depend on a hash seed.
    let mut by_content: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (index, content) in stored.iter().enumerate() {
        by_content.entry(content).or_default().push(index);
    }

    for (position, content) in incoming.iter().enumerate() {
        let Some(candidates) = by_content.get(content) else {
            continue;
        };
        let best = candidates
            .iter()
            .copied()
            .filter(|index| !claimed[*index])
            .min_by_key(|index| (index.abs_diff(position), *index));
        if let Some(index) = best {
            claimed[index] = true;
            matched[position] = Some(index);
        }
    }
}

/// Pass 2: trigram overlap, greedy, best first.
///
/// This is the pass that makes identity survive edits to a block's *own* text.
fn match_similar(
    stored: &[&str],
    incoming: &[&str],
    matched: &mut [Option<usize>],
    claimed: &mut [bool],
) {
    let free_stored: Vec<usize> = (0..stored.len()).filter(|i| !claimed[*i]).collect();
    let free_incoming: Vec<usize> = (0..incoming.len())
        .filter(|i| matched[*i].is_none())
        .collect();

    if free_stored.is_empty() || free_incoming.is_empty() {
        return;
    }
    if free_stored.len().saturating_mul(free_incoming.len()) > MAX_SIMILARITY_PAIRS {
        return;
    }

    let stored_grams: Vec<Vec<[char; 3]>> =
        free_stored.iter().map(|i| trigrams(stored[*i])).collect();
    let incoming_grams: Vec<Vec<[char; 3]>> = free_incoming
        .iter()
        .map(|i| trigrams(incoming[*i]))
        .collect();

    // (numerator, denominator, positional distance, incoming position, stored
    // position). Held as an exact ratio: comparing `a/b` against `c/d` by cross
    // multiplication in u128 has no rounding, so two machines cannot disagree
    // about which of two near-equal pairs is better.
    let mut pairs: Vec<(u128, u128, usize, usize, usize)> = Vec::new();
    for (a, stored_index) in free_stored.iter().enumerate() {
        for (b, incoming_index) in free_incoming.iter().enumerate() {
            let (shorter, longer) = {
                let (x, y) = (
                    stored_grams[a].len() as u128,
                    incoming_grams[b].len() as u128,
                );
                (x.min(y), x.max(y))
            };
            if shorter == 0 || !at_least(shorter, longer, LENGTH_RATIO_FLOOR) {
                continue;
            }
            let common = intersection(&stored_grams[a], &incoming_grams[b]);
            let (numerator, denominator) = (common as u128, shorter);
            if !at_least(numerator, denominator, SIMILARITY_THRESHOLD) {
                continue;
            }
            pairs.push((
                numerator,
                denominator,
                stored_index.abs_diff(*incoming_index),
                *incoming_index,
                *stored_index,
            ));
        }
    }

    // Best score first; then the pair that moved least; then body order. Every
    // key is an integer, so the order is total and identical everywhere.
    pairs.sort_by(|left, right| {
        (right.0 * left.1)
            .cmp(&(left.0 * right.1))
            .then(left.2.cmp(&right.2))
            .then(left.3.cmp(&right.3))
            .then(left.4.cmp(&right.4))
    });

    for (_, _, _, incoming_index, stored_index) in pairs {
        if matched[incoming_index].is_none() && !claimed[stored_index] {
            matched[incoming_index] = Some(stored_index);
            claimed[stored_index] = true;
        }
    }
}

/// `numerator / denominator >= threshold`, in integers.
fn at_least(numerator: u128, denominator: u128, threshold: (u128, u128)) -> bool {
    numerator * threshold.1 >= threshold.0 * denominator
}

/// Character trigrams of `text`, sorted, with duplicates kept.
///
/// Sorted rather than hashed so the multiset intersection below is a linear
/// merge with no map iteration in it. Padded at both ends so that a very short
/// block still yields trigrams to compare — without padding, a two-character
/// entry has none at all and could never match anything.
///
/// Characters, not bytes: a trigram must not straddle a UTF-8 boundary or an
/// accented word would score against itself as though it were half a word. No
/// case folding and no normalisation — a block is matched against the text as
/// written, and folding is one more rule two implementations could disagree on.
fn trigrams(text: &str) -> Vec<[char; 3]> {
    const PAD: char = '\u{2}';
    let chars: Vec<char> = std::iter::repeat_n(PAD, 2)
        .chain(text.chars())
        .chain(std::iter::repeat_n(PAD, 2))
        .collect();
    let mut grams: Vec<[char; 3]> = chars.windows(3).map(|w| [w[0], w[1], w[2]]).collect();
    grams.sort_unstable();
    grams
}

/// Size of the multiset intersection of two sorted trigram lists.
fn intersection(left: &[[char; 3]], right: &[[char; 3]]) -> usize {
    let (mut i, mut j, mut common) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        match left[i].cmp(&right[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                common += 1;
                i += 1;
                j += 1;
            }
        }
    }
    common
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The overlap of a text with itself is one.
    fn overlap(left: &str, right: &str) -> (usize, usize) {
        let (a, b) = (trigrams(left), trigrams(right));
        (intersection(&a, &b), a.len().min(b.len()))
    }

    #[test]
    fn overlap_of_identical_text_is_one() {
        let (common, shorter) = overlap("milk", "milk");
        assert_eq!(common, shorter);
    }

    /// The case that chose overlap over Sørensen–Dice. Dice scores this 0.48
    /// and would make it a new block.
    #[test]
    fn an_expanded_entry_scores_above_the_threshold() {
        let (common, shorter) = overlap("- eggs", "- half a dozen eggs");
        assert!(
            at_least(common as u128, shorter as u128, SIMILARITY_THRESHOLD),
            "{common}/{shorter}"
        );
    }

    #[test]
    fn two_unrelated_entries_score_below_the_threshold() {
        let (common, shorter) = overlap("- milk", "- bread");
        assert!(
            !at_least(common as u128, shorter as u128, SIMILARITY_THRESHOLD),
            "{common}/{shorter}"
        );
    }

    #[test]
    fn threshold_compares_as_an_exact_ratio() {
        assert!(at_least(1, 2, SIMILARITY_THRESHOLD));
        assert!(!at_least(499, 1000, SIMILARITY_THRESHOLD));
        assert!(at_least(501, 1000, SIMILARITY_THRESHOLD));
    }

    #[test]
    fn trigrams_are_sorted_and_character_aligned() {
        let grams = trigrams("café");
        assert!(grams.windows(2).all(|w| w[0] <= w[1]));
        // Characters, not bytes: "café" is five bytes and four characters, and
        // padding two either side gives one trigram per character plus two.
        assert_eq!(grams.len(), "café".chars().count() + 2);
    }
}

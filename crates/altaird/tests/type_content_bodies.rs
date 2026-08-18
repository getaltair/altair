//! Bodies: a note's text divided into blocks, matched against the blocks
//! already held, and written a row at a time.
//!
//! Wave 1.2 already proved division and identity matching as pure functions,
//! and `tests/body_division.rs` and `tests/body_identity.rs` are where that
//! lives. Nothing is re-proved here. What this suite is about is the three
//! things that only exist once those functions meet the store:
//!
//! - **Only what changed is written.** Asserted by reading each row's `xmin` —
//!   the transaction that last wrote it — rather than its contents, because a
//!   body write that deleted and reinserted every row would reach an identical
//!   end state and would still be wrong.
//! - **A block is a part.** Two people editing one block retain both values and
//!   the acknowledgement names the block; two people editing different blocks
//!   merge with nobody involved. This is the load-bearing claim of block-level
//!   granularity and it is the reason a body is not one field.
//! - **Authored body content graduates a bulk entity**, and nothing else here
//!   does.

mod common;

use altair_proto::v1;
use altaird::store::begin_write;
use altaird::store::entity::EntityType;
use altaird::store::ids::{EntityId, MemberId};
use altaird::write::body;
use altaird::write::content::BodyWrite;
use altaird::write::entity::{Ctx, Failed, Refusal};
use chrono::Utc;
use common::*;
use sqlx::Row;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Reading the store back
// ---------------------------------------------------------------------------

/// A block row, plus the identity of the transaction that last wrote it.
///
/// `version` is `xmin`, which Postgres advances on every write to a row and
/// never on a read. Two reads either side of an edit therefore say which rows
/// the edit actually touched, which is a stronger statement than the end state
/// can make: an implementation that rewrote every row with its existing values
/// would pass every content assertion in this file and fail every `version`
/// one.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Block {
    id: Uuid,
    position: i32,
    text: String,
    version: String,
}

async fn blocks(world: &World, entity: EntityId) -> Vec<Block> {
    sqlx::query(
        "SELECT id, position, text, xmin::text AS version FROM block \
         WHERE entity_id = $1 ORDER BY position",
    )
    .bind(entity.as_uuid())
    .fetch_all(&world.db.pool)
    .await
    .expect("blocks")
    .iter()
    .map(|r| Block {
        id: r.try_get("id").unwrap(),
        position: r.try_get("position").unwrap(),
        text: r.try_get("text").unwrap(),
        version: r.try_get("version").unwrap(),
    })
    .collect()
}

fn texts(blocks: &[Block]) -> Vec<&str> {
    blocks.iter().map(|b| b.text.as_str()).collect()
}

/// The conflicts recorded on an entity that name a block, in no order the
/// store promises.
struct BlockConflict {
    block: Uuid,
    mine: Option<String>,
    theirs: Option<String>,
    mine_member: Uuid,
    theirs_member: Uuid,
}

async fn block_conflicts(world: &World, entity: EntityId) -> Vec<BlockConflict> {
    sqlx::query(
        "SELECT block_id, mine_value, theirs_value, mine_member_id, theirs_member_id \
         FROM conflict WHERE entity_id = $1 AND block_id IS NOT NULL",
    )
    .bind(entity.as_uuid())
    .fetch_all(&world.db.pool)
    .await
    .expect("conflicts")
    .iter()
    .map(|r| BlockConflict {
        block: r.try_get("block_id").unwrap(),
        mine: r.try_get("mine_value").unwrap(),
        theirs: r.try_get("theirs_value").unwrap(),
        mine_member: r.try_get("mine_member_id").unwrap(),
        theirs_member: r.try_get("theirs_member_id").unwrap(),
    })
    .collect()
}

/// Which parts of an entity `entity_part_counter` holds a block identity for.
async fn recorded_blocks(world: &World, entity: EntityId) -> Vec<Uuid> {
    sqlx::query(
        "SELECT block_id FROM entity_part_counter \
         WHERE entity_id = $1 AND block_id IS NOT NULL ORDER BY block_id",
    )
    .bind(entity.as_uuid())
    .fetch_all(&world.db.pool)
    .await
    .expect("provenance")
    .iter()
    .map(|r| r.try_get("block_id").unwrap())
    .collect()
}

async fn bulk(world: &World, entity: EntityId) -> bool {
    sqlx::query("SELECT bulk FROM entity WHERE id = $1")
        .bind(entity.as_uuid())
        .fetch_one(&world.db.pool)
        .await
        .expect("entity")
        .try_get("bulk")
        .expect("bulk")
}

/// The blocks the change sequence named, most recent entry first.
async fn last_changed_blocks(world: &World, entity: EntityId) -> Vec<Uuid> {
    sqlx::query(
        "SELECT changed_block_ids FROM change WHERE entity_id = $1 \
         ORDER BY position DESC LIMIT 1",
    )
    .bind(entity.as_uuid())
    .fetch_one(&world.db.pool)
    .await
    .expect("a change")
    .try_get::<Option<Vec<Uuid>>, _>("changed_block_ids")
    .expect("changed blocks")
    .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Writing one
// ---------------------------------------------------------------------------

fn note_body(body: &str) -> v1::entity_content::Specific {
    v1::entity_content::Specific::Note(v1::NoteContent {
        body: Some(body.into()),
        cleared: Vec::new(),
    })
}

/// A note addressing its body as cleared. Field 1 is the body; see
/// `write::specific::knowledge::NOTE_BODY`.
fn note_body_cleared() -> v1::entity_content::Specific {
    v1::entity_content::Specific::Note(v1::NoteContent {
        body: None,
        cleared: vec![1],
    })
}

async fn create_with_body(world: &World, member: &altaird::auth::Member, body: &str) -> EntityId {
    world
        .create(member, note_body(body), v1::EntityContent::default())
        .await
}

/// The same, visible to both members. A note is private to its author unless
/// the write says otherwise, and two people cannot disagree about something
/// only one of them can see.
async fn shared_with_body(world: &World, body: &str) -> EntityId {
    world
        .create(
            &world.one,
            note_body(body),
            v1::EntityContent {
                audience_member_ids: vec![
                    world.one.membership_id().as_bytes().to_vec(),
                    world.two.membership_id().as_bytes().to_vec(),
                ],
                ..Default::default()
            },
        )
        .await
}

async fn write_body(
    world: &World,
    member: &altaird::auth::Member,
    entity: EntityId,
    base: i64,
    body: &str,
) -> v1::Acknowledgement {
    world
        .submit(
            member,
            edit_entity(
                entity,
                base as u64,
                v1::EntityContent {
                    specific: Some(note_body(body)),
                    ..Default::default()
                },
            ),
        )
        .await
}

async fn set_bulk(
    world: &World,
    member: &altaird::auth::Member,
    entity: EntityId,
    value: bool,
) -> v1::Acknowledgement {
    let base = world.counter(entity).await;
    world
        .submit(
            member,
            edit_entity(
                entity,
                base as u64,
                v1::EntityContent {
                    bulk: Some(value),
                    specific: Some(v1::entity_content::Specific::Note(
                        v1::NoteContent::default(),
                    )),
                    ..Default::default()
                },
            ),
        )
        .await
}

// ---------------------------------------------------------------------------
// A body round-trips
// ---------------------------------------------------------------------------

/// Three paragraphs in, three block rows out, in order, and the rows put the
/// body back together byte for byte.
///
/// The reassembly is the assertion that matters. Nothing may foreclose export
/// (DR-001), and a division that quietly dropped the author's blank lines would
/// store something the person cannot get back.
#[tokio::test]
async fn a_body_is_stored_as_its_blocks_in_order() {
    let world = World::new().await;
    let body = "first paragraph.\n\nsecond paragraph.\n\nthird paragraph.\n";
    let note = create_with_body(&world, &world.one, body).await;

    let rows = blocks(&world, note).await;
    assert_eq!(rows.len(), 3, "{rows:#?}");
    assert_eq!(
        rows.iter().map(|b| b.position).collect::<Vec<_>>(),
        vec![0, 1, 2]
    );
    assert_eq!(
        rows.iter().map(|b| b.text.clone()).collect::<String>(),
        body,
        "the blocks do not reassemble into the body"
    );
}

/// A list divides at its items, which is the case the substrate names: two
/// people adding to a shared shopping list is among the likeliest concurrent
/// edits in a household, and it has to merge.
#[tokio::test]
async fn a_list_divides_at_its_items() {
    let world = World::new().await;
    let note = create_with_body(&world, &world.one, "- milk\n- bread\n- eggs\n").await;
    assert_eq!(blocks(&world, note).await.len(), 3);
}

/// Clearing a body is a body of no text, which divides into no blocks.
#[tokio::test]
async fn clearing_a_body_removes_every_block() {
    let world = World::new().await;
    let note = create_with_body(&world, &world.one, "one.\n\ntwo.\n").await;
    let before = blocks(&world, note).await;
    assert_eq!(before.len(), 2);

    let base = world.counter(note).await;
    world
        .submit(
            &world.one,
            edit_entity(
                note,
                base as u64,
                v1::EntityContent {
                    specific: Some(note_body_cleared()),
                    ..Default::default()
                },
            ),
        )
        .await;

    assert!(blocks(&world, note).await.is_empty());
    // The rows are gone and so is everything keyed by them, because both tables
    // that name a block do so with `ON DELETE CASCADE`.
    assert!(recorded_blocks(&world, note).await.is_empty());
    // A poller is told which blocks went, or it would have to diff the body to
    // find out.
    let mut named = last_changed_blocks(&world, note).await;
    let mut expected: Vec<Uuid> = before.iter().map(|b| b.id).collect();
    named.sort();
    expected.sort();
    assert_eq!(named, expected);
}

// ---------------------------------------------------------------------------
// Only what changed is written
// ---------------------------------------------------------------------------

/// An edit to one block leaves its neighbours' rows alone — not rewritten with
/// the same values, not written at all.
///
/// This is the assertion the whole lane hangs on, and it is made by row version
/// rather than by content precisely because content cannot see the difference.
#[tokio::test]
async fn editing_one_block_does_not_write_its_neighbours() {
    let world = World::new().await;
    let note = create_with_body(&world, &world.one, "alpha.\n\nbravo.\n\ncharlie.\n").await;
    let before = blocks(&world, note).await;

    let base = world.counter(note).await;
    write_body(
        &world,
        &world.one,
        note,
        base,
        "alpha.\n\nbravo, amended.\n\ncharlie.\n",
    )
    .await;

    let after = blocks(&world, note).await;
    assert_eq!(after.len(), 3);
    assert_eq!(
        texts(&after),
        vec!["alpha.\n\n", "bravo, amended.\n\n", "charlie.\n"]
    );

    assert_eq!(after[0].version, before[0].version, "the first block moved");
    assert_ne!(
        after[1].version, before[1].version,
        "the edited block did not"
    );
    assert_eq!(after[2].version, before[2].version, "the last block moved");

    // And only the edited block is a part this write moved.
    assert_eq!(recorded_blocks(&world, note).await.len(), 3);
    assert_eq!(last_changed_blocks(&world, note).await, vec![before[1].id]);
}

/// Rewriting the body with the same text writes nothing at all. A client
/// resending what it already sent is not an edit.
#[tokio::test]
async fn resubmitting_the_same_body_writes_no_row() {
    let world = World::new().await;
    let body = "alpha.\n\nbravo.\n";
    let note = create_with_body(&world, &world.one, body).await;
    let before = blocks(&world, note).await;

    let base = world.counter(note).await;
    write_body(&world, &world.one, note, base, body).await;

    let after = blocks(&world, note).await;
    assert_eq!(
        after.iter().map(|b| b.version.clone()).collect::<Vec<_>>(),
        before.iter().map(|b| b.version.clone()).collect::<Vec<_>>(),
        "a body write rewrote rows it had no reason to"
    );
    assert!(last_changed_blocks(&world, note).await.is_empty());
}

/// Reversing a body renumbers what survives, and the intermediate states of
/// that renumbering violate `block_position_unique`.
///
/// **This is the test that proves the constraint is deferred inside the write
/// transaction**, rather than an implementation that happens to write in a
/// lucky order. Two blocks swapping positions collide whichever one is written
/// first, so there is no order that avoids it — only a deferred constraint, or
/// a renumbering dance the schema deliberately declined to require.
#[tokio::test]
async fn reversing_a_body_passes_through_a_position_collision() {
    let world = World::new().await;
    let note = create_with_body(&world, &world.one, "alpha.\n\nbravo.\n").await;
    let before = blocks(&world, note).await;

    let base = world.counter(note).await;
    let ack = write_body(&world, &world.one, note, base, "bravo.\n\nalpha.\n").await;
    applied(&ack);

    let after = blocks(&world, note).await;
    assert_eq!(texts(&after), vec!["bravo.\n\n", "alpha.\n"]);
    // Identity followed the text, not the position: both blocks are the ones
    // that were there, swapped over.
    assert_eq!(after[0].id, before[1].id);
    assert_eq!(after[1].id, before[0].id);
    // Both rows were rewritten — the row holds the body's order and its
    // verbatim bytes — so the swap really did pass through the collision rather
    // than skirting it.
    assert_ne!(after[0].version, before[1].version);
    assert_ne!(after[1].version, before[0].version);
    // A reorder moves no block's content, so neither is a part that moved.
    assert!(last_changed_blocks(&world, note).await.is_empty());

    // Said outright as well as demonstrated, so that a schema change making the
    // constraint immediate fails with a message a reader can act on rather than
    // as an unexplained store fault in the test above.
    let deferred: bool = sqlx::query(
        "SELECT condeferrable AND condeferred AS deferred FROM pg_constraint \
         WHERE conname = 'block_position_unique'",
    )
    .fetch_one(&world.db.pool)
    .await
    .expect("the constraint")
    .try_get("deferred")
    .expect("deferred");
    assert!(deferred, "block_position_unique is no longer deferred");
}

// ---------------------------------------------------------------------------
// Identity survives
// ---------------------------------------------------------------------------

/// Rewording a block keeps its identity. This is the half a block anchor
/// relies on: rewording an entry does not detach a relation anchored to it.
#[tokio::test]
async fn a_block_keeps_its_identity_across_an_edit_to_its_own_text() {
    let world = World::new().await;
    let note = create_with_body(&world, &world.one, "- eggs\n").await;
    let before = blocks(&world, note).await;

    let base = world.counter(note).await;
    write_body(&world, &world.one, note, base, "- half a dozen eggs\n").await;

    let after = blocks(&world, note).await;
    assert_eq!(after.len(), 1);
    assert_eq!(
        after[0].id, before[0].id,
        "an expanded entry became a new block"
    );
}

/// Editing a block's neighbours — inserting one above it, removing one below —
/// leaves its identity and its row alone.
#[tokio::test]
async fn a_block_keeps_its_identity_across_edits_to_its_neighbours() {
    let world = World::new().await;
    let note = create_with_body(&world, &world.one, "- milk\n- bread\n- eggs\n").await;
    let before = blocks(&world, note).await;
    let bread = before[1].clone();

    let base = world.counter(note).await;
    write_body(
        &world,
        &world.one,
        note,
        base,
        "- coffee\n- milk\n- bread\n",
    )
    .await;

    let after = blocks(&world, note).await;
    assert_eq!(texts(&after), vec!["- coffee\n", "- milk\n", "- bread\n"]);
    let moved = after
        .iter()
        .find(|b| b.id == bread.id)
        .expect("bread lost its identity when its neighbours changed");
    assert_eq!(moved.text, bread.text, "bread's text changed");
    assert_eq!(moved.position, 2, "bread did not follow the body's order");
    // Its row was written, because the body's order lives in it. Its *text* was
    // not, which is why it is not a part this write moved.
    assert_ne!(moved.version, bread.version);
    assert!(!last_changed_blocks(&world, note).await.contains(&bread.id));
}

/// A block the body no longer contains loses its row, and everything keyed by
/// the row goes with it.
///
/// The schema does the last part itself: `relation.anchor_block_id` is
/// `ON DELETE SET NULL`, so a relation anchored into a removed block survives
/// without its anchor — the substrate's rule for a removed block, enforced by
/// the store rather than by this file.
#[tokio::test]
async fn a_removed_block_takes_its_row_and_its_provenance_with_it() {
    let world = World::new().await;
    let note = create_with_body(&world, &world.one, "- milk\n- bread\n").await;
    let before = blocks(&world, note).await;
    let bread = before[1].id;

    let base = world.counter(note).await;
    write_body(&world, &world.one, note, base, "- milk\n").await;

    let after = blocks(&world, note).await;
    assert_eq!(texts(&after), vec!["- milk\n"]);
    assert!(!recorded_blocks(&world, note).await.contains(&bread));
    assert!(last_changed_blocks(&world, note).await.contains(&bread));
}

// ---------------------------------------------------------------------------
// What an anchor will be able to rely on
// ---------------------------------------------------------------------------
//
// `write/relation.rs` still refuses an anchor, and that refusal is not this
// lane's to lift. These two tests state what the implementation lifting it will
// find already true, and they set the anchors directly in the store because the
// write path has no way to yet.

/// Attach an anchor the write path cannot yet form.
async fn anchor(world: &World, relation: Uuid, entity: EntityId, block: Uuid, phrase: &str) {
    sqlx::query(
        "UPDATE relation SET anchor_entity_id = $2, anchor_block_id = $3, anchor_phrase = $4 \
         WHERE id = $1",
    )
    .bind(relation)
    .bind(entity.as_uuid())
    .bind(block)
    .bind(phrase)
    .execute(&world.db.pool)
    .await
    .expect("anchor");
}

async fn anchor_of(world: &World, relation: Uuid) -> (Option<Uuid>, Option<Uuid>, Option<String>) {
    let r = sqlx::query(
        "SELECT anchor_entity_id, anchor_block_id, anchor_phrase FROM relation WHERE id = $1",
    )
    .bind(relation)
    .fetch_one(&world.db.pool)
    .await
    .expect("relation");
    (
        r.try_get("anchor_entity_id").unwrap(),
        r.try_get("anchor_block_id").unwrap(),
        r.try_get("anchor_phrase").unwrap(),
    )
}

async fn relate(world: &World, from: EntityId, to: EntityId) -> Uuid {
    let ack = world
        .submit(
            &world.one,
            create_relation(Uuid::new_v4(), relation_between(from, to)),
        )
        .await;
    relation_id(&ack)
}

/// Rewording a block does not detach a relation anchored to it. The row keeps
/// its identity, so the anchor keeps pointing at the same block.
#[tokio::test]
async fn rewording_a_block_leaves_an_anchor_into_it_alone() {
    let world = World::new().await;
    let note = create_with_body(&world, &world.one, "- eggs\n\n- milk\n").await;
    let other = world.note(&world.one, "the other end").await;
    let eggs = blocks(&world, note).await[0].id;
    let relation = relate(&world, note, other).await;
    anchor(&world, relation, note, eggs, "eggs").await;

    let base = world.counter(note).await;
    write_body(
        &world,
        &world.one,
        note,
        base,
        "- half a dozen eggs\n\n- milk\n",
    )
    .await;

    assert_eq!(
        anchor_of(&world, relation).await,
        (Some(note.as_uuid()), Some(eggs), Some("eggs".into())),
        "rewording an entry detached a relation anchored to the entry itself"
    );
}

/// Removing a block leaves the relation, without its anchor — and, critically,
/// **the body write does not fail**.
///
/// The store detaches half the anchor by itself and refuses the row that
/// produces: `anchor_phrase` may not outlive `anchor_block_id`, so the delete
/// raises `relation_check3` from inside the cascading update and the whole
/// intent becomes a store fault. Migration one declines a unique index over the
/// unanchored pair for exactly this reason — *a body edit must never fail
/// because a relation lost an anchor* — and this is the same failure by another
/// route. `write/body.rs` clears the phrase first; without that line this test
/// is a fault rather than a failure.
#[tokio::test]
async fn removing_a_block_leaves_the_relation_without_its_anchor_and_does_not_fail() {
    let world = World::new().await;
    let note = create_with_body(&world, &world.one, "- eggs\n\n- milk\n").await;
    let other = world.note(&world.one, "the other end").await;
    let eggs = blocks(&world, note).await[0].id;
    let relation = relate(&world, note, other).await;
    anchor(&world, relation, note, eggs, "eggs").await;

    let base = world.counter(note).await;
    let ack = write_body(&world, &world.one, note, base, "- milk\n").await;
    applied(&ack);

    assert_eq!(
        anchor_of(&world, relation).await,
        (Some(note.as_uuid()), None, None),
        "the relation did not survive its block, or kept a phrase locating into nothing"
    );
    assert_eq!(
        world.relation_lifecycle(relation).await.as_deref(),
        Some("active")
    );
}

// ---------------------------------------------------------------------------
// A block is a part
// ---------------------------------------------------------------------------

/// Two people editing the same block from one base: both values retained, the
/// conflict names the block, and the acknowledgement carries its identity.
///
/// **A stale base is never a rejection.** The second write applies; what a
/// stale base produces is a conflict, and only over a part that actually
/// overlaps.
#[tokio::test]
async fn two_edits_to_one_block_retain_both_values() {
    let world = World::new().await;
    let note = shared_with_body(&world, "alpha.\n\nbravo.\n").await;
    let block = blocks(&world, note).await[0].id;
    let base = world.counter(note).await;

    write_body(&world, &world.one, note, base, "alpha, hers.\n\nbravo.\n").await;
    let ack = write_body(&world, &world.two, note, base, "alpha, his.\n\nbravo.\n").await;

    let applied = applied(&ack);
    let conflict = applied
        .conflict
        .as_ref()
        .expect("a conflict was not reported");
    assert_eq!(
        conflict.block_ids,
        vec![block.as_bytes().to_vec()],
        "the acknowledgement did not name the conflicted block"
    );
    assert!(conflict.content_fields.is_empty() && conflict.specific_fields.is_empty());

    let retained = block_conflicts(&world, note).await;
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].block, block);
    assert_eq!(retained[0].mine.as_deref(), Some("alpha, his."));
    assert_eq!(retained[0].theirs.as_deref(), Some("alpha, hers."));
    assert_eq!(retained[0].mine_member, world.two.membership_id());
    assert_eq!(retained[0].theirs_member, world.one.membership_id());

    // The write still applied. Both values are held; the arriving one is the
    // entity's.
    assert_eq!(
        texts(&blocks(&world, note).await),
        vec!["alpha, his.\n\n", "bravo.\n"]
    );
}

/// Two people editing different blocks of one note from different bases: both
/// land, nobody is involved, and the note holds both edits.
///
/// This is what block-level granularity buys, and it is the reason a body is
/// many parts rather than one field.
#[tokio::test]
async fn two_edits_to_different_blocks_merge() {
    let world = World::new().await;
    let note = shared_with_body(&world, "alpha.\n\nbravo.\n").await;
    let base = world.counter(note).await;

    write_body(&world, &world.one, note, base, "alpha, hers.\n\nbravo.\n").await;
    let ack = write_body(
        &world,
        &world.two,
        note,
        base,
        "alpha, hers.\n\nbravo, his.\n",
    )
    .await;

    let applied = applied(&ack);
    assert!(applied.conflict.is_none(), "{:?}", applied.conflict);
    assert!(block_conflicts(&world, note).await.is_empty());
    assert_eq!(
        texts(&blocks(&world, note).await),
        vec!["alpha, hers.\n\n", "bravo, his.\n"]
    );
}

/// Two writes producing the same block text are not divergent, whatever their
/// base counter said. Two people resolving one conflict the same way is the
/// likely case, and forming a second conflict out of two people agreeing is the
/// machinery working against its own purpose.
#[tokio::test]
async fn two_edits_settling_on_one_wording_are_not_a_conflict() {
    let world = World::new().await;
    let note = shared_with_body(&world, "alpha.\n").await;
    let base = world.counter(note).await;

    write_body(&world, &world.one, note, base, "alpha, agreed.\n").await;
    let ack = write_body(&world, &world.two, note, base, "alpha, agreed.\n").await;

    assert!(applied(&ack).conflict.is_none());
    assert!(block_conflicts(&world, note).await.is_empty());
}

/// Removing a paragraph does not conflict with somebody's edit to the ones
/// below it.
///
/// A removal renumbers everything after it, and if a position move counted as a
/// part moving, deleting the first paragraph of a long note would conflict with
/// every concurrent edit to the rest of it. That is the opposite of what block
/// grain is for.
#[tokio::test]
async fn a_block_that_only_shifted_position_is_not_a_conflicting_part() {
    let world = World::new().await;
    let note = shared_with_body(&world, "alpha.\n\nbravo.\n\ncharlie.\n").await;
    let base = world.counter(note).await;

    // She deletes the first paragraph, shifting the other two up.
    write_body(&world, &world.one, note, base, "bravo.\n\ncharlie.\n").await;
    // He, from the old base, rewords the last one.
    let ack = write_body(&world, &world.two, note, base, "bravo.\n\ncharlie, his.\n").await;

    assert!(
        applied(&ack).conflict.is_none(),
        "{:?}",
        applied(&ack).conflict
    );
    assert_eq!(
        texts(&blocks(&world, note).await),
        vec!["bravo.\n\n", "charlie, his.\n"]
    );
}

// ---------------------------------------------------------------------------
// Bulk graduation
// ---------------------------------------------------------------------------
//
// No Guidance type carries a body — `docs/altair-data-model.md:44` — so
// graduation is unreachable for a campaign, an arc, or a quest except by the
// person overriding it explicitly. That is not an omission from this section
// and there is nothing for it to test.

/// Authored body content graduates a bulk entity.
#[tokio::test]
async fn authored_body_content_graduates_a_bulk_entity() {
    let world = World::new().await;
    let note = world.note(&world.one, "a scan").await;
    set_bulk(&world, &world.one, note, true).await;
    assert!(bulk(&world, note).await);

    let base = world.counter(note).await;
    write_body(&world, &world.one, note, base, "somebody wrote this.\n").await;
    assert!(
        !bulk(&world, note).await,
        "authored body content did not graduate"
    );
}

/// A relation does not, because relations occur during ordinary bulk workflows.
///
/// Derived text does not either, and there is nothing to exercise: no path
/// writes derived text before the derivation worker in Wave 5. What keeps that
/// true is that graduation is written once, in `write/body.rs`, and a
/// derivation path has no reason to reach it.
#[tokio::test]
async fn a_relation_does_not_graduate_a_bulk_entity() {
    let world = World::new().await;
    let note = world.note(&world.one, "a scan").await;
    let other = world.note(&world.one, "something else").await;
    set_bulk(&world, &world.one, note, true).await;

    let ack = world
        .submit(
            &world.one,
            create_relation(Uuid::new_v4(), relation_between(note, other)),
        )
        .await;
    applied(&ack);

    assert!(bulk(&world, note).await, "a relation graduated an entity");
}

/// Editing metadata does not graduate. The vision's rule is that an entity
/// leaves the state when the person edits something *beyond* metadata, and a
/// title is metadata.
#[tokio::test]
async fn editing_a_title_does_not_graduate_a_bulk_entity() {
    let world = World::new().await;
    let note = world.note(&world.one, "a scan").await;
    set_bulk(&world, &world.one, note, true).await;

    let base = world.counter(note).await;
    world
        .submit(
            &world.one,
            edit_entity(
                note,
                base as u64,
                v1::EntityContent {
                    title: Some("a scan, renamed".into()),
                    specific: Some(v1::entity_content::Specific::Note(
                        v1::NoteContent::default(),
                    )),
                    ..Default::default()
                },
            ),
        )
        .await;

    assert!(bulk(&world, note).await);
}

/// Reordering or removing blocks does not graduate. Neither adds anything a
/// person wrote, and the transition is one-way — firing it on a deletion would
/// be an irreversible move made on the weakest evidence available.
#[tokio::test]
async fn a_reorder_or_a_removal_does_not_graduate() {
    let world = World::new().await;
    let note = create_with_body(&world, &world.one, "alpha.\n\nbravo.\n").await;
    set_bulk(&world, &world.one, note, true).await;

    let base = world.counter(note).await;
    write_body(&world, &world.one, note, base, "bravo.\n\nalpha.\n").await;
    assert!(bulk(&world, note).await, "a reorder graduated");

    let base = world.counter(note).await;
    write_body(&world, &world.one, note, base, "bravo.\n").await;
    assert!(bulk(&world, note).await, "a removal graduated");
}

/// The transition is one-way automatically, and the person overrides it in both
/// directions.
#[tokio::test]
async fn the_person_overrides_graduation_in_both_directions() {
    let world = World::new().await;
    let note = world.note(&world.one, "a scan").await;

    // Into the state, which nothing automatic ever does.
    set_bulk(&world, &world.one, note, true).await;
    assert!(bulk(&world, note).await);

    // Out of it, by authoring content.
    let base = world.counter(note).await;
    write_body(&world, &world.one, note, base, "somebody wrote this.\n").await;
    assert!(!bulk(&world, note).await);

    // And back into it, over the top of the heuristic. The heuristic will be
    // wrong sometimes and a filter that hides something a person needs is the
    // failure the whole mechanism exists to prevent.
    set_bulk(&world, &world.one, note, true).await;
    assert!(bulk(&world, note).await);

    // Out of it explicitly, with no body write involved.
    set_bulk(&world, &world.one, note, false).await;
    assert!(!bulk(&world, note).await);

    // Once out, a further body write leaves it out rather than writing again.
    let base = world.counter(note).await;
    write_body(
        &world,
        &world.one,
        note,
        base,
        "more.\n\nsomebody wrote this.\n",
    )
    .await;
    assert!(!bulk(&world, note).await);
}

/// **A known gap, pinned so it stays visible.** *"Initial value derives from
/// capture method"* — nothing derives it. `write::entity::create` inserts
/// `bulk = false` whatever the method was, and choosing which methods mean bulk
/// is a decision no document makes: the only named case is a file upload, and a
/// file is refused until Wave 2.3.
///
/// Delete this test when the derivation lands, rather than adjusting it.
#[tokio::test]
async fn bulk_is_not_yet_derived_from_the_capture_method() {
    let world = World::new().await;
    let note = world.note(&world.one, "a scan").await;
    assert!(!bulk(&world, note).await);
}

/// A creation carrying a body does not graduate, whatever the body says.
///
/// *"Initial value derives from capture method"*, and content that arrives with
/// a creation was captured rather than authored here — an import of two hundred
/// notes arrives with two hundred bodies, and graduating each of them would
/// make the column mean nothing for every type that carries one.
#[tokio::test]
async fn a_creation_carrying_a_body_does_not_graduate() {
    let world = World::new().await;
    let note = world
        .create(
            &world.one,
            note_body("captured, not authored.\n"),
            v1::EntityContent {
                bulk: Some(true),
                ..Default::default()
            },
        )
        .await;
    assert!(
        bulk(&world, note).await,
        "a creation graduated its own body"
    );
    assert_eq!(blocks(&world, note).await.len(), 1);

    // And the very next write authoring into it does graduate, so what is
    // withheld above is the occasion rather than the rule.
    let base = world.counter(note).await;
    write_body(
        &world,
        &world.one,
        note,
        base,
        "captured, not authored.\n\nand now authored.\n",
    )
    .await;
    assert!(!bulk(&world, note).await);
}

/// A write that states `bulk` is not overruled by its own body.
///
/// The person said what they wanted in the same message. A derived value does
/// not outrank an authored one, and the parts are applied before the body, so
/// without this the statement would be overwritten by the rule meant to guess
/// at it.
#[tokio::test]
async fn a_write_that_states_bulk_is_not_overruled_by_its_own_body() {
    let world = World::new().await;
    let note = create_with_body(&world, &world.one, "alpha.\n").await;

    let base = world.counter(note).await;
    world
        .submit(
            &world.one,
            edit_entity(
                note,
                base as u64,
                v1::EntityContent {
                    bulk: Some(true),
                    specific: Some(note_body("alpha, authored in the same breath.\n")),
                    ..Default::default()
                },
            ),
        )
        .await;

    assert!(bulk(&world, note).await, "graduation overruled the person");
    // The body still landed. Withholding graduation withholds nothing else.
    assert_eq!(
        texts(&blocks(&world, note).await),
        vec!["alpha, authored in the same breath.\n"]
    );
}

// ---------------------------------------------------------------------------
// A body on a type that carries none
// ---------------------------------------------------------------------------

/// The write path refuses it, and says why.
///
/// Reached directly rather than through the wire, because no type outside a
/// note and a shopping list has a body field for a client to fill in. The
/// refusal exists so that a type gaining one, or a caller wiring a dispatch
/// wrongly, gets an answer instead of a store fault.
#[tokio::test]
async fn a_body_on_a_type_that_carries_none_is_refused_by_the_write_path() {
    let world = World::new().await;
    let campaign = world
        .create(
            &world.one,
            v1::entity_content::Specific::Campaign(v1::CampaignContent::default()),
            v1::EntityContent::default(),
        )
        .await;

    let mut tx = begin_write(&world.db.pool).await.expect("a transaction");
    let mut ctx = Ctx {
        tx: &mut tx,
        member: MemberId::for_test(world.one.membership_id()),
        at: Utc::now(),
    };
    let outcome = body::apply(
        &mut ctx,
        campaign,
        EntityType::Campaign,
        &BodyWrite(Some("a campaign has no body.\n".into())),
        body::Occasion::Editing { states_bulk: false },
    )
    .await;
    let detail = match outcome {
        Err(Failed::Refused(Refusal::Malformed(detail))) => detail,
        Err(Failed::Refused(Refusal::NotAvailable)) => panic!("refused as unavailable"),
        Err(Failed::Store(e)) => panic!("the store answered instead of the write path: {e}"),
        Ok(_) => panic!("a campaign was given a body"),
    };
    assert!(
        detail.contains("campaign") && detail.contains("no body"),
        "{detail}"
    );
    tx.rollback().await.expect("rollback");

    // And the store would have refused it too, which is the check this one
    // stands in front of. Second, so that a write path that stopped refusing
    // would fail on the assertion above rather than pass on this one.
    let failed = sqlx::query(
        "INSERT INTO block (id, entity_id, body_type, position, text) \
         VALUES ($1, $2, 'campaign', 0, 'x')",
    )
    .bind(Uuid::new_v4())
    .bind(campaign.as_uuid())
    .execute(&world.db.pool)
    .await;
    assert!(failed.is_err(), "the store admitted a block on a campaign");
}

/// A note and a shopping list are the two the store admits, and the write path
/// mirrors that set rather than a smaller one of its own.
#[test]
fn the_write_path_admits_exactly_the_types_the_store_does() {
    use EntityType::*;
    for kind in [
        Campaign,
        Arc,
        Quest,
        Routine,
        FocusSession,
        CheckIn,
        Note,
        File,
        Item,
        Location,
        ShoppingList,
        Category,
    ] {
        assert_eq!(
            body::carries_a_body(kind),
            matches!(kind, Note | ShoppingList),
            "{kind:?}"
        );
    }
}

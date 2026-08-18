//! Writing a body: dividing it, matching it against the blocks already held,
//! and writing only what changed.
//!
//! **LANE: Bodies owns this file, and owns only this file.** The dispatch on
//! either side of it is written: [`super::content`] reads a body off the wire
//! into a [`BodyWrite`], [`super::entity`] hands one here for every verb that
//! can carry one, and everything this returns is already wired into
//! provenance, conflict detection, and the change sequence. What is missing is
//! the middle.
//!
//! # Why a body is not a part
//!
//! Everything else a write addresses is one part with one value. A body is one
//! field on the wire and many parts in the store: the substrate makes the
//! *block* the smallest independently addressable unit of a body, because
//! "treating the field as the unit would make two people working on different
//! paragraphs of a shared plan look like a disagreement". Which blocks a body
//! write touches is not knowable from the text alone — it depends on the blocks
//! already stored — so this is the one place where reading the wire cannot say
//! what a write touched, and the answer comes back from here instead.
//!
//! # What is already built, and must be used rather than rewritten
//!
//! Wave 1.2 landed both halves of the rule as pure functions, and DR-004 puts
//! them at the instance so devices cannot disagree about the units
//! reconciliation is decided in:
//!
//! - [`crate::body::divide`] — text to boundaries. Atomic constructs never
//!   split; list items do; a long unbroken stretch of prose being one block is
//!   correct rather than a bug.
//! - [`crate::body::reconcile`] — matching the new boundaries against the
//!   stored blocks so identity survives an edit to a block and to its
//!   neighbours.
//!
//! Nothing here decides a boundary or a match. This file is the wiring between
//! those two functions and the store, and the whole of its judgement is in
//! which rows it then writes and which parts it reports having moved.
//!
//! # Writing only what changed
//!
//! *"Boundaries are computed from the text and identity is not. A body write
//! recomputes boundaries, matches them against the rows already here, keeps the
//! identity of what survived, and writes only what changed."* — migration one,
//! above the `block` table.
//!
//! Taken literally, and the literal reading is the load-bearing one. A body
//! write that deleted every row and reinserted them would reach the same end
//! state and would be wrong twice over: every block would take a fresh
//! identity, so every anchor into the body would detach and every part's
//! provenance would reset, and the change sequence would name the whole body as
//! having moved when one paragraph did. So an unchanged block is not written at
//! all — not re-`UPDATE`d with the same values — and `tests/type_content_bodies.rs`
//! asserts that by reading the rows' `xmin` rather than their contents.
//!
//! **The intermediate states of a renumbering are not violations.**
//! `block_position_unique` is `DEFERRABLE INITIALLY DEFERRED` for exactly this:
//! reversing a body's blocks moves the first onto the second's position while
//! the second is still there. Nothing here sorts the updates to avoid that, and
//! nothing should — a renumbering dance would be an implementation of a
//! constraint the schema deliberately declined to impose.
//!
//! # Two different questions about one block, and they have different answers
//!
//! **Whether the row has to be rewritten** is about the verbatim slice: the row
//! holds the body's bytes and its order, so a block whose blank line moved, or
//! whose position shifted, needs writing or the body no longer reassembles.
//!
//! **Whether the part moved** is about the block's content — the same slice
//! with the surrounding blank space trimmed off, which is what
//! [`crate::body::content_of`] returns and what identity matching already
//! compares. Division states the rule in as many words: *a block is not
//! "reworded" because somebody pressed return underneath it.*
//!
//! The two come apart on every reorder, because whitespace between two blocks
//! is attributed to the block before it, so swapping two paragraphs changes
//! both verbatim slices while changing neither paragraph. Treating that as
//! movement would make reordering a body conflict with every concurrent edit to
//! it, name the whole body in the change sequence, and graduate a bulk entity
//! nobody wrote a word into. So the row write is decided on the verbatim text
//! and everything a part means is decided on the content.
//!
//! [`ResolvedBlock::text_changed`] is the verbatim comparison and is used for
//! the first question only. This is the one place this file deliberately reads
//! Wave 1.2's output more finely than its doc comment suggests, which is why it
//! is written down here.
//!
//! # What a body write reports having touched
//!
//! - **A block whose content moved** — carried identity, different content —
//!   is a part this write moved and a part it can conflict on. It goes in all
//!   three of [`BodyTouch::touching`], [`BodyTouch::written`] and
//!   [`BodyTouch::changed`].
//! - **A block this write created** goes in `written` and `changed` but not in
//!   `touching`. Its identity was minted here and no earlier write can have
//!   moved it, so it is not a part anything can disagree about.
//! - **A block that only moved position, or only gained a blank line**, goes in
//!   none of them, though its row is rewritten. Recording it as moved would
//!   make deleting one paragraph conflict with somebody's edit to every
//!   paragraph below it, which is the opposite of what block-level granularity
//!   is for.
//! - **A block this write removed** goes in `changed` alone. Removing one also
//!   lets go of the relations anchored into it, half of which the store does
//!   itself and half of which it refuses to — see [`detach_anchors`].
//!
//! ## A removed block carries no conflict, and the schema is what decides that
//!
//! Both tables keyed by a part name a block through
//! `REFERENCES block (id) ON DELETE CASCADE`. A conflict about a block that no
//! longer exists is therefore not a representable row: written before the
//! delete it cascades away, written after it violates the key. So a removal is
//! not a conflicted part, and where somebody else reworded a block that this
//! write deletes, their wording goes with the block and nothing is retained.
//!
//! That is a real loss and it is stated rather than hidden. It is the ordinary
//! case the substrate already describes — *overwriting someone else's work on a
//! shared entity is already possible, already unremarkable, and already
//! unprevented* — arriving at block grain, and the person who submitted the
//! removal asked for exactly the end state they got. Nothing quieter is
//! available without a schema that can hold a conflict about a row that is
//! gone.
//!
//! # Bulk graduation
//!
//! *"Relations and derived text do not graduate an entity, because both occur
//! during ordinary bulk workflows. Authored body content does."* — the
//! substrate. That sentence is why graduation lives here and not in a domain
//! lane: this is the only place that can see body content being authored.
//!
//! See [`Occasion`] for what the caller has to say, and [`graduate`] for the
//! rule itself.

use sqlx::Row;
use uuid::Uuid;

use crate::body::{Origin, Reconciliation, StoredBlock, content_of, reconcile};
use crate::store::entity::EntityType;
use crate::store::ids::EntityId;

use super::content::BodyWrite;
use super::entity::{Applied, Ctx, Refusal};
use super::parts::Part;

/// What writing a body touched, at the grain a conflict is decided at.
#[derive(Debug, Clone, Default)]
pub struct BodyTouch {
    /// Per block: the part, the text arriving, and the text it displaced. This
    /// is the shape conflict detection takes, and it is a list because a body
    /// write is many parts at once.
    pub touching: Vec<(Part, Option<String>, Option<String>)>,
    /// Every block part this write moved, for provenance. A superset of the
    /// parts that conflicted and a subset of the blocks the body has.
    pub written: Vec<Part>,
    /// The blocks the change sequence should name, so a poller learns which
    /// parts of a body it needs rather than the whole thing.
    pub changed: Vec<Uuid>,
}

/// Whether this type's content includes a body.
///
/// **The store refuses a block on any other type**, which is
/// `block.body_type CHECK (body_type IN ('note', 'shopping_list'))`. This is
/// the same fact stated where the write path can act on it, so a body on a
/// campaign is a refusal a client can read rather than a constraint violation
/// that takes the transaction down with it.
///
/// A shopping list is admitted here because the table admits it. Its *content*
/// is deferred in v0 and Tracking refuses it a step earlier, so nothing reaches
/// this file with one today; the type is listed anyway because the store's
/// check is what this mirrors, and mirroring it selectively would be a second
/// rule to keep in step.
#[must_use]
pub fn carries_a_body(kind: EntityType) -> bool {
    matches!(kind, EntityType::Note | EntityType::ShoppingList)
}

/// The blocks the store holds for an entity, in position order.
///
/// Position order and nothing else: [`reconcile`] takes the index in this slice
/// as the block's position, and uses it only to break ties between equally good
/// matches.
async fn stored_blocks(ctx: &mut Ctx<'_>, entity: EntityId) -> Applied<Vec<StoredBlock>> {
    let rows = sqlx::query("SELECT id, text FROM block WHERE entity_id = $1 ORDER BY position")
        .bind(entity.as_uuid())
        .fetch_all(ctx.tx.conn())
        .await?;
    let mut blocks = Vec::with_capacity(rows.len());
    for r in &rows {
        blocks.push(StoredBlock {
            id: r.try_get("id")?,
            text: r.try_get("text")?,
        });
    }
    Ok(blocks)
}

/// What the write around a body is, so far as a body needs to know.
///
/// **Read by bulk graduation and by nothing else**, which is why it carries the
/// two facts it does and no more.
///
/// # Why the caller has to say, when it looks as though this file could tell
///
/// Every signal available from inside [`apply`] is silently wrong, and each
/// looks right long enough to be reached for:
///
/// - **The entity's counter is 1 on a create *and* on the first edit after
///   one**, because [`super::entity::edit`] writes the new counter after the
///   body rather than before it. So a counter test misses exactly the first
///   edit somebody makes to a freshly captured entity, and misses it silently.
/// - **`xmin` on the entity row is the current transaction's on any edit that
///   also set a title, a category, or an audience**, because `apply_part` has
///   already `UPDATE`d the row by the time a body is written. So "was this row
///   inserted here" answers yes for half of all edits.
/// - **Prior `change` rows** would work and would make the write path depend on
///   the sequence trimmer not having run, which `0002_write_provenance.sql`
///   rejects in as many words for `entity_part_counter`.
/// - **Prior `entity_part_counter` rows** are absent for a create that carried
///   no content, so the first edit after one misreads as a create.
///
/// A parameter is the only honest answer, and it is one word at each of the two
/// call sites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Occasion {
    /// The entity is coming into existence in this write.
    Creating,
    /// The entity was already here. `states_bulk` is true where the same write
    /// addresses `bulk` itself.
    Editing { states_bulk: bool },
}

impl Occasion {
    /// Whether authored body content may graduate this entity.
    ///
    /// - **Never on a creation.** *"Initial value derives from capture
    ///   method"*, and content arriving with a creation was captured rather
    ///   than authored here — an import of two hundred notes arrives with two
    ///   hundred bodies, and graduating each of them would make the column mean
    ///   nothing for every type that carries one.
    /// - **Never against the person's own statement.** A write that says
    ///   `bulk` says what the person decided, and *"the transition is one-way,
    ///   and overridable by the user in both directions"*. A derived value does
    ///   not outrank an authored one — the same rule the standing constraints
    ///   state one layer up, about a person's edit to derived data.
    #[must_use]
    fn may_graduate(self) -> bool {
        matches!(self, Self::Editing { states_bulk: false })
    }
}

/// Leave the bulk state, because somebody authored content into the body.
///
/// # The rule
///
/// - **Authored body content graduates.** A block created, or a block whose
///   text changed. Both are somebody putting words into this entity.
/// - **A reorder or a removal does not.** Neither adds anything a person
///   wrote, and the transition is one-way, so firing it on a deletion would be
///   an irreversible move made on the weakest evidence available.
/// - **Relations and derived text do not**, which needs no code here: neither
///   reaches this file. Stated because it is the clause most likely to be
///   "fixed" by somebody adding a graduation to the relation path.
/// - **A creation and an explicit statement withhold it**, which is
///   [`Occasion`].
///
/// The `WHERE bulk` is what makes it one-way and idempotent: an entity that has
/// already graduated is not written again, so a body edit does not touch the
/// entity row for nothing.
///
/// **What still does not happen anywhere is the initial derivation.**
/// `create` inserts `bulk = false` whatever the capture method was, because no
/// document maps the open set of methods onto the state and the only named case
/// is a file upload, which is refused until Wave 2.3. That gap is pinned by a
/// test rather than guessed at here; choosing the mapping in the write path
/// would be a product decision taken in the wrong layer.
async fn graduate(ctx: &mut Ctx<'_>, entity: EntityId) -> Applied<()> {
    sqlx::query("UPDATE entity SET bulk = false WHERE id = $1 AND bulk")
        .bind(entity.as_uuid())
        .execute(ctx.tx.conn())
        .await?;
    Ok(())
}

/// Let go of the phrases anchored into blocks this write is about to remove.
///
/// **Without this, removing a block fails the body write.** The store detaches
/// half an anchor by itself — `relation.anchor_block_id` is
/// `ON DELETE SET NULL` — and the other half has no owner: `relation`'s
/// `CHECK (anchor_phrase IS NULL OR anchor_block_id IS NOT NULL)` refuses the
/// row the referential action produces, so the delete raises a constraint
/// violation and the whole intent becomes a store fault. Confirmed against
/// PostgreSQL 18 rather than reasoned about: the failure arrives as
/// `relation_check3`, from inside the cascading `UPDATE`.
///
/// That is the exact failure migration one declines a unique index to avoid —
/// *"a body edit must never fail because a relation lost an anchor, and that
/// outranks refusing a duplicate structurally"* — reintroduced by another
/// route. It is unreachable today only because `write::relation` still refuses
/// an anchor; it becomes reachable the moment that refusal expires, which is
/// the item that follows this lane.
///
/// Clearing the phrase is also what the substrate means rather than a way
/// around a check: *"where the block itself is removed the relation survives
/// without its anchor"*. A phrase locating into a block that is gone locates
/// into nothing. `anchor_entity_id` stays, because where the relation was
/// formed is still true.
async fn detach_anchors(ctx: &mut Ctx<'_>, removed: &[Uuid]) -> Applied<()> {
    sqlx::query("UPDATE relation SET anchor_phrase = NULL WHERE anchor_block_id = ANY($1)")
        .bind(removed)
        .execute(ctx.tx.conn())
        .await?;
    Ok(())
}

/// Write a body, returning what it touched.
///
/// # Errors
///
/// Refuses a body on a type that carries none, and faults only where the store
/// could not be written.
pub async fn apply(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    body: &BodyWrite,
    occasion: Occasion,
) -> Applied<BodyTouch> {
    if !carries_a_body(kind) {
        return Err(Refusal::Malformed(format!(
            "a body divides into blocks and only a note and a shopping list hold any, \
             so a {} has no body to write",
            super::entity::type_name(kind)
        ))
        .into());
    }

    let stored = stored_blocks(ctx, entity).await?;

    // Clearing a body is a body of no text, which divides into no blocks and
    // therefore removes every one of them. Distinct from an untouched body,
    // which never reaches this function at all.
    let text = body.0.as_deref().unwrap_or_default();
    let Reconciliation { blocks, removed } = reconcile(&stored, text);

    let mut touch = BodyTouch::default();

    // Removals first. Not because the unique constraint needs it — it is
    // deferred and does not — but because a removed block's provenance and any
    // conflict naming it cascade away with the row, and doing that before
    // anything else is written keeps the order of those cascades from mattering.
    if !removed.is_empty() {
        detach_anchors(ctx, &removed).await?;
        sqlx::query("DELETE FROM block WHERE entity_id = $1 AND id = ANY($2)")
            .bind(entity.as_uuid())
            .bind(&removed)
            .execute(ctx.tx.conn())
            .await?;
        touch.changed.extend(removed);
    }

    let body_type = super::entity::type_name(kind);
    let mut authored = false;

    for block in &blocks {
        let displaced = stored
            .iter()
            .find(|s| s.id == block.id)
            .map(|s| s.text.as_str());

        match block.origin {
            Origin::Created => {
                sqlx::query(
                    "INSERT INTO block (id, entity_id, body_type, position, text) \
                     VALUES ($1, $2, $3::entity_type, $4, $5)",
                )
                .bind(block.id)
                .bind(entity.as_uuid())
                .bind(body_type)
                .bind(i32::try_from(block.position).unwrap_or(i32::MAX))
                .bind(&block.text)
                .execute(ctx.tx.conn())
                .await?;
            }
            // The verbatim slice and the position are what the row holds, so
            // this is where `text_changed` is the right comparison and the only
            // place it is. A block that has neither is not written at all — see
            // the module documentation: this is the whole of "writes only what
            // changed", and it is one branch rather than a comment because the
            // obvious rewrite of this loop loses it.
            Origin::Carried if !block.text_changed && !block.moved => {}
            Origin::Carried => {
                sqlx::query("UPDATE block SET position = $2, text = $3 WHERE id = $1")
                    .bind(block.id)
                    .bind(i32::try_from(block.position).unwrap_or(i32::MAX))
                    .bind(&block.text)
                    .execute(ctx.tx.conn())
                    .await?;
            }
        }

        // And here it is the content, because this is the question about the
        // part rather than about the row.
        let arriving = content_of(&block.text);
        let before = displaced.map(content_of);
        if before == Some(arriving) {
            continue;
        }

        authored = true;
        touch.written.push(Part::Block(block.id));
        touch.changed.push(block.id);

        // Only a carried block can be one side of a disagreement. A created
        // block's identity was minted a few lines above and no earlier write
        // can have named it, so it would match nothing in `moved_since` and
        // adding it here would only cost the comparison.
        if let Some(before) = before {
            touch.touching.push((
                Part::Block(block.id),
                Some(arriving.to_owned()),
                Some(before.to_owned()),
            ));
        }
    }

    if authored && occasion.may_graduate() {
        graduate(ctx, entity).await?;
    }

    Ok(touch)
}

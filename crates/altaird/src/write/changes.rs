//! The change sequence: one entry per write, in commit order.
//!
//! # Read this before you make writes stop serialising on one row
//!
//! **Every write in the instance takes an exclusive lock on a single row, and
//! that is the design rather than an oversight.** It is the thing in this
//! codebase most likely to be identified by a future reader — including the
//! person who wrote it — as an obvious bottleneck and removed, so the reasoning
//! is here, next to the code, and not only in a document.
//!
//! A position is allocated by incrementing `change_position.next_position`
//! inside the writing transaction. The obvious replacement is a Postgres
//! sequence, which is faster, lock-free, and wrong:
//!
//! - A sequence is not transactional. Two writers take positions 7 and 8, the
//!   holder of 8 commits first, and the holder of 7 is still open.
//! - A client polling `Changes` sees 8, records its position as 8, and returns
//!   later asking for everything after 8.
//! - 7 commits in between and is never delivered to that client. Nothing
//!   detects it, because the client's position is a number and 7 is behind it.
//!
//! That is a partial answer presented as a complete one, which is the single
//! failure the change stream exists to prevent. A sequence also leaves gaps on
//! rollback, so a client cannot tell a hole it must wait for from a hole that
//! will never be filled.
//!
//! Allocating from a row inside the transaction makes commit order and position
//! order the same by construction, and a rolled-back write leaves no gap
//! because its increment rolls back with it. The cost is that writes serialise.
//! For one household that is not a cost worth paying anything to avoid.
//!
//! **The lock is taken deliberately early**, by [`hold_the_sequence`], before
//! anything an intent writes. Two things depend on that:
//!
//! - Position within a container is `max + 1`, which is only safe against a
//!   concurrent writer if the two are already serialised. Holding the sequence
//!   row is what makes the read-then-write safe without a second lock over
//!   the container.
//! - Taking it last would mean two transactions could do all their work
//!   concurrently and then collide at the end, which turns a contended write
//!   into wasted work rather than into a wait.
//!
//! # What an entry says
//!
//! An entry carries the audience *before* and *after* the write, because
//! broadening makes something appear to a member for the first time and
//! narrowing makes it vanish, and to that member those are creation and
//! disappearance. Assembling one member's stream from the current audience
//! alone would produce nothing at all for the member who lost sight of an
//! entity, which is the leak this column pair exists to prevent. Assembly
//! itself is the read path's, in Wave 3.2.

use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

use crate::store::WriteTx;
use crate::store::ids::{EntityId, MemberRef};

/// Take the sequence row's lock for the rest of this transaction.
///
/// Called once, before an intent writes anything. See the module docs for why
/// it is first rather than last.
pub async fn hold_the_sequence(tx: &mut WriteTx) -> sqlx::Result<()> {
    sqlx::query("SELECT next_position FROM change_position WHERE singleton FOR UPDATE")
        .fetch_one(tx.conn())
        .await?;
    Ok(())
}

/// Allocate the next position. Serialises; see the module docs.
async fn allocate(tx: &mut WriteTx) -> sqlx::Result<i64> {
    let row = sqlx::query(
        "UPDATE change_position SET next_position = next_position + 1 \
         WHERE singleton RETURNING next_position - 1 AS position",
    )
    .fetch_one(tx.conn())
    .await?;
    row.try_get("position")
}

/// What an entity write moved, from the change stream's point of view.
///
/// The audience pair is the whole reason this struct exists rather than the
/// call taking an entity id: a narrowing has to reach the member who lost sight
/// of the entity, and by the time the row is written the current audience no
/// longer names them.
pub struct EntityChange {
    pub entity: EntityId,
    pub audience_before: Option<Vec<MemberRef>>,
    pub audience_after: Option<Vec<MemberRef>>,
    pub author_before: Option<MemberRef>,
    pub author_after: Option<MemberRef>,
    /// Where a body moved, which blocks moved. Bodies are Wave 2.2, so this is
    /// empty here, and it is a parameter rather than an omission so that adding
    /// bodies does not mean revisiting the shape of an entry.
    pub changed_blocks: Vec<Uuid>,
}

fn ids(members: &Option<Vec<MemberRef>>) -> Option<Vec<Uuid>> {
    members
        .as_ref()
        .map(|m| m.iter().copied().map(MemberRef::as_uuid).collect())
}

/// Created, edited, removed, or restored. **Removal is a change carrying a
/// lifecycle state, not an absence**, which is why it is this and not
/// [`entity_gone`].
pub async fn entity_written(
    tx: &mut WriteTx,
    at: DateTime<Utc>,
    change: &EntityChange,
) -> sqlx::Result<i64> {
    write_entity_row(tx, at, "entity_written", change).await
}

/// Erased. The entity is gone and the sequence has to be able to say so, which
/// is why the change table carries no foreign key.
pub async fn entity_gone(
    tx: &mut WriteTx,
    at: DateTime<Utc>,
    change: &EntityChange,
) -> sqlx::Result<i64> {
    write_entity_row(tx, at, "entity_gone", change).await
}

async fn write_entity_row(
    tx: &mut WriteTx,
    at: DateTime<Utc>,
    kind: &str,
    change: &EntityChange,
) -> sqlx::Result<i64> {
    let position = allocate(tx).await?;
    sqlx::query(
        "INSERT INTO change (position, occurred_at, kind, entity_id, \
         audience_before, audience_after, author_before, author_after, \
         changed_block_ids) \
         VALUES ($1, $2, $3::change_kind, $4, $5, $6, $7, $8, $9)",
    )
    .bind(position)
    .bind(at)
    .bind(kind)
    .bind(change.entity.as_uuid())
    .bind(ids(&change.audience_before))
    .bind(ids(&change.audience_after))
    .bind(change.author_before.map(MemberRef::as_uuid))
    .bind(change.author_after.map(MemberRef::as_uuid))
    .bind(&change.changed_blocks)
    .execute(tx.conn())
    .await?;
    Ok(position)
}

/// Created, edited, or restored.
///
/// **A removed relation is [`relation_gone`], not this.** The wire carries a
/// relation with no lifecycle field, so there is no shape in which a change
/// says "this relation is now in holding". A member's stream is told the
/// connection is no longer there, and a restoration says it is there again.
/// That is the same rule as an entity a member can no longer see: nothing
/// distinguishes gone from no-longer-visible, deliberately.
pub async fn relation_written(
    tx: &mut WriteTx,
    at: DateTime<Utc>,
    relation: Uuid,
) -> sqlx::Result<i64> {
    write_relation_row(tx, at, "relation_written", relation).await
}

/// Removed, or removed outright because an endpoint was erased.
pub async fn relation_gone(
    tx: &mut WriteTx,
    at: DateTime<Utc>,
    relation: Uuid,
) -> sqlx::Result<i64> {
    write_relation_row(tx, at, "relation_gone", relation).await
}

async fn write_relation_row(
    tx: &mut WriteTx,
    at: DateTime<Utc>,
    kind: &str,
    relation: Uuid,
) -> sqlx::Result<i64> {
    // A relation entry carries no audience pair. A relation's visibility
    // follows its endpoints', which the change row cannot restate without
    // holding a copy that can go stale, so assembling a member's relation
    // changes is a question about the endpoints and belongs to the read path
    // in Wave 3.2.
    let position = allocate(tx).await?;
    sqlx::query(
        "INSERT INTO change (position, occurred_at, kind, relation_id) \
         VALUES ($1, $2, $3::change_kind, $4)",
    )
    .bind(position)
    .bind(at)
    .bind(kind)
    .bind(relation)
    .execute(tx.conn())
    .await?;
    Ok(position)
}

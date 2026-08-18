//! Per-part write provenance, and the conflict rule that reads it.
//!
//! # The rule, stated once
//!
//! An edit declares the counter it was based on. Three outcomes, from the
//! substrate:
//!
//! - Base current: apply.
//! - Base stale, the parts this write touches disjoint from what moved since:
//!   **apply, no conflict.**
//! - Base stale, overlapping: **conflict.** Both values are retained, and the
//!   write still applies.
//!
//! **A stale base is never a rejection.** The counter detects concurrency; it
//! does not gate admission, and no write is ever sent back to be retried. This
//! is the point at which reject-and-retry — the familiar shape of optimistic
//! concurrency control — would be reached for and would be wrong: a device
//! returning after three weeks cannot win a retry race against a household that
//! has been using the system meanwhile, so it would spin, then give up, in
//! exactly the case the durability guarantees exist for.
//!
//! # Writes producing the same value are not divergent
//!
//! Stated separately in the vision document and easy to drop: *whatever their
//! base counter said*. Two people resolving one conflict the same way is the
//! likely case, and forming a second conflict out of two people agreeing is the
//! machinery working against its own purpose. So an overlapping part whose
//! arriving value equals the value already stored is not a conflict, and the
//! comparison is over the text rendering both sides would be retained as.
//!
//! # Why a side table
//!
//! Answering "which parts moved between counter N and now" needs a record of
//! it. Nothing in migration one had one. See `0002_write_provenance.sql` for
//! the two shapes rejected and why.

use std::collections::HashMap;

use sqlx::Row;
use uuid::Uuid;

use crate::store::WriteTx;
use crate::store::ids::{EntityId, MemberId, MemberRef};

use super::parts::{Part, PartKey};

/// A part that moved since some base counter, and who moved it.
#[derive(Debug, Clone)]
pub struct Moved {
    pub part: Part,
    pub by: MemberRef,
}

/// Every part of this entity that moved strictly after `base`.
///
/// Strictly after: a part that last moved *at* the base counter is a part the
/// editing client had already seen, so it is not something that moved since.
pub async fn moved_since(
    tx: &mut WriteTx,
    entity: EntityId,
    base: i64,
) -> sqlx::Result<Vec<Moved>> {
    let rows = sqlx::query(
        "SELECT field_name, block_id, member_id FROM entity_part_counter \
         WHERE entity_id = $1 AND counter > $2",
    )
    .bind(entity.as_uuid())
    .bind(base)
    .fetch_all(tx.conn())
    .await?;

    let mut moved = Vec::with_capacity(rows.len());
    for r in &rows {
        let key = match (
            r.try_get::<Option<String>, _>("field_name")?,
            r.try_get::<Option<Uuid>, _>("block_id")?,
        ) {
            (Some(name), None) => PartKey::Field(name),
            (None, Some(id)) => PartKey::Block(id),
            // The table's own check refuses both and neither, so this is
            // unreachable rather than defended against.
            _ => continue,
        };
        // A name this build does not know is a part it cannot conflict on. See
        // `Part::from_key` for why guessing would be worse than skipping.
        if let Some(part) = Part::from_key(&key) {
            moved.push(Moved {
                part,
                by: MemberRef::from_uuid(r.try_get("member_id")?),
            });
        }
    }
    Ok(moved)
}

/// Record that these parts moved at `counter`, written by `member`.
///
/// Upserts, because a part moves many times and only the last time matters.
pub async fn record(
    tx: &mut WriteTx,
    entity: EntityId,
    parts: &[Part],
    counter: i64,
    member: MemberId,
) -> sqlx::Result<()> {
    for part in parts {
        let (field, block) = match part.key() {
            PartKey::Field(name) => (Some(name), None),
            PartKey::Block(id) => (None, Some(id)),
        };
        sqlx::query(
            "INSERT INTO entity_part_counter \
             (entity_id, field_name, block_id, counter, member_id) \
             VALUES ($1, $2, $3, $4, $5) \
             ON CONFLICT (entity_id, field_name, block_id) \
             DO UPDATE SET counter = excluded.counter, member_id = excluded.member_id",
        )
        .bind(entity.as_uuid())
        .bind(field)
        .bind(block)
        .bind(counter)
        .bind(member.as_uuid())
        .execute(tx.conn())
        .await?;
    }
    Ok(())
}

/// Everything provenance holds about one entity, removed on erasure.
///
/// Erasure does not delete the entity row — it leaves a tombstone — so the
/// schema's cascades never fire and every dependent row is removed explicitly.
pub async fn erase(tx: &mut WriteTx, entity: EntityId) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM entity_part_counter WHERE entity_id = $1")
        .bind(entity.as_uuid())
        .execute(tx.conn())
        .await?;
    Ok(())
}

/// One part whose two values are both retained.
#[derive(Debug, Clone)]
pub struct Retained {
    pub part: Part,
    /// The arriving value, which becomes the entity's current value.
    pub mine: String,
    /// Who is submitting this write.
    pub mine_member: MemberId,
    /// The value this write displaced.
    pub theirs: Option<String>,
    /// Who last moved the part before this write.
    pub theirs_member: MemberRef,
}

/// Decide, for one write, which of its parts conflict.
///
/// `touching` is what the write addresses, paired with the text each side would
/// be retained as: the arriving value, and the value already stored. Both are
/// optional because clearing a part is setting it to nothing.
///
/// Returns the subset that overlaps what moved since `base` and disagrees with
/// it. Everything else applies silently, which is the whole point of scoping a
/// write to its parts.
pub fn detect(
    touching: &[(Part, Option<String>, Option<String>)],
    moved: &[Moved],
    member: MemberId,
) -> Vec<Retained> {
    let by_part: HashMap<&Part, &Moved> = moved.iter().map(|m| (&m.part, m)).collect();

    touching
        .iter()
        .filter_map(|(part, arriving, stored)| {
            let m = by_part.get(part)?;
            // Writes producing the same value are not divergent, whatever their
            // base counter said.
            if arriving == stored {
                return None;
            }
            Some(Retained {
                part: part.clone(),
                mine: arriving.clone().unwrap_or_default(),
                mine_member: member,
                theirs: stored.clone(),
                theirs_member: m.by,
            })
        })
        .collect()
}

/// Retain both values of a conflicted part on the entity.
///
/// # Which side is `mine`
///
/// The store's two slots are named `mine` and `theirs`, and the wire's are too,
/// but the wire's are **reader-relative**: a client renders whichever side
/// carries its own member as "mine". A stored row cannot be reader-relative, so
/// the names here are slots and the member id on each side is the fact that
/// matters — which is exactly what the substrate requires, that *whose edit
/// each value was is known and may be shown*.
///
/// The reading taken, stated so it is not re-derived: **`mine` is the arriving
/// value** — the one this write applies, submitted by the member who will read
/// the acknowledgement — and **`theirs` is the value it displaced**. It is the
/// reading that makes the acknowledgement's own perspective the stored one, and
/// nothing downstream may key on the slot name without also reading the member.
pub async fn retain(
    tx: &mut WriteTx,
    entity: EntityId,
    retained: &[Retained],
    at: chrono::DateTime<chrono::Utc>,
) -> sqlx::Result<()> {
    for r in retained {
        let (field, block) = match r.part.key() {
            PartKey::Field(name) => (Some(name), None),
            PartKey::Block(id) => (None, Some(id)),
        };
        sqlx::query(
            "INSERT INTO conflict \
             (id, entity_id, field_name, block_id, mine_member_id, mine_value, \
              theirs_member_id, theirs_value, detected_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(Uuid::new_v4())
        .bind(entity.as_uuid())
        .bind(field)
        .bind(block)
        .bind(r.mine_member.as_uuid())
        .bind(&r.mine)
        .bind(r.theirs_member.as_uuid())
        .bind(r.theirs.as_deref())
        .bind(at)
        .execute(tx.conn())
        .await?;
    }
    Ok(())
}

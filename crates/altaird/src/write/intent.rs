//! Intent identity, and the acknowledgement the instance already issued.
//!
//! # Why the acknowledgement is held rather than recomputed
//!
//! Replay is idempotent: resubmitting an intent returns the acknowledgement the
//! instance already issued, unchanged, rather than producing a second effect.
//! That promise cannot be kept by re-deriving an answer, because the answer
//! depends on state that has since moved — a counter, a conflict, whether the
//! entity has since been erased. It is kept by holding what was said.
//!
//! **The row is written in the transaction it acknowledges.** A row here and no
//! write, or a write and no row here, break the promise in different
//! directions: the first answers "applied" for something that never happened,
//! the second applies a second effect on the next retry. There is no ordering
//! of two transactions that fixes this, which is why there is only one.
//!
//! # Why identity alone is not enough
//!
//! An intent identity comes from a client, so it is an assertion rather than a
//! fact. The same value arriving from a different member is not the same
//! intent, and the stored answer is not theirs to receive — it names entities
//! they may have no business knowing exist.
//!
//! So a replay is looked up by identity *and* member, and an identity held by
//! somebody else is refused as [`Outcome::not_available`], which is the same
//! nothing every other unavailable thing produces. Refusing it as a distinct
//! condition would turn the intent table into an oracle for identifiers.

use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

use crate::store::WriteTx;
use crate::store::ids::EntityId;

use super::outcome::{ConflictParts, Outcome, RefusalReason};

/// What was found under an intent identity.
pub enum Held {
    /// This member submitted it before, and this is what they were told.
    Mine(Outcome),
    /// Somebody else holds the identity. Answered as nothing; see the module
    /// docs.
    SomebodyElses,
    /// Never seen.
    Absent,
}

/// Look an intent up by its identity.
pub async fn held(tx: &mut WriteTx, intent: Uuid, member: Uuid) -> sqlx::Result<Held> {
    let row = sqlx::query(
        "SELECT member_id, outcome::text AS outcome, entity_ids, relation_ids, counter, \
         conflict_content_fields, conflict_specific_fields, conflict_block_ids, \
         refusal::text AS refusal, refusal_detail, original_entity_id, new_entity_id \
         FROM intent WHERE id = $1",
    )
    .bind(intent)
    .fetch_optional(tx.conn())
    .await?;

    let Some(row) = row else {
        return Ok(Held::Absent);
    };
    if row.try_get::<Uuid, _>("member_id")? != member {
        return Ok(Held::SomebodyElses);
    }

    let outcome = match row.try_get::<String, _>("outcome")?.as_str() {
        "applied" => {
            let conflict = ConflictParts {
                content_fields: row
                    .try_get::<Option<Vec<i32>>, _>("conflict_content_fields")?
                    .unwrap_or_default()
                    .into_iter()
                    .map(|n| n as u32)
                    .collect(),
                specific_fields: row
                    .try_get::<Option<Vec<i32>>, _>("conflict_specific_fields")?
                    .unwrap_or_default()
                    .into_iter()
                    .map(|n| n as u32)
                    .collect(),
                block_ids: row
                    .try_get::<Option<Vec<Uuid>>, _>("conflict_block_ids")?
                    .unwrap_or_default(),
            };
            Outcome::Applied {
                entities: row
                    .try_get::<Option<Vec<Uuid>>, _>("entity_ids")?
                    .unwrap_or_default()
                    .into_iter()
                    .map(EntityId::from_uuid)
                    .collect(),
                relations: row
                    .try_get::<Option<Vec<Uuid>>, _>("relation_ids")?
                    .unwrap_or_default(),
                counter: row.try_get("counter")?,
                conflict: (!conflict.is_empty()).then_some(conflict),
            }
        }
        "refused" => Outcome::Refused {
            reason: match row.try_get::<String, _>("refusal")?.as_str() {
                "malformed" => RefusalReason::Malformed,
                _ => RefusalReason::NotAvailable,
            },
            detail: row
                .try_get::<Option<String>, _>("refusal_detail")?
                .unwrap_or_default(),
        },
        _ => Outcome::Recreated {
            original: EntityId::from_uuid(row.try_get("original_entity_id")?),
            new: EntityId::from_uuid(row.try_get("new_entity_id")?),
            counter: row.try_get::<Option<i64>, _>("counter")?.unwrap_or(1),
        },
    };

    Ok(Held::Mine(outcome))
}

/// Write the acknowledgement this transaction is about to commit.
///
/// Every column beyond the outcome belongs to one outcome and is null for the
/// others, which is the price migration one paid for one table over three.
///
/// # Returns false when somebody else got there first
///
/// Two submissions of one intent identity can both find it absent and both
/// proceed. They meet here, and `ON CONFLICT DO NOTHING` is what makes the
/// loser's answer *the acknowledgement already issued* rather than a store
/// fault. A bare insert makes a concurrent replay indistinguishable from the
/// instance being down, which is precisely backwards: the instance is fine and
/// the answer already exists. A retry after a stalled or dropped submission is
/// exactly this shape, and it is what an outbox does by design.
///
/// The row count is trustworthy. Postgres blocks a conflicting insert until
/// the transaction holding the key ends, and lets it through if that
/// transaction rolls back — so zero rows means a *committed* row is there, not
/// a racing one that may yet vanish.
///
/// The caller must discard whatever it wrote before answering: the effect
/// belongs to the transaction that won.
pub async fn hold(
    tx: &mut WriteTx,
    intent: Uuid,
    member: Uuid,
    at: DateTime<Utc>,
    outcome: &Outcome,
) -> sqlx::Result<bool> {
    let (name, entities, relations, counter, conflict, refusal, detail, original, new) =
        match outcome {
            Outcome::Applied {
                entities,
                relations,
                counter,
                conflict,
            } => (
                "applied",
                Some(entities.iter().map(|e| e.as_uuid()).collect::<Vec<_>>()),
                Some(relations.clone()),
                *counter,
                conflict.clone(),
                None,
                None,
                None,
                None,
            ),
            Outcome::Refused { reason, detail } => (
                "refused",
                None,
                None,
                None,
                None,
                Some(match reason {
                    RefusalReason::NotAvailable => "not_available",
                    RefusalReason::Malformed => "malformed",
                }),
                Some(detail.clone()),
                None,
                None,
            ),
            Outcome::Recreated {
                original,
                new,
                counter,
            } => (
                "recreated",
                None,
                None,
                Some(*counter),
                None,
                None,
                None,
                Some(original.as_uuid()),
                Some(new.as_uuid()),
            ),
        };

    let conflict = conflict.unwrap_or_default();
    let held = sqlx::query(
        "INSERT INTO intent \
         (id, member_id, received_at, outcome, entity_ids, relation_ids, counter, \
          conflict_content_fields, conflict_specific_fields, conflict_block_ids, \
          refusal, refusal_detail, original_entity_id, new_entity_id) \
         VALUES ($1, $2, $3, $4::intent_outcome, $5, $6, $7, $8, $9, $10, \
                 $11::refusal_reason, $12, $13, $14) \
         ON CONFLICT (id) DO NOTHING",
    )
    .bind(intent)
    .bind(member)
    .bind(at)
    .bind(name)
    .bind(entities)
    .bind(relations)
    .bind(counter)
    .bind(as_i32(&conflict.content_fields))
    .bind(as_i32(&conflict.specific_fields))
    .bind(&conflict.block_ids)
    .bind(refusal)
    .bind(detail)
    .bind(original)
    .bind(new)
    .execute(tx.conn())
    .await?;
    Ok(held.rows_affected() == 1)
}

/// The schema stores field numbers as `integer[]` and the wire carries them as
/// `uint32`. Field numbers are permanent and small, so the narrowing cannot
/// lose one.
fn as_i32(v: &[u32]) -> Option<Vec<i32>> {
    (!v.is_empty()).then(|| v.iter().map(|n| *n as i32).collect())
}

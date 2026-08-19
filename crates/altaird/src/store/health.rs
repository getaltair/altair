//! Aggregate counts `GetHealth` reports.
//!
//! Deliberately unscoped, and for a sharper reason than the other unscoped
//! queries in [`super::entity`]: `GetHealth` resolves no member at all (see
//! `service.rs` on why), so there is nothing to scope by. These answer for
//! the whole household, which is what an operator asking "is the instance
//! healthy" wants.

use sqlx::Row;

use super::tx::ReadTx;

/// Active entities with no embedding row of their own.
///
/// Nothing in v0 writes `embedding` — Wave 5 builds the derivation worker —
/// so today this is every active entity, which is the honest answer with that
/// worker entirely absent. It stays honest once the worker exists, too:
/// it counts `entity` against `embedding`, never `derivation_queue`, so
/// losing the queue costs time and nothing here.
pub async fn derivation_outstanding(tx: &mut ReadTx) -> sqlx::Result<u64> {
    let row = sqlx::query(
        "SELECT count(*) AS n FROM entity e WHERE e.lifecycle = 'active' \
         AND NOT EXISTS (SELECT 1 FROM embedding em WHERE em.entity_id = e.id)",
    )
    .fetch_one(tx.conn())
    .await?;
    let n: i64 = row.try_get("n")?;
    Ok(n as u64)
}

/// Intents refused, all-time. Never how many are waiting to send — the wire
/// comment on the field is explicit that the second question belongs to a
/// client's own outbox and not to this count.
pub async fn intents_refused(tx: &mut ReadTx) -> sqlx::Result<u64> {
    let row = sqlx::query("SELECT count(*) AS n FROM intent WHERE outcome = 'refused'")
        .fetch_one(tx.conn())
        .await?;
    let n: i64 = row.try_get("n")?;
    Ok(n as u64)
}

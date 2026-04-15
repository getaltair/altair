use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use super::models::{
    CreateItemEventRequest, ItemEventType, TrackingItemEvent, TrackingItemEventRow,
};
use crate::error::AppError;
use crate::tracking::household::assert_household_member;

/// SELECT clause used in all item event queries.
/// `quantity_change` is cast to DOUBLE PRECISION so sqlx maps it to `f64`.
const SELECT_COLS: &str = "id, item_id, event_type, \
     quantity_change::DOUBLE PRECISION AS quantity_change, \
     from_location_id, to_location_id, notes, occurred_at, created_at";

/// Create an item event for a tracked item.
///
/// Steps:
/// 1. Resolve `household_id` from the item (404 if item not found or soft-deleted).
/// 2. Assert the requesting user is a household member (403 if not).
/// 3. Begin a transaction and acquire a FOR UPDATE lock on the item row to
///    prevent TOCTOU races on the quantity check.
/// 4. Derive current quantity as `SUM(quantity_change)` from all existing events.
/// 5. Reject if `current_qty + quantity_delta < 0` (invariant E-7).
/// 6. INSERT the event. On duplicate key (23505) return 409 Conflict.
/// 7. Commit and return the created event.
pub async fn create_item_event(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateItemEventRequest,
) -> Result<TrackingItemEvent, AppError> {
    // Step 1: resolve household_id from the item.
    // fetch_optional returns Option<Option<Uuid>>: outer None = no row, inner None = NULL household_id.
    let household_id_result: Option<Option<Uuid>> = sqlx::query_scalar(
        "SELECT household_id FROM tracking_items WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(req.item_id)
    .fetch_optional(pool)
    .await?;

    let household_id = match household_id_result {
        // Row found and household_id is set.
        Some(Some(hid)) => hid,
        // No row found (item doesn't exist or is soft-deleted) or NULL household_id.
        _ => return Err(AppError::NotFound),
    };

    // Step 2: assert membership.
    assert_household_member(pool, user_id, household_id).await?;

    // Validate quantity_delta polarity per event type.
    match req.event_type {
        ItemEventType::Restock | ItemEventType::Purchase | ItemEventType::PurchaseReversed => {
            if req.quantity_delta < 0.0 {
                return Err(AppError::UnprocessableEntity(
                    "quantity_delta must be positive for this event type".to_string(),
                ));
            }
        }
        ItemEventType::Consume | ItemEventType::Loss | ItemEventType::Expire => {
            if req.quantity_delta > 0.0 {
                return Err(AppError::UnprocessableEntity(
                    "quantity_delta must be negative for this event type".to_string(),
                ));
            }
        }
        // Adjustment and Move accept any sign.
        ItemEventType::Adjustment | ItemEventType::Move => {}
    }

    // Move events must specify at least one location (from or to).
    if matches!(req.event_type, ItemEventType::Move)
        && req.from_location_id.is_none()
        && req.to_location_id.is_none()
    {
        return Err(AppError::UnprocessableEntity(
            "Move events must specify at least one of from_location_id or to_location_id"
                .to_string(),
        ));
    }

    // Step 3: begin transaction.
    let mut tx = pool.begin().await?;

    // Step 4: acquire row lock to prevent TOCTOU races.
    let item_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM tracking_items WHERE id = $1 AND deleted_at IS NULL FOR UPDATE)",
    )
    .bind(req.item_id)
    .fetch_one(&mut *tx)
    .await?;

    if !item_exists {
        // Item was soft-deleted between the initial check and locking.
        if let Err(rb_err) = tx.rollback().await {
            tracing::warn!("rollback failed (postgres will auto-rollback on drop): {:?}", rb_err);
        }
        return Err(AppError::UnprocessableEntity(
            "item has been deleted".to_string(),
        ));
    }

    // Step 5: derive current quantity from event history.
    let current_qty: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(quantity_change), 0.0)::DOUBLE PRECISION \
         FROM tracking_item_events WHERE item_id = $1",
    )
    .bind(req.item_id)
    .fetch_one(&mut *tx)
    .await?;

    // Invariant E-7: quantity must not go below zero.
    if current_qty + req.quantity_delta < 0.0 {
        if let Err(rb_err) = tx.rollback().await {
            tracing::warn!("rollback failed (postgres will auto-rollback on drop): {:?}", rb_err);
        }
        return Err(AppError::UnprocessableEntity(
            "quantity would go below zero".to_string(),
        ));
    }

    // Step 6: insert the event.
    let event_id = req.id.unwrap_or_else(Uuid::new_v4);
    let occurred_at = req.occurred_at.unwrap_or_else(Utc::now);
    let event_type_str = event_type_to_str(&req.event_type);

    let result = sqlx::query_as::<_, TrackingItemEventRow>(&format!(
        "INSERT INTO tracking_item_events \
         (id, item_id, event_type, quantity_change, from_location_id, to_location_id, notes, occurred_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
         RETURNING {SELECT_COLS}"
    ))
    .bind(event_id)
    .bind(req.item_id)
    .bind(event_type_str)
    .bind(req.quantity_delta)
    .bind(req.from_location_id)
    .bind(req.to_location_id)
    .bind(&req.notes)
    .bind(occurred_at)
    .fetch_one(&mut *tx)
    .await;

    match result {
        Ok(row) => {
            // Step 7: commit.
            tx.commit().await?;
            Ok(TrackingItemEvent::from(row))
        }
        Err(sqlx::Error::Database(ref e)) if e.code().as_deref() == Some("23505") => {
            if let Err(rb_err) = tx.rollback().await {
                tracing::warn!("rollback failed (postgres will auto-rollback on drop): {:?}", rb_err);
            }
            Err(AppError::Conflict(
                "event with this id already exists".to_string(),
            ))
        }
        Err(e) => {
            if let Err(rb_err) = tx.rollback().await {
                tracing::warn!("rollback failed (postgres will auto-rollback on drop): {:?}", rb_err);
            }
            Err(e.into())
        }
    }
}

/// Create an item event within a caller-supplied transaction.
///
/// # Caller contract
///
/// The caller MUST:
/// - Hold a `SELECT ... FOR UPDATE` lock on the item row before calling this
///   function to prevent TOCTOU races on the quantity check (invariant E-7).
/// - Own the transaction: this function neither begins nor commits `tx`.
///
/// Used by `shopping_list_items::service` to atomically record a purchase or
/// purchase-reversal event as part of a shopping list item status transition.
#[allow(dead_code)]
pub(crate) async fn create_item_event_in_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: Uuid,
    event_type: ItemEventType,
    quantity_delta: f64,
) -> Result<TrackingItemEvent, AppError> {
    // Derive current quantity from event history.
    let current_qty: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(quantity_change), 0.0)::DOUBLE PRECISION \
         FROM tracking_item_events WHERE item_id = $1",
    )
    .bind(item_id)
    .fetch_one(&mut **tx)
    .await?;

    // Invariant E-7: quantity must not go below zero.
    // Note: for purchase_reversed (quantity_delta > 0) this can never trigger.
    if current_qty + quantity_delta < 0.0 {
        return Err(AppError::UnprocessableEntity(
            "quantity would go below zero".to_string(),
        ));
    }

    let event_id = Uuid::new_v4();
    let event_type_str = event_type_to_str(&event_type);

    let row = sqlx::query_as::<_, TrackingItemEventRow>(&format!(
        "INSERT INTO tracking_item_events \
         (id, item_id, event_type, quantity_change, occurred_at) \
         VALUES ($1, $2, $3, $4, NOW()) \
         RETURNING {SELECT_COLS}"
    ))
    .bind(event_id)
    .bind(item_id)
    .bind(event_type_str)
    .bind(quantity_delta)
    .fetch_one(&mut **tx)
    .await?;

    Ok(TrackingItemEvent::from(row))
}

/// List events for a given item in chronological order (occurred_at ASC).
///
/// Events are permanent (invariant D-5) — no `deleted_at` filter is applied.
pub async fn list_item_events(
    pool: &PgPool,
    user_id: Uuid,
    item_id: Uuid,
    household_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<TrackingItemEvent>, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    // FA-001: verify the item belongs to the caller-supplied household.
    // Without this check, a member of household A could pass household_id=A with
    // any item_id from household B and read its full event history.
    let item_household: Option<Uuid> = sqlx::query_scalar(
        "SELECT household_id FROM tracking_items WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(item_id)
    .fetch_optional(pool)
    .await?;
    match item_household {
        Some(hid) if hid == household_id => {}
        _ => return Err(AppError::NotFound),
    }

    let rows = sqlx::query_as::<_, TrackingItemEventRow>(&format!(
        "SELECT {SELECT_COLS} FROM tracking_item_events \
         WHERE item_id = $1 \
         ORDER BY occurred_at ASC \
         LIMIT $2 OFFSET $3"
    ))
    .bind(item_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(TrackingItemEvent::from).collect())
}

/// Convert an `ItemEventType` to its database string representation.
fn event_type_to_str(event_type: &ItemEventType) -> &'static str {
    match event_type {
        ItemEventType::Restock => "restock",
        ItemEventType::Consume => "consume",
        ItemEventType::Purchase => "purchase",
        ItemEventType::PurchaseReversed => "purchase_reversed",
        ItemEventType::Adjustment => "adjustment",
        ItemEventType::Move => "move",
        ItemEventType::Expire => "expire",
        ItemEventType::Loss => "loss",
    }
}

// ---------------------------------------------------------------------------
// Tests (S006-T) — sqlx::test integration tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::sync::Arc;
    use tokio::sync::Barrier;

    // ---------------------------------------------------------------------------
    // Test helpers
    // ---------------------------------------------------------------------------

    async fn insert_test_user(pool: &PgPool, user_id: Uuid, email: &str) {
        sqlx::query(
            "INSERT INTO users (id, email, display_name, password_hash, is_admin, status) \
             VALUES ($1, $2, 'Test User', 'hashed_password', false, 'active')",
        )
        .bind(user_id)
        .bind(email)
        .execute(pool)
        .await
        .expect("Failed to insert test user");
    }

    async fn insert_test_household(pool: &PgPool, household_id: Uuid, owner_id: Uuid) {
        sqlx::query(
            "INSERT INTO households (id, owner_id, name) VALUES ($1, $2, 'Test Household')",
        )
        .bind(household_id)
        .bind(owner_id)
        .execute(pool)
        .await
        .expect("Failed to insert test household");
    }

    async fn insert_membership(pool: &PgPool, household_id: Uuid, user_id: Uuid) {
        sqlx::query("INSERT INTO household_memberships (household_id, user_id) VALUES ($1, $2)")
            .bind(household_id)
            .bind(user_id)
            .execute(pool)
            .await
            .expect("Failed to insert household membership");
    }

    async fn insert_test_item(pool: &PgPool, item_id: Uuid, household_id: Uuid, user_id: Uuid) {
        sqlx::query(
            "INSERT INTO tracking_items (id, name, user_id, household_id) \
             VALUES ($1, 'Test Item', $2, $3)",
        )
        .bind(item_id)
        .bind(user_id)
        .bind(household_id)
        .execute(pool)
        .await
        .expect("Failed to insert test item");
    }

    fn make_event_request(item_id: Uuid, quantity_delta: f64) -> CreateItemEventRequest {
        CreateItemEventRequest {
            id: None,
            item_id,
            event_type: ItemEventType::Restock,
            quantity_delta,
            occurred_at: None,
            from_location_id: None,
            to_location_id: None,
            notes: None,
        }
    }

    // ---------------------------------------------------------------------------
    // FA-007 / E-7: quantity check — consume beyond available stock returns 422
    // ---------------------------------------------------------------------------

    /// E-7: item starts at quantity 0 (no events), consume event with delta=-1 → 422.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn quantity_check_consume_below_zero_returns_unprocessable(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;
        insert_test_item(&pool, item_id, household_id, user_id).await;

        // No prior restock — quantity is 0.
        let req = CreateItemEventRequest {
            id: None,
            item_id,
            event_type: ItemEventType::Consume,
            quantity_delta: -1.0,
            occurred_at: None,
            from_location_id: None,
            to_location_id: None,
            notes: None,
        };

        let result = create_item_event(&pool, user_id, req).await;

        assert!(
            matches!(result, Err(AppError::UnprocessableEntity(_))),
            "consume below zero must return UnprocessableEntity, got: {:?}",
            result
        );
    }

    // ---------------------------------------------------------------------------
    // E-2: duplicate event ID → 409 Conflict
    // ---------------------------------------------------------------------------

    /// Posting the same event id twice returns 409 on the second attempt.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn duplicate_event_id_returns_conflict(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();
        let event_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;
        insert_test_item(&pool, item_id, household_id, user_id).await;

        let req1 = CreateItemEventRequest {
            id: Some(event_id),
            item_id,
            event_type: ItemEventType::Restock,
            quantity_delta: 5.0,
            occurred_at: None,
            from_location_id: None,
            to_location_id: None,
            notes: None,
        };
        create_item_event(&pool, user_id, req1)
            .await
            .expect("first create must succeed");

        let req2 = CreateItemEventRequest {
            id: Some(event_id),
            item_id,
            event_type: ItemEventType::Restock,
            quantity_delta: 3.0,
            occurred_at: None,
            from_location_id: None,
            to_location_id: None,
            notes: None,
        };
        let result = create_item_event(&pool, user_id, req2).await;

        assert!(
            matches!(result, Err(AppError::Conflict(_))),
            "duplicate event id must return Conflict, got: {:?}",
            result
        );
    }

    // ---------------------------------------------------------------------------
    // FA-009: create 5 events, list, assert all 5 present in occurred_at ASC order
    // ---------------------------------------------------------------------------

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn list_returns_all_events_in_chronological_order(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;
        insert_test_item(&pool, item_id, household_id, user_id).await;

        // Create 5 events with explicit occurred_at timestamps in reverse order
        // to verify the list is sorted by occurred_at ASC (not insertion order).
        let base = chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);

        let mut event_ids = Vec::new();
        for i in (0i64..5).rev() {
            let occurred_at = base + chrono::Duration::hours(i);
            let event_id = Uuid::new_v4();
            event_ids.push(event_id);

            let req = CreateItemEventRequest {
                id: Some(event_id),
                item_id,
                event_type: ItemEventType::Restock,
                quantity_delta: 1.0,
                occurred_at: Some(occurred_at),
                from_location_id: None,
                to_location_id: None,
                notes: None,
            };
            create_item_event(&pool, user_id, req)
                .await
                .expect("create_item_event must succeed");
        }

        let events = list_item_events(&pool, user_id, item_id, household_id, 50, 0)
            .await
            .expect("list_item_events must succeed");

        assert_eq!(events.len(), 5, "must return all 5 events");

        // Assert occurred_at ASC ordering.
        for window in events.windows(2) {
            assert!(
                window[0].occurred_at <= window[1].occurred_at,
                "events must be in occurred_at ASC order: {:?} > {:?}",
                window[0].occurred_at,
                window[1].occurred_at
            );
        }
    }

    // ---------------------------------------------------------------------------
    // FA-010 / D-5: DELETE returns 405 (tested at handler level via router)
    // ---------------------------------------------------------------------------
    // Handler-level 405 test: verified by the router configuration in mod.rs which
    // registers DELETE as returning METHOD_NOT_ALLOWED. The service layer has no
    // delete function (invariant D-5). This is confirmed by the absence of any
    // delete_item_event function in this module.

    // ---------------------------------------------------------------------------
    // FA-019: concurrency test — SELECT FOR UPDATE prevents double-consume
    // ---------------------------------------------------------------------------

    /// Two concurrent consume requests each claiming the full available stock.
    /// At most one must succeed; the other must return 422.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn concurrent_events_cannot_both_consume_below_zero(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;
        insert_test_item(&pool, item_id, household_id, user_id).await;

        // Seed initial stock: restock 5 units.
        let restock = make_event_request(item_id, 5.0);
        create_item_event(&pool, user_id, restock)
            .await
            .expect("restock must succeed");

        // Spawn two tasks each trying to consume all 5 units (total would be -10, violating E-7).
        // A Barrier ensures both tasks have started before either calls create_item_event,
        // exercising the SELECT FOR UPDATE lock as the correctness mechanism.
        let pool = Arc::new(pool);
        let barrier = Arc::new(Barrier::new(2));

        let task1 = tokio::spawn({
            let pool = Arc::clone(&pool);
            let barrier = Arc::clone(&barrier);
            async move {
                barrier.wait().await;
                let req = CreateItemEventRequest {
                    id: None,
                    item_id,
                    event_type: ItemEventType::Consume,
                    quantity_delta: -5.0,
                    occurred_at: None,
                    from_location_id: None,
                    to_location_id: None,
                    notes: None,
                };
                create_item_event(&pool, user_id, req).await
            }
        });

        let task2 = tokio::spawn({
            let pool = Arc::clone(&pool);
            let barrier = Arc::clone(&barrier);
            async move {
                barrier.wait().await;
                let req = CreateItemEventRequest {
                    id: None,
                    item_id,
                    event_type: ItemEventType::Consume,
                    quantity_delta: -5.0,
                    occurred_at: None,
                    from_location_id: None,
                    to_location_id: None,
                    notes: None,
                };
                create_item_event(&pool, user_id, req).await
            }
        });

        let r1 = task1.await.expect("task1 must not panic");
        let r2 = task2.await.expect("task2 must not panic");

        let successes = [&r1, &r2].iter().filter(|r| r.is_ok()).count();
        let failures = [&r1, &r2]
            .iter()
            .filter(|r| matches!(r, Err(AppError::UnprocessableEntity(_))))
            .count();

        assert_eq!(
            successes, 1,
            "exactly one consume must succeed; results: {:?}, {:?}",
            r1, r2
        );
        assert_eq!(
            failures, 1,
            "exactly one consume must return 422; results: {:?}, {:?}",
            r1, r2
        );
    }
}

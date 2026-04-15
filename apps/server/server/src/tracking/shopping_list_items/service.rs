use sqlx::PgPool;
use uuid::Uuid;

use super::models::{
    CreateShoppingListItemRequest, ShoppingListItemStatus, TrackingShoppingListItem,
    TrackingShoppingListItemRow, UpdateShoppingListItemRequest,
};
use crate::error::AppError;
use crate::tracking::household::assert_household_member;
use crate::tracking::item_events::models::ItemEventType;
use crate::tracking::item_events::service::create_item_event_in_tx;

/// Columns returned by SELECT/RETURNING on `tracking_shopping_list_items`.
///
/// `quantity` is cast to INTEGER so sqlx maps it to `i32` (column is NUMERIC).
const SELECT_COLS: &str = "id, shopping_list_id, item_id, name, quantity::INTEGER AS quantity, \
     status, created_at, updated_at, deleted_at";

/// Convert a `ShoppingListItemStatus` to its DB string representation.
fn status_to_str(status: &ShoppingListItemStatus) -> &'static str {
    match status {
        ShoppingListItemStatus::Pending => "pending",
        ShoppingListItemStatus::Purchased => "purchased",
        ShoppingListItemStatus::Removed => "removed",
    }
}

/// Parse a status string from the DB into a `ShoppingListItemStatus`.
///
/// Returns `AppError::Internal` for unrecognised values — this should be
/// unreachable in practice given the DB constraint.
fn str_to_status(s: &str) -> Result<ShoppingListItemStatus, AppError> {
    match s {
        "pending" => Ok(ShoppingListItemStatus::Pending),
        "purchased" => Ok(ShoppingListItemStatus::Purchased),
        "removed" => Ok(ShoppingListItemStatus::Removed),
        other => Err(AppError::Internal(anyhow::anyhow!(
            "unrecognised shopping list item status in DB: {other}"
        ))),
    }
}

/// Fetch the `household_id` for a shopping list, returning NotFound if absent or soft-deleted.
async fn fetch_list_household(pool: &PgPool, list_id: Uuid) -> Result<Uuid, AppError> {
    let household_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT household_id FROM tracking_shopping_lists WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(list_id)
    .fetch_optional(pool)
    .await?;

    household_id.ok_or(AppError::NotFound)
}

/// Add an item to a shopping list.
///
/// Steps:
/// 1. Fetch shopping list → 404 if missing.
/// 2. Assert requesting user is a household member → 403 if not.
/// 3. If `item_id` is Some, validate it belongs to the same household (E-9) → 422 if not.
/// 4. INSERT the row with status='pending'.
/// 5. Return the created item.
pub async fn add_shopping_list_item(
    pool: &PgPool,
    user_id: Uuid,
    list_id: Uuid,
    req: CreateShoppingListItemRequest,
) -> Result<TrackingShoppingListItem, AppError> {
    let household_id = fetch_list_household(pool, list_id).await?;
    assert_household_member(pool, user_id, household_id).await?;

    // Invariant E-9: linked item must belong to the same household.
    if let Some(item_id) = req.item_id {
        let item_household: Option<Option<Uuid>> = sqlx::query_scalar(
            "SELECT household_id FROM tracking_items WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(item_id)
        .fetch_optional(pool)
        .await?;

        match item_household {
            Some(Some(hid)) if hid == household_id => {}
            _ => {
                return Err(AppError::UnprocessableEntity(
                    "item does not belong to this household".to_string(),
                ));
            }
        }
    }

    let quantity = req.quantity.unwrap_or(1);

    let result = sqlx::query_as::<_, TrackingShoppingListItemRow>(&format!(
        "INSERT INTO tracking_shopping_list_items \
         (shopping_list_id, item_id, name, quantity, status) \
         VALUES ($1, $2, $3, $4, 'pending') \
         RETURNING {SELECT_COLS}"
    ))
    .bind(list_id)
    .bind(req.item_id)
    .bind(&req.name)
    .bind(quantity)
    .fetch_one(pool)
    .await;

    match result {
        Ok(row) => Ok(TrackingShoppingListItem::from(row)),
        Err(sqlx::Error::Database(ref e)) if e.code().as_deref() == Some("23505") => Err(
            AppError::Conflict("shopping list item already exists".to_string()),
        ),
        Err(e) => Err(e.into()),
    }
}

/// Update the status of a shopping list item.
///
/// Steps:
/// 1. Load current item + list's household_id via JOIN → 404 if missing.
/// 2. Assert household membership → 403.
/// 3. Validate the status transition via `can_transition_to` → 422 if invalid.
/// 4. Execute the update, optionally inside a transaction that also records an
///    inventory event when an item is linked:
///    - Pending → Purchased: Consume event (-1.0)
///    - Purchased → Pending: PurchaseReversed event (+1.0)
///    - Other valid transitions (e.g. Pending → Removed): simple UPDATE
pub async fn update_shopping_list_item(
    pool: &PgPool,
    user_id: Uuid,
    list_id: Uuid,
    item_id: Uuid,
    req: UpdateShoppingListItemRequest,
) -> Result<TrackingShoppingListItem, AppError> {
    // Step 1: load current item and its list's household_id in one query.
    #[derive(sqlx::FromRow)]
    struct CurrentRow {
        status: String,
        linked_item_id: Option<Uuid>,
        household_id: Uuid,
    }

    let current: Option<CurrentRow> = sqlx::query_as::<_, CurrentRow>(
        "SELECT sli.status, sli.item_id AS linked_item_id, sl.household_id \
         FROM tracking_shopping_list_items sli \
         JOIN tracking_shopping_lists sl ON sl.id = sli.shopping_list_id \
         WHERE sli.id = $1 AND sli.shopping_list_id = $2 AND sli.deleted_at IS NULL",
    )
    .bind(item_id)
    .bind(list_id)
    .fetch_optional(pool)
    .await?;

    let current = current.ok_or(AppError::NotFound)?;
    let household_id = current.household_id;

    // Step 2: assert membership.
    assert_household_member(pool, user_id, household_id).await?;

    // Step 3: validate transition.
    let current_status = str_to_status(&current.status)?;
    if !current_status.can_transition_to(&req.status) {
        return Err(AppError::UnprocessableEntity(
            "invalid status transition".to_string(),
        ));
    }

    let new_status_str = status_to_str(&req.status);
    let linked_item_id = current.linked_item_id;

    // Step 4: execute update, optionally with an inventory event.
    match (&req.status, linked_item_id) {
        // Pending → Purchased with a linked item: consume inventory.
        (ShoppingListItemStatus::Purchased, Some(inv_item_id)) => {
            let mut tx = pool.begin().await?;

            // Acquire row lock on the inventory item; check it still exists (not soft-deleted).
            let item_exists = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM tracking_items WHERE id = $1 AND deleted_at IS NULL FOR UPDATE)",
            )
            .bind(inv_item_id)
            .fetch_one(&mut *tx)
            .await?;

            if !item_exists {
                if let Err(rb_err) = tx.rollback().await {
                    tracing::warn!(
                        "rollback failed (postgres will auto-rollback on drop): {:?}",
                        rb_err
                    );
                }
                return Err(AppError::NotFound);
            }

            // Update shopping list item status inside the transaction.
            let row = sqlx::query_as::<_, TrackingShoppingListItemRow>(&format!(
                "UPDATE tracking_shopping_list_items \
                 SET status = $1, updated_at = NOW() \
                 WHERE id = $2 AND deleted_at IS NULL \
                 RETURNING {SELECT_COLS}"
            ))
            .bind(new_status_str)
            .bind(item_id)
            .fetch_optional(&mut *tx)
            .await?;

            let row = match row {
                Some(r) => r,
                None => {
                    if let Err(rb_err) = tx.rollback().await {
                        tracing::warn!(
                            "rollback failed (postgres will auto-rollback on drop): {:?}",
                            rb_err
                        );
                    }
                    return Err(AppError::NotFound);
                }
            };

            // Record consume event. If quantity check fails (E-7), rollback and propagate.
            let event_result =
                create_item_event_in_tx(&mut tx, inv_item_id, ItemEventType::Consume, -1.0).await;

            if let Err(e) = event_result {
                if let Err(rb_err) = tx.rollback().await {
                    tracing::warn!(
                        "rollback failed (postgres will auto-rollback on drop): {:?}",
                        rb_err
                    );
                }
                return Err(e);
            }

            tx.commit().await?;
            Ok(TrackingShoppingListItem::from(row))
        }

        // Purchased → Pending with a linked item: reverse the consumption.
        //
        // Note: adding quantity back (delta > 0) can never violate E-7 (quantity
        // cannot go below zero when we're adding stock).
        (ShoppingListItemStatus::Pending, Some(inv_item_id)) => {
            let mut tx = pool.begin().await?;

            // Acquire row lock on the inventory item; check it still exists (not soft-deleted).
            let item_exists = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM tracking_items WHERE id = $1 AND deleted_at IS NULL FOR UPDATE)",
            )
            .bind(inv_item_id)
            .fetch_one(&mut *tx)
            .await?;

            if !item_exists {
                if let Err(rb_err) = tx.rollback().await {
                    tracing::warn!(
                        "rollback failed (postgres will auto-rollback on drop): {:?}",
                        rb_err
                    );
                }
                return Err(AppError::NotFound);
            }

            let row = sqlx::query_as::<_, TrackingShoppingListItemRow>(&format!(
                "UPDATE tracking_shopping_list_items \
                 SET status = $1, updated_at = NOW() \
                 WHERE id = $2 AND deleted_at IS NULL \
                 RETURNING {SELECT_COLS}"
            ))
            .bind(new_status_str)
            .bind(item_id)
            .fetch_optional(&mut *tx)
            .await?;

            let row = match row {
                Some(r) => r,
                None => {
                    if let Err(rb_err) = tx.rollback().await {
                        tracing::warn!(
                            "rollback failed (postgres will auto-rollback on drop): {:?}",
                            rb_err
                        );
                    }
                    return Err(AppError::NotFound);
                }
            };

            let event_result =
                create_item_event_in_tx(&mut tx, inv_item_id, ItemEventType::PurchaseReversed, 1.0)
                    .await;

            if let Err(e) = event_result {
                if let Err(rb_err) = tx.rollback().await {
                    tracing::warn!(
                        "rollback failed (postgres will auto-rollback on drop): {:?}",
                        rb_err
                    );
                }
                return Err(e);
            }

            tx.commit().await?;
            Ok(TrackingShoppingListItem::from(row))
        }

        // All other valid transitions (Pending → Removed, or no linked item):
        // simple UPDATE without a transaction.
        _ => {
            let row = sqlx::query_as::<_, TrackingShoppingListItemRow>(&format!(
                "UPDATE tracking_shopping_list_items \
                 SET status = $1, updated_at = NOW() \
                 WHERE id = $2 AND deleted_at IS NULL \
                 RETURNING {SELECT_COLS}"
            ))
            .bind(new_status_str)
            .bind(item_id)
            .fetch_optional(pool)
            .await?;

            row.map(TrackingShoppingListItem::from)
                .ok_or(AppError::NotFound)
        }
    }
}

/// Soft-delete a shopping list item.
pub async fn remove_shopping_list_item(
    pool: &PgPool,
    user_id: Uuid,
    list_id: Uuid,
    item_id: Uuid,
) -> Result<(), AppError> {
    let household_id = fetch_list_household(pool, list_id).await?;
    assert_household_member(pool, user_id, household_id).await?;

    let result = sqlx::query(
        "UPDATE tracking_shopping_list_items \
         SET deleted_at = NOW(), updated_at = NOW() \
         WHERE id = $1 AND shopping_list_id = $2 AND deleted_at IS NULL",
    )
    .bind(item_id)
    .bind(list_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

/// List all active items in a shopping list, ordered by creation time.
pub async fn list_shopping_list_items(
    pool: &PgPool,
    user_id: Uuid,
    list_id: Uuid,
    household_id: Uuid,
) -> Result<Vec<TrackingShoppingListItem>, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let rows = sqlx::query_as::<_, TrackingShoppingListItemRow>(&format!(
        "SELECT {SELECT_COLS} \
         FROM tracking_shopping_list_items \
         WHERE shopping_list_id = $1 AND deleted_at IS NULL \
         ORDER BY created_at ASC"
    ))
    .bind(list_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(TrackingShoppingListItem::from)
        .collect())
}

// ---------------------------------------------------------------------------
// Integration tests (S008-T)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use uuid::Uuid;

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

    async fn insert_test_shopping_list(pool: &PgPool, list_id: Uuid, household_id: Uuid) {
        sqlx::query(
            "INSERT INTO tracking_shopping_lists (id, name, household_id) \
             VALUES ($1, 'Test List', $2)",
        )
        .bind(list_id)
        .bind(household_id)
        .execute(pool)
        .await
        .expect("Failed to insert test shopping list");
    }

    /// Restock an item so it has available quantity.
    async fn restock_item(pool: &PgPool, item_id: Uuid, qty: f64) {
        sqlx::query(
            "INSERT INTO tracking_item_events (id, item_id, event_type, quantity_change, occurred_at) \
             VALUES (gen_random_uuid(), $1, 'restock', $2, NOW())",
        )
        .bind(item_id)
        .bind(qty)
        .execute(pool)
        .await
        .expect("Failed to restock item");
    }

    // ---------------------------------------------------------------------------
    // Unit test: ShoppingListItemStatus::can_transition_to — all 9 combinations
    // ---------------------------------------------------------------------------

    #[test]
    fn can_transition_to_exhaustive() {
        use ShoppingListItemStatus::*;

        // Valid transitions
        assert!(
            Pending.can_transition_to(&Purchased),
            "Pending → Purchased must be allowed"
        );
        assert!(
            Pending.can_transition_to(&Removed),
            "Pending → Removed must be allowed"
        );
        assert!(
            Purchased.can_transition_to(&Pending),
            "Purchased → Pending must be allowed"
        );

        // Invalid transitions
        assert!(
            !Pending.can_transition_to(&Pending),
            "Pending → Pending must be denied"
        );
        assert!(
            !Purchased.can_transition_to(&Purchased),
            "Purchased → Purchased must be denied"
        );
        assert!(
            !Purchased.can_transition_to(&Removed),
            "Purchased → Removed must be denied"
        );
        assert!(
            !Removed.can_transition_to(&Pending),
            "Removed → Pending must be denied"
        );
        assert!(
            !Removed.can_transition_to(&Purchased),
            "Removed → Purchased must be denied"
        );
        assert!(
            !Removed.can_transition_to(&Removed),
            "Removed → Removed must be denied"
        );
    }

    // ---------------------------------------------------------------------------
    // FA-012 / E-9: item_id from a different household → 422 on add
    // ---------------------------------------------------------------------------

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn add_item_with_wrong_household_returns_unprocessable(pool: PgPool) {
        let user_a = Uuid::new_v4();
        let user_b = Uuid::new_v4();
        let household_a = Uuid::new_v4();
        let household_b = Uuid::new_v4();
        let list_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();

        insert_test_user(&pool, user_a, "user_a@example.com").await;
        insert_test_user(&pool, user_b, "user_b@example.com").await;
        insert_test_household(&pool, household_a, user_a).await;
        insert_test_household(&pool, household_b, user_b).await;
        insert_membership(&pool, household_a, user_a).await;
        insert_membership(&pool, household_b, user_b).await;

        // Shopping list belongs to household_a.
        insert_test_shopping_list(&pool, list_id, household_a).await;

        // Item belongs to household_b (different household).
        insert_test_item(&pool, item_id, household_b, user_b).await;

        let result = add_shopping_list_item(
            &pool,
            user_a,
            list_id,
            CreateShoppingListItemRequest {
                name: "Milk".to_string(),
                quantity: None,
                item_id: Some(item_id),
            },
        )
        .await;

        assert!(
            matches!(result, Err(AppError::UnprocessableEntity(_))),
            "item from different household must return 422, got: {:?}",
            result
        );
    }

    // ---------------------------------------------------------------------------
    // FA-013: pending → purchased creates a consume event in the same transaction
    // ---------------------------------------------------------------------------

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn purchase_creates_consume_event(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();
        let list_id = Uuid::new_v4();
        let inv_item_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;
        insert_test_shopping_list(&pool, list_id, household_id).await;
        insert_test_item(&pool, inv_item_id, household_id, user_id).await;

        // Restock the item so consuming it won't violate E-7.
        restock_item(&pool, inv_item_id, 5.0).await;

        // Add item to shopping list, linked to the inventory item.
        let sli = add_shopping_list_item(
            &pool,
            user_id,
            list_id,
            CreateShoppingListItemRequest {
                name: "Milk".to_string(),
                quantity: None,
                item_id: Some(inv_item_id),
            },
        )
        .await
        .expect("add_shopping_list_item failed");

        // Mark as purchased.
        let updated = update_shopping_list_item(
            &pool,
            user_id,
            list_id,
            sli.id,
            UpdateShoppingListItemRequest {
                status: ShoppingListItemStatus::Purchased,
            },
        )
        .await
        .expect("update_shopping_list_item failed");

        assert_eq!(updated.status, "purchased");

        // Assert that a consume event was recorded for the inventory item.
        let event_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tracking_item_events \
             WHERE item_id = $1 AND event_type = 'consume'",
        )
        .bind(inv_item_id)
        .fetch_one(&pool)
        .await
        .expect("query failed");

        assert_eq!(
            event_count, 1,
            "exactly one consume event must exist after purchase"
        );
    }

    // ---------------------------------------------------------------------------
    // FA-014: pending → removed succeeds
    // ---------------------------------------------------------------------------

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn pending_to_removed_succeeds(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();
        let list_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;
        insert_test_shopping_list(&pool, list_id, household_id).await;

        let sli = add_shopping_list_item(
            &pool,
            user_id,
            list_id,
            CreateShoppingListItemRequest {
                name: "Bread".to_string(),
                quantity: None,
                item_id: None,
            },
        )
        .await
        .expect("add_shopping_list_item failed");

        let updated = update_shopping_list_item(
            &pool,
            user_id,
            list_id,
            sli.id,
            UpdateShoppingListItemRequest {
                status: ShoppingListItemStatus::Removed,
            },
        )
        .await
        .expect("update_shopping_list_item failed");

        assert_eq!(updated.status, "removed");
    }

    // ---------------------------------------------------------------------------
    // FA-015: removed → any transition returns UnprocessableEntity
    // ---------------------------------------------------------------------------

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn removed_to_any_returns_unprocessable(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();
        let list_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;
        insert_test_shopping_list(&pool, list_id, household_id).await;

        let sli = add_shopping_list_item(
            &pool,
            user_id,
            list_id,
            CreateShoppingListItemRequest {
                name: "Eggs".to_string(),
                quantity: None,
                item_id: None,
            },
        )
        .await
        .expect("add_shopping_list_item failed");

        // Transition to Removed.
        update_shopping_list_item(
            &pool,
            user_id,
            list_id,
            sli.id,
            UpdateShoppingListItemRequest {
                status: ShoppingListItemStatus::Removed,
            },
        )
        .await
        .expect("transition to removed failed");

        // Try Removed → Pending.
        let result = update_shopping_list_item(
            &pool,
            user_id,
            list_id,
            sli.id,
            UpdateShoppingListItemRequest {
                status: ShoppingListItemStatus::Pending,
            },
        )
        .await;

        assert!(
            matches!(result, Err(AppError::UnprocessableEntity(_))),
            "Removed → Pending must return 422, got: {:?}",
            result
        );

        // Try Removed → Purchased.
        let result2 = update_shopping_list_item(
            &pool,
            user_id,
            list_id,
            sli.id,
            UpdateShoppingListItemRequest {
                status: ShoppingListItemStatus::Purchased,
            },
        )
        .await;

        assert!(
            matches!(result2, Err(AppError::UnprocessableEntity(_))),
            "Removed → Purchased must return 422, got: {:?}",
            result2
        );
    }

    // ---------------------------------------------------------------------------
    // FA-016: purchased → pending inserts a compensating purchase_reversed event
    // ---------------------------------------------------------------------------

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn purchased_to_pending_inserts_purchase_reversed_event(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();
        let list_id = Uuid::new_v4();
        let inv_item_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;
        insert_test_shopping_list(&pool, list_id, household_id).await;
        insert_test_item(&pool, inv_item_id, household_id, user_id).await;

        // Restock so consume won't fail.
        restock_item(&pool, inv_item_id, 5.0).await;

        // Add and purchase the item.
        let sli = add_shopping_list_item(
            &pool,
            user_id,
            list_id,
            CreateShoppingListItemRequest {
                name: "Butter".to_string(),
                quantity: None,
                item_id: Some(inv_item_id),
            },
        )
        .await
        .expect("add_shopping_list_item failed");

        update_shopping_list_item(
            &pool,
            user_id,
            list_id,
            sli.id,
            UpdateShoppingListItemRequest {
                status: ShoppingListItemStatus::Purchased,
            },
        )
        .await
        .expect("purchase transition failed");

        // Reverse: Purchased → Pending.
        let updated = update_shopping_list_item(
            &pool,
            user_id,
            list_id,
            sli.id,
            UpdateShoppingListItemRequest {
                status: ShoppingListItemStatus::Pending,
            },
        )
        .await
        .expect("purchase reversal failed");

        assert_eq!(updated.status, "pending");

        // Assert that a purchase_reversed event was recorded.
        let reversed_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tracking_item_events \
             WHERE item_id = $1 AND event_type = 'purchase_reversed'",
        )
        .bind(inv_item_id)
        .fetch_one(&pool)
        .await
        .expect("query failed");

        assert_eq!(
            reversed_count, 1,
            "exactly one purchase_reversed event must exist after reversal"
        );
    }
}

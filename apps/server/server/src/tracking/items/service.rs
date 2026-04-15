use sqlx::PgPool;
use uuid::Uuid;

use super::models::{CreateItemRequest, TrackingItem, TrackingItemRow, UpdateItemRequest};
use crate::error::AppError;
use crate::tracking::household::assert_household_member;

/// SELECT clause used in all item queries.
/// `quantity` is cast to DOUBLE PRECISION so sqlx maps it to `f64`.
const SELECT_COLS: &str = "id, name, description, quantity::DOUBLE PRECISION AS quantity, barcode, \
     location_id, category_id, user_id, household_id, initiative_id, \
     expires_at, created_at, updated_at, deleted_at";

/// Validate that `location_id` belongs to `household_id` (invariant E-8).
async fn validate_location_household(
    pool: &PgPool,
    location_id: Uuid,
    household_id: Uuid,
) -> Result<(), AppError> {
    let loc_household_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT household_id FROM tracking_locations WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(location_id)
    .fetch_optional(pool)
    .await?;

    match loc_household_id {
        Some(hid) if hid == household_id => Ok(()),
        _ => Err(AppError::UnprocessableEntity(
            "location does not belong to this household".to_string(),
        )),
    }
}

pub async fn create_item(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateItemRequest,
) -> Result<TrackingItem, AppError> {
    assert_household_member(pool, user_id, req.household_id).await?;

    if let Some(location_id) = req.location_id {
        validate_location_household(pool, location_id, req.household_id).await?;
    }

    let item_id = req.id.unwrap_or_else(Uuid::new_v4);

    let result = sqlx::query_as::<_, TrackingItemRow>(&format!(
        "INSERT INTO tracking_items \
         (id, name, description, barcode, location_id, category_id, user_id, household_id, expires_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
         RETURNING {SELECT_COLS}"
    ))
    .bind(item_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.barcode)
    .bind(req.location_id)
    .bind(req.category_id)
    .bind(user_id)
    .bind(req.household_id)
    .bind(req.expires_at)
    .fetch_one(pool)
    .await;

    match result {
        Ok(row) => Ok(TrackingItem::from(row)),
        Err(sqlx::Error::Database(ref e)) if e.code().as_deref() == Some("23505") => Err(
            AppError::Conflict("item with this id already exists".to_string()),
        ),
        Err(e) => Err(e.into()),
    }
}

pub async fn list_items(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    category_id: Option<Uuid>,
    location_id: Option<Uuid>,
    limit: i64,
    offset: i64,
) -> Result<Vec<TrackingItem>, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let rows = sqlx::query_as::<_, TrackingItemRow>(&format!(
        "SELECT {SELECT_COLS} FROM tracking_items \
         WHERE household_id = $1 AND deleted_at IS NULL \
         AND ($2::uuid IS NULL OR category_id = $2) \
         AND ($3::uuid IS NULL OR location_id = $3) \
         ORDER BY name ASC \
         LIMIT $4 OFFSET $5"
    ))
    .bind(household_id)
    .bind(category_id)
    .bind(location_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(TrackingItem::from).collect())
}

pub async fn get_item(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    item_id: Uuid,
) -> Result<TrackingItem, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let row = sqlx::query_as::<_, TrackingItemRow>(&format!(
        "SELECT {SELECT_COLS} FROM tracking_items \
         WHERE id = $1 AND household_id = $2 AND deleted_at IS NULL"
    ))
    .bind(item_id)
    .bind(household_id)
    .fetch_optional(pool)
    .await?;

    row.map(TrackingItem::from).ok_or(AppError::NotFound)
}

pub async fn update_item(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    item_id: Uuid,
    req: UpdateItemRequest,
) -> Result<TrackingItem, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    if let Some(location_id) = req.location_id {
        validate_location_household(pool, location_id, household_id).await?;
    }

    let row = sqlx::query_as::<_, TrackingItemRow>(&format!(
        "UPDATE tracking_items \
         SET name        = COALESCE($3, name), \
             description = COALESCE($4, description), \
             barcode     = COALESCE($5, barcode), \
             location_id = COALESCE($6, location_id), \
             category_id = COALESCE($7, category_id), \
             expires_at  = COALESCE($8, expires_at), \
             updated_at  = NOW() \
         WHERE id = $1 AND household_id = $2 AND deleted_at IS NULL \
         RETURNING {SELECT_COLS}"
    ))
    .bind(item_id)
    .bind(household_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.barcode)
    .bind(req.location_id)
    .bind(req.category_id)
    .bind(req.expires_at)
    .fetch_optional(pool)
    .await?;

    row.map(TrackingItem::from).ok_or(AppError::NotFound)
}

pub async fn delete_item(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    item_id: Uuid,
) -> Result<(), AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let result = sqlx::query(
        "UPDATE tracking_items \
         SET deleted_at = NOW(), updated_at = NOW() \
         WHERE id = $1 AND household_id = $2 AND deleted_at IS NULL",
    )
    .bind(item_id)
    .bind(household_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests (S005-T) — sqlx::test integration tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

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

    async fn insert_test_location(
        pool: &PgPool,
        location_id: Uuid,
        household_id: Uuid,
        name: &str,
    ) {
        sqlx::query("INSERT INTO tracking_locations (id, household_id, name) VALUES ($1, $2, $3)")
            .bind(location_id)
            .bind(household_id)
            .bind(name)
            .execute(pool)
            .await
            .expect("Failed to insert test location");
    }

    async fn insert_test_category(
        pool: &PgPool,
        category_id: Uuid,
        household_id: Uuid,
        name: &str,
    ) {
        sqlx::query("INSERT INTO tracking_categories (id, household_id, name) VALUES ($1, $2, $3)")
            .bind(category_id)
            .bind(household_id)
            .bind(name)
            .execute(pool)
            .await
            .expect("Failed to insert test category");
    }

    fn make_create_request(household_id: Uuid, name: &str) -> CreateItemRequest {
        CreateItemRequest {
            id: None,
            name: name.to_string(),
            household_id,
            description: None,
            barcode: None,
            location_id: None,
            category_id: None,
            expires_at: None,
        }
    }

    /// FA-004: member creates item with caller-supplied UUID; response.id matches.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_item_with_client_supplied_uuid(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();
        let supplied_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;

        let req = CreateItemRequest {
            id: Some(supplied_id),
            name: "Widget".to_string(),
            household_id,
            description: None,
            barcode: None,
            location_id: None,
            category_id: None,
            expires_at: None,
        };

        let item = create_item(&pool, user_id, req)
            .await
            .expect("create_item must succeed for a member");

        assert_eq!(
            item.id, supplied_id,
            "response id must match the caller-supplied UUID"
        );
        assert_eq!(item.name, "Widget");
    }

    /// FA-005: duplicate UUID returns Conflict (409).
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn duplicate_uuid_returns_conflict(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;

        let req1 = CreateItemRequest {
            id: Some(item_id),
            name: "First".to_string(),
            household_id,
            description: None,
            barcode: None,
            location_id: None,
            category_id: None,
            expires_at: None,
        };
        create_item(&pool, user_id, req1)
            .await
            .expect("first create must succeed");

        let req2 = CreateItemRequest {
            id: Some(item_id),
            name: "Duplicate".to_string(),
            household_id,
            description: None,
            barcode: None,
            location_id: None,
            category_id: None,
            expires_at: None,
        };
        let result = create_item(&pool, user_id, req2).await;

        assert!(
            matches!(result, Err(AppError::Conflict(_))),
            "duplicate UUID must return Conflict, got: {:?}",
            result
        );
    }

    /// FA-006 (E-8): location_id from a different household returns UnprocessableEntity (422).
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn location_from_different_household_returns_unprocessable(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_a = Uuid::new_v4();
        let household_b = Uuid::new_v4();
        let other_user = Uuid::new_v4();
        let location_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_user(&pool, other_user, "other@example.com").await;
        insert_test_household(&pool, household_a, user_id).await;
        insert_test_household(&pool, household_b, other_user).await;
        insert_membership(&pool, household_a, user_id).await;
        insert_membership(&pool, household_b, other_user).await;

        // Location belongs to household_b, but item will be for household_a
        insert_test_location(&pool, location_id, household_b, "Other Location").await;

        let req = CreateItemRequest {
            id: None,
            name: "Mismatched".to_string(),
            household_id: household_a,
            description: None,
            barcode: None,
            location_id: Some(location_id),
            category_id: None,
            expires_at: None,
        };

        let result = create_item(&pool, user_id, req).await;

        assert!(
            matches!(result, Err(AppError::UnprocessableEntity(_))),
            "location from different household must return UnprocessableEntity, got: {:?}",
            result
        );
    }

    /// Item list filters by category_id: only the matching item is returned.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn list_filters_by_category_id(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();
        let category_a = Uuid::new_v4();
        let category_b = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;
        insert_test_category(&pool, category_a, household_id, "Category A").await;
        insert_test_category(&pool, category_b, household_id, "Category B").await;

        let req_a = CreateItemRequest {
            id: None,
            name: "Item A".to_string(),
            household_id,
            description: None,
            barcode: None,
            location_id: None,
            category_id: Some(category_a),
            expires_at: None,
        };
        let req_b = CreateItemRequest {
            id: None,
            name: "Item B".to_string(),
            household_id,
            description: None,
            barcode: None,
            location_id: None,
            category_id: Some(category_b),
            expires_at: None,
        };

        create_item(&pool, user_id, req_a)
            .await
            .expect("create item A failed");
        let item_b = create_item(&pool, user_id, req_b)
            .await
            .expect("create item B failed");

        let results = list_items(&pool, user_id, household_id, Some(category_b), None, 50, 0)
            .await
            .expect("list_items failed");

        assert_eq!(results.len(), 1, "filter by category_b must return 1 item");
        assert_eq!(
            results[0].id, item_b.id,
            "returned item must be Item B (category_b)"
        );
    }

    /// FA-017: soft-deleted item is absent from list results.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn soft_deleted_item_absent_from_list(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;

        let item = create_item(
            &pool,
            user_id,
            make_create_request(household_id, "To Delete"),
        )
        .await
        .expect("create_item failed");

        delete_item(&pool, user_id, household_id, item.id)
            .await
            .expect("delete_item failed");

        let list = list_items(&pool, user_id, household_id, None, None, 50, 0)
            .await
            .expect("list_items failed");

        assert!(
            list.iter().all(|i| i.id != item.id),
            "soft-deleted item must not appear in list"
        );
    }
}

use sqlx::PgPool;
use uuid::Uuid;

use super::models::{
    CreateCategoryRequest, TrackingCategory, TrackingCategoryRow, UpdateCategoryRequest,
};
use crate::error::AppError;
use crate::tracking::household::assert_household_member;

pub async fn list_categories(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
) -> Result<Vec<TrackingCategory>, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let rows = sqlx::query_as::<_, TrackingCategoryRow>(
        "SELECT id, name, household_id, created_at, updated_at, deleted_at \
         FROM tracking_categories \
         WHERE household_id = $1 AND deleted_at IS NULL \
         ORDER BY name ASC",
    )
    .bind(household_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(TrackingCategory::from).collect())
}

pub async fn get_category(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    category_id: Uuid,
) -> Result<TrackingCategory, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let row = sqlx::query_as::<_, TrackingCategoryRow>(
        "SELECT id, name, household_id, created_at, updated_at, deleted_at \
         FROM tracking_categories \
         WHERE id = $1 AND household_id = $2 AND deleted_at IS NULL",
    )
    .bind(category_id)
    .bind(household_id)
    .fetch_optional(pool)
    .await?;

    row.map(TrackingCategory::from).ok_or(AppError::NotFound)
}

pub async fn create_category(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateCategoryRequest,
) -> Result<TrackingCategory, AppError> {
    assert_household_member(pool, user_id, req.household_id).await?;

    let row = sqlx::query_as::<_, TrackingCategoryRow>(
        "INSERT INTO tracking_categories (name, household_id) \
         VALUES ($1, $2) \
         RETURNING id, name, household_id, created_at, updated_at, deleted_at",
    )
    .bind(&req.name)
    .bind(req.household_id)
    .fetch_one(pool)
    .await?;

    Ok(TrackingCategory::from(row))
}

pub async fn update_category(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    category_id: Uuid,
    req: UpdateCategoryRequest,
) -> Result<TrackingCategory, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let row = sqlx::query_as::<_, TrackingCategoryRow>(
        "UPDATE tracking_categories \
         SET name = COALESCE($3, name), updated_at = NOW() \
         WHERE id = $1 AND household_id = $2 AND deleted_at IS NULL \
         RETURNING id, name, household_id, created_at, updated_at, deleted_at",
    )
    .bind(category_id)
    .bind(household_id)
    .bind(&req.name)
    .fetch_optional(pool)
    .await?;

    row.map(TrackingCategory::from).ok_or(AppError::NotFound)
}

pub async fn delete_category(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    category_id: Uuid,
) -> Result<(), AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let result = sqlx::query(
        "UPDATE tracking_categories \
         SET deleted_at = NOW(), updated_at = NOW() \
         WHERE id = $1 AND household_id = $2 AND deleted_at IS NULL",
    )
    .bind(category_id)
    .bind(household_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests (S004-T) — sqlx::test integration tests
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

    /// Non-member calling list_categories gets Forbidden.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn non_member_list_returns_forbidden(pool: PgPool) {
        let owner = Uuid::new_v4();
        let non_member = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, owner, "owner@example.com").await;
        insert_test_user(&pool, non_member, "nonmember@example.com").await;
        insert_test_household(&pool, household_id, owner).await;
        insert_membership(&pool, household_id, owner).await;
        // non_member has no membership

        let result = list_categories(&pool, non_member, household_id).await;
        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "non-member must receive Forbidden, got: {:?}",
            result
        );
    }

    /// Member creates a category and gets the created category back with matching fields.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn member_creates_category_fields_match(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;

        let req = CreateCategoryRequest {
            name: "Electronics".to_string(),
            household_id,
        };

        let category = create_category(&pool, user_id, req)
            .await
            .expect("create_category must succeed for a member");

        assert_eq!(category.name, "Electronics", "name must match request");
        // deleted_at and household_id must not appear in TrackingCategory (compile-time proof via struct fields)
    }

    /// Two households with categories: listing one household returns none from the other.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn list_isolates_by_household(pool: PgPool) {
        let user_a = Uuid::new_v4();
        let user_b = Uuid::new_v4();
        let household_a = Uuid::new_v4();
        let household_b = Uuid::new_v4();

        insert_test_user(&pool, user_a, "user_a@example.com").await;
        insert_test_user(&pool, user_b, "user_b@example.com").await;
        insert_test_household(&pool, household_a, user_a).await;
        insert_test_household(&pool, household_b, user_b).await;
        insert_membership(&pool, household_a, user_a).await;
        insert_membership(&pool, household_b, user_b).await;

        create_category(
            &pool,
            user_a,
            CreateCategoryRequest {
                name: "Household A Category".to_string(),
                household_id: household_a,
            },
        )
        .await
        .expect("create for household_a failed");

        create_category(
            &pool,
            user_b,
            CreateCategoryRequest {
                name: "Household B Category".to_string(),
                household_id: household_b,
            },
        )
        .await
        .expect("create for household_b failed");

        let a_list = list_categories(&pool, user_a, household_a)
            .await
            .expect("list for household_a failed");

        assert_eq!(
            a_list.len(),
            1,
            "household_a list must have exactly 1 category"
        );
        assert_eq!(
            a_list[0].name, "Household A Category",
            "household_a must only see its own categories"
        );

        // Verify no household_b categories appear
        assert!(
            a_list.iter().all(|c| c.name != "Household B Category"),
            "household_b categories must not appear in household_a list"
        );
    }

    /// Soft-deleted category must not appear in list results.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn soft_deleted_category_absent_from_list(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;

        let category = create_category(
            &pool,
            user_id,
            CreateCategoryRequest {
                name: "To Delete".to_string(),
                household_id,
            },
        )
        .await
        .expect("create_category failed");

        delete_category(&pool, user_id, household_id, category.id)
            .await
            .expect("delete_category failed");

        let list = list_categories(&pool, user_id, household_id)
            .await
            .expect("list_categories failed");

        assert!(
            list.iter().all(|c| c.id != category.id),
            "soft-deleted category must not appear in list"
        );
    }
}

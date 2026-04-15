use sqlx::PgPool;
use uuid::Uuid;

use super::models::{
    CreateShoppingListRequest, TrackingShoppingList, TrackingShoppingListRow,
    UpdateShoppingListRequest,
};
use crate::error::AppError;
use crate::tracking::household::assert_household_member;

pub async fn list_shopping_lists(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
) -> Result<Vec<TrackingShoppingList>, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let rows = sqlx::query_as::<_, TrackingShoppingListRow>(
        "SELECT id, name, household_id, created_at, updated_at, deleted_at \
         FROM tracking_shopping_lists \
         WHERE household_id = $1 AND deleted_at IS NULL \
         ORDER BY created_at DESC",
    )
    .bind(household_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(TrackingShoppingList::from).collect())
}

pub async fn get_shopping_list(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    list_id: Uuid,
) -> Result<TrackingShoppingList, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let row = sqlx::query_as::<_, TrackingShoppingListRow>(
        "SELECT id, name, household_id, created_at, updated_at, deleted_at \
         FROM tracking_shopping_lists \
         WHERE id = $1 AND household_id = $2 AND deleted_at IS NULL",
    )
    .bind(list_id)
    .bind(household_id)
    .fetch_optional(pool)
    .await?;

    row.map(TrackingShoppingList::from)
        .ok_or(AppError::NotFound)
}

pub async fn create_shopping_list(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateShoppingListRequest,
) -> Result<TrackingShoppingList, AppError> {
    assert_household_member(pool, user_id, req.household_id).await?;

    let row = sqlx::query_as::<_, TrackingShoppingListRow>(
        "INSERT INTO tracking_shopping_lists (id, name, household_id) \
         VALUES (gen_random_uuid(), $1, $2) \
         RETURNING id, name, household_id, created_at, updated_at, deleted_at",
    )
    .bind(&req.name)
    .bind(req.household_id)
    .fetch_one(pool)
    .await?;

    Ok(TrackingShoppingList::from(row))
}

pub async fn update_shopping_list(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    list_id: Uuid,
    req: UpdateShoppingListRequest,
) -> Result<TrackingShoppingList, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let row = sqlx::query_as::<_, TrackingShoppingListRow>(
        "UPDATE tracking_shopping_lists \
         SET \
           name       = COALESCE($3, name), \
           updated_at = NOW() \
         WHERE id = $1 AND household_id = $2 AND deleted_at IS NULL \
         RETURNING id, name, household_id, created_at, updated_at, deleted_at",
    )
    .bind(list_id)
    .bind(household_id)
    .bind(&req.name)
    .fetch_optional(pool)
    .await?;

    row.map(TrackingShoppingList::from)
        .ok_or(AppError::NotFound)
}

pub async fn delete_shopping_list(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    list_id: Uuid,
) -> Result<(), AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let result = sqlx::query(
        "UPDATE tracking_shopping_lists \
         SET deleted_at = NOW(), updated_at = NOW() \
         WHERE id = $1 AND household_id = $2 AND deleted_at IS NULL",
    )
    .bind(list_id)
    .bind(household_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Integration tests (S007-T)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
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

    /// Non-member gets Forbidden when calling list.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn non_member_gets_forbidden(pool: PgPool) {
        let owner = Uuid::new_v4();
        let non_member = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, owner, "owner@example.com").await;
        insert_test_user(&pool, non_member, "nonmember@example.com").await;
        insert_test_household(&pool, household_id, owner).await;
        insert_membership(&pool, household_id, owner).await;
        // non_member has no membership row

        let result = list_shopping_lists(&pool, non_member, household_id).await;
        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "non-member must get Forbidden, got: {:?}",
            result
        );
    }

    /// Member creates a shopping list and gets it back with correct fields.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn member_creates_and_gets_shopping_list(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "member@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;

        let created = create_shopping_list(
            &pool,
            user_id,
            CreateShoppingListRequest {
                name: "Weekly Groceries".to_string(),
                household_id,
            },
        )
        .await
        .expect("create_shopping_list failed");

        assert_eq!(created.name, "Weekly Groceries");

        let fetched = get_shopping_list(&pool, user_id, household_id, created.id)
            .await
            .expect("get_shopping_list failed");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, "Weekly Groceries");
    }

    /// Two households each have a shopping list; listing one does not surface the other's list.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn list_scoped_to_household(pool: PgPool) {
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

        create_shopping_list(
            &pool,
            user_a,
            CreateShoppingListRequest {
                name: "Household A List".to_string(),
                household_id: household_a,
            },
        )
        .await
        .expect("create for household_a failed");

        create_shopping_list(
            &pool,
            user_b,
            CreateShoppingListRequest {
                name: "Household B List".to_string(),
                household_id: household_b,
            },
        )
        .await
        .expect("create for household_b failed");

        let a_lists = list_shopping_lists(&pool, user_a, household_a)
            .await
            .expect("list for household_a failed");

        assert_eq!(a_lists.len(), 1, "household_a should have exactly 1 list");
        assert_eq!(
            a_lists[0].name, "Household A List",
            "household_a list name mismatch"
        );
    }

    /// Soft-deleted shopping list does not appear in list results.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn soft_deleted_list_absent_from_list(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "softdelete@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;

        let list = create_shopping_list(
            &pool,
            user_id,
            CreateShoppingListRequest {
                name: "To Be Deleted".to_string(),
                household_id,
            },
        )
        .await
        .expect("create_shopping_list failed");

        delete_shopping_list(&pool, user_id, household_id, list.id)
            .await
            .expect("delete_shopping_list failed");

        let results = list_shopping_lists(&pool, user_id, household_id)
            .await
            .expect("list_shopping_lists failed");

        assert!(
            results.is_empty(),
            "soft-deleted list must not appear in results"
        );

        // Verify the row still exists in the DB (soft delete only).
        let deleted_at: Option<DateTime<Utc>> =
            sqlx::query_scalar("SELECT deleted_at FROM tracking_shopping_lists WHERE id = $1")
                .bind(list.id)
                .fetch_one(&pool)
                .await
                .expect("Row must still exist after soft delete");

        assert!(
            deleted_at.is_some(),
            "deleted_at must be non-null after soft delete"
        );
    }
}

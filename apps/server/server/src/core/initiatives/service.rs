use sqlx::PgPool;
use uuid::Uuid;

use super::models::{CreateInitiativeRequest, Initiative, UpdateInitiativeRequest};
use crate::error::AppError;

pub async fn list_initiatives(pool: &PgPool, user_id: Uuid) -> Result<Vec<Initiative>, AppError> {
    let rows = sqlx::query_as::<_, Initiative>(
        "SELECT id, user_id, title, description, status, created_at, updated_at, deleted_at \
         FROM initiatives \
         WHERE user_id = $1 AND deleted_at IS NULL \
         ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

    Ok(rows)
}

pub async fn get_initiative(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<Initiative, AppError> {
    let row = sqlx::query_as::<_, Initiative>(
        "SELECT id, user_id, title, description, status, created_at, updated_at, deleted_at \
         FROM initiatives \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

    row.ok_or(AppError::NotFound)
}

pub async fn create_initiative(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateInitiativeRequest,
) -> Result<Initiative, AppError> {
    let row = sqlx::query_as::<_, Initiative>(
        "INSERT INTO initiatives (id, user_id, title, description, status) \
         VALUES (gen_random_uuid(), $1, $2, $3, $4) \
         RETURNING id, user_id, title, description, status, created_at, updated_at, deleted_at",
    )
    .bind(user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.status)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

    Ok(row)
}

pub async fn update_initiative(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    req: UpdateInitiativeRequest,
) -> Result<Initiative, AppError> {
    // Only update fields that were provided.
    let row = sqlx::query_as::<_, Initiative>(
        "UPDATE initiatives \
         SET \
           title       = COALESCE($3, title), \
           description = COALESCE($4, description), \
           status      = COALESCE($5, status), \
           updated_at  = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL \
         RETURNING id, user_id, title, description, status, created_at, updated_at, deleted_at",
    )
    .bind(id)
    .bind(user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.status)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

    row.ok_or(AppError::NotFound)
}

pub async fn delete_initiative(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query(
        "UPDATE initiatives \
         SET deleted_at = NOW(), updated_at = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests (S032) — sqlx::test integration tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::initiatives::models::{CreateInitiativeRequest, UpdateInitiativeRequest};
    use crate::error::AppError;
    use sqlx::PgPool;

    /// Insert a minimal test user directly into the database.
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

    // FA-009 / SEC-1: each user only sees their own initiatives
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn user_isolation_list_returns_only_own_records(pool: PgPool) {
        let user_a = Uuid::new_v4();
        let user_b = Uuid::new_v4();
        insert_test_user(&pool, user_a, "user_a@example.com").await;
        insert_test_user(&pool, user_b, "user_b@example.com").await;

        // Create one initiative for each user.
        create_initiative(
            &pool,
            user_a,
            CreateInitiativeRequest {
                title: "User A Initiative".to_string(),
                description: None,
                status: Some("draft".to_string()),
            },
        )
        .await
        .expect("Failed to create initiative for user_a");

        create_initiative(
            &pool,
            user_b,
            CreateInitiativeRequest {
                title: "User B Initiative".to_string(),
                description: None,
                status: Some("draft".to_string()),
            },
        )
        .await
        .expect("Failed to create initiative for user_b");

        let a_list = list_initiatives(&pool, user_a)
            .await
            .expect("list for user_a failed");
        let b_list = list_initiatives(&pool, user_b)
            .await
            .expect("list for user_b failed");

        assert_eq!(a_list.len(), 1, "user_a should see exactly 1 initiative");
        assert_eq!(
            a_list[0].user_id, user_a,
            "user_a result must belong to user_a"
        );
        assert_eq!(a_list[0].title, "User A Initiative");

        assert_eq!(b_list.len(), 1, "user_b should see exactly 1 initiative");
        assert_eq!(
            b_list[0].user_id, user_b,
            "user_b result must belong to user_b"
        );
        assert_eq!(b_list[0].title, "User B Initiative");
    }

    // PATCH partial update: only provided fields change; others remain unchanged
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn partial_update_leaves_unspecified_fields_unchanged(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "patch_user@example.com").await;

        let initiative = create_initiative(
            &pool,
            user_id,
            CreateInitiativeRequest {
                title: "Original Title".to_string(),
                description: Some("Original description".to_string()),
                status: Some("draft".to_string()),
            },
        )
        .await
        .expect("Failed to create initiative");

        // Only update the title; leave description and status as None.
        let updated = update_initiative(
            &pool,
            initiative.id,
            user_id,
            UpdateInitiativeRequest {
                title: Some("New Title".to_string()),
                description: None,
                status: None,
            },
        )
        .await
        .expect("update_initiative failed");

        assert_eq!(updated.title, "New Title", "title should have been updated");
        assert_eq!(
            updated.description,
            Some("Original description".to_string()),
            "description must remain unchanged"
        );
        assert_eq!(
            updated.status,
            Some("draft".to_string()),
            "status must remain unchanged"
        );
    }

    // FA-009: GET by wrong user returns NotFound
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn get_initiative_wrong_user_returns_not_found(pool: PgPool) {
        let owner = Uuid::new_v4();
        let other_user = Uuid::new_v4();
        insert_test_user(&pool, owner, "owner@example.com").await;
        insert_test_user(&pool, other_user, "other@example.com").await;

        let initiative = create_initiative(
            &pool,
            owner,
            CreateInitiativeRequest {
                title: "Owner's Initiative".to_string(),
                description: None,
                status: Some("draft".to_string()),
            },
        )
        .await
        .expect("Failed to create initiative");

        let result = get_initiative(&pool, initiative.id, other_user).await;

        assert!(
            matches!(result, Err(AppError::NotFound)),
            "get with wrong user_id must return NotFound, got: {:?}",
            result
        );
    }

    // Soft delete: sets deleted_at; row still exists in DB
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn soft_delete_sets_deleted_at_row_still_exists(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "soft_delete_user@example.com").await;

        let initiative = create_initiative(
            &pool,
            user_id,
            CreateInitiativeRequest {
                title: "To Be Deleted".to_string(),
                description: None,
                status: Some("draft".to_string()),
            },
        )
        .await
        .expect("Failed to create initiative");

        delete_initiative(&pool, initiative.id, user_id)
            .await
            .expect("delete_initiative failed");

        // Verify the row still exists but deleted_at is set.
        let deleted_at: Option<chrono::DateTime<chrono::Utc>> =
            sqlx::query_scalar("SELECT deleted_at FROM initiatives WHERE id = $1")
                .bind(initiative.id)
                .fetch_one(&pool)
                .await
                .expect("Row must still exist after soft delete");

        assert!(
            deleted_at.is_some(),
            "deleted_at must be non-null after soft delete"
        );

        // Confirm the service-level get returns NotFound (row excluded from normal queries).
        let get_result = get_initiative(&pool, initiative.id, user_id).await;
        assert!(
            matches!(get_result, Err(AppError::NotFound)),
            "soft-deleted initiative must not be returned by get_initiative"
        );
    }
}

use sqlx::PgPool;
use uuid::Uuid;

use super::models::{CreateEpicRequest, Epic, UpdateEpicRequest};
use crate::error::AppError;

/// Verify that the initiative identified by `initiative_id` is owned by `user_id`.
/// Returns `AppError::Forbidden` if the initiative does not exist or belongs to another user.
pub async fn check_initiative_ownership(
    pool: &PgPool,
    initiative_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let exists: Option<bool> = sqlx::query_scalar(
        "SELECT TRUE FROM initiatives \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(initiative_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    if exists.is_none() {
        return Err(AppError::Forbidden);
    }
    Ok(())
}

pub async fn list_epics(
    pool: &PgPool,
    user_id: Uuid,
    initiative_id: Option<Uuid>,
) -> Result<Vec<Epic>, AppError> {
    let rows = if let Some(init_id) = initiative_id {
        sqlx::query_as::<_, Epic>(
            "SELECT id, user_id, initiative_id, title, description, status, sort_order, \
                    created_at, updated_at, deleted_at \
             FROM guidance_epics \
             WHERE user_id = $1 AND initiative_id = $2 AND deleted_at IS NULL \
             ORDER BY sort_order ASC, created_at ASC",
        )
        .bind(user_id)
        .bind(init_id)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, Epic>(
            "SELECT id, user_id, initiative_id, title, description, status, sort_order, \
                    created_at, updated_at, deleted_at \
             FROM guidance_epics \
             WHERE user_id = $1 AND deleted_at IS NULL \
             ORDER BY sort_order ASC, created_at ASC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?
    };

    Ok(rows)
}

pub async fn get_epic(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Epic, AppError> {
    let row = sqlx::query_as::<_, Epic>(
        "SELECT id, user_id, initiative_id, title, description, status, sort_order, \
                created_at, updated_at, deleted_at \
         FROM guidance_epics \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or(AppError::NotFound)
}

pub async fn create_epic(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateEpicRequest,
) -> Result<Epic, AppError> {
    check_initiative_ownership(pool, req.initiative_id, user_id).await?;

    let row = sqlx::query_as::<_, Epic>(
        "INSERT INTO guidance_epics \
            (id, user_id, initiative_id, title, description, status, sort_order) \
         VALUES \
            (gen_random_uuid(), $1, $2, $3, $4, COALESCE($5, 'not_started'), COALESCE($6, 0)) \
         RETURNING id, user_id, initiative_id, title, description, status, sort_order, \
                   created_at, updated_at, deleted_at",
    )
    .bind(user_id)
    .bind(req.initiative_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.status)
    .bind(req.sort_order)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn update_epic(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    req: UpdateEpicRequest,
) -> Result<Epic, AppError> {
    let row = sqlx::query_as::<_, Epic>(
        "UPDATE guidance_epics \
         SET \
           title       = COALESCE($3, title), \
           description = COALESCE($4, description), \
           status      = COALESCE($5, status), \
           sort_order  = COALESCE($6, sort_order), \
           updated_at  = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL \
         RETURNING id, user_id, initiative_id, title, description, status, sort_order, \
                   created_at, updated_at, deleted_at",
    )
    .bind(id)
    .bind(user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.status)
    .bind(req.sort_order)
    .fetch_optional(pool)
    .await?;

    row.ok_or(AppError::NotFound)
}

pub async fn delete_epic(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query(
        "UPDATE guidance_epics \
         SET deleted_at = NOW(), updated_at = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests (S002-T) — sqlx::test integration tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guidance::epics::models::{CreateEpicRequest, EpicStatus, UpdateEpicRequest};
    use crate::error::AppError;
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

    async fn insert_test_initiative(
        pool: &PgPool,
        initiative_id: Uuid,
        user_id: Uuid,
        title: &str,
    ) {
        sqlx::query(
            "INSERT INTO initiatives (id, user_id, title, status) \
             VALUES ($1, $2, $3, 'draft')",
        )
        .bind(initiative_id)
        .bind(user_id)
        .bind(title)
        .execute(pool)
        .await
        .expect("Failed to insert test initiative");
    }

    // A-G-01: create epic with valid initiative_id returns the created epic
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_epic_with_valid_initiative_succeeds(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let initiative_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "user@example.com").await;
        insert_test_initiative(&pool, initiative_id, user_id, "My Initiative").await;

        let epic = create_epic(
            &pool,
            user_id,
            CreateEpicRequest {
                initiative_id,
                title: "Phase 1".to_string(),
                description: Some("First phase".to_string()),
                status: None,
                sort_order: None,
            },
        )
        .await
        .expect("create_epic should succeed");

        assert_eq!(epic.user_id, user_id);
        assert_eq!(epic.initiative_id, initiative_id);
        assert_eq!(epic.title, "Phase 1");
        assert_eq!(epic.description, Some("First phase".to_string()));
        assert_eq!(epic.status, EpicStatus::NotStarted);
        assert_eq!(epic.sort_order, 0);
        assert!(epic.deleted_at.is_none());
    }

    // A-G-02: create epic with initiative owned by another user returns Forbidden
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_epic_with_other_users_initiative_returns_forbidden(pool: PgPool) {
        let owner = Uuid::new_v4();
        let attacker = Uuid::new_v4();
        let initiative_id = Uuid::new_v4();
        insert_test_user(&pool, owner, "owner@example.com").await;
        insert_test_user(&pool, attacker, "attacker@example.com").await;
        insert_test_initiative(&pool, initiative_id, owner, "Owner's Initiative").await;

        let result = create_epic(
            &pool,
            attacker,
            CreateEpicRequest {
                initiative_id,
                title: "Malicious Epic".to_string(),
                description: None,
                status: None,
                sort_order: None,
            },
        )
        .await;

        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "expected Forbidden, got: {:?}",
            result
        );
    }

    // A-G-01: create epic returns 201-equivalent with correct fields
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_epic_returns_correct_fields(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let initiative_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "fields@example.com").await;
        insert_test_initiative(&pool, initiative_id, user_id, "Initiative").await;

        let epic = create_epic(
            &pool,
            user_id,
            CreateEpicRequest {
                initiative_id,
                title: "Test Epic".to_string(),
                description: None,
                status: Some(EpicStatus::InProgress),
                sort_order: Some(5),
            },
        )
        .await
        .expect("create_epic should succeed");

        assert_eq!(epic.status, EpicStatus::InProgress, "status should match provided value");
        assert_eq!(epic.sort_order, 5, "sort_order should match provided value");
        assert!(epic.id != Uuid::nil(), "id must be set");
        assert!(epic.created_at <= epic.updated_at || epic.created_at == epic.updated_at);
    }

    // get epic with wrong user_id returns NotFound
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn get_epic_wrong_user_returns_not_found(pool: PgPool) {
        let owner = Uuid::new_v4();
        let other = Uuid::new_v4();
        let initiative_id = Uuid::new_v4();
        insert_test_user(&pool, owner, "owner2@example.com").await;
        insert_test_user(&pool, other, "other@example.com").await;
        insert_test_initiative(&pool, initiative_id, owner, "Owner Initiative").await;

        let epic = create_epic(
            &pool,
            owner,
            CreateEpicRequest {
                initiative_id,
                title: "Owner Epic".to_string(),
                description: None,
                status: None,
                sort_order: None,
            },
        )
        .await
        .expect("create_epic should succeed");

        let result = get_epic(&pool, epic.id, other).await;
        assert!(
            matches!(result, Err(AppError::NotFound)),
            "expected NotFound, got: {:?}",
            result
        );
    }

    // soft-delete excludes row from subsequent list
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn soft_delete_excludes_from_list(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let initiative_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "delete@example.com").await;
        insert_test_initiative(&pool, initiative_id, user_id, "Initiative").await;

        let epic = create_epic(
            &pool,
            user_id,
            CreateEpicRequest {
                initiative_id,
                title: "To Delete".to_string(),
                description: None,
                status: None,
                sort_order: None,
            },
        )
        .await
        .expect("create_epic should succeed");

        // Verify it appears in list before deletion.
        let before = list_epics(&pool, user_id, None).await.expect("list_epics failed");
        assert_eq!(before.len(), 1);

        delete_epic(&pool, epic.id, user_id).await.expect("delete_epic failed");

        // After deletion it must be excluded from list.
        let after = list_epics(&pool, user_id, None).await.expect("list_epics failed");
        assert!(after.is_empty(), "soft-deleted epic must not appear in list");

        // Confirm deleted_at is set in DB.
        let deleted_at: Option<chrono::DateTime<chrono::Utc>> =
            sqlx::query_scalar("SELECT deleted_at FROM guidance_epics WHERE id = $1")
                .bind(epic.id)
                .fetch_one(&pool)
                .await
                .expect("Row must still exist after soft delete");
        assert!(deleted_at.is_some(), "deleted_at must be non-null after soft delete");
    }

    // partial update leaves unspecified fields unchanged
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn partial_update_leaves_unspecified_fields_unchanged(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let initiative_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "partial@example.com").await;
        insert_test_initiative(&pool, initiative_id, user_id, "Initiative").await;

        let epic = create_epic(
            &pool,
            user_id,
            CreateEpicRequest {
                initiative_id,
                title: "Original Title".to_string(),
                description: Some("Original description".to_string()),
                status: None,
                sort_order: Some(3),
            },
        )
        .await
        .expect("create_epic should succeed");

        // Only update the title; leave all others as None.
        let updated = update_epic(
            &pool,
            epic.id,
            user_id,
            UpdateEpicRequest {
                title: Some("New Title".to_string()),
                description: None,
                status: None,
                sort_order: None,
            },
        )
        .await
        .expect("update_epic failed");

        assert_eq!(updated.title, "New Title", "title should be updated");
        assert_eq!(
            updated.description,
            Some("Original description".to_string()),
            "description must remain unchanged"
        );
        assert_eq!(updated.status, EpicStatus::NotStarted, "status must remain unchanged");
        assert_eq!(updated.sort_order, 3, "sort_order must remain unchanged");
    }
}

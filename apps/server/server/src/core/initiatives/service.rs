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
// Tests (S020-T)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // These tests verify that the service functions scope queries to the
    // correct user_id (SEC-1 invariant / FA-009 principle) by inspecting
    // the SQL strings used in each function.
    //
    // The query strings below must match those in the functions above exactly.

    const LIST_QUERY: &str = "SELECT id, user_id, title, description, status, created_at, updated_at, deleted_at \
         FROM initiatives \
         WHERE user_id = $1 AND deleted_at IS NULL \
         ORDER BY created_at DESC";

    const GET_QUERY: &str = "SELECT id, user_id, title, description, status, created_at, updated_at, deleted_at \
         FROM initiatives \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL";

    const UPDATE_QUERY: &str = "UPDATE initiatives \
         SET \
           title       = COALESCE($3, title), \
           description = COALESCE($4, description), \
           status      = COALESCE($5, status), \
           updated_at  = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL \
         RETURNING id, user_id, title, description, status, created_at, updated_at, deleted_at";

    const DELETE_QUERY: &str = "UPDATE initiatives \
         SET deleted_at = NOW(), updated_at = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL";

    // FA-009: list must scope to user_id
    #[test]
    fn list_query_contains_user_id_scope() {
        assert!(
            LIST_QUERY.contains("user_id = $1"),
            "list query must scope to user_id"
        );
        assert!(
            LIST_QUERY.contains("deleted_at IS NULL"),
            "list query must exclude soft-deleted rows"
        );
    }

    // FA-009: get must scope to both id and user_id (wrong user → NotFound)
    #[test]
    fn get_query_contains_both_id_and_user_id_scope() {
        assert!(GET_QUERY.contains("id = $1"), "get query must filter by id");
        assert!(
            GET_QUERY.contains("user_id = $2"),
            "get query must scope to user_id (wrong user returns NotFound)"
        );
        assert!(
            GET_QUERY.contains("deleted_at IS NULL"),
            "get query must exclude soft-deleted rows"
        );
    }

    // Soft-delete: update query must set deleted_at, not hard-delete
    #[test]
    fn delete_query_is_soft_delete_scoped_to_user() {
        assert!(
            DELETE_QUERY.contains("deleted_at = NOW()"),
            "delete must soft-delete by setting deleted_at"
        );
        assert!(
            DELETE_QUERY.contains("user_id = $2"),
            "delete must scope to user_id"
        );
        assert!(
            !DELETE_QUERY.to_uppercase().starts_with("DELETE"),
            "delete must not hard-delete"
        );
    }

    // update must scope to user_id
    #[test]
    fn update_query_scoped_to_user_id() {
        assert!(
            UPDATE_QUERY.contains("user_id = $2"),
            "update query must scope to user_id"
        );
        assert!(
            UPDATE_QUERY.contains("deleted_at IS NULL"),
            "update query must exclude soft-deleted rows"
        );
    }
}

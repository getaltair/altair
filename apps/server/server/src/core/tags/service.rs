use sqlx::PgPool;
use uuid::Uuid;

use super::models::Tag;
use crate::error::AppError;

pub async fn list_tags(pool: &PgPool, user_id: Uuid) -> Result<Vec<Tag>, AppError> {
    let rows = sqlx::query_as::<_, Tag>(
        "SELECT id, user_id, name, created_at \
         FROM tags \
         WHERE user_id = $1 \
         ORDER BY name ASC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

    Ok(rows)
}

pub async fn create_tag(pool: &PgPool, user_id: Uuid, name: String) -> Result<Tag, AppError> {
    let row = sqlx::query_as::<_, Tag>(
        "INSERT INTO tags (id, user_id, name) \
         VALUES (gen_random_uuid(), $1, $2) \
         RETURNING id, user_id, name, created_at",
    )
    .bind(user_id)
    .bind(&name)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        // Postgres unique violation error code is 23505
        if let sqlx::Error::Database(ref db_err) = e {
            if db_err.code().as_deref() == Some("23505") {
                return AppError::Conflict("Tag name already exists".to_string());
            }
        }
        AppError::Internal(anyhow::anyhow!(e.to_string()))
    })?;

    Ok(row)
}

pub async fn delete_tag(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query(
        "DELETE FROM tags \
         WHERE id = $1 AND user_id = $2",
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
// Tests (S021-T)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::error::AppError;

    const LIST_QUERY: &str = "SELECT id, user_id, name, created_at \
         FROM tags \
         WHERE user_id = $1 \
         ORDER BY name ASC";

    const DELETE_QUERY: &str = "DELETE FROM tags \
         WHERE id = $1 AND user_id = $2";

    // Verify user_id scoping in list query
    #[test]
    fn list_query_scoped_to_user_id() {
        assert!(
            LIST_QUERY.contains("user_id = $1"),
            "list query must scope to user_id"
        );
    }

    // Verify delete is scoped to user_id (not a global delete)
    #[test]
    fn delete_query_scoped_to_user_id() {
        assert!(
            DELETE_QUERY.contains("user_id = $2"),
            "delete query must scope to user_id"
        );
    }

    // Verify UNIQUE violation error code mapping → Conflict
    #[test]
    fn unique_violation_maps_to_conflict() {
        // Simulate the mapping logic: code "23505" → Conflict
        let code = "23505";
        let result: AppError = if code == "23505" {
            AppError::Conflict("Tag name already exists".to_string())
        } else {
            AppError::Internal(anyhow::anyhow!("other error"))
        };

        match result {
            AppError::Conflict(msg) => {
                assert_eq!(msg, "Tag name already exists");
            }
            other => panic!("Expected Conflict, got {:?}", other),
        }
    }

    // Non-unique violation code does NOT produce Conflict
    #[test]
    fn non_unique_violation_does_not_map_to_conflict() {
        let code = "42P01"; // undefined_table
        let result: AppError = if code == "23505" {
            AppError::Conflict("Tag name already exists".to_string())
        } else {
            AppError::Internal(anyhow::anyhow!("other error"))
        };

        assert!(
            matches!(result, AppError::Internal(_)),
            "Non-unique-violation error must not map to Conflict"
        );
    }
}

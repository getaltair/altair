use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use super::models::{CreateTagRequest, Tag, UpdateTagRequest};

/// Create a new tag owned by the given user.
///
/// If `household_id` is provided, verifies the user is a member of that
/// household before creating the tag.
pub async fn create_tag(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateTagRequest,
) -> Result<Tag, AppError> {
    // If household_id is provided, verify the user is a member
    if let Some(household_id) = req.household_id {
        let is_member =
            crate::core::households::service::is_member(pool, household_id, user_id).await?;
        if !is_member {
            return Err(AppError::Forbidden(
                "You are not a member of this household".to_string(),
            ));
        }
    }

    sqlx::query_as::<_, Tag>(
        r#"INSERT INTO tags (user_id, household_id, name, color)
           VALUES ($1, $2, $3, $4)
           RETURNING id, user_id, household_id, name, color, created_at"#,
    )
    .bind(user_id)
    .bind(req.household_id)
    .bind(&req.name)
    .bind(&req.color)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

/// List tags visible to the user.
///
/// When `household_id` is provided, returns tags belonging to that household
/// (after verifying membership). Otherwise returns all tags owned by the user
/// directly.
pub async fn list_tags(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Option<Uuid>,
) -> Result<Vec<Tag>, AppError> {
    if let Some(hid) = household_id {
        // Verify user is a member of the household
        let is_member =
            crate::core::households::service::is_member(pool, hid, user_id).await?;
        if !is_member {
            return Err(AppError::Forbidden(
                "You are not a member of this household".to_string(),
            ));
        }

        sqlx::query_as::<_, Tag>(
            r#"SELECT id, user_id, household_id, name, color, created_at
               FROM tags
               WHERE household_id = $1"#,
        )
        .bind(hid)
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)
    } else {
        sqlx::query_as::<_, Tag>(
            r#"SELECT id, user_id, household_id, name, color, created_at
               FROM tags
               WHERE user_id = $1"#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)
    }
}

/// Update a tag's fields.
///
/// Only the tag owner can update it. Uses COALESCE to apply partial updates,
/// leaving unchanged fields at their current values.
pub async fn update_tag(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    req: UpdateTagRequest,
) -> Result<Tag, AppError> {
    let tag = sqlx::query_as::<_, Tag>(
        r#"UPDATE tags
           SET name = COALESCE($1, name),
               color = COALESCE($2, color)
           WHERE id = $3 AND user_id = $4
           RETURNING id, user_id, household_id, name, color, created_at"#,
    )
    .bind(&req.name)
    .bind(&req.color)
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| {
        AppError::NotFound("Tag not found or you do not have permission".to_string())
    })?;

    Ok(tag)
}

/// Hard-delete a tag.
///
/// Only the tag owner can delete it. This permanently removes the row.
pub async fn delete_tag(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query(
        "DELETE FROM tags WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Tag not found or you do not have permission".to_string(),
        ));
    }

    Ok(())
}

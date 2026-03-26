use sqlx::{PgPool, QueryBuilder};
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
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Tag already exists".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
            AppError::BadRequest("Invalid field value".to_string())
        }
        _ => AppError::Database(e),
    })
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

/// Update a tag's fields with true partial-update semantics.
///
/// Only the tag owner can update it. Each field follows these rules:
/// - `name`: `None` leaves it unchanged, `Some(val)` sets it to `val`
/// - `color`: `None` leaves it unchanged, `Some(None)` sets it to NULL,
///   `Some(Some(val))` sets it to `val`
pub async fn update_tag(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    req: UpdateTagRequest,
) -> Result<Tag, AppError> {
    let mut qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE tags SET name = name");

    if let Some(ref name) = req.name {
        qb.push(", name = ");
        qb.push_bind(name.clone());
    }

    match &req.color {
        Some(None) => {
            qb.push(", color = NULL");
        }
        Some(Some(val)) => {
            qb.push(", color = ");
            qb.push_bind(val.clone());
        }
        None => {}
    }

    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" AND user_id = ");
    qb.push_bind(user_id);
    qb.push(" RETURNING id, user_id, household_id, name, color, created_at");

    let tag = qb
        .build_query_as::<Tag>()
        .fetch_optional(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Tag already exists".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
                AppError::BadRequest("Invalid field value".to_string())
            }
            _ => AppError::Database(e),
        })?
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

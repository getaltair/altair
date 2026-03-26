use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::error::AppError;
use super::models::*;

/// Create a new epic for the given user.
///
/// Priority defaults to "medium" when not specified in the request.
pub async fn create_epic(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateEpicRequest,
) -> Result<GuidanceEpic, AppError> {
    let priority = req.priority.as_deref().unwrap_or("medium");

    sqlx::query_as::<_, GuidanceEpic>(
        r#"INSERT INTO guidance_epics (initiative_id, user_id, name, description, priority)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, initiative_id, user_id, name, description, status, priority, created_at, updated_at"#,
    )
    .bind(req.initiative_id)
    .bind(user_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(priority)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

/// List all epics owned by the user
pub async fn list_epics(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<GuidanceEpic>, AppError> {
    sqlx::query_as::<_, GuidanceEpic>(
        r#"SELECT id, initiative_id, user_id, name, description, status, priority, created_at, updated_at
           FROM guidance_epics
           WHERE user_id = $1
           ORDER BY created_at DESC"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Get a single epic by ID, scoped to the user
pub async fn get_epic(
    pool: &PgPool,
    epic_id: Uuid,
    user_id: Uuid,
) -> Result<GuidanceEpic, AppError> {
    sqlx::query_as::<_, GuidanceEpic>(
        r#"SELECT id, initiative_id, user_id, name, description, status, priority, created_at, updated_at
           FROM guidance_epics
           WHERE id = $1 AND user_id = $2"#,
    )
    .bind(epic_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Epic not found".to_string()))
}

/// Update an epic's fields using dynamic query building.
///
/// Only provided fields are updated. Returns 404 if the epic does not exist
/// or does not belong to the user.
pub async fn update_epic(
    pool: &PgPool,
    epic_id: Uuid,
    user_id: Uuid,
    req: &UpdateEpicRequest,
) -> Result<GuidanceEpic, AppError> {
    let mut qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE guidance_epics SET updated_at = now()");

    if let Some(ref name) = req.name {
        qb.push(", name = ");
        qb.push_bind(name.clone());
    }

    if let Some(ref description) = req.description {
        qb.push(", description = ");
        qb.push_bind(description.clone());
    }

    if let Some(ref status) = req.status {
        qb.push(", status = ");
        qb.push_bind(status.clone());
    }

    if let Some(ref priority) = req.priority {
        qb.push(", priority = ");
        qb.push_bind(priority.clone());
    }

    if let Some(ref initiative_id) = req.initiative_id {
        qb.push(", initiative_id = ");
        qb.push_bind(*initiative_id);
    }

    qb.push(" WHERE id = ");
    qb.push_bind(epic_id);
    qb.push(" AND user_id = ");
    qb.push_bind(user_id);
    qb.push(" RETURNING id, initiative_id, user_id, name, description, status, priority, created_at, updated_at");

    qb.build_query_as::<GuidanceEpic>()
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Epic not found".to_string()))
}

/// Delete an epic by ID, scoped to the user.
///
/// Returns 404 if the epic does not exist or does not belong to the user.
pub async fn delete_epic(
    pool: &PgPool,
    epic_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM guidance_epics WHERE id = $1 AND user_id = $2")
        .bind(epic_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Epic not found".to_string()));
    }

    Ok(())
}

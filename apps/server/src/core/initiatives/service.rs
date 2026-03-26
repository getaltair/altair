use sqlx::PgPool;
use uuid::Uuid;

use crate::contracts::INITIATIVE_STATUSES;
use crate::error::AppError;
use super::models::{CreateInitiativeRequest, Initiative, UpdateInitiativeRequest};

/// Create a new initiative owned by the given user.
///
/// If `household_id` is provided, verifies the user is a member of that
/// household before creating the initiative. Status defaults to "active"
/// when not specified in the request.
pub async fn create_initiative(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateInitiativeRequest,
) -> Result<Initiative, AppError> {
    // Validate status if provided
    let status = req.status.as_deref().unwrap_or("active");
    if !INITIATIVE_STATUSES.contains(&status) {
        return Err(AppError::BadRequest(format!(
            "Invalid status '{}'. Must be one of: {}",
            status,
            INITIATIVE_STATUSES.join(", ")
        )));
    }

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

    sqlx::query_as::<_, Initiative>(
        r#"INSERT INTO initiatives (user_id, household_id, name, description, status)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, user_id, household_id, name, description, status, created_at, updated_at"#,
    )
    .bind(user_id)
    .bind(req.household_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(status)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

/// List initiatives visible to the user.
///
/// When `household_id` is provided, returns initiatives belonging to that
/// household (after verifying membership). Otherwise returns all initiatives
/// owned by the user directly.
pub async fn list_initiatives(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Option<Uuid>,
) -> Result<Vec<Initiative>, AppError> {
    if let Some(hid) = household_id {
        // Verify user is a member of the household
        let is_member =
            crate::core::households::service::is_member(pool, hid, user_id).await?;
        if !is_member {
            return Err(AppError::Forbidden(
                "You are not a member of this household".to_string(),
            ));
        }

        sqlx::query_as::<_, Initiative>(
            r#"SELECT id, user_id, household_id, name, description, status, created_at, updated_at
               FROM initiatives
               WHERE household_id = $1"#,
        )
        .bind(hid)
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)
    } else {
        sqlx::query_as::<_, Initiative>(
            r#"SELECT id, user_id, household_id, name, description, status, created_at, updated_at
               FROM initiatives
               WHERE user_id = $1"#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)
    }
}

/// Get a single initiative by ID.
///
/// The user must either own the initiative directly or be a member of
/// the household the initiative belongs to.
pub async fn get_initiative(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<Initiative, AppError> {
    let initiative = sqlx::query_as::<_, Initiative>(
        r#"SELECT id, user_id, household_id, name, description, status, created_at, updated_at
           FROM initiatives
           WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Initiative not found".to_string()))?;

    // Check access: user owns it directly or is a member of its household
    if initiative.user_id == user_id {
        return Ok(initiative);
    }

    if let Some(household_id) = initiative.household_id {
        let is_member =
            crate::core::households::service::is_member(pool, household_id, user_id).await?;
        if is_member {
            return Ok(initiative);
        }
    }

    Err(AppError::Forbidden(
        "You do not have access to this initiative".to_string(),
    ))
}

/// Update an initiative's fields.
///
/// Only the initiative owner can update it. Uses COALESCE to apply partial
/// updates, leaving unchanged fields at their current values.
pub async fn update_initiative(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    req: UpdateInitiativeRequest,
) -> Result<Initiative, AppError> {
    // Validate status if provided
    if let Some(ref status) = req.status
        && !INITIATIVE_STATUSES.contains(&status.as_str())
    {
        return Err(AppError::BadRequest(format!(
            "Invalid status '{}'. Must be one of: {}",
            status,
            INITIATIVE_STATUSES.join(", ")
        )));
    }

    let initiative = sqlx::query_as::<_, Initiative>(
        r#"UPDATE initiatives
           SET name = COALESCE($1, name),
               description = COALESCE($2, description),
               status = COALESCE($3, status),
               updated_at = now()
           WHERE id = $4 AND user_id = $5
           RETURNING id, user_id, household_id, name, description, status, created_at, updated_at"#,
    )
    .bind(&req.name)
    .bind(&req.description)
    .bind(&req.status)
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| {
        AppError::NotFound("Initiative not found or you do not have permission".to_string())
    })?;

    Ok(initiative)
}

/// Soft-delete an initiative by setting its status to "archived".
///
/// Only the initiative owner can archive it. This does not remove the row.
pub async fn soft_delete_initiative(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query(
        r#"UPDATE initiatives
           SET status = 'archived', updated_at = now()
           WHERE id = $1 AND user_id = $2"#,
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Initiative not found or you do not have permission".to_string(),
        ));
    }

    Ok(())
}

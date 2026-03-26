use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::contracts::InitiativeStatus;
use crate::error::AppError;
use super::models::{CreateInitiativeRequest, Initiative, UpdateInitiativeRequest};

/// Create a new initiative owned by the given user.
///
/// If `household_id` is provided, verifies the user is a member of that
/// household before creating the initiative. Status defaults to `Active`
/// when not specified in the request.
pub async fn create_initiative(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateInitiativeRequest,
) -> Result<Initiative, AppError> {
    let status = req.status.unwrap_or(InitiativeStatus::Active);

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
    .bind(status.as_str())
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Initiative already exists".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
            AppError::BadRequest("Invalid field value".to_string())
        }
        _ => AppError::Database(e),
    })
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

/// Update an initiative's fields with true partial-update semantics.
///
/// Only the initiative owner can update it. Each field follows these rules:
/// - `name`: `None` leaves it unchanged, `Some(val)` sets it to `val`
/// - `description`: `None` leaves it unchanged, `Some(None)` sets it to NULL,
///   `Some(Some(val))` sets it to `val`
/// - `status`: `None` leaves it unchanged, `Some(val)` sets it to `val`
pub async fn update_initiative(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    req: UpdateInitiativeRequest,
) -> Result<Initiative, AppError> {
    let mut qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE initiatives SET updated_at = now()");

    if let Some(ref name) = req.name {
        qb.push(", name = ");
        qb.push_bind(name.clone());
    }

    match &req.description {
        Some(None) => {
            qb.push(", description = NULL");
        }
        Some(Some(val)) => {
            qb.push(", description = ");
            qb.push_bind(val.clone());
        }
        None => {}
    }

    if let Some(ref status) = req.status {
        qb.push(", status = ");
        qb.push_bind(status.as_str());
    }

    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" AND user_id = ");
    qb.push_bind(user_id);
    qb.push(" RETURNING id, user_id, household_id, name, description, status, created_at, updated_at");

    let initiative = qb
        .build_query_as::<Initiative>()
        .fetch_optional(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Initiative already exists".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
                AppError::BadRequest("Invalid field value".to_string())
            }
            _ => AppError::Database(e),
        })?
        .ok_or_else(|| {
            AppError::NotFound(
                "Initiative not found or you do not have permission".to_string(),
            )
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
           SET status = $1, updated_at = now()
           WHERE id = $2 AND user_id = $3"#,
    )
    .bind(InitiativeStatus::Archived.as_str())
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

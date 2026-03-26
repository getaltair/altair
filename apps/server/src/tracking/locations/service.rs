use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::error::AppError;
use super::models::{CreateLocationRequest, TrackingLocation, UpdateLocationRequest};

/// Create a new tracking location within a household.
///
/// The caller is responsible for verifying household membership before
/// invoking this function.
pub async fn create_location(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateLocationRequest,
) -> Result<TrackingLocation, AppError> {
    sqlx::query_as::<_, TrackingLocation>(
        r#"INSERT INTO tracking_locations (user_id, household_id, name, description, parent_location_id)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, user_id, household_id, name, description, parent_location_id, created_at, updated_at"#,
    )
    .bind(user_id)
    .bind(req.household_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(req.parent_location_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Location already exists".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
            AppError::BadRequest("Invalid parent location or household".to_string())
        }
        _ => AppError::Database(e),
    })
}

/// List all tracking locations belonging to a household.
pub async fn list_locations(
    pool: &PgPool,
    household_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<TrackingLocation>, AppError> {
    sqlx::query_as::<_, TrackingLocation>(
        r#"SELECT id, user_id, household_id, name, description, parent_location_id, created_at, updated_at
           FROM tracking_locations
           WHERE household_id = $1
           ORDER BY name
           LIMIT $2 OFFSET $3"#,
    )
    .bind(household_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Get a single tracking location by ID.
///
/// Returns `AppError::NotFound` if no location exists with the given ID.
pub async fn get_location(
    pool: &PgPool,
    id: Uuid,
) -> Result<TrackingLocation, AppError> {
    sqlx::query_as::<_, TrackingLocation>(
        r#"SELECT id, user_id, household_id, name, description, parent_location_id, created_at, updated_at
           FROM tracking_locations
           WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Location not found".to_string()))
}

/// Update a tracking location's fields with true partial-update semantics.
///
/// Only fields present in the request are modified. The `description` field
/// uses double-option semantics: `None` leaves it unchanged, `Some(None)`
/// sets it to NULL, `Some(Some(val))` sets it to `val`.
pub async fn update_location(
    pool: &PgPool,
    id: Uuid,
    req: UpdateLocationRequest,
) -> Result<TrackingLocation, AppError> {
    let mut qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE tracking_locations SET updated_at = now()");

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

    match &req.parent_location_id {
        Some(None) => {
            qb.push(", parent_location_id = NULL");
        }
        Some(Some(val)) => {
            qb.push(", parent_location_id = ");
            qb.push_bind(*val);
        }
        None => {}
    }

    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" RETURNING id, user_id, household_id, name, description, parent_location_id, created_at, updated_at");

    qb.build_query_as::<TrackingLocation>()
        .fetch_optional(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::BadRequest("Invalid parent location".to_string())
            }
            _ => AppError::Database(e),
        })?
        .ok_or_else(|| AppError::NotFound("Location not found".to_string()))
}

/// Delete a tracking location by ID.
///
/// Returns `AppError::NotFound` if no location exists with the given ID.
pub async fn delete_location(
    pool: &PgPool,
    id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM tracking_locations WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::Conflict("Cannot delete: items still reference this location".to_string())
            }
            _ => AppError::Database(e),
        })?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Location not found".to_string()));
    }

    Ok(())
}

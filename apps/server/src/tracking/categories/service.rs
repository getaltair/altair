use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::error::AppError;
use super::models::{CreateCategoryRequest, TrackingCategory, UpdateCategoryRequest};

/// Create a new tracking category within a household.
///
/// The caller is responsible for verifying household membership before
/// invoking this function.
pub async fn create_category(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateCategoryRequest,
) -> Result<TrackingCategory, AppError> {
    sqlx::query_as::<_, TrackingCategory>(
        r#"INSERT INTO tracking_categories (user_id, household_id, name, description, parent_category_id)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, user_id, household_id, name, description, parent_category_id, created_at, updated_at"#,
    )
    .bind(user_id)
    .bind(req.household_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(req.parent_category_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Category already exists".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
            AppError::BadRequest("Invalid parent category or household".to_string())
        }
        _ => AppError::Database(e),
    })
}

/// List all tracking categories belonging to a household.
pub async fn list_categories(
    pool: &PgPool,
    household_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<TrackingCategory>, AppError> {
    sqlx::query_as::<_, TrackingCategory>(
        r#"SELECT id, user_id, household_id, name, description, parent_category_id, created_at, updated_at
           FROM tracking_categories
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

/// Get a single tracking category by ID.
///
/// Returns `AppError::NotFound` if no category exists with the given ID.
pub async fn get_category(
    pool: &PgPool,
    id: Uuid,
) -> Result<TrackingCategory, AppError> {
    sqlx::query_as::<_, TrackingCategory>(
        r#"SELECT id, user_id, household_id, name, description, parent_category_id, created_at, updated_at
           FROM tracking_categories
           WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Category not found".to_string()))
}

/// Update a tracking category's fields with true partial-update semantics.
///
/// Only fields present in the request are modified. The `description` field
/// uses double-option semantics: `None` leaves it unchanged, `Some(None)`
/// sets it to NULL, `Some(Some(val))` sets it to `val`.
pub async fn update_category(
    pool: &PgPool,
    id: Uuid,
    req: UpdateCategoryRequest,
) -> Result<TrackingCategory, AppError> {
    let mut qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE tracking_categories SET updated_at = now()");

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

    match &req.parent_category_id {
        Some(None) => {
            qb.push(", parent_category_id = NULL");
        }
        Some(Some(val)) => {
            qb.push(", parent_category_id = ");
            qb.push_bind(*val);
        }
        None => {}
    }

    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" RETURNING id, user_id, household_id, name, description, parent_category_id, created_at, updated_at");

    qb.build_query_as::<TrackingCategory>()
        .fetch_optional(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::BadRequest("Invalid parent category".to_string())
            }
            _ => AppError::Database(e),
        })?
        .ok_or_else(|| AppError::NotFound("Category not found".to_string()))
}

/// Delete a tracking category by ID.
///
/// Returns `AppError::NotFound` if no category exists with the given ID.
pub async fn delete_category(
    pool: &PgPool,
    id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM tracking_categories WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::Conflict("Cannot delete: items still reference this category".to_string())
            }
            _ => AppError::Database(e),
        })?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Category not found".to_string()));
    }

    Ok(())
}

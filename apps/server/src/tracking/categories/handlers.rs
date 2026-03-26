use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::middleware::AuthenticatedUser;
use crate::auth::service::get_user_household_ids;
use crate::error::AppError;
use super::{
    models::{CreateCategoryRequest, TrackingCategory, UpdateCategoryRequest},
    service,
};

/// Query parameters for listing tracking categories
#[derive(Debug, Deserialize)]
pub struct ListCategoriesParams {
    pub household_id: Uuid,
}

/// Verify the authenticated user is a member of the given household
async fn verify_household_membership(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
) -> Result<(), AppError> {
    let household_ids = get_user_household_ids(pool, user_id).await?;
    if !household_ids.contains(&household_id) {
        return Err(AppError::Forbidden(
            "Not a member of this household".to_string(),
        ));
    }
    Ok(())
}

/// Create a new tracking category within a household
pub async fn create_category(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<TrackingCategory>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    verify_household_membership(&pool, auth.user_id, body.household_id).await?;

    let category = service::create_category(&pool, auth.user_id, body).await?;
    Ok((StatusCode::CREATED, Json(category)))
}

/// List all tracking categories for a household
pub async fn list_categories(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Query(params): Query<ListCategoriesParams>,
) -> Result<Json<Vec<TrackingCategory>>, AppError> {
    verify_household_membership(&pool, auth.user_id, params.household_id).await?;

    let categories = service::list_categories(&pool, params.household_id).await?;
    Ok(Json(categories))
}

/// Get a single tracking category by ID
pub async fn get_category(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<TrackingCategory>, AppError> {
    let category = service::get_category(&pool, id).await?;

    verify_household_membership(&pool, auth.user_id, category.household_id).await?;

    Ok(Json(category))
}

/// Update an existing tracking category (partial update)
pub async fn update_category(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateCategoryRequest>,
) -> Result<Json<TrackingCategory>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let existing = service::get_category(&pool, id).await?;
    verify_household_membership(&pool, auth.user_id, existing.household_id).await?;

    let category = service::update_category(&pool, id, body).await?;
    Ok(Json(category))
}

/// Delete a tracking category
pub async fn delete_category(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let existing = service::get_category(&pool, id).await?;
    verify_household_membership(&pool, auth.user_id, existing.household_id).await?;

    service::delete_category(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

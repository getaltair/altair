use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::middleware::AuthenticatedUser;
use crate::error::AppError;
use crate::tracking::{verify_household_membership, PaginationParams};
use super::{
    models::{CreateLocationRequest, TrackingLocation, UpdateLocationRequest},
    service,
};

/// Query parameters for listing tracking locations
#[derive(Debug, Deserialize)]
pub struct ListLocationsParams {
    pub household_id: Uuid,
}

/// Create a new tracking location within a household
pub async fn create_location(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateLocationRequest>,
) -> Result<(StatusCode, Json<TrackingLocation>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    verify_household_membership(&pool, auth.user_id, body.household_id).await?;

    let location = service::create_location(&pool, auth.user_id, body).await?;
    Ok((StatusCode::CREATED, Json(location)))
}

/// List all tracking locations for a household
pub async fn list_locations(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Query(params): Query<ListLocationsParams>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<TrackingLocation>>, AppError> {
    verify_household_membership(&pool, auth.user_id, params.household_id).await?;

    let locations = service::list_locations(
        &pool,
        params.household_id,
        pagination.limit_or_default(),
        pagination.offset_or_default(),
    )
    .await?;
    Ok(Json(locations))
}

/// Get a single tracking location by ID
pub async fn get_location(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<TrackingLocation>, AppError> {
    let location = service::get_location(&pool, id).await?;

    verify_household_membership(&pool, auth.user_id, location.household_id).await?;

    Ok(Json(location))
}

/// Update an existing tracking location (partial update)
pub async fn update_location(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateLocationRequest>,
) -> Result<Json<TrackingLocation>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let existing = service::get_location(&pool, id).await?;
    verify_household_membership(&pool, auth.user_id, existing.household_id).await?;

    let location = service::update_location(&pool, id, body).await?;
    Ok(Json(location))
}

/// Delete a tracking location
pub async fn delete_location(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let existing = service::get_location(&pool, id).await?;
    verify_household_membership(&pool, auth.user_id, existing.household_id).await?;

    service::delete_location(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

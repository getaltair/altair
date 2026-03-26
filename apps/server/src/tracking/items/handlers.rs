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
    models::{
        CreateItemEventRequest, CreateItemRequest, TrackingItem, TrackingItemEvent,
        UpdateItemRequest,
    },
    service,
};

/// Query parameters for listing items
#[derive(Debug, Deserialize)]
pub struct ListItemsQuery {
    pub household_id: Uuid,
}

/// Query parameters for listing low-stock items
#[derive(Debug, Deserialize)]
pub struct LowStockQuery {
    pub household_id: Uuid,
}

/// Create a new tracking item
pub async fn create_item(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<TrackingItem>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    verify_household_membership(&pool, auth.user_id, body.household_id).await?;

    let item = service::create_item(&pool, auth.user_id, body).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

/// List all tracking items for a household
pub async fn list_items(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Query(query): Query<ListItemsQuery>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<TrackingItem>>, AppError> {
    verify_household_membership(&pool, auth.user_id, query.household_id).await?;

    let items = service::list_items(
        &pool,
        query.household_id,
        pagination.limit_or_default(),
        pagination.offset_or_default(),
    )
    .await?;
    Ok(Json(items))
}

/// Get a single tracking item by ID
pub async fn get_item(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<TrackingItem>, AppError> {
    let item = service::get_item(&pool, id).await?;

    verify_household_membership(&pool, auth.user_id, item.household_id).await?;

    Ok(Json(item))
}

/// Update an existing tracking item
pub async fn update_item(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateItemRequest>,
) -> Result<Json<TrackingItem>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Verify the item exists and user has access
    let existing = service::get_item(&pool, id).await?;
    verify_household_membership(&pool, auth.user_id, existing.household_id).await?;

    let item = service::update_item(&pool, id, body).await?;
    Ok(Json(item))
}

/// Delete a tracking item
pub async fn delete_item(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // Verify the item exists and user has access
    let existing = service::get_item(&pool, id).await?;
    verify_household_membership(&pool, auth.user_id, existing.household_id).await?;

    service::delete_item(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Create a new event for a tracking item
pub async fn create_item_event(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(item_id): Path<Uuid>,
    Json(body): Json<CreateItemEventRequest>,
) -> Result<(StatusCode, Json<TrackingItemEvent>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Get item first to verify household membership
    let item = service::get_item(&pool, item_id).await?;
    verify_household_membership(&pool, auth.user_id, item.household_id).await?;

    let event = service::create_item_event(&pool, item_id, auth.user_id, body).await?;
    Ok((StatusCode::CREATED, Json(event)))
}

/// List all events for a tracking item
pub async fn list_item_events(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(item_id): Path<Uuid>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<TrackingItemEvent>>, AppError> {
    // Get item first to verify household membership
    let item = service::get_item(&pool, item_id).await?;
    verify_household_membership(&pool, auth.user_id, item.household_id).await?;

    let events = service::list_item_events(
        &pool,
        item_id,
        pagination.limit_or_default(),
        pagination.offset_or_default(),
    )
    .await?;
    Ok(Json(events))
}

/// List items that are below their minimum quantity threshold
pub async fn list_low_stock_items(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Query(query): Query<LowStockQuery>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<TrackingItem>>, AppError> {
    verify_household_membership(&pool, auth.user_id, query.household_id).await?;

    let items = service::list_low_stock_items(
        &pool,
        query.household_id,
        pagination.limit_or_default(),
        pagination.offset_or_default(),
    )
    .await?;
    Ok(Json(items))
}

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use super::models::{CreateItemRequest, UpdateItemRequest};
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;
use crate::tracking::HouseholdQuery;

#[derive(Debug, Deserialize)]
pub struct ListItemsQuery {
    pub household_id: Uuid,
    pub category_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    50
}

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<ListItemsQuery>,
) -> Result<impl IntoResponse, AppError> {
    if params.limit < 0 || params.offset < 0 {
        return Err(AppError::BadRequest(
            "limit and offset must be non-negative".to_string(),
        ));
    }
    let items = service::list_items(
        &state.db,
        auth.user_id,
        params.household_id,
        params.category_id,
        params.location_id,
        params.limit,
        params.offset,
    )
    .await?;
    Ok(Json(items))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateItemRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("name must not be empty".to_string()));
    }
    let item = service::create_item(&state.db, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn get_one(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(item_id): Path<Uuid>,
    Query(params): Query<HouseholdQuery>,
) -> Result<impl IntoResponse, AppError> {
    let item = service::get_item(&state.db, auth.user_id, params.household_id, item_id).await?;
    Ok(Json(item))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(item_id): Path<Uuid>,
    Query(params): Query<HouseholdQuery>,
    Json(req): Json<UpdateItemRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.name.is_none()
        && req.description.is_none()
        && req.barcode.is_none()
        && req.location_id.is_none()
        && req.category_id.is_none()
        && req.expires_at.is_none()
    {
        return Err(AppError::BadRequest(
            "at least one field must be provided".to_string(),
        ));
    }
    if let Some(name) = &req.name
        && name.trim().is_empty()
    {
        return Err(AppError::BadRequest("name must not be empty".to_string()));
    }
    let item =
        service::update_item(&state.db, auth.user_id, params.household_id, item_id, req).await?;
    Ok(Json(item))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(item_id): Path<Uuid>,
    Query(params): Query<HouseholdQuery>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_item(&state.db, auth.user_id, params.household_id, item_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

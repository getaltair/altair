use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use super::models::{CreateShoppingListItemRequest, UpdateShoppingListItemRequest};
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;
use crate::tracking::HouseholdQuery;

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(list_id): Path<Uuid>,
    Query(q): Query<HouseholdQuery>,
) -> Result<impl IntoResponse, AppError> {
    let items =
        service::list_shopping_list_items(&state.db, auth.user_id, list_id, q.household_id).await?;
    Ok(Json(items))
}

pub async fn add(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(list_id): Path<Uuid>,
    Json(req): Json<CreateShoppingListItemRequest>,
) -> Result<impl IntoResponse, AppError> {
    let item = service::add_shopping_list_item(&state.db, auth.user_id, list_id, req).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((list_id, item_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateShoppingListItemRequest>,
) -> Result<impl IntoResponse, AppError> {
    let item =
        service::update_shopping_list_item(&state.db, auth.user_id, list_id, item_id, req).await?;
    Ok(Json(item))
}

pub async fn remove(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((list_id, item_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    service::remove_shopping_list_item(&state.db, auth.user_id, list_id, item_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

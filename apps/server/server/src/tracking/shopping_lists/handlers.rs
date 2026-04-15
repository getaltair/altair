use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use super::models::{CreateShoppingListRequest, UpdateShoppingListRequest};
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;
use crate::tracking::HouseholdQuery;

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(q): Query<HouseholdQuery>,
) -> Result<impl IntoResponse, AppError> {
    let lists = service::list_shopping_lists(&state.db, auth.user_id, q.household_id).await?;
    Ok(Json(lists))
}

pub async fn get(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(q): Query<HouseholdQuery>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let list = service::get_shopping_list(&state.db, auth.user_id, q.household_id, id).await?;
    Ok(Json(list))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateShoppingListRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("name must not be empty".to_string()));
    }
    let list = service::create_shopping_list(&state.db, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(list)))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(q): Query<HouseholdQuery>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateShoppingListRequest>,
) -> Result<impl IntoResponse, AppError> {
    match &req.name {
        None => return Err(AppError::BadRequest("at least one field must be provided".to_string())),
        Some(name) if name.trim().is_empty() => {
            return Err(AppError::BadRequest("name must not be empty".to_string()))
        }
        Some(_) => {}
    }
    let list =
        service::update_shopping_list(&state.db, auth.user_id, q.household_id, id, req).await?;
    Ok(Json(list))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(q): Query<HouseholdQuery>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_shopping_list(&state.db, auth.user_id, q.household_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

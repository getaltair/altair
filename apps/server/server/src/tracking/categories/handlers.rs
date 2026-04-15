use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use super::models::{CreateCategoryRequest, UpdateCategoryRequest};
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;
use crate::tracking::HouseholdQuery;

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<HouseholdQuery>,
) -> Result<impl IntoResponse, AppError> {
    let categories = service::list_categories(&state.db, auth.user_id, params.household_id).await?;
    Ok(Json(categories))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("name must not be empty".to_string()));
    }
    let category = service::create_category(&state.db, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(category)))
}

pub async fn get_one(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Query(params): Query<HouseholdQuery>,
) -> Result<impl IntoResponse, AppError> {
    let category = service::get_category(&state.db, auth.user_id, params.household_id, id).await?;
    Ok(Json(category))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Query(params): Query<HouseholdQuery>,
    Json(req): Json<UpdateCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    match &req.name {
        None => return Err(AppError::BadRequest("at least one field must be provided".to_string())),
        Some(name) if name.trim().is_empty() => {
            return Err(AppError::BadRequest("name must not be empty".to_string()))
        }
        Some(_) => {}
    }
    let category =
        service::update_category(&state.db, auth.user_id, params.household_id, id, req).await?;
    Ok(Json(category))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Query(params): Query<HouseholdQuery>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_category(&state.db, auth.user_id, params.household_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

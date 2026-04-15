use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use super::models::{CreateLocationRequest, UpdateLocationRequest};
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;
use crate::tracking::HouseholdQuery;

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<HouseholdQuery>,
) -> Result<impl IntoResponse, AppError> {
    let locations = service::list_locations(&state.db, auth.user_id, query.household_id).await?;
    Ok(Json(locations))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateLocationRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("name must not be empty".to_string()));
    }
    let location = service::create_location(&state.db, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(location)))
}

pub async fn get_one(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Query(query): Query<HouseholdQuery>,
) -> Result<impl IntoResponse, AppError> {
    let location = service::get_location(&state.db, auth.user_id, query.household_id, id).await?;
    Ok(Json(location))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Query(query): Query<HouseholdQuery>,
    Json(req): Json<UpdateLocationRequest>,
) -> Result<impl IntoResponse, AppError> {
    match &req.name {
        None => {
            return Err(AppError::BadRequest(
                "at least one field must be provided".to_string(),
            ));
        }
        Some(name) if name.trim().is_empty() => {
            return Err(AppError::BadRequest("name must not be empty".to_string()));
        }
        Some(_) => {}
    }
    let location =
        service::update_location(&state.db, auth.user_id, query.household_id, id, req).await?;
    Ok(Json(location))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Query(query): Query<HouseholdQuery>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_location(&state.db, auth.user_id, query.household_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

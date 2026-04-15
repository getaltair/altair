use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use super::models::{CreateCheckinRequest, UpdateCheckinRequest};
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let checkins = service::list_checkins(&state.db, auth.user_id).await?;
    Ok(Json(checkins))
}

pub async fn get_one(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let checkin = service::get_checkin(&state.db, id, auth.user_id).await?;
    Ok(Json(checkin))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateCheckinRequest>,
) -> Result<impl IntoResponse, AppError> {
    let checkin = service::create_checkin(&state.db, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(checkin)))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCheckinRequest>,
) -> Result<impl IntoResponse, AppError> {
    let checkin = service::update_checkin(&state.db, id, auth.user_id, req).await?;
    Ok(Json(checkin))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_checkin(&state.db, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

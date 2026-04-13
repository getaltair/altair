use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use super::models::{CreateInitiativeRequest, UpdateInitiativeRequest};
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let initiatives = service::list_initiatives(&state.db, auth.user_id).await?;
    Ok(Json(initiatives))
}

pub async fn get_one(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let initiative = service::get_initiative(&state.db, id, auth.user_id).await?;
    Ok(Json(initiative))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateInitiativeRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.title.trim().is_empty() {
        return Err(AppError::BadRequest("title must not be empty".to_string()));
    }
    let initiative = service::create_initiative(&state.db, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(initiative)))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateInitiativeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let initiative = service::update_initiative(&state.db, id, auth.user_id, req).await?;
    Ok(Json(initiative))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_initiative(&state.db, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

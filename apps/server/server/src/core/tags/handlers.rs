use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use super::models::CreateTagRequest;
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let tags = service::list_tags(&state.db, auth.user_id).await?;
    Ok(Json(tags))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateTagRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.name.trim().is_empty() {
        return Err(AppError::BadRequest("name must not be empty".to_string()));
    }
    let tag = service::create_tag(&state.db, auth.user_id, req.name).await?;
    Ok((StatusCode::CREATED, Json(tag)))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_tag(&state.db, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

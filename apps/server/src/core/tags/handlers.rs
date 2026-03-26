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
use super::{
    models::{CreateTagRequest, Tag, UpdateTagRequest},
    service,
};

/// Query parameters for listing tags
#[derive(Debug, Deserialize)]
pub struct ListTagsQuery {
    pub household_id: Option<Uuid>,
}

/// Create a new tag owned by the authenticated user
pub async fn create_tag(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateTagRequest>,
) -> Result<(StatusCode, Json<Tag>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let tag = service::create_tag(&pool, auth.user_id, body).await?;
    Ok((StatusCode::CREATED, Json(tag)))
}

/// List tags visible to the authenticated user, optionally filtered by household
pub async fn list_tags(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Query(query): Query<ListTagsQuery>,
) -> Result<Json<Vec<Tag>>, AppError> {
    let tags = service::list_tags(&pool, auth.user_id, query.household_id).await?;
    Ok(Json(tags))
}

/// Update an existing tag (partial update)
pub async fn update_tag(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTagRequest>,
) -> Result<Json<Tag>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let tag = service::update_tag(&pool, id, auth.user_id, body).await?;
    Ok(Json(tag))
}

/// Hard-delete a tag permanently
pub async fn delete_tag(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service::delete_tag(&pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

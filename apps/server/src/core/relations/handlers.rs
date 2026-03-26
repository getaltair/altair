use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use super::{
    models::{CreateRelationRequest, EntityRelation, RelationQuery, UpdateRelationStatusRequest},
    service,
};
use crate::auth::middleware::AuthenticatedUser;
use crate::error::AppError;

/// Create a new entity relation between two domain objects
pub async fn create_relation(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateRelationRequest>,
) -> Result<(StatusCode, Json<EntityRelation>), AppError> {
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let relation = service::create_relation(&pool, auth.user_id, body).await?;
    Ok((StatusCode::CREATED, Json(relation)))
}

/// Query entity relations with optional filters (at least one entity id required)
pub async fn query_relations(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Query(query): Query<RelationQuery>,
) -> Result<Json<Vec<EntityRelation>>, AppError> {
    let relations = service::query_relations(&pool, auth.user_id, query).await?;
    Ok(Json(relations))
}

/// Update the status of an existing entity relation
pub async fn update_relation_status(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateRelationStatusRequest>,
) -> Result<Json<EntityRelation>, AppError> {
    body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let relation = service::update_relation_status(&pool, id, auth.user_id, body).await?;
    Ok(Json(relation))
}

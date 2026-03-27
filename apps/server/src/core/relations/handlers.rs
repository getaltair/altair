use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use super::{
    models::{
        CreateRelationRequest, EntityRelation, RelationGraphResponse, RelationQuery,
        UpdateRelationStatusRequest,
    },
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

/// Get all relations for a given entity in both directions
pub async fn get_relations_for_entity(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
) -> Result<Json<Vec<EntityRelation>>, AppError> {
    let relations =
        service::get_relations_for_entity(&pool, auth.user_id, &entity_type, entity_id).await?;
    Ok(Json(relations))
}

/// Get the one-hop relation graph for an entity with direction labels
pub async fn get_relation_graph(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
) -> Result<Json<RelationGraphResponse>, AppError> {
    let graph =
        service::get_relation_graph(&pool, auth.user_id, &entity_type, entity_id).await?;
    Ok(Json(graph))
}

/// Get all suggested relations for the authenticated user
pub async fn get_suggested_relations(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EntityRelation>>, AppError> {
    let relations = service::get_suggested_relations(&pool, auth.user_id).await?;
    Ok(Json(relations))
}

/// Accept a relation (set status to accepted, update last_confirmed_at)
pub async fn accept_relation(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<EntityRelation>, AppError> {
    let relation = service::accept_relation(&pool, id, auth.user_id).await?;
    Ok(Json(relation))
}

/// Dismiss a relation (set status to dismissed)
pub async fn dismiss_relation(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<EntityRelation>, AppError> {
    let relation = service::dismiss_relation(&pool, id, auth.user_id).await?;
    Ok(Json(relation))
}

/// Reject a relation (set status to rejected)
pub async fn reject_relation(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<EntityRelation>, AppError> {
    let relation = service::reject_relation(&pool, id, auth.user_id).await?;
    Ok(Json(relation))
}

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::models::CreateItemEventRequest;
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;

#[derive(Debug, Deserialize)]
pub struct ListEventsQuery {
    pub household_id: Uuid,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    50
}

/// GET /api/tracking/items/{id}/events
pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(item_id): Path<Uuid>,
    Query(params): Query<ListEventsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let events = service::list_item_events(
        &state.db,
        auth.user_id,
        item_id,
        params.household_id,
        params.limit,
        params.offset,
    )
    .await?;
    Ok(Json(events))
}

/// POST /api/tracking/items/{id}/events
pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateItemEventRequest>,
) -> Result<impl IntoResponse, AppError> {
    let event = service::create_item_event(&state.db, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(event)))
}

/// DELETE /api/tracking/items/{id}/events/{event_id} — always 405 (invariant D-5)
pub async fn delete_not_allowed() -> impl IntoResponse {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        Json(json!({"error": "item events are immutable"})),
    )
}

/// Build a handler value for the delete-not-allowed route.
pub fn delete_handler() -> routing::MethodRouter<AppState> {
    routing::delete(delete_not_allowed)
}

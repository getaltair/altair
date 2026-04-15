use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;
use crate::sync::models::{
    ConflictListResponse, ConflictPageParams, ResolveConflictRequest, SyncUploadRequest,
    SyncUploadResponse,
};

/// POST /api/sync/push
///
/// Accepts a batch of client mutations. Requires a valid JWT (AuthUser extractor).
/// Returns per-mutation results: accepted, deduplicated, or conflicted.
pub async fn push_handler(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<SyncUploadRequest>,
) -> Result<Json<SyncUploadResponse>, AppError> {
    let results = service::apply_mutations(&state.db, req.mutations, auth).await?;
    Ok(Json(SyncUploadResponse { results }))
}

/// GET /api/sync/conflicts
///
/// Returns a paginated list of pending sync conflicts for the authenticated user.
/// Cursor-based pagination: pass `cursor` (a conflict id) to fetch the next page.
/// Default limit is 20; maximum is 100.
pub async fn list_conflicts_handler(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<ConflictPageParams>,
) -> Result<Json<ConflictListResponse>, AppError> {
    let limit = params.limit.unwrap_or(20).min(100).max(1);
    let conflicts = service::list_conflicts(&state.db, auth.user_id, params.cursor, limit).await?;
    let next_cursor = if conflicts.len() == limit as usize {
        conflicts.last().map(|c| c.id)
    } else {
        None
    };
    Ok(Json(ConflictListResponse {
        conflicts,
        next_cursor,
    }))
}

/// POST /api/sync/conflicts/:id/resolve
///
/// Marks a conflict as accepted or rejected. Returns 200 on success, 404 if the
/// conflict does not exist, 403 if it belongs to a different user, 409 if already resolved.
pub async fn resolve_conflict_handler(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<ResolveConflictRequest>,
) -> Result<StatusCode, AppError> {
    service::resolve_conflict(&state.db, id, auth.user_id, req.resolution.as_str()).await?;
    Ok(StatusCode::OK)
}

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
    use crate::sync::models::ConflictRecord;

    let limit = params.limit.unwrap_or(20).min(100).max(1);

    let conflicts: Vec<ConflictRecord> = if let Some(cursor) = params.cursor {
        sqlx::query_as(
            "SELECT id, entity_type, entity_id, base_version, current_version, \
             incoming_payload, current_payload, created_at \
             FROM sync_conflicts \
             WHERE user_id = $1 \
               AND resolution = 'pending' \
               AND created_at < (SELECT created_at FROM sync_conflicts WHERE id = $2) \
             ORDER BY created_at DESC \
             LIMIT $3",
        )
        .bind(auth.user_id)
        .bind(cursor)
        .bind(limit)
        .fetch_all(&state.db)
        .await
        .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?
    } else {
        sqlx::query_as(
            "SELECT id, entity_type, entity_id, base_version, current_version, \
             incoming_payload, current_payload, created_at \
             FROM sync_conflicts \
             WHERE user_id = $1 \
               AND resolution = 'pending' \
             ORDER BY created_at DESC \
             LIMIT $2",
        )
        .bind(auth.user_id)
        .bind(limit)
        .fetch_all(&state.db)
        .await
        .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?
    };

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
/// conflict does not exist, 403 if it belongs to a different user.
pub async fn resolve_conflict_handler(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<ResolveConflictRequest>,
) -> Result<StatusCode, AppError> {
    let resolution_str = req.resolution.as_str();

    let result = sqlx::query(
        "UPDATE sync_conflicts \
         SET resolution = $1, resolved_at = now() \
         WHERE id = $2 AND user_id = $3",
    )
    .bind(resolution_str)
    .bind(id)
    .bind(auth.user_id)
    .execute(&state.db)
    .await
    .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    if result.rows_affected() == 1 {
        return Ok(StatusCode::OK);
    }

    // Disambiguate: does the row exist at all?
    let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM sync_conflicts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    match exists {
        None => Err(AppError::NotFound),
        Some(_) => Err(AppError::Forbidden),
    }
}

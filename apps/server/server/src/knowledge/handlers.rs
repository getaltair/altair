use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use super::models::{CreateNoteRequest, CreateSnapshotRequest, NoteListQuery, UpdateNoteRequest};
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;

pub async fn create_note(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateNoteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let note = service::create_note(&state.db, req, auth.user_id).await?;
    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn list_notes(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<NoteListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let notes = service::list_notes(&state.db, auth.user_id, query.initiative_id).await?;
    Ok(Json(notes))
}

pub async fn get_note(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(note_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let note = service::get_note(&state.db, note_id, auth.user_id).await?;
    Ok(Json(note))
}

pub async fn update_note(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(note_id): Path<Uuid>,
    Json(req): Json<UpdateNoteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let note = service::update_note(&state.db, note_id, req, auth.user_id).await?;
    Ok(Json(note))
}

pub async fn delete_note(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(note_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_note(&state.db, note_id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_snapshot(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(note_id): Path<Uuid>,
    Json(req): Json<CreateSnapshotRequest>,
) -> Result<impl IntoResponse, AppError> {
    let snapshot = service::create_snapshot(&state.db, note_id, req, auth.user_id).await?;
    Ok((StatusCode::CREATED, Json(snapshot)))
}

pub async fn list_snapshots(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(note_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let snapshots = service::list_snapshots(&state.db, note_id, auth.user_id).await?;
    Ok(Json(snapshots))
}

pub async fn list_backlinks(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(note_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let notes = service::list_backlinks(&state.db, note_id, auth.user_id).await?;
    Ok(Json(notes))
}

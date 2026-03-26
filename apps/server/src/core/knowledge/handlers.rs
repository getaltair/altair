use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::middleware::AuthenticatedUser;
use crate::core::relations::models::EntityRelation;
use crate::error::AppError;
use super::{
    models::{
        CreateNoteRequest, CreateSnapshotRequest, KnowledgeNote, KnowledgeNoteSnapshot,
        ListNotesQuery, UpdateNoteRequest,
    },
    service,
};

/// Create a new knowledge note owned by the authenticated user.
/// Returns 201 with the created note.
pub async fn create_note(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateNoteRequest>,
) -> Result<(StatusCode, Json<KnowledgeNote>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let note = service::create_note(&pool, auth.user_id, body).await?;
    Ok((StatusCode::CREATED, Json(note)))
}

/// List knowledge notes owned by or shared with the authenticated user.
pub async fn list_notes(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Query(query): Query<ListNotesQuery>,
) -> Result<Json<Vec<KnowledgeNote>>, AppError> {
    let notes = service::list_notes(&pool, auth.user_id, query).await?;
    Ok(Json(notes))
}

/// Get a single knowledge note by ID
pub async fn get_note(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<KnowledgeNote>, AppError> {
    let note = service::get_note(&pool, id, auth.user_id).await?;
    Ok(Json(note))
}

/// Update an existing knowledge note (partial update)
pub async fn update_note(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateNoteRequest>,
) -> Result<Json<KnowledgeNote>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let note = service::update_note(&pool, id, auth.user_id, body).await?;
    Ok(Json(note))
}

/// Hard-delete a knowledge note. Returns 204 on success.
pub async fn delete_note(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service::delete_note(&pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// List snapshots for a knowledge note
pub async fn list_snapshots(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(note_id): Path<Uuid>,
) -> Result<Json<Vec<KnowledgeNoteSnapshot>>, AppError> {
    let snapshots = service::list_snapshots(&pool, note_id, auth.user_id).await?;
    Ok(Json(snapshots))
}

/// Create a snapshot of a knowledge note's current content. Returns 201.
pub async fn create_snapshot(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(note_id): Path<Uuid>,
    Json(body): Json<CreateSnapshotRequest>,
) -> Result<(StatusCode, Json<KnowledgeNoteSnapshot>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let snapshot = service::create_snapshot(&pool, note_id, auth.user_id, body).await?;
    Ok((StatusCode::CREATED, Json(snapshot)))
}

/// Get all entity relations originating from a knowledge note
pub async fn get_note_relations(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(note_id): Path<Uuid>,
) -> Result<Json<Vec<EntityRelation>>, AppError> {
    let relations = service::get_note_relations(&pool, note_id, auth.user_id).await?;
    Ok(Json(relations))
}

/// Get all entity relations pointing to a knowledge note (backlinks)
pub async fn get_note_backlinks(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(note_id): Path<Uuid>,
) -> Result<Json<Vec<EntityRelation>>, AppError> {
    let backlinks = service::get_note_backlinks(&pool, note_id, auth.user_id).await?;
    Ok(Json(backlinks))
}

/// Associate a tag with a knowledge note. Returns 201 on success.
pub async fn add_note_tag(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path((note_id, tag_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    service::add_note_tag(&pool, note_id, tag_id, auth.user_id).await?;
    Ok(StatusCode::CREATED)
}

/// Remove a tag association from a knowledge note. Returns 204 on success.
pub async fn remove_note_tag(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path((note_id, tag_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    service::remove_note_tag(&pool, note_id, tag_id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Associate an attachment with a knowledge note. Returns 201 on success.
pub async fn add_note_attachment(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path((note_id, attachment_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    service::add_note_attachment(&pool, note_id, attachment_id, auth.user_id).await?;
    Ok(StatusCode::CREATED)
}

/// Remove an attachment association from a knowledge note. Returns 204 on success.
pub async fn remove_note_attachment(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path((note_id, attachment_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    service::remove_note_attachment(&pool, note_id, attachment_id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

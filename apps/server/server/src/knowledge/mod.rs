pub mod handlers;
pub mod models;
pub mod service;

use crate::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    use axum::routing::{get, post};
    Router::new()
        .route(
            "/api/knowledge/notes",
            post(handlers::create_note).get(handlers::list_notes),
        )
        .route(
            "/api/knowledge/notes/{note_id}",
            get(handlers::get_note)
                .put(handlers::update_note)
                .delete(handlers::delete_note),
        )
        .route(
            "/api/knowledge/notes/{note_id}/snapshots",
            post(handlers::create_snapshot).get(handlers::list_snapshots),
        )
        .route(
            "/api/knowledge/notes/{note_id}/backlinks",
            get(handlers::list_backlinks),
        )
}

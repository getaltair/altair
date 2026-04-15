pub mod handlers;
pub mod models;
pub mod service;

pub use models::*;

use crate::AppState;
use axum::Router;
use axum::routing::{get, post};

/// Sync engine router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/sync/push", post(handlers::push_handler))
        .route("/api/sync/conflicts", get(handlers::list_conflicts_handler))
        .route(
            "/api/sync/conflicts/{id}/resolve",
            post(handlers::resolve_conflict_handler),
        )
}

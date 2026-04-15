pub mod handlers;
pub mod models;
pub mod service;

use crate::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    use axum::routing::{delete, get};
    Router::new()
        .route(
            "/api/entity_relations",
            get(handlers::list).post(handlers::create),
        )
        .route("/api/entity_relations/{id}", delete(handlers::delete))
}

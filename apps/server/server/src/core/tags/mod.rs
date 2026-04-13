pub mod handlers;
pub mod models;
pub mod service;

use crate::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    use axum::routing::{delete, get, post};
    Router::new()
        .route("/api/tags", get(handlers::list).post(handlers::create))
        .route("/api/tags/{id}", delete(handlers::delete))
}

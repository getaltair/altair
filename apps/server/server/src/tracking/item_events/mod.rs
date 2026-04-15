pub mod handlers;
pub mod models;
pub mod service;

use crate::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    use axum::routing::get;
    Router::new()
        .route(
            "/api/tracking/items/{id}/events",
            get(handlers::list).post(handlers::create),
        )
        .route(
            "/api/tracking/items/{id}/events/{event_id}",
            handlers::delete_handler(),
        )
}

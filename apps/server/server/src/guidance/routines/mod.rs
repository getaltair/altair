pub mod handlers;
pub mod models;
pub mod service;

use crate::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    use axum::routing::{get, post};
    Router::new()
        .route(
            "/api/guidance/routines",
            get(handlers::list).post(handlers::create),
        )
        .route(
            "/api/guidance/routines/{id}",
            get(handlers::get_one)
                .patch(handlers::update)
                .delete(handlers::delete),
        )
        .route("/api/guidance/routines/{id}/spawn", post(handlers::spawn))
}

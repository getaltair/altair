pub mod handlers;
pub mod models;
pub mod service;

use crate::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    use axum::routing::get;
    Router::new()
        .route(
            "/api/tracking/shopping_lists",
            get(handlers::list).post(handlers::create),
        )
        .route(
            "/api/tracking/shopping_lists/{id}",
            get(handlers::get)
                .patch(handlers::update)
                .delete(handlers::delete),
        )
}

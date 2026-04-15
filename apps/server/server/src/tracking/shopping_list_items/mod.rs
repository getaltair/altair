pub mod handlers;
pub mod models;
pub mod service;

use crate::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    use axum::routing::get;
    Router::new()
        .route(
            "/api/tracking/shopping_lists/{id}/items",
            get(handlers::list).post(handlers::add),
        )
        .route(
            "/api/tracking/shopping_lists/{id}/items/{item_id}",
            axum::routing::patch(handlers::update).delete(handlers::remove),
        )
}

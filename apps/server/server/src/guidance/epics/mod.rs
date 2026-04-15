pub mod handlers;
pub mod models;
pub mod service;

use crate::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    use axum::routing::get;
    Router::new()
        .route(
            "/api/guidance/epics",
            get(handlers::list).post(handlers::create),
        )
        .route(
            "/api/guidance/epics/{id}",
            get(handlers::get_one)
                .patch(handlers::update)
                .delete(handlers::delete),
        )
}

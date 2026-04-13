pub mod extractor;
pub mod handlers;
pub mod models;
pub mod service;

use axum::Router;

use crate::AppState;

pub fn auth_router() -> Router<AppState> {
    use axum::routing::{get, post};
    Router::new()
        .route("/api/auth/register", post(handlers::register))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/refresh", post(handlers::refresh))
        .route("/api/auth/logout", post(handlers::logout))
        .route("/api/auth/me", get(handlers::me))
        .route("/api/auth/.well-known/jwks.json", get(handlers::jwks))
}

/// Parse a named cookie value from a raw `Cookie` header string.
///
/// Shared by `handlers` and `extractor` to avoid duplicating cookie-parsing logic.
pub(crate) fn parse_cookie_value(header: &str, name: &str) -> Option<String> {
    header.split(';').find_map(|part| {
        let part = part.trim();
        part.strip_prefix(&format!("{}=", name))
            .map(|v| v.to_string())
    })
}

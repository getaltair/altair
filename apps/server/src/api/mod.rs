mod health;

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

use crate::auth::handlers as auth_handlers;
use crate::core::households::handlers as household_handlers;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub config: crate::config::Config,
}

impl axum::extract::FromRef<AppState> for sqlx::PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl axum::extract::FromRef<AppState> for crate::config::Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

/// Create and configure the main API router
///
/// Sets up middleware (CORS, tracing, compression) and registers all routes.
///
/// # Arguments
///
/// * `state` - Application state containing the database pool and configuration
///
/// # Middleware
///
/// - **CORS**: Currently allows all origins (will be tightened for production)
/// - **Trace**: Request tracing for observability
/// - **Compression**: GZIP compression for responses
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Public routes
        .route("/health", get(health::health))
        .route("/auth/register", post(auth_handlers::register))
        .route("/auth/login", post(auth_handlers::login))
        // Protected routes (auth enforced via AuthenticatedUser extractor in each handler)
        .route("/auth/logout", post(auth_handlers::logout))
        .route(
            "/auth/me",
            get(auth_handlers::get_me).put(auth_handlers::update_me),
        )
        .route(
            "/core/households",
            post(household_handlers::create_household).get(household_handlers::list_households),
        )
        .route(
            "/core/households/:id/members",
            post(household_handlers::invite_member),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

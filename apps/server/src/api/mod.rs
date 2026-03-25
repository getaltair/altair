mod health;

pub use health::health;

use axum::{
    Router,
    routing::get,
};
use sqlx::PgPool;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    compression::CompressionLayer,
};

/// Create and configure the main API router
///
/// Sets up middleware (CORS, tracing, compression) and registers all routes.
///
/// # Arguments
///
/// * `pool` - PostgreSQL connection pool to be injected into handlers via State
///
/// # Middleware
///
/// - **CORS**: Currently allows all origins (will be tightened in auth step)
/// - **Trace**: Request tracing for observability
/// - **Compression**: GZIP compression for responses
///
/// # Example
///
/// ```no_run
/// use sqlx::PgPool;
/// use crate::api::create_router;
///
/// let pool = PgPool::connect("...").await?;
/// let router = create_router(pool);
/// ```
pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .with_state(pool)
}

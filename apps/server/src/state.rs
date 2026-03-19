//! Application state for the Axum server.
//!
//! This module defines the shared application state used across
//! all route handlers. It currently holds the PostgreSQL connection
//! pool and can be extended with additional state as needed.

use axum::extract::FromRef;
use sqlx::PgPool;

/// Shared application state.
///
/// This struct holds the database connection pool and any other
/// shared state needed by route handlers. It derives `Clone` and
/// `FromRef` to enable efficient state sharing across handlers.
///
/// # Example
///
/// ```ignore
/// use axum::Router;
/// use crate::state::AppState;
///
/// let state = AppState { pool };
/// let app = Router::new()
///     .route("/health", get(handlers::health_check))
///     .with_state(state);
/// ```
#[derive(Clone, FromRef)]
pub struct AppState {
	/// PostgreSQL connection pool
	pub pool: PgPool,
}

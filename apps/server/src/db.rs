use sqlx::Error as SqlxError;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Creates a PostgreSQL connection pool with the specified configuration.
///
/// # Arguments
///
/// * `database_url` - The PostgreSQL connection string (e.g., "postgres://user:pass@host:port/db")
///
/// # Returns
///
/// Returns a `Result` containing the configured `PgPool` or a `sqlx::Error`.
///
/// # Pool Configuration
///
/// - `max_connections`: 10
/// - `min_connections`: 2
/// - `acquire_timeout`: 30 seconds
pub async fn create_pool(database_url: &str) -> Result<PgPool, SqlxError> {
	PgPoolOptions::new()
		.max_connections(10)
		.min_connections(2)
		.acquire_timeout(Duration::from_secs(30))
		.connect(database_url)
		.await
}

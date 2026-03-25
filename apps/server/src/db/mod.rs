mod migrations;

pub use migrations::run_migrations;

use crate::config::Config;
use crate::error::{AppError, Result};
use sqlx::{PgPool, postgres::PgPoolOptions};

/// Create a PostgreSQL connection pool
///
/// Creates a connection pool with sensible defaults for production use.
///
/// # Arguments
///
/// * `config` - Application configuration containing database URL
///
/// # Pool Configuration
///
/// - **Min connections**: 5
/// - **Max connections**: 20
/// - **Acquire timeout**: 30 seconds
/// - **Test on acquire**: Enabled (verifies connections before use)
///
/// # Returns
///
/// * `Ok(PgPool)` on successful pool creation
/// * `Err(AppError)` if connection fails
///
/// # Example
///
/// ```no_run
/// use crate::config::Config;
/// use crate::db::create_pool;
///
/// let config = Config::load()?;
/// let pool = create_pool(&config)?;
/// ```
pub async fn create_pool(config: &Config) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .min_connections(5)
        .max_connections(20)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .test_before_acquire(true)
        .connect(&config.database_url)
        .await
        .map_err(|e| {
            AppError::Database(e)
        })?;

    tracing::info!("Database connection pool created successfully");
    Ok(pool)
}

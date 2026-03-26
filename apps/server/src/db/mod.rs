mod migrations;

pub use migrations::run_migrations;

use crate::config::Config;
use crate::error::{AppError, Result};
use sqlx::{PgPool, postgres::PgPoolOptions};

/// Create a PostgreSQL connection pool
///
/// Creates a connection pool with configurable settings.
///
/// # Arguments
///
/// * `config` - Application configuration containing database URL and pool settings
///
/// # Pool Configuration
///
/// - **Min connections**: From config.db_min_conn() (default: 5)
/// - **Max connections**: From config.db_max_conn() (default: 20)
/// - **Acquire timeout**: From config.db_timeout_sec() (default: 30 seconds)
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
/// let pool = create_pool(&config).await?;
/// ```
pub async fn create_pool(config: &Config) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .min_connections(config.db_min_conn())
        .max_connections(config.db_max_conn())
        .acquire_timeout(std::time::Duration::from_secs(config.db_timeout_sec()))
        .test_before_acquire(true)
        .connect(config.database_url())
        .await
        .map_err(AppError::Database)?;

    tracing::info!("Database connection pool created successfully");
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;

    /// Helper to set up test environment
    fn setup_test_env() {
        unsafe { std::env::set_var("DATABASE_URL", "test_url") }
        unsafe { std::env::set_var("PORT", "3001") }
    }

    /// Helper to clean up test environment
    fn teardown_test_env() {
        unsafe { std::env::remove_var("DATABASE_URL") }
        unsafe { std::env::remove_var("PORT") }
    }

    #[tokio::test]
    async fn test_create_pool_with_invalid_database_url() {
        // Set up test environment with an invalid database URL
        setup_test_env();
        unsafe {
            std::env::set_var(
                "DATABASE_URL",
                "postgresql://invalid:invalid@localhost:9999/nonexistent",
            )
        }

        // Create a Config with invalid database URL
        // Note: Config fields are private, but tests in same crate can access them
        let config = Config {
            database_url: "postgresql://invalid:invalid@localhost:9999/nonexistent".to_string(),
            port: 3001,
            log_level: "info".to_string(),
            environment: "test".to_string(),
            db_min_conn: 5,
            db_max_conn: 20,
            db_timeout_sec: 30,
            session_ttl_hours: 72,
            jwt_secret: "test_secret".to_string(),
            powersync_url: "http://localhost:8080".to_string(),
        };

        // Attempt to create pool with invalid URL should return Database error
        let result = create_pool(&config).await;

        assert!(
            result.is_err(),
            "create_pool should return error for invalid database URL"
        );

        match result {
            Err(AppError::Database(_)) => {
                // Expected - connection should fail with Database error
            }
            Err(other) => {
                panic!(
                    "Expected Database error, got {:?}. This should be a Database error",
                    other
                );
            }
            Ok(_) => {
                panic!("create_pool should fail with invalid database URL");
            }
        }

        // Clean up
        teardown_test_env();
    }

    #[tokio::test]
    async fn test_create_pool_with_nonexistent_host() {
        setup_test_env();
        unsafe {
            std::env::set_var(
                "DATABASE_URL",
                "postgresql://user:pass@nonexistent-host.example.com:5432/db",
            )
        }

        #[allow(dead_code)]
        let config = Config {
            database_url: "postgresql://user:pass@nonexistent-host.example.com:5432/db".to_string(),
            port: 3001,
            log_level: "info".to_string(),
            environment: "test".to_string(),
            db_min_conn: 5,
            db_max_conn: 20,
            db_timeout_sec: 30,
            session_ttl_hours: 72,
            jwt_secret: "test_secret".to_string(),
            powersync_url: "http://localhost:8080".to_string(),
        };

        let result = create_pool(&config).await;
        assert!(
            result.is_err(),
            "create_pool should fail with nonexistent host"
        );

        // Clean up
        teardown_test_env();
    }

    #[tokio::test]
    async fn test_create_pool_with_malformed_url() {
        setup_test_env();
        unsafe { std::env::set_var("DATABASE_URL", "not-a-valid-url") }

        #[allow(dead_code)]
        let config = Config {
            database_url: "not-a-valid-url".to_string(),
            port: 3001,
            log_level: "info".to_string(),
            environment: "test".to_string(),
            db_min_conn: 5,
            db_max_conn: 20,
            db_timeout_sec: 30,
            session_ttl_hours: 72,
            jwt_secret: "test_secret".to_string(),
            powersync_url: "http://localhost:8080".to_string(),
        };

        let result = create_pool(&config).await;
        assert!(
            result.is_err(),
            "create_pool should fail with malformed URL"
        );

        // Clean up
        teardown_test_env();
    }
}

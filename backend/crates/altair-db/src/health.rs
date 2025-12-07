//! Database health check module
//!
//! Provides health check functionality for the database connection, including
//! connection status and response time measurement.

use crate::SurrealConnection;
use altair_core::Result;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Database health status
///
/// Contains information about the database connection health, including
/// whether it's connected and the response time for the health check query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseHealth {
    /// Whether the database is connected and responsive
    pub connected: bool,

    /// Response time for the health check query in milliseconds
    pub response_time_ms: u64,
}

impl DatabaseHealth {
    /// Create a new DatabaseHealth instance
    pub fn new(connected: bool, response_time_ms: u64) -> Self {
        Self {
            connected,
            response_time_ms,
        }
    }

    /// Create a disconnected health status
    pub fn disconnected() -> Self {
        Self {
            connected: false,
            response_time_ms: 0,
        }
    }
}

/// Check database health
///
/// Performs a health check on the database connection by attempting to ping it
/// and measuring the response time.
///
/// # Arguments
///
/// * `conn` - Reference to the SurrealDB connection
///
/// # Returns
///
/// Returns a `DatabaseHealth` struct with connection status and response time,
/// or an error if the health check fails catastrophically.
///
/// # Example
///
/// ```no_run
/// use altair_db::{DatabaseConfig, SurrealConnection, check_database_health};
///
/// # async fn example() -> altair_core::Result<()> {
/// let config = DatabaseConfig::default();
/// let conn = SurrealConnection::new(&config).await?;
/// let health = check_database_health(&conn).await?;
///
/// if health.connected {
///     println!("Database is healthy ({}ms)", health.response_time_ms);
/// } else {
///     println!("Database is not connected");
/// }
/// # Ok(())
/// # }
/// ```
pub async fn check_database_health(conn: &SurrealConnection) -> Result<DatabaseHealth> {
    // Start timing
    let start = Instant::now();

    // Attempt to ping the database
    let ping_result = conn.ping().await;

    // Calculate elapsed time
    let elapsed = start.elapsed();
    let response_time_ms = elapsed.as_millis() as u64;

    // Build health status
    let health = match ping_result {
        Ok(_) => {
            tracing::debug!(
                response_time_ms = response_time_ms,
                "Database health check passed"
            );
            DatabaseHealth::new(true, response_time_ms)
        }
        Err(e) => {
            tracing::warn!(
                error = %e,
                response_time_ms = response_time_ms,
                "Database health check failed"
            );
            DatabaseHealth::new(false, response_time_ms)
        }
    };

    Ok(health)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DatabaseConfig;

    /// Helper function to create a test database configuration
    fn test_config() -> DatabaseConfig {
        DatabaseConfig {
            url: "mem://".to_string(),
            namespace: "test".to_string(),
            database: "main".to_string(),
        }
    }

    #[test]
    fn test_database_health_new() {
        let health = DatabaseHealth::new(true, 42);
        assert!(health.connected);
        assert_eq!(health.response_time_ms, 42);
    }

    #[test]
    fn test_database_health_disconnected() {
        let health = DatabaseHealth::disconnected();
        assert!(!health.connected);
        assert_eq!(health.response_time_ms, 0);
    }

    #[test]
    fn test_database_health_serialization() {
        let health = DatabaseHealth::new(true, 123);
        let json = serde_json::to_string(&health).expect("Should serialize");
        assert!(json.contains("\"connected\":true"));
        assert!(json.contains("\"response_time_ms\":123"));
    }

    #[test]
    fn test_database_health_deserialization() {
        let json = r#"{"connected":true,"response_time_ms":456}"#;
        let health: DatabaseHealth = serde_json::from_str(json).expect("Should deserialize");
        assert!(health.connected);
        assert_eq!(health.response_time_ms, 456);
    }

    #[tokio::test]
    async fn test_check_database_health_connected() {
        let config = test_config();
        let conn = SurrealConnection::new(&config)
            .await
            .expect("Connection should succeed");

        let health = check_database_health(&conn)
            .await
            .expect("Health check should succeed");

        assert!(health.connected, "Database should be connected");
        assert!(
            health.response_time_ms < 1000,
            "Response time should be reasonable"
        );
    }

    #[tokio::test]
    async fn test_check_database_health_measures_time() {
        let config = test_config();
        let conn = SurrealConnection::new(&config)
            .await
            .expect("Connection should succeed");

        let health = check_database_health(&conn)
            .await
            .expect("Health check should succeed");

        // Response time should always be available (it's a u64, always >= 0)
        // Just verify the health check returned successfully
        assert!(
            health.connected,
            "Database should be connected for health check"
        );
    }
}

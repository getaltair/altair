//! SurrealDB connection implementation
//!
//! This module provides the actual SurrealDB client implementation for embedded database
//! connections using SurrealKV.

use crate::DatabaseConfig;
use altair_core::{Error, Result};
use surrealdb::{Surreal, engine::local::Db};

/// SurrealDB connection wrapper
///
/// This struct wraps the SurrealDB client and provides type-safe database operations.
/// It uses the SurrealKV engine for embedded database access.
pub struct SurrealConnection {
    /// The underlying SurrealDB client
    db: Surreal<Db>,

    /// Configuration used to establish the connection
    config: DatabaseConfig,
}

impl SurrealConnection {
    /// Create a new SurrealDB connection
    ///
    /// # Arguments
    ///
    /// * `config` - Database configuration containing URL, namespace, and database name
    ///
    /// # Returns
    ///
    /// Returns a connected SurrealConnection instance or an error if connection fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use altair_db::{DatabaseConfig, SurrealConnection};
    ///
    /// # async fn example() -> altair_core::Result<()> {
    /// let config = DatabaseConfig::default();
    /// let conn = SurrealConnection::new(&config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        // Connect to SurrealDB - use appropriate engine based on URL
        // mem:// for in-memory testing, surrealkv:// for production
        let db = if config.url.starts_with("mem://") {
            Surreal::new::<surrealdb::engine::local::Mem>(())
                .await
                .map_err(|e| Error::database(format!("Failed to connect to SurrealDB: {}", e)))?
        } else {
            Surreal::new::<surrealdb::engine::local::SurrealKv>(&config.url)
                .await
                .map_err(|e| Error::database(format!("Failed to connect to SurrealDB: {}", e)))?
        };

        // Select namespace and database
        db.use_ns(&config.namespace)
            .use_db(&config.database)
            .await
            .map_err(|e| {
                Error::database(format!(
                    "Failed to select namespace/database {}/{}: {}",
                    config.namespace, config.database, e
                ))
            })?;

        tracing::info!(
            namespace = %config.namespace,
            database = %config.database,
            url = %config.url,
            "Connected to SurrealDB"
        );

        Ok(Self {
            db,
            config: config.clone(),
        })
    }

    /// Ping the database to verify connectivity
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if the database is accessible, or an error if not.
    pub async fn ping(&self) -> Result<()> {
        // Execute a simple query to verify connectivity
        // Use RETURN instead of SELECT to avoid needing a FROM clause
        let response = self
            .db
            .query("RETURN 1")
            .await
            .map_err(|e| Error::database(format!("Database ping failed: {}", e)))?;

        // Check that we can actually retrieve the result (validates connection)
        response
            .check()
            .map_err(|e| Error::database(format!("Database ping failed: {}", e)))?;

        Ok(())
    }

    /// Get a reference to the underlying Surreal client
    ///
    /// This is useful for advanced operations that need direct access to the client.
    pub fn client(&self) -> &Surreal<Db> {
        &self.db
    }

    /// Get the configuration
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// Execute a SQL query and return the result as JSON
    pub async fn query(&self, sql: &str) -> Result<serde_json::Value> {
        let mut result = self
            .db
            .query(sql)
            .await
            .map_err(|e| Error::database(format!("Query execution failed: {}", e)))?;

        // Take the first result from the response as SurrealDB Value, then convert to JSON
        let value: surrealdb::Value = result
            .take(0)
            .map_err(|e| Error::database(format!("Failed to extract query result: {}", e)))?;

        // Convert SurrealDB Value to serde_json::Value
        let json = serde_json::to_value(&value)
            .map_err(|e| Error::database(format!("Failed to serialize query result: {}", e)))?;

        Ok(json)
    }

    /// Health check - returns true if database is accessible
    pub async fn health_check(&self) -> Result<bool> {
        match self.ping().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a test database configuration with unique path
    fn test_config() -> DatabaseConfig {
        // Use in-memory database for tests to avoid file system conflicts
        DatabaseConfig {
            url: "mem://".to_string(),
            namespace: "test".to_string(),
            database: "main".to_string(),
            username: None,
            password: None,
        }
    }

    #[tokio::test]
    async fn test_surreal_connection_new() {
        let config = test_config();
        let result = SurrealConnection::new(&config).await;
        assert!(result.is_ok(), "Connection should succeed");
    }

    #[tokio::test]
    async fn test_surreal_connection_ping() {
        let config = test_config();
        let conn = SurrealConnection::new(&config)
            .await
            .expect("Connection should succeed");

        let result = conn.ping().await;
        assert!(result.is_ok(), "Ping should succeed: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_surreal_connection_config() {
        let config = test_config();
        let conn = SurrealConnection::new(&config)
            .await
            .expect("Connection should succeed");

        assert_eq!(conn.config().namespace, "test");
        assert_eq!(conn.config().database, "main");
    }

    #[tokio::test]
    async fn test_health_check() {
        let config = test_config();
        let conn = SurrealConnection::new(&config)
            .await
            .expect("Connection should succeed");

        let healthy = conn.health_check().await.expect("Health check should run");
        assert!(healthy, "Database should be healthy");
    }

    #[tokio::test]
    async fn test_query() {
        let config = test_config();
        let conn = SurrealConnection::new(&config)
            .await
            .expect("Connection should succeed");

        // Use RETURN instead of SELECT (which requires FROM clause)
        let result = conn.query("RETURN 1").await;
        assert!(
            result.is_ok(),
            "Query should execute successfully: {:?}",
            result.err()
        );
    }
}

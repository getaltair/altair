//! SurrealDB client wrapper
//!
//! Provides a convenient wrapper around the SurrealDB client with connection
//! management, error handling, and common operations.

use altair_core::{Error, Result};
use serde::Serialize;
use serde::de::DeserializeOwned;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use tracing::{debug, info};

/// SurrealDB client configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Database URL (e.g., "ws://localhost:8000", "surrealkv://data/db")
    pub url: String,

    /// Namespace
    pub namespace: String,

    /// Database name
    pub database: String,

    /// Root username (optional, for remote databases)
    pub username: Option<String>,

    /// Root password (optional, for remote databases)
    pub password: Option<String>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "surrealkv://~/.local/share/altair/db".to_string(),
            namespace: "altair".to_string(),
            database: "main".to_string(),
            username: None,
            password: None,
        }
    }
}

/// SurrealDB client wrapper
///
/// Provides a convenient interface for database operations with proper
/// error handling and connection management.
pub struct DatabaseClient {
    /// SurrealDB client
    db: Surreal<Any>,
    /// Client configuration
    config: DatabaseConfig,
}

impl DatabaseClient {
    /// Create a new database client and connect
    ///
    /// # Arguments
    ///
    /// * `config` - Database configuration
    ///
    /// # Errors
    ///
    /// Returns an error if connection fails or authentication fails.
    pub async fn connect(config: DatabaseConfig) -> Result<Self> {
        info!(
            "Connecting to SurrealDB at {} (ns: {}, db: {})",
            config.url, config.namespace, config.database
        );

        // Create client using the Any engine (supports all connection types)
        let db = surrealdb::engine::any::connect(&config.url)
            .await
            .map_err(|e| Error::database(format!("Failed to create SurrealDB client: {}", e)))?;

        // Authenticate if credentials provided
        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            db.signin(Root { username, password })
                .await
                .map_err(|e| Error::database(format!("Authentication failed: {}", e)))?;
        }

        // Use namespace and database
        db.use_ns(&config.namespace)
            .use_db(&config.database)
            .await
            .map_err(|e| {
                Error::database(format!(
                    "Failed to use namespace/database {}/{}: {}",
                    config.namespace, config.database, e
                ))
            })?;

        info!("Successfully connected to SurrealDB");

        Ok(Self { db, config })
    }

    /// Execute a raw SurrealQL query
    ///
    /// # Arguments
    ///
    /// * `sql` - SurrealQL query string
    ///
    /// # Errors
    ///
    /// Returns an error if query execution fails.
    pub async fn execute(&self, sql: &str) -> Result<()> {
        debug!("Executing query: {}", sql);

        self.db
            .query(sql)
            .await
            .map_err(|e| Error::database(format!("Query execution failed: {}", e)))?;

        Ok(())
    }

    /// Select all records from a table
    ///
    /// # Arguments
    ///
    /// * `table` - Table name
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type to deserialize records into
    ///
    /// # Errors
    ///
    /// Returns an error if query fails or deserialization fails.
    pub async fn select<T>(&self, table: &str) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        debug!("Selecting all records from table: {}", table);

        let records: Vec<T> = self
            .db
            .select(table)
            .await
            .map_err(|e| Error::database(format!("Failed to select from {}: {}", table, e)))?;

        debug!("Retrieved {} records from {}", records.len(), table);
        Ok(records)
    }

    /// Select a specific record by ID
    ///
    /// # Arguments
    ///
    /// * `table` - Table name
    /// * `id` - Record ID
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type to deserialize record into
    ///
    /// # Errors
    ///
    /// Returns an error if query fails or record doesn't exist.
    pub async fn select_by_id<T>(&self, table: &str, id: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        debug!("Selecting record {}:{}", table, id);

        let record: Option<T> =
            self.db.select((table, id)).await.map_err(|e| {
                Error::database(format!("Failed to select {}:{}: {}", table, id, e))
            })?;

        Ok(record)
    }

    /// Create a new record
    ///
    /// # Arguments
    ///
    /// * `table` - Table name
    /// * `data` - Record data
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type of data to insert
    /// * `R` - Type of returned record
    ///
    /// # Errors
    ///
    /// Returns an error if insert fails or validation fails.
    pub async fn create<T, R>(&self, table: &str, data: T) -> Result<Option<R>>
    where
        T: Serialize + 'static,
        R: DeserializeOwned + 'static,
    {
        debug!("Creating record in table: {}", table);

        let record: Option<R> =
            self.db.create(table).content(data).await.map_err(|e| {
                Error::database(format!("Failed to create record in {}: {}", table, e))
            })?;

        Ok(record)
    }

    /// Update a record by ID
    ///
    /// # Arguments
    ///
    /// * `table` - Table name
    /// * `id` - Record ID
    /// * `data` - Updated record data
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type of data to update
    /// * `R` - Type of returned record
    ///
    /// # Errors
    ///
    /// Returns an error if update fails.
    pub async fn update<T, R>(&self, table: &str, id: &str, data: T) -> Result<Option<R>>
    where
        T: Serialize + 'static,
        R: DeserializeOwned + 'static,
    {
        debug!("Updating record {}:{}", table, id);

        let record: Option<R> = self
            .db
            .update((table, id))
            .content(data)
            .await
            .map_err(|e| Error::database(format!("Failed to update {}:{}: {}", table, id, e)))?;

        Ok(record)
    }

    /// Delete a record by ID
    ///
    /// # Arguments
    ///
    /// * `table` - Table name
    /// * `id` - Record ID
    ///
    /// # Errors
    ///
    /// Returns an error if delete fails.
    pub async fn delete(&self, table: &str, id: &str) -> Result<()> {
        debug!("Deleting record {}:{}", table, id);

        let _: Option<serde_json::Value> =
            self.db.delete((table, id)).await.map_err(|e| {
                Error::database(format!("Failed to delete {}:{}: {}", table, id, e))
            })?;

        Ok(())
    }

    /// Get the underlying SurrealDB client
    ///
    /// Useful for advanced operations not covered by the wrapper.
    pub fn inner(&self) -> &Surreal<Any> {
        &self.db
    }

    /// Get the database configuration
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// Health check - verify database is accessible
    ///
    /// # Errors
    ///
    /// Returns an error if database is not accessible.
    pub async fn health_check(&self) -> Result<bool> {
        debug!("Performing health check");

        // Try a simple query
        let result = self.db.health().await;

        match result {
            Ok(_) => {
                debug!("Health check passed");
                Ok(true)
            }
            Err(e) => {
                debug!("Health check failed: {}", e);
                Err(Error::database(format!("Health check failed: {}", e)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_default_config() {
        let config = DatabaseConfig::default();
        assert_eq!(config.namespace, "altair");
        assert_eq!(config.database, "main");
        assert!(config.url.contains("altair"));
    }

    #[tokio::test]
    async fn test_connect_in_memory() {
        let config = DatabaseConfig {
            url: "mem://".to_string(),
            namespace: "test".to_string(),
            database: "test".to_string(),
            username: None,
            password: None,
        };

        let client = DatabaseClient::connect(config).await;
        assert!(client.is_ok(), "Failed to connect to in-memory database");
    }

    #[tokio::test]
    async fn test_health_check() {
        let config = DatabaseConfig {
            url: "mem://".to_string(),
            namespace: "test".to_string(),
            database: "test".to_string(),
            username: None,
            password: None,
        };

        let client = DatabaseClient::connect(config).await.unwrap();
        let health = client.health_check().await;
        assert!(health.is_ok(), "Health check should pass");
    }

    #[tokio::test]
    async fn test_execute_query() {
        let config = DatabaseConfig {
            url: "mem://".to_string(),
            namespace: "test".to_string(),
            database: "test".to_string(),
            username: None,
            password: None,
        };

        let client = DatabaseClient::connect(config).await.unwrap();
        let result = client.execute("DEFINE TABLE test;").await;
        assert!(result.is_ok(), "Failed to execute query");
    }
}

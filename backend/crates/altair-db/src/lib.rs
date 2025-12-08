//! Altair Database - SurrealDB integration layer
//!
//! This crate provides database access and operations for Altair applications.
//! It handles:
//! - SurrealDB connection management (embedded and cloud)
//! - Schema migrations
//! - CRUD operations for domain entities
//! - Change feed integration for sync
//! - Query builders and helpers

pub mod client;
pub mod migration;
pub mod schema;

// Re-export commonly used types
pub use client::{DatabaseClient, DatabaseConfig};
pub use migration::{Migration, MigrationRecord, MigrationRunner};

// Re-export schema types for convenience
pub use schema::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert_eq!(config.namespace, "altair");
        assert_eq!(config.database, "main");
        assert!(config.url.contains("altair"));
    }

    #[tokio::test]
    async fn test_client_creation() {
        let config = DatabaseConfig {
            url: "mem://".to_string(),
            namespace: "test".to_string(),
            database: "test".to_string(),
            username: None,
            password: None,
        };

        let client = DatabaseClient::connect(config).await;
        assert!(client.is_ok());
    }
}

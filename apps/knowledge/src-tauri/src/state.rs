//! Application state management for Knowledge app
//!
//! This module defines the AppState that holds shared resources across
//! the Tauri application lifecycle. The state is initialized during app
//! setup and managed by Tauri's state management system.

use altair_core::{AppConfig, Error, LogGuard, Result, init_logging};
use altair_db::{DatabaseConfig, SurrealConnection};

/// Application state shared across all Tauri commands
///
/// This struct holds all shared resources needed by the application:
/// - Database connection for data persistence
/// - Application configuration
/// - Log guard to keep logging active
///
/// # Lifecycle
///
/// AppState is created during application setup and dropped when the
/// application exits. The Drop implementation ensures graceful shutdown.
pub struct AppState {
    /// Database connection (SurrealDB embedded)
    pub db: SurrealConnection,

    /// Application configuration (will be used in future phases)
    #[allow(dead_code)]
    pub config: AppConfig,

    /// Log guard - MUST be kept alive for logging to work
    /// This field prevents the background logging thread from being dropped
    _log_guard: LogGuard,
}

impl AppState {
    /// Initialize application state
    ///
    /// This method performs startup in the correct order:
    /// 1. Initialize logging (must be first for tracing to work)
    /// 2. Connect to database
    /// 3. Log successful initialization
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Logging initialization fails
    /// - Database connection fails
    /// - Configuration is invalid
    pub async fn new(config: AppConfig) -> Result<Self> {
        // Step 1: Initialize logging FIRST so we can trace everything else
        let log_guard = init_logging(&config)?;

        tracing::info!(
            "Initializing Knowledge app state with log_level={}, db_path={:?}",
            config.log_level,
            config.database_path
        );

        // Step 2: Connect to database
        let db_path = config.database_path.display().to_string();
        let url = if db_path.starts_with("mem://") {
            db_path // Use mem:// directly for in-memory testing
        } else {
            format!("surrealkv://{}", db_path)
        };
        let db_config = DatabaseConfig {
            url,
            namespace: "altair".to_string(),
            database: "knowledge".to_string(),
            username: None,
            password: None,
        };

        let db = SurrealConnection::new(&db_config)
            .await
            .map_err(|e| Error::Database(format!("Failed to connect to database: {}", e)))?;

        tracing::info!("Database connection established successfully");

        // Step 3: Return initialized state
        Ok(Self {
            db,
            config,
            _log_guard: log_guard,
        })
    }
}

impl Drop for AppState {
    /// Log graceful shutdown when state is dropped
    fn drop(&mut self) {
        tracing::info!("Knowledge app state shutting down gracefully");
    }
}

#[cfg(test)]
impl AppState {
    /// Create AppState for testing without logging initialization
    ///
    /// This bypasses logging setup to avoid conflicts when running multiple tests.
    /// Only use in tests!
    pub async fn new_for_test(config: AppConfig) -> Result<Self> {
        // Connect to database only (skip logging for tests)
        let db_path = config.database_path.display().to_string();
        let url = if db_path.starts_with("mem://") {
            db_path // Use mem:// directly for in-memory testing
        } else {
            format!("surrealkv://{}", db_path)
        };
        let db_config = DatabaseConfig {
            url,
            namespace: "altair".to_string(),
            database: "knowledge-test".to_string(),
            username: None,
            password: None,
        };

        let db = SurrealConnection::new(&db_config)
            .await
            .map_err(|e| Error::Database(format!("Failed to connect to database: {}", e)))?;

        // Create a dummy log guard (tests don't need actual logging)
        let log_guard = LogGuard::dummy();

        Ok(Self {
            db,
            config,
            _log_guard: log_guard,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_app_state_initialization() {
        // Create test config with in-memory database
        let config = AppConfig {
            database_path: PathBuf::from("mem://"),
            ..Default::default()
        };

        // Initialize state for testing
        let result = AppState::new_for_test(config).await;

        // Should succeed
        assert!(result.is_ok(), "AppState initialization should succeed");

        // State should be dropped cleanly when going out of scope
        drop(result);
    }
}

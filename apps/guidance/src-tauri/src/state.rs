//! Application state management for Guidance app
//!
//! This module defines the AppState that holds shared resources across
//! the Tauri application lifecycle. The state is initialized during app
//! setup and managed by Tauri's state management system.

use altair_core::{AppConfig, Error, LogGuard, Result, init_logging};
use altair_db::{DatabaseConfig, SurrealConnection};
use altair_storage::{StorageConfig, StorageService};

/// Application state shared across all Tauri commands
///
/// This struct holds all shared resources needed by the application:
/// - Database connection for data persistence
/// - Storage service for S3-compatible file storage
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

    /// Storage service for S3-compatible object storage (optional until configured)
    ///
    /// This is `None` if storage credentials haven't been configured yet.
    /// Users can configure storage via settings or environment variables.
    pub storage: Option<StorageService>,

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
    /// 3. Initialize storage service (if credentials available)
    /// 4. Log successful initialization
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
            "Initializing Guidance app state with log_level={}, db_path={:?}",
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
            database: "guidance".to_string(),
            username: None,
            password: None,
        };

        let db = SurrealConnection::new(&db_config)
            .await
            .map_err(|e| Error::Database(format!("Failed to connect to database: {}", e)))?;

        tracing::info!("Database connection established successfully");

        // Step 3: Initialize storage service (optional - graceful degradation if not configured)
        let storage = Self::init_storage().await;
        if storage.is_some() {
            tracing::info!("Storage service initialized successfully");
        } else {
            tracing::info!(
                "Storage service not configured - attachments disabled until configured"
            );
        }

        // Step 4: Return initialized state
        Ok(Self {
            db,
            storage,
            config,
            _log_guard: log_guard,
        })
    }

    /// Initialize storage service from environment or keychain
    ///
    /// Attempts to load storage configuration from:
    /// 1. Environment variables (STORAGE_ENDPOINT, STORAGE_BUCKET, etc.)
    /// 2. OS keychain (for credentials)
    ///
    /// Returns `None` if storage is not configured, allowing the app to run
    /// without attachment support.
    async fn init_storage() -> Option<StorageService> {
        // Try environment variables first (for development/CI)
        let storage_config = match StorageConfig::from_env() {
            Ok(config) => {
                tracing::debug!("Loaded storage config from environment");
                config
            }
            Err(_) => {
                // Fall back to keychain
                match StorageConfig::from_keychain() {
                    Ok(config) => {
                        tracing::debug!("Loaded storage config from keychain");
                        config
                    }
                    Err(e) => {
                        tracing::debug!("No storage credentials found: {}", e);
                        return None;
                    }
                }
            }
        };

        // Create storage service
        match StorageService::new(&storage_config).await {
            Ok(service) => Some(service),
            Err(e) => {
                tracing::warn!("Failed to initialize storage service: {}", e);
                None
            }
        }
    }
}

impl Drop for AppState {
    /// Log graceful shutdown when state is dropped
    fn drop(&mut self) {
        tracing::info!("Guidance app state shutting down gracefully");
    }
}

impl AppState {
    /// Create AppState for testing without logging initialization
    ///
    /// This bypasses logging setup to avoid conflicts when running multiple tests.
    /// Only use in tests! Available in both unit tests and integration tests.
    #[doc(hidden)]
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
            database: "guidance-test".to_string(),
            username: None,
            password: None,
        };

        let db = SurrealConnection::new(&db_config)
            .await
            .map_err(|e| Error::Database(format!("Failed to connect to database: {}", e)))?;

        // Create a dummy log guard (tests don't need actual logging)
        let log_guard = LogGuard::dummy();

        // Storage is None for tests (no S3 in test environment)
        Ok(Self {
            db,
            storage: None,
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

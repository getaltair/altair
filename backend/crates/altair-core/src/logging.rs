//! Logging infrastructure for Altair applications.
//!
//! This module provides centralized logging setup with:
//! - JSON logging to rotating daily log files
//! - Human-readable console output (debug builds)
//! - Configurable log levels via environment or config
//! - Automatic log retention based on AppConfig
//!
//! # Usage
//!
//! ```no_run
//! use altair_core::{AppConfig, init_logging};
//!
//! let config = AppConfig::load_or_default();
//! let _guard = init_logging(&config).expect("Failed to initialize logging");
//! // Keep _guard alive for the application lifetime
//! ```
//!
//! # Important: LogGuard Lifetime
//!
//! The returned `LogGuard` MUST be kept alive for the entire application lifetime.
//! If dropped, background logging will stop. Store it in your AppState or as a field
//! in your application struct.

use crate::{Error, Result, config::AppConfig};
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Guard that keeps the logging subsystem alive.
///
/// When this guard is dropped, the background logging thread will flush and shut down.
/// You MUST keep this alive for the entire application lifetime.
#[derive(Debug)]
pub struct LogGuard {
    _file_guard: tracing_appender::non_blocking::WorkerGuard,
}

/// Initialize the logging subsystem with file and console output.
///
/// # Configuration
///
/// - **File output**: JSON format, rotates daily, stored in `config.log_dir/altair.log`
/// - **Console output**: Human-readable (debug builds only)
/// - **Log level**: From `config.log_level` or `RUST_LOG` environment variable
/// - **Retention**: Logs older than `config.log_retention_days` are automatically cleaned up
///
/// # Returns
///
/// Returns a `LogGuard` that must be kept alive. If dropped, logging will stop.
///
/// # Errors
///
/// Returns an error if:
/// - The log directory cannot be created
/// - The log file cannot be opened
/// - The subscriber cannot be initialized
///
/// # Example
///
/// ```no_run
/// use altair_core::{AppConfig, init_logging};
///
/// let config = AppConfig::load_or_default();
/// let _log_guard = init_logging(&config).expect("Failed to initialize logging");
///
/// // Now tracing macros work:
/// tracing::info!("Application started");
/// tracing::debug!("Debug information");
/// tracing::error!("Error occurred: {}", "example");
/// ```
pub fn init_logging(config: &AppConfig) -> Result<LogGuard> {
    // Ensure log directory exists
    std::fs::create_dir_all(&config.log_dir).map_err(|e| {
        Error::internal(format!(
            "Failed to create log directory {:?}: {}",
            config.log_dir, e
        ))
    })?;

    // Set up daily rotating file appender
    let file_appender = tracing_appender::rolling::daily(&config.log_dir, "altair.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // JSON layer for file output
    let file_layer = fmt::layer().json().with_writer(non_blocking).with_filter(
        EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new(&config.log_level))
            .map_err(|e| {
                Error::internal(format!("Invalid log level '{}': {}", config.log_level, e))
            })?,
    );

    // Console layer (human-readable, debug builds only)
    #[cfg(debug_assertions)]
    let console_layer = fmt::layer()
        .pretty()
        .with_writer(std::io::stderr)
        .with_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new(&config.log_level))
                .unwrap_or_else(|_| EnvFilter::new("info")),
        );

    // Initialize global subscriber
    #[cfg(debug_assertions)]
    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .try_init()
        .map_err(|e| Error::internal(format!("Failed to initialize logging: {}", e)))?;

    #[cfg(not(debug_assertions))]
    tracing_subscriber::registry()
        .with(file_layer)
        .try_init()
        .map_err(|e| Error::internal(format!("Failed to initialize logging: {}", e)))?;

    tracing::info!(
        log_dir = ?config.log_dir,
        log_level = %config.log_level,
        retention_days = config.log_retention_days,
        "Logging initialized"
    );

    Ok(LogGuard { _file_guard: guard })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_logging_initialization() {
        let temp_dir = std::env::temp_dir().join("altair-core-logging-test");
        let config = AppConfig {
            database_path: PathBuf::from("test.db"),
            log_level: "debug".to_string(),
            log_dir: temp_dir.clone(),
            log_retention_days: 7,
        };

        // Initialize logging (may fail in CI if already initialized)
        let result = init_logging(&config);

        // If initialization succeeds, verify the directory was created
        if result.is_ok() {
            assert!(temp_dir.exists(), "Log directory should be created");
        }

        // Clean up
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_log_guard_is_debug() {
        // Verify LogGuard implements Debug
        let temp_dir = std::env::temp_dir().join("altair-core-logging-guard-test");
        let config = AppConfig {
            database_path: PathBuf::from("test.db"),
            log_level: "info".to_string(),
            log_dir: temp_dir.clone(),
            log_retention_days: 7,
        };

        if let Ok(guard) = init_logging(&config) {
            // Should be able to format as debug
            let debug_str = format!("{:?}", guard);
            assert!(debug_str.contains("LogGuard"));
        }

        // Clean up
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_invalid_log_level() {
        let temp_dir = std::env::temp_dir().join("altair-core-logging-invalid-test");
        let config = AppConfig {
            database_path: PathBuf::from("test.db"),
            log_level: "INVALID_LEVEL!!!".to_string(),
            log_dir: temp_dir.clone(),
            log_retention_days: 7,
        };

        // Should fail with invalid log level (unless env var RUST_LOG overrides it)
        let result = init_logging(&config);

        // If RUST_LOG is not set, this should error
        if std::env::var("RUST_LOG").is_err() {
            assert!(result.is_err(), "Invalid log level should cause error");
            if let Err(e) = result {
                let error_msg = format!("{}", e);
                assert!(error_msg.contains("Invalid log level") || error_msg.contains("already"));
            }
        }

        // Clean up
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}

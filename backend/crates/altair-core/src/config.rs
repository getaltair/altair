//! Application configuration types
//!
//! This module provides the `AppConfig` type for managing application settings.
//! Configuration can be loaded from TOML files or use platform-aware defaults.

use serde::Deserialize;
use std::path::PathBuf;

/// Application configuration
///
/// This struct holds all application-level configuration including database paths,
/// logging settings, and other runtime parameters. It can be loaded from a TOML
/// file or initialized with platform-appropriate defaults.
///
/// # Examples
///
/// ```
/// use altair_core::AppConfig;
///
/// // Use default configuration
/// let config = AppConfig::default();
/// assert_eq!(config.log_level(), "INFO");
/// assert_eq!(config.log_retention_days(), 7);
///
/// // Load from file (falls back to defaults if file doesn't exist)
/// let config = AppConfig::load_or_default();
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Path to the SurrealDB database file
    pub database_path: PathBuf,
    /// Log level (TRACE, DEBUG, INFO, WARN, ERROR)
    pub log_level: String,
    /// Directory for log files
    pub log_dir: PathBuf,
    /// Number of days to retain log files
    pub log_retention_days: u8,
}

impl AppConfig {
    /// Load configuration from a TOML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML configuration file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed
    pub fn load_from_file(path: PathBuf) -> crate::Result<Self> {
        let contents = std::fs::read_to_string(&path).map_err(|e| {
            crate::Error::internal(format!("Failed to read config file {:?}: {}", path, e))
        })?;

        let config: Self = toml::from_str(&contents).map_err(|e| {
            crate::Error::internal(format!("Failed to parse config file {:?}: {}", path, e))
        })?;

        Ok(config)
    }

    /// Load configuration from file, or use defaults if file doesn't exist
    ///
    /// This method attempts to load from `$CONFIG_DIR/altair/config.toml` where
    /// `$CONFIG_DIR` is platform-specific (e.g., `~/.config` on Linux). If the
    /// file doesn't exist, it returns the default configuration.
    pub fn load_or_default() -> Self {
        let config_path = Self::default_config_path();

        if config_path.exists() {
            Self::load_from_file(config_path).unwrap_or_else(|e| {
                tracing::warn!("Failed to load config, using defaults: {}", e);
                Self::default()
            })
        } else {
            Self::default()
        }
    }

    /// Get the default configuration file path
    ///
    /// Returns platform-specific path:
    /// - Linux: `~/.config/altair/config.toml`
    /// - macOS: `~/Library/Application Support/altair/config.toml`
    /// - Windows: `%APPDATA%\altair\config.toml`
    pub fn default_config_path() -> PathBuf {
        directories::ProjectDirs::from("", "", "altair")
            .map(|dirs| dirs.config_dir().join("config.toml"))
            .unwrap_or_else(|| PathBuf::from("config.toml"))
    }

    /// Get the log level
    pub fn log_level(&self) -> &str {
        &self.log_level
    }

    /// Get the log retention period in days
    pub fn log_retention_days(&self) -> u8 {
        self.log_retention_days
    }

    /// Get the database path
    pub fn database_path(&self) -> &PathBuf {
        &self.database_path
    }

    /// Get the log directory
    pub fn log_dir(&self) -> &PathBuf {
        &self.log_dir
    }
}

impl Default for AppConfig {
    /// Create default configuration with platform-aware paths
    ///
    /// Uses `directories` crate to determine appropriate paths for the current platform:
    /// - Database: `$DATA_DIR/altair/db/`
    /// - Logs: `$DATA_DIR/altair/logs/`
    ///
    /// Where `$DATA_DIR` is:
    /// - Linux: `~/.local/share`
    /// - macOS: `~/Library/Application Support`
    /// - Windows: `%APPDATA%`
    fn default() -> Self {
        let project_dirs = directories::ProjectDirs::from("", "", "altair");

        let (database_path, log_dir) = if let Some(dirs) = project_dirs {
            let data_dir = dirs.data_dir();
            (data_dir.join("db"), data_dir.join("logs"))
        } else {
            // Fallback if platform directories can't be determined
            (PathBuf::from("data/db"), PathBuf::from("data/logs"))
        };

        Self {
            database_path,
            log_level: "INFO".to_string(),
            log_dir,
            log_retention_days: 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_default() {
        let config = AppConfig::default();
        assert_eq!(config.log_level(), "INFO");
        assert_eq!(config.log_retention_days(), 7);
        assert!(config.database_path().to_string_lossy().contains("altair"));
        assert!(config.log_dir().to_string_lossy().contains("altair"));
    }

    #[test]
    fn test_load_from_file() {
        // Create temporary config file
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_config.toml");

        let toml_content = r#"
            database_path = "/tmp/test_db"
            log_level = "DEBUG"
            log_dir = "/tmp/test_logs"
            log_retention_days = 14
        "#;

        std::fs::write(&config_path, toml_content).unwrap();

        // Load config
        let config = AppConfig::load_from_file(config_path.clone()).unwrap();
        assert_eq!(config.log_level(), "DEBUG");
        assert_eq!(config.log_retention_days(), 14);
        assert_eq!(config.database_path(), &PathBuf::from("/tmp/test_db"));
        assert_eq!(config.log_dir(), &PathBuf::from("/tmp/test_logs"));

        // Cleanup
        std::fs::remove_file(config_path).ok();
    }

    #[test]
    fn test_load_from_file_invalid_path() {
        let result = AppConfig::load_from_file(PathBuf::from("/nonexistent/config.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_file_invalid_toml() {
        // Create temporary file with invalid TOML
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("invalid_config.toml");

        let mut file = std::fs::File::create(&config_path).unwrap();
        file.write_all(b"invalid toml content [[[").unwrap();
        drop(file);

        let result = AppConfig::load_from_file(config_path.clone());
        assert!(result.is_err());

        // Cleanup
        std::fs::remove_file(config_path).ok();
    }

    #[test]
    fn test_load_or_default_nonexistent() {
        // This should return defaults without error
        let config = AppConfig::load_or_default();
        assert_eq!(config.log_level(), "INFO");
    }

    #[test]
    fn test_getters() {
        let config = AppConfig {
            database_path: PathBuf::from("/test/db"),
            log_level: "WARN".to_string(),
            log_dir: PathBuf::from("/test/logs"),
            log_retention_days: 30,
        };

        assert_eq!(config.log_level(), "WARN");
        assert_eq!(config.log_retention_days(), 30);
        assert_eq!(config.database_path(), &PathBuf::from("/test/db"));
        assert_eq!(config.log_dir(), &PathBuf::from("/test/logs"));
    }

    #[test]
    fn test_default_config_path() {
        let path = AppConfig::default_config_path();
        // Should contain 'altair' and 'config.toml'
        let path_str = path.to_string_lossy();
        assert!(path_str.contains("altair"));
        assert!(path_str.contains("config.toml"));
    }
}

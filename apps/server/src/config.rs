use crate::error::{AppError, Result};
use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// PostgreSQL database connection URL
    pub database_url: String,
    /// Server port to listen on
    pub port: u16,
    /// Log level for tracing
    pub log_level: String,
    /// Environment (development, production)
    pub environment: String,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// Required environment variables:
    /// - `DATABASE_URL` - PostgreSQL connection string
    ///
    /// Optional environment variables with defaults:
    /// - `PORT` - Server port (default: 3000)
    /// - `RUST_LOG` - Log level (default: "info")
    /// - `APP_ENV` - Environment (default: "development")
    pub fn load() -> Result<Self> {
        Ok(Config {
            database_url: Self::require_env("DATABASE_URL")?,
            port: Self::parse_env_var("PORT", "3000")?,
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            environment: env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
        })
    }

    /// Get the database URL as a string slice
    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }

    /// Get a required environment variable, returning an error if missing
    fn require_env(key: &str) -> Result<String> {
        env::var(key).map_err(|_| {
            AppError::Internal(format!("Required environment variable '{}' is not set", key))
        })
    }

    /// Parse an environment variable to a specific type with a default value
    fn parse_env_var<T: std::str::FromStr>(key: &str, default: &str) -> Result<T> {
        env::var(key)
            .unwrap_or_else(|_| default.to_string())
            .parse()
            .map_err(|_| {
                AppError::Internal(format!(
                    "Failed to parse environment variable '{}': invalid value",
                    key
                ))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_env_var_with_default() {
        // This test requires environment to be clean for PORT
        let result = Config::parse_env_var::<u16>("_TEST_NONEXISTENT_VAR", "3000");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3000);
    }

    #[test]
    fn test_parse_env_var_with_value() {
        env::set_var("_TEST_PARSE_VAR", "4000");
        let result = Config::parse_env_var::<u16>("_TEST_PARSE_VAR", "3000");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4000);
        env::remove_var("_TEST_PARSE_VAR");
    }

    #[test]
    fn test_is_development() {
        let config = Config {
            database_url: "test".to_string(),
            port: 3000,
            log_level: "info".to_string(),
            environment: "development".to_string(),
        };
        assert!(config.is_development());
        assert!(!config.is_production());
    }

    #[test]
    fn test_is_production() {
        let config = Config {
            database_url: "test".to_string(),
            port: 3000,
            log_level: "warn".to_string(),
            environment: "production".to_string(),
        };
        assert!(!config.is_development());
        assert!(config.is_production());
    }
}

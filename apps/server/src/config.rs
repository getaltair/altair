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
    /// Minimum database pool connections
    pub db_min_conn: u32,
    /// Maximum database pool connections
    pub db_max_conn: u32,
    /// Database connection acquire timeout in seconds
    pub db_timeout_sec: u64,
    /// Session time-to-live in hours
    pub session_ttl_hours: u64,
    /// Secret key for signing PowerSync JWTs (HS256)
    pub jwt_secret: String,
    /// PowerSync service URL
    pub powersync_url: String,
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
    /// - `DB_MIN_CONN` - Minimum database pool connections (default: 5)
    /// - `DB_MAX_CONN` - Maximum database pool connections (default: 20)
    /// - `DB_TIMEOUT_SEC` - Database connection acquire timeout in seconds (default: 30)
    /// - `SESSION_TTL_HOURS` - Session time-to-live in hours (default: 72)
    /// - `POWERSYNC_JWT_SECRET` - Secret for signing PowerSync JWTs (default: dev secret)
    /// - `POWERSYNC_URL` - PowerSync service URL (default: "http://localhost:8080")
    pub fn load() -> Result<Self> {
        let environment = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        let is_production = environment == "production";

        let jwt_secret = if is_production {
            Self::require_env("POWERSYNC_JWT_SECRET")?
        } else {
            env::var("POWERSYNC_JWT_SECRET").unwrap_or_else(|_| {
                tracing::warn!("POWERSYNC_JWT_SECRET not set, using insecure dev default");
                "dev_powersync_secret_change_in_production".to_string()
            })
        };

        let powersync_url = env::var("POWERSYNC_URL").unwrap_or_else(|_| {
            if !is_production {
                tracing::warn!("POWERSYNC_URL not set, using localhost default");
            }
            "http://localhost:8080".to_string()
        });

        Ok(Config {
            database_url: Self::require_env("DATABASE_URL")?,
            port: Self::parse_env_var("PORT", "3000")?,
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            environment,
            db_min_conn: Self::parse_env_var("DB_MIN_CONN", "5")?,
            db_max_conn: Self::parse_env_var("DB_MAX_CONN", "20")?,
            db_timeout_sec: Self::parse_env_var("DB_TIMEOUT_SEC", "30")?,
            session_ttl_hours: Self::parse_env_var("SESSION_TTL_HOURS", "72")?,
            jwt_secret,
            powersync_url,
        })
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn environment(&self) -> &str {
        &self.environment
    }

    pub fn log_level(&self) -> &str {
        &self.log_level
    }

    pub fn db_min_conn(&self) -> u32 {
        self.db_min_conn
    }

    pub fn db_max_conn(&self) -> u32 {
        self.db_max_conn
    }

    pub fn db_timeout_sec(&self) -> u64 {
        self.db_timeout_sec
    }

    pub fn session_ttl_hours(&self) -> u64 {
        self.session_ttl_hours
    }

    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }

    pub fn powersync_url(&self) -> &str {
        &self.powersync_url
    }

    /// Get a required environment variable, returning an error if missing
    fn require_env(key: &str) -> Result<String> {
        env::var(key).map_err(|_| {
            AppError::Internal(format!("Required environment variable '{}' is not set", key))
        })
    }

    /// Return a Config pre-filled with safe test defaults.
    ///
    /// Intended for unit tests that need a Config but do not care about
    /// specific values. Matches the defaults used across the test suite.
    #[cfg(test)]
    pub fn test_default() -> Config {
        Config {
            database_url: "postgresql://test:test@localhost:5432/test".to_string(),
            port: 3000,
            log_level: "info".to_string(),
            environment: "development".to_string(),
            db_min_conn: 5,
            db_max_conn: 20,
            db_timeout_sec: 30,
            session_ttl_hours: 72,
            jwt_secret: "test_jwt_secret_for_powersync".to_string(),
            powersync_url: "http://localhost:8080".to_string(),
        }
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
        unsafe { env::set_var("_TEST_PARSE_VAR", "4000") };
        let result = Config::parse_env_var::<u16>("_TEST_PARSE_VAR", "3000");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4000);
        unsafe { env::remove_var("_TEST_PARSE_VAR") };
    }

    #[test]
    fn test_is_development() {
        let config = Config::test_default();
        assert!(config.is_development());
        assert!(!config.is_production());
    }

    #[test]
    fn test_is_production() {
        let mut config = Config::test_default();
        config.environment = "production".to_string();
        assert!(!config.is_development());
        assert!(config.is_production());
    }
}

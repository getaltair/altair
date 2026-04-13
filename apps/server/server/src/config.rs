use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    #[allow(dead_code)]
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_string());

        let database_url = std::env::var("DATABASE_URL")
            .context("DATABASE_URL environment variable is required")?;

        Ok(Self {
            bind_addr,
            database_url,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[serial_test::serial]
    #[test]
    fn missing_database_url_returns_error() {
        let saved = std::env::var("DATABASE_URL").ok();
        // SAFETY: Mutating process environment is not thread-safe. serial_test
        // enforces that all tests in this module run sequentially, preventing
        // concurrent env var access.
        unsafe {
            std::env::remove_var("DATABASE_URL");
        }

        let result = Config::from_env();
        assert!(
            result.is_err(),
            "Expected error when DATABASE_URL is missing"
        );

        if let Some(val) = saved {
            // SAFETY: Same threading constraint as the remove_var call above —
            // serial_test enforces serialization across all tests in this module.
            unsafe {
                std::env::set_var("DATABASE_URL", val);
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn happy_path_returns_correct_values() {
        let saved_db = std::env::var("DATABASE_URL").ok();
        let saved_bind = std::env::var("BIND_ADDR").ok();

        // SAFETY: serial_test enforces that all tests in this module run
        // sequentially, preventing concurrent env var access.
        unsafe {
            std::env::set_var("DATABASE_URL", "postgres://test:test@localhost/testdb");
            std::env::set_var("BIND_ADDR", "127.0.0.1:9000");
        }

        let result = Config::from_env();
        assert!(result.is_ok(), "Expected Ok when all env vars are set");
        let config = result.unwrap();
        assert_eq!(config.database_url, "postgres://test:test@localhost/testdb");
        assert_eq!(config.bind_addr, "127.0.0.1:9000");

        // SAFETY: Same threading constraint as above.
        unsafe {
            match saved_db {
                Some(val) => std::env::set_var("DATABASE_URL", val),
                None => std::env::remove_var("DATABASE_URL"),
            }
            match saved_bind {
                Some(val) => std::env::set_var("BIND_ADDR", val),
                None => std::env::remove_var("BIND_ADDR"),
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn missing_bind_addr_falls_back_to_default() {
        let saved_db = std::env::var("DATABASE_URL").ok();
        let saved_bind = std::env::var("BIND_ADDR").ok();

        // SAFETY: serial_test enforces that all tests in this module run
        // sequentially, preventing concurrent env var access.
        unsafe {
            std::env::set_var("DATABASE_URL", "postgres://test:test@localhost/testdb");
            std::env::remove_var("BIND_ADDR");
        }

        let result = Config::from_env();
        assert!(result.is_ok(), "Expected Ok when DATABASE_URL is set");
        let config = result.unwrap();
        assert_eq!(config.bind_addr, "0.0.0.0:8000");

        // SAFETY: Same threading constraint as above.
        unsafe {
            match saved_db {
                Some(val) => std::env::set_var("DATABASE_URL", val),
                None => std::env::remove_var("DATABASE_URL"),
            }
            match saved_bind {
                Some(val) => std::env::set_var("BIND_ADDR", val),
                None => std::env::remove_var("BIND_ADDR"),
            }
        }
    }
}

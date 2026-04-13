use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    #[allow(dead_code)]
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let bind_addr = std::env::var("BIND_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:8000".to_string());

        let database_url = std::env::var("DATABASE_URL")
            .context("DATABASE_URL environment variable is required")?;

        Ok(Self { bind_addr, database_url })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_database_url_returns_error() {
        let saved = std::env::var("DATABASE_URL").ok();
        // SAFETY: single-threaded test; no other threads reading DATABASE_URL concurrently.
        unsafe {
            std::env::remove_var("DATABASE_URL");
        }

        let result = Config::from_env();
        assert!(result.is_err(), "Expected error when DATABASE_URL is missing");

        if let Some(val) = saved {
            // SAFETY: restoring the variable after test cleanup.
            unsafe {
                std::env::set_var("DATABASE_URL", val);
            }
        }
    }
}

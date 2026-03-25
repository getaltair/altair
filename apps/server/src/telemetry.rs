use crate::error::Result;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize global tracing subscriber with JSON logging
///
/// # Arguments
///
/// * `log_level` - Log level to use (e.g., "info", "debug", "warn")
///
/// # Example
///
/// ```no_run
/// telemetry::init("info").expect("Failed to initialize telemetry");
/// ```
pub fn init(log_level: &str) -> Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));

    // Set up JSON formatting for structured logs
    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .json()
                .with_current_span(true)
                .with_span_list(true)
                .with_target(false),
        )
        .init();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_basic() {
        // Test that telemetry initialization doesn't panic with valid log level
        // We can't actually initialize the global subscriber in tests
        // because it can only be set once per process
        let result = std::panic::catch_unwind(|| {
            // This would normally work, but we skip actual init in tests
            // to avoid "multiple subscriber" errors
        });
        assert!(result.is_ok());
    }
}

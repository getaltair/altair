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
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|e| {
        tracing::warn!(
            "Failed to parse RUST_LOG from environment ({}), falling back to log_level='{}'",
            e,
            log_level
        );
        EnvFilter::new(log_level)
    });

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

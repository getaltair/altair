//! Altair Server - Backend Foundation
//!
//! This is the main entry point for the Altair backend service.
//! It initializes configuration, telemetry, database connectivity, and serves the API.

mod api;
mod config;
mod db;
mod error;
mod telemetry;

use config::Config;
use error::Result;
use tokio::signal;

/// Main entry point for the Altair server
///
/// # Execution Flow
///
/// 1. Load environment variables from .env (if present)
/// 2. Load configuration from environment
/// 3. Initialize structured logging with tracing
/// 4. Create database connection pool
/// 5. Run database migrations
/// 6. Create and configure API router
/// 7. Bind and serve HTTP server
/// 8. Handle graceful shutdown on SIGTERM/SIGINT
#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if present (for local development)
    dotenvy::dotenv().ok();

    // Load configuration from environment variables
    let config = Config::load()?;
    tracing::info!(
        "Starting Altair server in {} mode on port {}",
        config.environment,
        config.port
    );

    // Initialize structured logging
    telemetry::init(&config.log_level)?;

    // Create database connection pool
    let pool = db::create_pool(&config).await?;

    // Run database migrations
    db::run_migrations(&pool).await?;

    // Create API router with all routes and middleware
    let app = api::create_router(pool);

    // Build the TCP listener
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| {
        error::AppError::Internal(format!("Failed to bind to {}: {}", addr, e))
    })?;

    tracing::info!("Server listening on {}", addr);

    // Configure graceful shutdown
    let shutdown_signal = shutdown_signal();

    // Serve the application with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .map_err(|e| {
            error::AppError::Internal(format!("Server error: {}", e))
        })?;

    tracing::info!("Server shutdown complete");
    Ok(())
}

/// Create a future that completes when a shutdown signal is received
///
/// # Signals Handled
///
/// - SIGINT (Ctrl+C)
/// - SIGTERM (termination signal)
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        tracing::info!("Received Ctrl+C, initiating graceful shutdown");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
        tracing::info!("Received SIGTERM, initiating graceful shutdown");
    };

    #[cfg(unix)]
    {
        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
    }

    #[cfg(not(unix))]
    {
        tokio::select! {
            _ = ctrl_c => {},
        }
    }
}

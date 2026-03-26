//! Altair Server - Backend Foundation
//!
//! This is main entry point for Altair backend service.
//! It initializes configuration, telemetry, database connectivity, and serves API.

mod api;
mod auth;
mod config;
mod contracts;
mod core;
mod db;
mod error;
mod serde_util;
mod telemetry;

use api::AppState;
use config::Config;
use error::Result;
use tokio::signal;

/// Main entry point for Altair server
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
    // Log debug if file not found (expected in production)
    // Log warn if file fails to load (e.g., malformed)
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(e) => {
            // dotenvy::Error doesn't have a kind() method, so we just log the error
            tracing::warn!("Failed to load .env: {}", e);
        }
    }

    // Load configuration from environment variables
    let config = Config::load()?;
    tracing::info!(
        "Starting Altair server in {} mode on port {}",
        config.environment(),
        config.port()
    );

    // Initialize structured logging
    telemetry::init(config.log_level())?;

    // Create database connection pool
    let pool = db::create_pool(&config).await?;

    // Run database migrations
    db::run_migrations(&pool).await?;

    // Create API router with all routes and middleware
    let state = AppState { pool, config: config.clone() };
    let app = api::create_router(state);

    // Build TCP listener
    let addr = format!("0.0.0.0:{}", config.port());
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| error::AppError::Internal(format!("Failed to bind to {}: {}", addr, e)))?;

    tracing::info!("Server listening on {}", addr);

    // Configure graceful shutdown
    let shutdown_signal = shutdown_signal();

    // Serve application with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .map_err(|e| error::AppError::Internal(format!("Server error: {}", e)))?;

    tracing::info!("Server shutdown complete");
    Ok(())
}

/// Create a future that completes when a shutdown signal is received
///
/// # Signals Handled
///
/// - SIGINT (Ctrl+C)
/// - SIGTERM (termination signal, Unix only)
async fn shutdown_signal() {
    let ctrl_c = async {
        match signal::ctrl_c().await {
            Ok(()) => {
                tracing::info!("Received Ctrl+C, initiating graceful shutdown");
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to install Ctrl+C handler: {}. Graceful shutdown will not work on Ctrl+C.",
                    e
                );
                // Wait forever since signal handler failed
                std::future::pending::<()>().await;
            }
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match signal::unix::signal(signal::unix::SignalKind::terminate()) {
            Ok(mut stream) => {
                stream.recv().await;
                tracing::info!("Received SIGTERM, initiating graceful shutdown");
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to install SIGTERM handler: {}. Graceful shutdown will not work on SIGTERM.",
                    e
                );
                // Wait forever since signal handler failed
                std::future::pending::<()>().await;
            }
        }
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

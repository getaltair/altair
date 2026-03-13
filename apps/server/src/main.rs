mod attachments;
mod auth;
mod config;
mod core;
mod db;
mod error;
mod guidance;
mod handlers;
mod knowledge;
mod search;
mod sync;
mod tracking;

use std::net::SocketAddr;

use anyhow::{Context, Result};
use axum::{Router, routing::get};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
	let config = config::Config::from_env();

	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer().json())
		.with(tracing_subscriber::EnvFilter::new(&config.log_level))
		.init();

	tracing::info!("Starting Altair server");
	tracing::info!(host = %config.host, port = %config.port, log_level = %config.log_level, "Configuration loaded");

	let pool = db::create_pool(&config.database_url)
		.await
		.context("Failed to create database connection pool")?;
	tracing::info!("Database connection pool created");

	let app = Router::new()
		.route("/health", get(handlers::health::health_check))
		.route("/users/me", get(handlers::users::me))
		.nest("/auth", auth::router())
		.nest("/core", core::handlers::router())
		.nest("/guidance", guidance::router())
		.nest("/knowledge", knowledge::router())
		.nest("/tracking", tracking::router())
		.nest("/attachments", attachments::router())
		.nest("/sync", sync::router())
		.nest("/search", search::router())
		.layer(TraceLayer::new_for_http())
		.with_state(pool);

	let addr: SocketAddr = format!("{}:{}", config.host, config.port)
		.parse()
		.context(format!(
			"Invalid host:port combination: {}:{}",
			config.host, config.port
		))?;
	let listener = tokio::net::TcpListener::bind(addr)
		.await
		.context(format!("Failed to bind to port {}", config.port))?;
	tracing::info!("Server listening on {}", addr);

	axum::serve(listener, app)
		.with_graceful_shutdown(shutdown_signal())
		.await
		.context("Server error")?;

	tracing::info!("Server shutdown complete");
	Ok(())
}

/// Waits for shutdown signal (SIGINT or SIGTERM).
///
/// On Unix systems, listens for both SIGINT (Ctrl+C) and SIGTERM.
/// On non-Unix systems, only listens for SIGINT via ctrl_c().
async fn shutdown_signal() {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("Failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("Failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {},
		_ = terminate => {},
	}

	tracing::info!("Shutdown signal received");
}

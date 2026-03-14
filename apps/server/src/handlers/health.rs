use axum::extract::State;
use axum::response::Json;
use chrono::Utc;
use serde::Serialize;
use serde_json::{Value, json};
use sqlx::postgres::PgPool;
use utoipa::ToSchema;

use crate::error::AppError;

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
	status: String,
	database: String,
	timestamp: String,
}

/// Health check endpoint that verifies database connectivity.
///
/// Executes a simple `SELECT 1` query against the database to ensure
/// the connection pool is functioning correctly.
///
/// # Returns
///
/// Returns a JSON response with:
/// - `status`: "healthy"
/// - `database`: "connected"
/// - `timestamp`: ISO 8601 formatted timestamp
#[utoipa::path(
	get,
	path = "/health",
	responses(
		(status = 200, description = "Service health check", body = HealthResponse, example = json!({"status": "healthy", "database": "connected", "timestamp": "2024-01-01T00:00:00Z"}))
	)
)]
pub async fn health_check(State(pool): State<PgPool>) -> Result<Json<Value>, AppError> {
	sqlx::query("SELECT 1").fetch_one(&pool).await?;

	let response = json!({
		"status": "healthy",
		"database": "connected",
		"timestamp": Utc::now().to_rfc3339()
	});

	Ok(Json(response))
}

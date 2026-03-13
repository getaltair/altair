use axum::extract::State;
use axum::response::Json;
use chrono::Utc;
use serde_json::{Value, json};
use sqlx::postgres::PgPool;

use crate::error::AppError;

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
pub async fn health_check(State(pool): State<PgPool>) -> Result<Json<Value>, AppError> {
	sqlx::query("SELECT 1").fetch_one(&pool).await?;

	let response = json!({
		"status": "healthy",
		"database": "connected",
		"timestamp": Utc::now().to_rfc3339()
	});

	Ok(Json(response))
}

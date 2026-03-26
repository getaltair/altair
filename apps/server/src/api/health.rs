use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use chrono::Utc;
use serde::Serialize;
use sqlx::PgPool;

/// Health check response
#[derive(Serialize)]
pub struct HealthResponse {
    /// HTTP status code returned by the endpoint
    pub http_status_code: String,
    /// Database connection status
    pub db: String,
    /// Current timestamp in ISO 8601 format
    pub timestamp: String,
}

/// Health check endpoint handler
///
/// Verifies service health by checking database connectivity.
///
/// # Returns
///
/// * `200 OK` with status "ok" and db "connected" if database is reachable
/// * `503 Service Unavailable` with status "degraded" and db "disconnected" if database fails
///
/// # Example
///
/// ```no_run
/// use axum::Json;
/// use sqlx::PgPool;
///
/// // Returns: {"status":"ok","db":"connected","timestamp":"2024-01-01T12:00:00Z"}
/// pub async fn health(pool: State<PgPool>) -> Json<HealthResponse> { ... }
/// ```
pub async fn health(State(pool): State<PgPool>) -> impl IntoResponse {
    // Simple query to verify database connectivity
    let (status, db_status) = match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => (StatusCode::OK, "connected".to_string()),
        Err(e) => {
            tracing::error!("Health check failed: database query error: {:?}", e);
            (StatusCode::SERVICE_UNAVAILABLE, "disconnected".to_string())
        }
    };

    let response = HealthResponse {
        http_status_code: status.as_u16().to_string(),
        db: db_status,
        timestamp: Utc::now().to_rfc3339(),
    };

    let body = Json(response);
    (status, body).into_response()
}

use axum::{
	http::StatusCode,
	response::{IntoResponse, Json},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)] // Variants reserved for future use (P3-003, P3-005)
pub enum AppError {
	#[error("Database error")]
	Database(#[from] sqlx::Error),

	#[error("Configuration error: {0}")]
	Config(String),

	#[error("Forbidden")]
	Forbidden,

	#[error("Not found: {0}")]
	NotFound(String),

	#[error("Internal server error")]
	Internal(String),

	#[error("Bad request: {0}")]
	BadRequest(String),

	#[error("Conflict: {0}")]
	Conflict(String),
}

impl IntoResponse for AppError {
	fn into_response(self) -> axum::response::Response {
		let (status, error_message) = match self {
			AppError::Database(ref err) => {
				if matches!(err, sqlx::Error::RowNotFound) {
					(StatusCode::NOT_FOUND, "Resource not found".to_string())
				} else {
					(
						StatusCode::INTERNAL_SERVER_ERROR,
						"Database error".to_string(),
					)
				}
			}
			AppError::Config(_) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				"Configuration error".to_string(),
			),
			AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
			AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
			AppError::Internal(_) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				"Internal server error".to_string(),
			),
			AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
			AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
		};

		let body = json!({ "error": error_message });
		(status, Json(body)).into_response()
	}
}

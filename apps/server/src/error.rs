use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use thiserror::Error;

/// Unified application error type with HTTP response mapping
#[derive(Error, Debug)]
pub enum AppError {
    /// Database-related errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Not found error (404)
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Bad request error (400)
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Unauthorized error (401)
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Internal server error (500)
    #[error("Internal server error: {0}")]
    Internal(String),
}

/// Error response format
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: &'static str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match &self {
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_error",
                    format!("Database error: {}", err),
                )
            }
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "not_found",
                msg.clone(),
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "bad_request",
                msg.clone(),
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                msg.clone(),
            ),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    msg.clone(),
                )
            }
        };

        let body = Json(ErrorResponse {
            error: message,
            code: error_code,
        });

        (status, body).into_response()
    }
}

/// Result type alias for operations that can fail with AppError
pub type Result<T> = std::result::Result<T, AppError>;

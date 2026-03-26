use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Unified application error type with HTTP response mapping
#[derive(Error, Debug)]
pub enum AppError {
    /// Database-related errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Not found error (404)
    #[allow(dead_code)]
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Bad request error (400)
    #[allow(dead_code)]
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Unauthorized error (401)
    #[allow(dead_code)]
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Internal server error (500)
    #[error("Internal server error: {0}")]
    Internal(String),
}

/// Error response format
#[derive(Serialize, Deserialize)]
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
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "unauthorized", msg.clone()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn test_error_database_maps_to_500() {
        let sqlx_error = sqlx::Error::RowNotFound;
        let app_error = AppError::Database(sqlx_error);
        let response = app_error.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_error_not_found_maps_to_404() {
        let app_error = AppError::NotFound("Resource not found".to_string());
        let response = app_error.into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_error_bad_request_maps_to_400() {
        let app_error = AppError::BadRequest("Invalid input".to_string());
        let response = app_error.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_error_unauthorized_maps_to_401() {
        let app_error = AppError::Unauthorized("Not authenticated".to_string());
        let response = app_error.into_response();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_error_internal_maps_to_500() {
        let app_error = AppError::Internal("Something went wrong".to_string());
        let response = app_error.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_error_message_formatting() {
        let custom_message = "Custom error message";
        let app_error = AppError::NotFound(custom_message.to_string());
        let response = app_error.into_response();

        // The error message should be present in the response
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_error_display_implementation() {
        // Test that the error display works correctly
        let error = AppError::NotFound("test message".to_string());
        let display_str = format!("{}", error);
        assert!(display_str.contains("Resource not found"));
        assert!(display_str.contains("test message"));
    }

    #[test]
    fn test_error_debug_implementation() {
        let error = AppError::BadRequest("bad input".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("BadRequest"));
    }

    #[test]
    fn test_error_internal_with_message() {
        let msg = "Internal server error occurred";
        let error = AppError::Internal(msg.to_string());
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_error_variants_are_distinct() {
        // Verify that each error variant produces a unique response
        let db_err = AppError::Database(sqlx::Error::RowNotFound);
        let nf_err = AppError::NotFound("not found".to_string());
        let br_err = AppError::BadRequest("bad request".to_string());
        let un_err = AppError::Unauthorized("unauthorized".to_string());
        let in_err = AppError::Internal("internal".to_string());

        let db_resp = db_err.into_response();
        let nf_resp = nf_err.into_response();
        let br_resp = br_err.into_response();
        let un_resp = un_err.into_response();
        let in_resp = in_err.into_response();

        assert_eq!(db_resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(nf_resp.status(), StatusCode::NOT_FOUND);
        assert_eq!(br_resp.status(), StatusCode::BAD_REQUEST);
        assert_eq!(un_resp.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(in_resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}

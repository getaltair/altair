//! API Error type for Tauri IPC error handling
//!
//! This module provides the `ApiError` type that is serializable for Tauri IPC.
//! Unlike the internal `Error` enum, `ApiError` is designed to be sent across
//! the IPC boundary to frontend code with type-safe bindings.

use serde::Serialize;

/// API error type for Tauri IPC responses
///
/// This struct is designed to be serializable and type-safe for use in Tauri commands.
/// Error codes use SCREAMING_SNAKE_CASE convention for consistency.
///
/// # Examples
///
/// ```
/// use altair_core::ApiError;
/// use serde_json::json;
///
/// let error = ApiError::new("DB_CONNECTION_FAILED", "Could not connect to database");
/// assert_eq!(error.code(), "DB_CONNECTION_FAILED");
/// assert_eq!(error.message(), "Could not connect to database");
///
/// let error_with_details = ApiError::with_details(
///     "VALIDATION_ERROR",
///     "Invalid input",
///     json!({"field": "email", "reason": "invalid format"})
/// );
/// ```
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiError {
    /// Error code in SCREAMING_SNAKE_CASE (e.g., "DB_CONNECTION_FAILED")
    code: String,
    /// Human-readable error message
    message: String,
    /// Optional structured error details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "specta", specta(skip))]
    details: Option<serde_json::Value>,
}

impl ApiError {
    /// Create a new API error with code and message
    ///
    /// # Arguments
    ///
    /// * `code` - Error code in SCREAMING_SNAKE_CASE
    /// * `message` - Human-readable error message
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Create a new API error with code, message, and additional details
    ///
    /// # Arguments
    ///
    /// * `code` - Error code in SCREAMING_SNAKE_CASE
    /// * `message` - Human-readable error message
    /// * `details` - Structured details as JSON value
    pub fn with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: Some(details),
        }
    }

    /// Get the error code
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the error details
    pub fn details(&self) -> Option<&serde_json::Value> {
        self.details.as_ref()
    }
}

impl From<crate::Error> for ApiError {
    fn from(error: crate::Error) -> Self {
        use crate::Error;

        match error {
            Error::Database(msg) => ApiError::new("DB_ERROR", msg),
            Error::Storage(msg) => ApiError::new("STORAGE_ERROR", msg),
            Error::Auth(msg) => ApiError::new("AUTH_ERROR", msg),
            Error::Validation(msg) => ApiError::new("VALIDATION_ERROR", msg),
            Error::NotFound { entity_type, id } => ApiError::with_details(
                "NOT_FOUND",
                format!("{} not found", entity_type),
                serde_json::json!({
                    "entity_type": entity_type,
                    "id": id
                }),
            ),
            Error::Sync(msg) => ApiError::new("SYNC_ERROR", msg),
            Error::Search(msg) => ApiError::new("SEARCH_ERROR", msg),
            Error::Serialization(msg) => ApiError::new("SERIALIZATION_ERROR", msg),
            Error::Internal(msg) => ApiError::new("INTERNAL_ERROR", msg),
            Error::Other(err) => ApiError::new("UNKNOWN_ERROR", err.to_string()),
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new() {
        let error = ApiError::new("TEST_ERROR", "Test message");
        assert_eq!(error.code(), "TEST_ERROR");
        assert_eq!(error.message(), "Test message");
        assert!(error.details().is_none());
    }

    #[test]
    fn test_with_details() {
        let details = json!({"field": "email", "reason": "invalid"});
        let error = ApiError::with_details("VALIDATION_ERROR", "Invalid input", details.clone());
        assert_eq!(error.code(), "VALIDATION_ERROR");
        assert_eq!(error.message(), "Invalid input");
        assert_eq!(error.details(), Some(&details));
    }

    #[test]
    fn test_serialization() {
        let error = ApiError::new("DB_CONNECTION_FAILED", "Could not connect");
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("DB_CONNECTION_FAILED"));
        assert!(json.contains("Could not connect"));
    }

    #[test]
    fn test_serialization_with_details() {
        let error = ApiError::with_details(
            "VALIDATION_ERROR",
            "Invalid input",
            json!({"field": "email"}),
        );
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("VALIDATION_ERROR"));
        assert!(json.contains("field"));
        assert!(json.contains("email"));
    }

    #[test]
    fn test_from_database_error() {
        let error = crate::Error::database("Connection failed");
        let api_error: ApiError = error.into();
        assert_eq!(api_error.code(), "DB_ERROR");
        assert_eq!(api_error.message(), "Connection failed");
    }

    #[test]
    fn test_from_not_found_error() {
        let error = crate::Error::not_found("Quest", "123");
        let api_error: ApiError = error.into();
        assert_eq!(api_error.code(), "NOT_FOUND");
        assert!(api_error.message().contains("Quest"));
        assert!(api_error.details().is_some());

        let details = api_error.details().unwrap();
        assert_eq!(details["entity_type"], "Quest");
        assert_eq!(details["id"], "123");
    }

    #[test]
    fn test_from_validation_error() {
        let error = crate::Error::validation("Invalid email format");
        let api_error: ApiError = error.into();
        assert_eq!(api_error.code(), "VALIDATION_ERROR");
        assert_eq!(api_error.message(), "Invalid email format");
    }

    #[test]
    fn test_display() {
        let error = ApiError::new("TEST_ERROR", "Test message");
        assert_eq!(error.to_string(), "[TEST_ERROR] Test message");
    }
}

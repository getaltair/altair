//! Storage error types for altair-storage crate
//!
//! This module defines specific error types for S3-compatible storage operations
//! including credential issues, S3 API errors, validation failures, and quota limits.

use thiserror::Error;

/// Errors specific to storage operations
#[derive(Debug, Error)]
pub enum StorageError {
    /// Credentials not found in keychain or configuration
    #[error("Storage credentials not found: {0}")]
    CredentialsNotFound(String),

    /// S3 API error from aws-sdk-s3
    #[error("S3 error: {operation} - {message}")]
    S3Error {
        /// The S3 operation that failed (e.g., "GetObject", "PutObject")
        operation: String,
        /// Error message from the S3 API
        message: String,
    },

    /// Invalid MIME type for upload
    #[error("Invalid MIME type: {mime_type} is not allowed")]
    InvalidMimeType {
        /// The rejected MIME type
        mime_type: String,
    },

    /// Storage quota exceeded
    #[error(
        "Storage quota exceeded: {bytes_used}/{bytes_limit} bytes used, cannot add {bytes_requested} bytes"
    )]
    QuotaExceeded {
        /// Current bytes used
        bytes_used: u64,
        /// Maximum allowed bytes
        bytes_limit: u64,
        /// Bytes requested for new upload
        bytes_requested: u64,
    },

    /// Object not found in storage
    #[error("Object not found: {key}")]
    ObjectNotFound {
        /// The S3 object key that was not found
        key: String,
    },

    /// Configuration validation error
    #[error("Invalid configuration: {field} - {message}")]
    InvalidConfig {
        /// The configuration field that failed validation
        field: String,
        /// Description of the validation failure
        message: String,
    },

    /// Presigned URL generation error
    #[error("Presigned URL error: {0}")]
    PresignedUrlError(String),

    /// Checksum calculation or verification error
    #[error("Checksum error: {0}")]
    ChecksumError(String),

    /// Keychain/keyring access error
    #[error("Keychain error: {0}")]
    KeychainError(String),

    /// Minio process management error
    #[error("Minio error: {0}")]
    MinioError(String),

    /// Minio startup failed with no fallback configured
    #[error(
        "Minio startup failed: {message}. Set STORAGE_ENDPOINT environment variable to use an external S3-compatible endpoint"
    )]
    MinioStartupFailed {
        /// Description of why startup failed
        message: String,
    },

    /// Image processing error (for thumbnails)
    #[error("Image processing error: {0}")]
    ImageError(String),

    /// Generic IO error
    #[error("IO error: {0}")]
    IoError(String),
}

/// Result type alias for storage operations
pub type StorageResult<T> = std::result::Result<T, StorageError>;

// Implement From conversions for common error types

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::IoError(err.to_string())
    }
}

impl From<keyring::Error> for StorageError {
    fn from(err: keyring::Error) -> Self {
        match err {
            keyring::Error::NoEntry => {
                StorageError::CredentialsNotFound("No credentials stored in keychain".to_string())
            }
            _ => StorageError::KeychainError(err.to_string()),
        }
    }
}

impl From<image::ImageError> for StorageError {
    fn from(err: image::ImageError) -> Self {
        StorageError::ImageError(err.to_string())
    }
}

impl StorageError {
    /// Create an S3 error from an operation name and error
    pub fn s3(operation: impl Into<String>, message: impl Into<String>) -> Self {
        StorageError::S3Error {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Create a credentials not found error
    pub fn credentials_not_found(message: impl Into<String>) -> Self {
        StorageError::CredentialsNotFound(message.into())
    }

    /// Create an invalid MIME type error
    pub fn invalid_mime_type(mime_type: impl Into<String>) -> Self {
        StorageError::InvalidMimeType {
            mime_type: mime_type.into(),
        }
    }

    /// Create a quota exceeded error
    pub fn quota_exceeded(bytes_used: u64, bytes_limit: u64, bytes_requested: u64) -> Self {
        StorageError::QuotaExceeded {
            bytes_used,
            bytes_limit,
            bytes_requested,
        }
    }

    /// Create an object not found error
    pub fn object_not_found(key: impl Into<String>) -> Self {
        StorageError::ObjectNotFound { key: key.into() }
    }

    /// Create a config validation error
    pub fn invalid_config(field: impl Into<String>, message: impl Into<String>) -> Self {
        StorageError::InvalidConfig {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a presigned URL error
    pub fn presigned_url(message: impl Into<String>) -> Self {
        StorageError::PresignedUrlError(message.into())
    }

    /// Create a checksum error
    pub fn checksum(message: impl Into<String>) -> Self {
        StorageError::ChecksumError(message.into())
    }

    /// Create a Minio startup failed error
    pub fn minio_startup_failed(message: impl Into<String>) -> Self {
        StorageError::MinioStartupFailed {
            message: message.into(),
        }
    }
}

/// Convert StorageError to altair_core::Error for integration with the rest of the codebase
impl From<StorageError> for altair_core::Error {
    fn from(err: StorageError) -> Self {
        altair_core::Error::storage(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s3_error_display() {
        let err = StorageError::s3("GetObject", "Object not found");
        assert_eq!(err.to_string(), "S3 error: GetObject - Object not found");
    }

    #[test]
    fn test_quota_exceeded_display() {
        let err = StorageError::quota_exceeded(4_000_000_000, 5_000_000_000, 2_000_000_000);
        assert!(err.to_string().contains("4000000000"));
        assert!(err.to_string().contains("5000000000"));
        assert!(err.to_string().contains("2000000000"));
    }

    #[test]
    fn test_invalid_config_display() {
        let err = StorageError::invalid_config("endpoint", "must be a valid URL");
        assert_eq!(
            err.to_string(),
            "Invalid configuration: endpoint - must be a valid URL"
        );
    }

    #[test]
    fn test_conversion_to_altair_error() {
        let storage_err = StorageError::object_not_found("test/file.txt");
        let core_err: altair_core::Error = storage_err.into();
        assert!(matches!(core_err, altair_core::Error::Storage(_)));
    }
}

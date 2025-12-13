//! # altair-storage
//!
//! S3-compatible object storage integration for Altair.
//!
//! This crate provides a unified interface for interacting with S3-compatible
//! storage services (AWS S3, Backblaze B2, MinIO, etc.).
//!
//! ## Architecture
//!
//! The storage system uses a presigned URL approach for uploads and downloads:
//!
//! 1. **Upload Flow**:
//!    - Client requests an upload URL via `request_upload()`
//!    - Backend validates MIME type and quota, generates presigned PUT URL
//!    - Client uploads directly to S3 using the presigned URL
//!    - Client calls `confirm_upload()` to verify and create attachment record
//!
//! 2. **Download Flow**:
//!    - Client requests a download URL via `get_download_url()`
//!    - Backend generates presigned GET URL (1 hour expiration)
//!    - Client downloads directly from S3
//!
//! ## Features
//!
//! - **Presigned URLs**: Time-limited upload/download URLs for direct S3 access
//! - **MIME Type Validation**: Only allowed file types can be uploaded
//! - **Quota Management**: Per-user storage limits with quota tracking
//! - **Thumbnail Generation**: Automatic thumbnail creation for images
//! - **Secure Credentials**: OS keychain integration for credential storage
//!
//! ## Modules
//!
//! - [`config`]: Configuration management and keychain integration
//! - [`error`]: Error types for storage operations
//! - [`client`]: S3 client wrapper with common operations
//! - [`presigned`]: Presigned URL generation for uploads and downloads

pub mod client;
pub mod config;
pub mod error;
pub mod presigned;

// Re-export commonly used types
pub use client::{ObjectMetadata, S3Client};
pub use config::StorageConfig;
pub use error::{StorageError, StorageResult};
pub use presigned::{
    DOWNLOAD_URL_EXPIRATION_SECS, PresignedDownload, PresignedUpload, PresignedUrlService,
    UPLOAD_URL_EXPIRATION_SECS, generate_object_key, generate_thumbnail_key,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify that key types are exported and usable
        let _: fn(&str, &str) -> String = generate_object_key;
        let _: fn(&str) -> String = generate_thumbnail_key;
    }

    #[test]
    fn test_error_conversion() {
        // Verify StorageError converts to altair_core::Error
        let storage_err = StorageError::object_not_found("test/file.txt");
        let core_err: altair_core::Error = storage_err.into();
        assert!(matches!(core_err, altair_core::Error::Storage(_)));
    }
}

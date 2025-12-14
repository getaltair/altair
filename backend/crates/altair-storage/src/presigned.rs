//! Presigned URL generation module
//!
//! This module handles generating presigned URLs for secure, time-limited
//! upload and download operations. Clients upload files directly to S3
//! using presigned PUT URLs, avoiding the need to proxy data through the backend.

use crate::client::S3Client;
use crate::error::{StorageError, StorageResult};
use aws_sdk_s3::presigning::PresigningConfig;
use std::time::Duration;
use tracing::instrument;
use uuid::Uuid;

/// Default expiration time for upload URLs (15 minutes)
pub const UPLOAD_URL_EXPIRATION_SECS: u64 = 15 * 60;

/// Default expiration time for download URLs (1 hour)
pub const DOWNLOAD_URL_EXPIRATION_SECS: u64 = 60 * 60;

/// A presigned upload URL with metadata
#[derive(Debug, Clone)]
pub struct PresignedUpload {
    /// The presigned PUT URL
    pub url: String,
    /// The S3 object key where the file will be uploaded
    pub key: String,
    /// URL expiration time in seconds
    pub expires_in_secs: u64,
    /// Required content type for the upload
    pub content_type: String,
    /// Maximum allowed content length in bytes
    pub max_content_length: u64,
}

/// A presigned download URL
#[derive(Debug, Clone)]
pub struct PresignedDownload {
    /// The presigned GET URL
    pub url: String,
    /// The S3 object key
    pub key: String,
    /// URL expiration time in seconds
    pub expires_in_secs: u64,
}

/// Service for generating presigned URLs
#[derive(Clone)]
pub struct PresignedUrlService {
    client: S3Client,
}

impl PresignedUrlService {
    /// Create a new presigned URL service
    pub fn new(client: S3Client) -> Self {
        Self { client }
    }

    /// Generate a presigned upload (PUT) URL
    ///
    /// The generated URL enforces:
    /// - Content-Type matching the provided MIME type
    /// - Content-Length not exceeding max_content_length
    /// - Expiration after 15 minutes by default
    ///
    /// # Arguments
    /// * `user_id` - User ID for namespacing the object key
    /// * `filename` - Original filename (sanitized and used in the key)
    /// * `content_type` - Required MIME type for the upload
    /// * `max_content_length` - Maximum file size in bytes
    ///
    /// # Returns
    /// `PresignedUpload` containing the URL, key, and constraints
    #[instrument(skip(self), fields(bucket = %self.client.bucket()))]
    pub async fn generate_upload_url(
        &self,
        user_id: &str,
        filename: &str,
        content_type: &str,
        max_content_length: u64,
    ) -> StorageResult<PresignedUpload> {
        // Generate a unique object key: {user_id}/{uuid}-{sanitized_filename}
        let key = generate_object_key(user_id, filename);

        // Create presigning config with expiration
        let presigning_config = PresigningConfig::builder()
            .expires_in(Duration::from_secs(UPLOAD_URL_EXPIRATION_SECS))
            .build()
            .map_err(|e| StorageError::presigned_url(format!("Failed to build config: {}", e)))?;

        // Build the PUT request with content constraints
        let presigned_request = self
            .client
            .inner()
            .put_object()
            .bucket(self.client.bucket())
            .key(&key)
            .content_type(content_type)
            .content_length(max_content_length as i64)
            .presigned(presigning_config)
            .await
            .map_err(|e| StorageError::presigned_url(format!("Failed to generate URL: {}", e)))?;

        let url = presigned_request.uri().to_string();

        tracing::info!(
            key = key,
            content_type = content_type,
            max_size = max_content_length,
            "Generated presigned upload URL"
        );

        Ok(PresignedUpload {
            url,
            key,
            expires_in_secs: UPLOAD_URL_EXPIRATION_SECS,
            content_type: content_type.to_string(),
            max_content_length,
        })
    }

    /// Generate a presigned download (GET) URL
    ///
    /// The generated URL:
    /// - Expires after 1 hour by default
    /// - Allows downloading the file directly from S3
    ///
    /// # Arguments
    /// * `key` - The S3 object key to generate a download URL for
    ///
    /// # Returns
    /// `PresignedDownload` containing the URL and expiration info
    #[instrument(skip(self), fields(bucket = %self.client.bucket()))]
    pub async fn generate_download_url(&self, key: &str) -> StorageResult<PresignedDownload> {
        self.generate_download_url_with_expiration(key, DOWNLOAD_URL_EXPIRATION_SECS)
            .await
    }

    /// Generate a presigned download URL with custom expiration
    ///
    /// # Arguments
    /// * `key` - The S3 object key
    /// * `expires_in_secs` - Custom expiration time in seconds
    #[instrument(skip(self), fields(bucket = %self.client.bucket()))]
    pub async fn generate_download_url_with_expiration(
        &self,
        key: &str,
        expires_in_secs: u64,
    ) -> StorageResult<PresignedDownload> {
        let presigning_config = PresigningConfig::builder()
            .expires_in(Duration::from_secs(expires_in_secs))
            .build()
            .map_err(|e| StorageError::presigned_url(format!("Failed to build config: {}", e)))?;

        let presigned_request = self
            .client
            .inner()
            .get_object()
            .bucket(self.client.bucket())
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| StorageError::presigned_url(format!("Failed to generate URL: {}", e)))?;

        let url = presigned_request.uri().to_string();

        tracing::debug!(
            key = key,
            expires_in = expires_in_secs,
            "Generated presigned download URL"
        );

        Ok(PresignedDownload {
            url,
            key: key.to_string(),
            expires_in_secs,
        })
    }

    /// Generate a presigned URL for viewing thumbnails
    ///
    /// Uses a shorter expiration (5 minutes) since thumbnails are
    /// frequently re-requested and cached by browsers.
    #[instrument(skip(self))]
    pub async fn generate_thumbnail_url(&self, key: &str) -> StorageResult<PresignedDownload> {
        // 5 minute expiration for thumbnails
        self.generate_download_url_with_expiration(key, 5 * 60)
            .await
    }
}

/// Generate a unique S3 object key
///
/// Format: `{user_id}/{uuid}-{sanitized_filename}`
///
/// This ensures:
/// - Objects are namespaced by user ID
/// - No collisions due to UUID
/// - Original filename preserved for reference
pub fn generate_object_key(user_id: &str, filename: &str) -> String {
    let uuid = Uuid::new_v4();
    let sanitized_filename = sanitize_filename(filename);
    format!("{}/{}-{}", user_id, uuid, sanitized_filename)
}

/// Generate a thumbnail key from an original object key
///
/// Adds `_thumb` suffix before the file extension.
/// Example: `user123/abc-photo.jpg` → `user123/abc-photo_thumb.jpg`
pub fn generate_thumbnail_key(original_key: &str) -> String {
    if let Some(dot_pos) = original_key.rfind('.') {
        format!(
            "{}_thumb{}",
            &original_key[..dot_pos],
            &original_key[dot_pos..]
        )
    } else {
        format!("{}_thumb", original_key)
    }
}

/// Sanitize a filename for use in S3 object keys
///
/// - Replaces spaces with underscores
/// - Removes potentially problematic characters
/// - Limits length to 100 characters
/// - Preserves file extension
fn sanitize_filename(filename: &str) -> String {
    // Characters safe for S3 keys
    let sanitized: String = filename
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                // Replace spaces and any other unsafe characters with underscore
                '_'
            }
        })
        .collect();

    // Limit length while preserving extension
    if sanitized.len() > 100 {
        if let Some(dot_pos) = sanitized.rfind('.') {
            let ext = &sanitized[dot_pos..];
            let name_max = 100 - ext.len();
            format!("{}{}", &sanitized[..name_max], ext)
        } else {
            sanitized[..100].to_string()
        }
    } else {
        sanitized
    }
}

/// Extract user ID from an object key
///
/// Assumes format: `{user_id}/{uuid}-{filename}`
pub fn extract_user_id_from_key(key: &str) -> Option<&str> {
    key.split('/').next()
}

impl std::fmt::Debug for PresignedUrlService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PresignedUrlService")
            .field("client", &self.client)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_object_key_format() {
        let key = generate_object_key("user123", "photo.jpg");
        assert!(key.starts_with("user123/"));
        assert!(key.ends_with("-photo.jpg"));
        // UUID is 36 chars, so total should be: "user123/" (8) + UUID (36) + "-photo.jpg" (10) = 54
        assert!(key.len() > 50);
    }

    #[test]
    fn test_generate_thumbnail_key() {
        assert_eq!(
            generate_thumbnail_key("user123/abc-photo.jpg"),
            "user123/abc-photo_thumb.jpg"
        );
        assert_eq!(
            generate_thumbnail_key("user123/abc-photo.png"),
            "user123/abc-photo_thumb.png"
        );
        assert_eq!(
            generate_thumbnail_key("user123/abc-photo"),
            "user123/abc-photo_thumb"
        );
    }

    #[test]
    fn test_sanitize_filename_spaces() {
        assert_eq!(sanitize_filename("my photo.jpg"), "my_photo.jpg");
    }

    #[test]
    fn test_sanitize_filename_special_chars() {
        assert_eq!(
            sanitize_filename("photo (1) [final].jpg"),
            "photo__1___final_.jpg"
        );
    }

    #[test]
    fn test_sanitize_filename_long_name() {
        let long_name = "a".repeat(150) + ".jpg";
        let sanitized = sanitize_filename(&long_name);
        assert_eq!(sanitized.len(), 100);
        assert!(sanitized.ends_with(".jpg"));
    }

    #[test]
    fn test_sanitize_filename_preserves_safe_chars() {
        assert_eq!(
            sanitize_filename("my-photo_v2.test.jpg"),
            "my-photo_v2.test.jpg"
        );
    }

    #[test]
    fn test_extract_user_id_from_key() {
        assert_eq!(
            extract_user_id_from_key("user123/abc-photo.jpg"),
            Some("user123")
        );
        assert_eq!(
            extract_user_id_from_key("user:abc123/def-file.txt"),
            Some("user:abc123")
        );
    }

    #[test]
    fn test_presigned_upload_struct() {
        let upload = PresignedUpload {
            url: "https://example.com/upload".to_string(),
            key: "user123/abc-photo.jpg".to_string(),
            expires_in_secs: 900,
            content_type: "image/jpeg".to_string(),
            max_content_length: 10_000_000,
        };

        assert_eq!(upload.expires_in_secs, 900);
        assert_eq!(upload.content_type, "image/jpeg");
    }
}

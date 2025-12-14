//! Storage service module
//!
//! This module provides the main StorageService that orchestrates upload and download flows.
//! It coordinates MIME validation, quota checks, presigned URL generation, checksum calculation,
//! and attachment record creation.

use crate::checksum::calculate_object_checksum;
use crate::client::S3Client;
use crate::config::StorageConfig;
use crate::error::{StorageError, StorageResult};
use crate::mime::{MediaType, classify_media_type, validate_mime_type};
use crate::presigned::{PresignedDownload, PresignedUpload, PresignedUrlService};
use tracing::instrument;

/// Maximum file size for uploads (100MB per spec)
pub const MAX_FILE_SIZE_BYTES: u64 = 100 * 1024 * 1024;

/// Default storage quota for new users (5GB per spec)
pub const DEFAULT_QUOTA_BYTES: u64 = 5 * 1024 * 1024 * 1024;

/// Result of a successful upload request
#[derive(Debug, Clone)]
pub struct UploadRequest {
    /// The presigned URL details
    pub presigned: PresignedUpload,
    /// The classified media type
    pub media_type: MediaType,
}

/// Information about a confirmed upload
#[derive(Debug, Clone)]
pub struct UploadConfirmation {
    /// The S3 object key
    pub storage_key: String,
    /// Original filename
    pub filename: String,
    /// MIME type
    pub mime_type: String,
    /// File size in bytes
    pub size_bytes: u64,
    /// SHA-256 checksum (hex-encoded)
    pub checksum: String,
    /// Classified media type
    pub media_type: MediaType,
    /// Whether thumbnail generation should be triggered
    pub should_generate_thumbnail: bool,
}

/// Storage service for managing file uploads and downloads
///
/// This is the main entry point for storage operations. It coordinates:
/// - MIME type validation
/// - Quota checking (placeholder until Phase 4)
/// - Presigned URL generation
/// - Upload confirmation with checksum calculation
/// - Download URL generation
#[derive(Clone)]
pub struct StorageService {
    client: S3Client,
    presigned: PresignedUrlService,
    bucket: String,
}

impl StorageService {
    /// Create a new storage service from configuration
    #[instrument(skip(config), fields(bucket = %config.bucket()))]
    pub async fn new(config: &StorageConfig) -> StorageResult<Self> {
        let client = S3Client::new(config).await?;
        let presigned = PresignedUrlService::new(client.clone());

        tracing::info!("StorageService initialized for bucket: {}", config.bucket());

        Ok(Self {
            bucket: config.bucket().to_string(),
            client,
            presigned,
        })
    }

    /// Create a storage service from an existing S3 client
    pub fn from_client(client: S3Client) -> Self {
        let presigned = PresignedUrlService::new(client.clone());
        let bucket = client.bucket().to_string();

        Self {
            client,
            presigned,
            bucket,
        }
    }

    /// Get the bucket name
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Get the underlying S3 client
    pub fn client(&self) -> &S3Client {
        &self.client
    }

    /// Request an upload URL for a file
    ///
    /// This performs validation before generating a presigned upload URL:
    /// 1. Validates MIME type is allowed
    /// 2. Validates file size is within limits
    /// 3. Checks user quota (placeholder - always succeeds for now)
    /// 4. Generates presigned PUT URL
    ///
    /// # Arguments
    /// * `user_id` - The user requesting the upload (for namespacing)
    /// * `filename` - Original filename
    /// * `mime_type` - Claimed MIME type of the file
    /// * `size_bytes` - File size in bytes
    ///
    /// # Returns
    /// `UploadRequest` containing the presigned URL and media classification
    ///
    /// # Errors
    /// - `StorageError::InvalidMimeType` if MIME type not allowed
    /// - `StorageError::QuotaExceeded` if file too large or quota exceeded
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn request_upload(
        &self,
        user_id: &str,
        filename: &str,
        mime_type: &str,
        size_bytes: u64,
    ) -> StorageResult<UploadRequest> {
        // Step 1: Validate MIME type
        validate_mime_type(mime_type)?;

        // Step 2: Validate file size
        if size_bytes > MAX_FILE_SIZE_BYTES {
            return Err(StorageError::quota_exceeded(
                0, // bytes_used - placeholder
                MAX_FILE_SIZE_BYTES,
                size_bytes,
            ));
        }

        // Step 3: Check quota (placeholder - will be implemented in Phase 4)
        // For now, quota checking is a no-op
        // self.check_quota(user_id, size_bytes).await?;

        // Step 4: Classify media type
        let media_type = classify_media_type(mime_type);

        // Step 5: Generate presigned upload URL
        let presigned = self
            .presigned
            .generate_upload_url(user_id, filename, mime_type, size_bytes)
            .await?;

        tracing::info!(
            user_id = user_id,
            filename = filename,
            mime_type = mime_type,
            size_bytes = size_bytes,
            media_type = %media_type,
            key = presigned.key,
            "Upload URL generated"
        );

        Ok(UploadRequest {
            presigned,
            media_type,
        })
    }

    /// Confirm an upload completed successfully
    ///
    /// This verifies the upload and prepares attachment metadata:
    /// 1. Checks that the object exists in S3 (HEAD request)
    /// 2. Retrieves object metadata (size, content-type)
    /// 3. Calculates SHA-256 checksum (streaming for large files)
    /// 4. Returns confirmation data for attachment record creation
    ///
    /// Note: This method does NOT create the attachment record in the database.
    /// That responsibility belongs to the command layer which has access to altair-db.
    ///
    /// # Arguments
    /// * `storage_key` - The S3 object key from the upload request
    /// * `original_filename` - The original filename (for record)
    /// * `claimed_mime_type` - The MIME type claimed during request
    ///
    /// # Returns
    /// `UploadConfirmation` with all metadata needed for attachment record
    ///
    /// # Errors
    /// - `StorageError::ObjectNotFound` if object doesn't exist
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn confirm_upload(
        &self,
        storage_key: &str,
        original_filename: &str,
        claimed_mime_type: &str,
    ) -> StorageResult<UploadConfirmation> {
        // Step 1: Verify object exists via HEAD request
        let metadata = self.client.head_object(storage_key).await?;

        // Step 2: Calculate checksum (streaming for large files)
        let checksum = calculate_object_checksum(&self.client, storage_key, metadata.size).await?;

        // Step 3: Classify media type
        let media_type = classify_media_type(claimed_mime_type);

        // Step 4: Determine if thumbnail should be generated
        let should_generate_thumbnail = media_type.supports_thumbnail();

        tracing::info!(
            storage_key = storage_key,
            size_bytes = metadata.size,
            checksum = checksum,
            media_type = %media_type,
            should_generate_thumbnail = should_generate_thumbnail,
            "Upload confirmed"
        );

        Ok(UploadConfirmation {
            storage_key: storage_key.to_string(),
            filename: original_filename.to_string(),
            mime_type: claimed_mime_type.to_string(),
            size_bytes: metadata.size,
            checksum,
            media_type,
            should_generate_thumbnail,
        })
    }

    /// Generate a download URL for an object
    ///
    /// Creates a presigned GET URL for downloading a file. The URL is valid
    /// for 1 hour by default.
    ///
    /// # Arguments
    /// * `storage_key` - The S3 object key
    ///
    /// # Returns
    /// `PresignedDownload` with the URL and expiration info
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn get_download_url(&self, storage_key: &str) -> StorageResult<PresignedDownload> {
        self.presigned.generate_download_url(storage_key).await
    }

    /// Generate a download URL with custom expiration
    ///
    /// # Arguments
    /// * `storage_key` - The S3 object key
    /// * `expires_in_secs` - Custom expiration time in seconds
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn get_download_url_with_expiration(
        &self,
        storage_key: &str,
        expires_in_secs: u64,
    ) -> StorageResult<PresignedDownload> {
        self.presigned
            .generate_download_url_with_expiration(storage_key, expires_in_secs)
            .await
    }

    /// Generate a thumbnail URL for an object
    ///
    /// Creates a presigned GET URL for downloading a thumbnail. The URL has
    /// a shorter expiration (5 minutes) since thumbnails are frequently cached.
    ///
    /// # Arguments
    /// * `thumbnail_key` - The S3 object key for the thumbnail
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn get_thumbnail_url(&self, thumbnail_key: &str) -> StorageResult<PresignedDownload> {
        self.presigned.generate_thumbnail_url(thumbnail_key).await
    }

    /// Delete an object from storage
    ///
    /// Removes the object from S3. This operation is idempotent - deleting
    /// a non-existent object does not return an error.
    ///
    /// # Arguments
    /// * `storage_key` - The S3 object key to delete
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn delete_object(&self, storage_key: &str) -> StorageResult<()> {
        self.client.delete_object(storage_key).await
    }

    /// Delete an object and its thumbnail
    ///
    /// Removes both the main object and its thumbnail (if exists) from S3.
    ///
    /// # Arguments
    /// * `storage_key` - The S3 object key to delete
    /// * `thumbnail_key` - Optional thumbnail key to delete
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn delete_with_thumbnail(
        &self,
        storage_key: &str,
        thumbnail_key: Option<&str>,
    ) -> StorageResult<()> {
        // Delete main object
        self.client.delete_object(storage_key).await?;

        // Delete thumbnail if present
        if let Some(thumb_key) = thumbnail_key {
            // Ignore errors for thumbnail deletion (may not exist)
            if let Err(e) = self.client.delete_object(thumb_key).await {
                tracing::warn!(
                    thumbnail_key = thumb_key,
                    error = %e,
                    "Failed to delete thumbnail (may not exist)"
                );
            }
        }

        Ok(())
    }

    /// Check if an object exists
    ///
    /// # Arguments
    /// * `storage_key` - The S3 object key to check
    ///
    /// # Returns
    /// `true` if the object exists, `false` otherwise
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn object_exists(&self, storage_key: &str) -> StorageResult<bool> {
        self.client.object_exists(storage_key).await
    }

    /// Perform a health check on the storage service
    ///
    /// Verifies connectivity to S3 and bucket access.
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> StorageResult<()> {
        self.client.health_check().await
    }

    /// Spawn a background task to generate a thumbnail
    ///
    /// This method spawns an asynchronous task that:
    /// 1. Downloads the original image
    /// 2. Generates a thumbnail (256×256 max, JPEG 80% quality)
    /// 3. Uploads the thumbnail to S3
    /// 4. Calls the provided callback with the result
    ///
    /// The task runs independently and doesn't block the caller.
    ///
    /// # Arguments
    /// * `storage_key` - The S3 key of the original image
    /// * `on_complete` - Optional callback invoked when thumbnail is ready
    ///
    /// # Example
    /// ```ignore
    /// // Simple spawn without callback
    /// service.spawn_thumbnail_generation("user123/photo.jpg", None);
    ///
    /// // With callback for database update
    /// service.spawn_thumbnail_generation(
    ///     "user123/photo.jpg",
    ///     Some(Arc::new(|key, result| Box::pin(async move {
    ///         if let Ok(thumb) = result {
    ///             // Update attachment record with thumb.thumbnail_key
    ///         }
    ///     }))),
    /// );
    /// ```
    #[instrument(skip(self, on_complete), fields(bucket = %self.bucket))]
    pub fn spawn_thumbnail_generation(
        &self,
        storage_key: &str,
        on_complete: Option<crate::background::ThumbnailCallback>,
    ) {
        crate::background::spawn_thumbnail_task(crate::background::ThumbnailTaskOptions {
            client: self.client.clone(),
            storage_key: storage_key.to_string(),
            on_complete,
        });
    }

    /// Generate a thumbnail synchronously (blocking)
    ///
    /// Unlike `spawn_thumbnail_generation`, this method waits for the thumbnail
    /// to be generated and uploaded before returning.
    ///
    /// # Arguments
    /// * `storage_key` - The S3 key of the original image
    ///
    /// # Returns
    /// `ThumbnailResult` with the thumbnail key and metadata
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn generate_thumbnail(
        &self,
        storage_key: &str,
    ) -> StorageResult<crate::thumbnail::ThumbnailResult> {
        crate::background::generate_thumbnail_for_object(&self.client, storage_key).await
    }

    /// Confirm upload and optionally spawn thumbnail generation
    ///
    /// This is a convenience method that combines `confirm_upload` with
    /// automatic thumbnail spawning for supported media types.
    ///
    /// # Arguments
    /// * `storage_key` - The S3 object key
    /// * `original_filename` - The original filename
    /// * `claimed_mime_type` - The MIME type claimed during request
    /// * `on_thumbnail_complete` - Optional callback for thumbnail completion
    ///
    /// # Returns
    /// `UploadConfirmation` with metadata; thumbnail generation runs in background
    #[instrument(skip(self, on_thumbnail_complete), fields(bucket = %self.bucket))]
    pub async fn confirm_upload_with_thumbnail(
        &self,
        storage_key: &str,
        original_filename: &str,
        claimed_mime_type: &str,
        on_thumbnail_complete: Option<crate::background::ThumbnailCallback>,
    ) -> StorageResult<UploadConfirmation> {
        // First, confirm the upload
        let confirmation = self
            .confirm_upload(storage_key, original_filename, claimed_mime_type)
            .await?;

        // Spawn thumbnail generation if applicable
        if confirmation.should_generate_thumbnail {
            tracing::info!(
                storage_key = storage_key,
                media_type = %confirmation.media_type,
                "Spawning background thumbnail generation"
            );
            self.spawn_thumbnail_generation(storage_key, on_thumbnail_complete);
        }

        Ok(confirmation)
    }
}

impl std::fmt::Debug for StorageService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageService")
            .field("bucket", &self.bucket)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_file_size() {
        assert_eq!(MAX_FILE_SIZE_BYTES, 100 * 1024 * 1024);
    }

    #[test]
    fn test_default_quota() {
        assert_eq!(DEFAULT_QUOTA_BYTES, 5 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_upload_confirmation_struct() {
        let confirmation = UploadConfirmation {
            storage_key: "user123/abc-photo.jpg".to_string(),
            filename: "photo.jpg".to_string(),
            mime_type: "image/jpeg".to_string(),
            size_bytes: 1024 * 1024,
            checksum: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
                .to_string(),
            media_type: MediaType::Photo,
            should_generate_thumbnail: true,
        };

        assert!(confirmation.should_generate_thumbnail);
        assert_eq!(confirmation.media_type, MediaType::Photo);
    }

    #[test]
    fn test_upload_request_struct() {
        let presigned = PresignedUpload {
            url: "https://example.com/upload".to_string(),
            key: "user123/abc-photo.jpg".to_string(),
            expires_in_secs: 900,
            content_type: "image/jpeg".to_string(),
            max_content_length: 10_000_000,
        };

        let request = UploadRequest {
            presigned,
            media_type: MediaType::Photo,
        };

        assert_eq!(request.media_type, MediaType::Photo);
        assert!(request.presigned.key.starts_with("user123/"));
    }
}

//! Storage command handlers for Tauri IPC
//!
//! This module provides the bridge between Svelte frontend and S3-compatible
//! object storage functionality, exposing commands for file uploads, downloads,
//! and quota management.

use altair_core::Result;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::Thing;

// Re-export storage types
pub use altair_storage::{
    MediaType, QuotaInfo, StorageService, ThumbnailResult, UploadConfirmation, UploadRequest,
};

/// Input for requesting an upload URL
///
/// Used by `storage_request_upload` Tauri command to get a presigned upload URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RequestUploadInput {
    /// Original filename (used for key generation)
    pub filename: String,
    /// MIME type of the file to upload
    pub mime_type: String,
    /// File size in bytes (for quota check)
    pub size_bytes: u64,
}

/// Response from requesting an upload URL
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RequestUploadResponse {
    /// Presigned PUT URL for direct upload to S3
    pub upload_url: String,
    /// The S3 object key where the file will be stored
    pub storage_key: String,
    /// URL expiration time in seconds
    pub expires_in_secs: u64,
    /// Required content type for the upload
    pub content_type: String,
    /// Maximum allowed content length in bytes
    pub max_content_length: u64,
    /// Classified media type (photo, audio, document, etc.)
    pub media_type: String,
}

/// Input for confirming an upload
///
/// Used by `storage_confirm_upload` Tauri command after client uploads to S3.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ConfirmUploadInput {
    /// The S3 object key that was uploaded to
    pub storage_key: String,
    /// Original filename
    pub filename: String,
    /// Expected MIME type
    pub mime_type: String,
}

/// Response from confirming an upload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ConfirmUploadResponse {
    /// The created attachment record ID
    pub attachment_id: String,
    /// The S3 object key
    pub storage_key: String,
    /// Actual file size in bytes
    pub size_bytes: u64,
    /// SHA-256 checksum (hex-encoded)
    pub checksum: String,
    /// Classified media type
    pub media_type: String,
    /// Whether a thumbnail is being generated
    pub thumbnail_pending: bool,
}

/// Input for getting a download URL
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GetUrlInput {
    /// Attachment ID to get URL for
    pub attachment_id: String,
    /// Whether to get thumbnail URL (if available)
    pub thumbnail: bool,
}

/// Response with download URL
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GetUrlResponse {
    /// Presigned GET URL for download
    pub url: String,
    /// URL expiration time in seconds
    pub expires_in_secs: u64,
    /// The storage key for the object
    pub storage_key: String,
    /// Whether this is a thumbnail URL
    pub is_thumbnail: bool,
}

/// Input for deleting an attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DeleteInput {
    /// Attachment ID to delete
    pub attachment_id: String,
}

/// Response for quota information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct QuotaResponse {
    /// Current bytes used by the user
    pub bytes_used: u64,
    /// Maximum allowed bytes for the user
    pub bytes_limit: u64,
    /// Bytes available for additional uploads
    pub bytes_available: u64,
    /// Percentage of quota used (0.0 - 100.0)
    pub percentage_used: f64,
}

impl From<QuotaInfo> for QuotaResponse {
    fn from(info: QuotaInfo) -> Self {
        Self {
            bytes_used: info.bytes_used,
            bytes_limit: info.bytes_limit,
            bytes_available: info.bytes_available,
            percentage_used: info.percentage_used,
        }
    }
}

// Helper function to get current user from session
async fn get_current_user<C: surrealdb::Connection>(db: &Surreal<C>) -> Result<(Thing, String)> {
    // Get token from keychain
    let keychain = altair_auth::local::keychain::KeychainStorage::new();
    let token = keychain
        .get_token()
        .await?
        .ok_or_else(|| altair_core::Error::Auth("No active session".to_string()))?;

    // Get session from database
    let session = altair_db::queries::get_session_by_token(db, &token)
        .await?
        .ok_or_else(|| altair_core::Error::Auth("Session not found".to_string()))?;

    // Check if expired
    if session.is_expired() {
        return Err(altair_core::Error::Auth("Session expired".to_string()));
    }

    // Return user ID and device ID
    Ok((session.user, session.device_id))
}

// ============================================================================
// Command Implementations
// ============================================================================

/// Request a presigned upload URL
///
/// Validates MIME type and quota, then generates a presigned PUT URL for direct
/// S3 upload. The client uses this URL to upload directly to storage.
///
/// # Arguments
/// * `db` - Database connection
/// * `storage` - Storage service
/// * `input` - Upload request parameters
///
/// # Returns
/// `RequestUploadResponse` with presigned URL and metadata
pub async fn storage_request_upload<C: surrealdb::Connection>(
    db: &Surreal<C>,
    storage: &StorageService,
    input: RequestUploadInput,
) -> Result<RequestUploadResponse> {
    let (user_id, device_id) = get_current_user(db).await?;

    // Check quota before generating URL
    altair_storage::check_quota(db, &user_id, input.size_bytes, &device_id).await?;

    // Request upload from storage service
    let upload_request = storage
        .request_upload(
            &user_id.to_string(),
            &input.filename,
            &input.mime_type,
            input.size_bytes,
        )
        .await?;

    let media_type_str = match upload_request.media_type {
        altair_storage::MediaType::Photo => "photo",
        altair_storage::MediaType::Audio => "audio",
        altair_storage::MediaType::Video => "video",
        altair_storage::MediaType::Document => "document",
        altair_storage::MediaType::Other => "other",
    };

    Ok(RequestUploadResponse {
        upload_url: upload_request.presigned.url,
        storage_key: upload_request.presigned.key,
        expires_in_secs: upload_request.presigned.expires_in_secs,
        content_type: upload_request.presigned.content_type,
        max_content_length: upload_request.presigned.max_content_length,
        media_type: media_type_str.to_string(),
    })
}

/// Confirm an upload and create attachment record
///
/// After the client uploads to S3, this command:
/// 1. Verifies the object exists in S3
/// 2. Calculates the checksum
/// 3. Creates an attachment record in the database
/// 4. Updates the user's quota
/// 5. Optionally triggers thumbnail generation for images
///
/// # Arguments
/// * `db` - Database connection
/// * `storage` - Storage service
/// * `input` - Confirmation parameters
///
/// # Returns
/// `ConfirmUploadResponse` with attachment metadata
pub async fn storage_confirm_upload<C: surrealdb::Connection>(
    db: &Surreal<C>,
    storage: &StorageService,
    input: ConfirmUploadInput,
) -> Result<ConfirmUploadResponse> {
    let (user_id, device_id) = get_current_user(db).await?;

    // Confirm upload with storage service (HEAD check + checksum calculation)
    let confirmation = storage
        .confirm_upload(&input.storage_key, &input.filename, &input.mime_type)
        .await?;

    // Convert storage MediaType to DB MediaType
    let db_media_type = match confirmation.media_type {
        altair_storage::MediaType::Photo => altair_db::schema::MediaType::Photo,
        altair_storage::MediaType::Audio => altair_db::schema::MediaType::Audio,
        altair_storage::MediaType::Video => altair_db::schema::MediaType::Video,
        altair_storage::MediaType::Document => altair_db::schema::MediaType::Document,
        altair_storage::MediaType::Other => altair_db::schema::MediaType::Document, // Default to document
    };

    // Create attachment record in database
    let attachment = altair_db::queries::create_attachment(
        db,
        confirmation.filename.clone(),
        confirmation.mime_type.clone(),
        confirmation.size_bytes as i32,
        confirmation.storage_key.clone(),
        confirmation.checksum.clone(),
        db_media_type,
        user_id.clone(),
        device_id.clone(),
    )
    .await?;

    // Update user quota
    altair_storage::increment_quota(db, &user_id, confirmation.size_bytes).await?;

    // Spawn thumbnail generation if needed
    let thumbnail_pending = if confirmation.should_generate_thumbnail {
        let storage_key_for_thumb = confirmation.storage_key.clone();
        let attachment_id = attachment.id.clone();
        let db_clone_for_callback = db.clone();

        // Spawn background thumbnail generation with callback
        storage.spawn_thumbnail_generation(
            &storage_key_for_thumb,
            Some(std::sync::Arc::new(move |_storage_key, result| {
                let attachment_id = attachment_id.clone();
                let db = db_clone_for_callback.clone();
                Box::pin(async move {
                    if let Ok(thumb_result) = result
                        && let Some(ref id) = attachment_id
                        && let Err(e) = altair_db::queries::update_attachment_thumbnail(
                            &db,
                            id,
                            thumb_result.thumbnail_key,
                        )
                        .await
                    {
                        tracing::error!(error = %e, "Failed to update attachment thumbnail");
                    }
                })
            })),
        );
        true
    } else {
        false
    };

    let media_type_str = match confirmation.media_type {
        altair_storage::MediaType::Photo => "photo",
        altair_storage::MediaType::Audio => "audio",
        altair_storage::MediaType::Video => "video",
        altair_storage::MediaType::Document => "document",
        altair_storage::MediaType::Other => "other",
    };

    Ok(ConfirmUploadResponse {
        attachment_id: attachment.id.map(|t| t.to_string()).unwrap_or_default(),
        storage_key: confirmation.storage_key,
        size_bytes: confirmation.size_bytes,
        checksum: confirmation.checksum,
        media_type: media_type_str.to_string(),
        thumbnail_pending,
    })
}

/// Get a presigned download URL for an attachment
///
/// # Arguments
/// * `db` - Database connection
/// * `storage` - Storage service
/// * `input` - URL request parameters
///
/// # Returns
/// `GetUrlResponse` with presigned download URL
pub async fn storage_get_url<C: surrealdb::Connection>(
    db: &Surreal<C>,
    storage: &StorageService,
    input: GetUrlInput,
) -> Result<GetUrlResponse> {
    let (user_id, _device_id) = get_current_user(db).await?;

    // Parse attachment ID to Thing
    let attachment_id = parse_thing(&input.attachment_id)?;

    // Get attachment record
    let attachment = altair_db::queries::get_attachment_by_id(db, &attachment_id).await?;

    // Verify ownership
    if attachment.owner != user_id {
        return Err(altair_core::Error::Auth(
            "Not authorized to access this attachment".to_string(),
        ));
    }

    // Determine which key to use
    let (storage_key, is_thumbnail) = if input.thumbnail {
        if let Some(ref thumb_key) = attachment.thumbnail_key {
            (thumb_key.clone(), true)
        } else {
            // Fall back to original if no thumbnail
            (attachment.storage_key, false)
        }
    } else {
        (attachment.storage_key, false)
    };

    // Generate presigned download URL
    let presigned = storage.get_download_url(&storage_key).await?;

    Ok(GetUrlResponse {
        url: presigned.url,
        expires_in_secs: presigned.expires_in_secs,
        storage_key,
        is_thumbnail,
    })
}

/// Delete an attachment
///
/// Deletes the S3 object, removes the attachment record, and updates quota.
///
/// # Arguments
/// * `db` - Database connection
/// * `storage` - Storage service
/// * `input` - Delete parameters
///
/// # Returns
/// `Ok(())` on success
pub async fn storage_delete<C: surrealdb::Connection>(
    db: &Surreal<C>,
    storage: &StorageService,
    input: DeleteInput,
) -> Result<()> {
    let (user_id, _device_id) = get_current_user(db).await?;

    // Parse attachment ID to Thing
    let attachment_id = parse_thing(&input.attachment_id)?;

    // Get attachment record
    let attachment = altair_db::queries::get_attachment_by_id(db, &attachment_id).await?;

    // Verify ownership
    if attachment.owner != user_id {
        return Err(altair_core::Error::Auth(
            "Not authorized to delete this attachment".to_string(),
        ));
    }

    let size_bytes = attachment.size_bytes as u64;

    // Delete from S3 (with thumbnail if exists)
    storage
        .delete_with_thumbnail(&attachment.storage_key, attachment.thumbnail_key.as_deref())
        .await?;

    // Delete attachment record
    altair_db::queries::delete_attachment(db, &attachment_id).await?;

    // Decrement quota
    altair_storage::decrement_quota(db, &user_id, size_bytes).await?;

    Ok(())
}

/// Get user's storage quota information
///
/// # Arguments
/// * `db` - Database connection
///
/// # Returns
/// `QuotaResponse` with usage and limits
pub async fn storage_get_quota<C: surrealdb::Connection>(db: &Surreal<C>) -> Result<QuotaResponse> {
    let (user_id, device_id) = get_current_user(db).await?;

    let quota = altair_storage::get_quota(db, &user_id, &device_id).await?;

    Ok(quota.into())
}

// Helper function to parse a Thing from string
fn parse_thing(id_str: &str) -> Result<Thing> {
    // Handle both "table:id" format and just "id" format
    if let Some((table, id)) = id_str.split_once(':') {
        Ok(Thing::from((table.to_string(), id.to_string())))
    } else {
        // Assume attachment table if no prefix
        Ok(Thing::from(("attachment".to_string(), id_str.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_upload_input_serialization() {
        let input = RequestUploadInput {
            filename: "test.jpg".to_string(),
            mime_type: "image/jpeg".to_string(),
            size_bytes: 1024,
        };

        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("test.jpg"));
        assert!(json.contains("image/jpeg"));

        let deserialized: RequestUploadInput = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.filename, input.filename);
        assert_eq!(deserialized.mime_type, input.mime_type);
        assert_eq!(deserialized.size_bytes, input.size_bytes);
    }

    #[test]
    fn test_confirm_upload_input_serialization() {
        let input = ConfirmUploadInput {
            storage_key: "user123/abc-test.jpg".to_string(),
            filename: "test.jpg".to_string(),
            mime_type: "image/jpeg".to_string(),
        };

        let json = serde_json::to_string(&input).unwrap();
        let deserialized: ConfirmUploadInput = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.storage_key, input.storage_key);
    }

    #[test]
    fn test_quota_response_conversion() {
        let info = QuotaInfo {
            bytes_used: 1000,
            bytes_limit: 5000,
            bytes_available: 4000,
            percentage_used: 20.0,
            last_reconciled: None,
        };

        let response: QuotaResponse = info.into();
        assert_eq!(response.bytes_used, 1000);
        assert_eq!(response.bytes_limit, 5000);
        assert_eq!(response.bytes_available, 4000);
        assert_eq!(response.percentage_used, 20.0);
    }

    #[test]
    fn test_parse_thing() {
        let thing = parse_thing("attachment:abc123").unwrap();
        assert_eq!(thing.tb, "attachment");
        assert_eq!(thing.id.to_string(), "abc123");

        let thing2 = parse_thing("abc123").unwrap();
        assert_eq!(thing2.tb, "attachment");
        assert_eq!(thing2.id.to_string(), "abc123");
    }
}

//! Storage commands for Guidance app
//!
//! Provides Tauri commands for S3-compatible object storage operations:
//! - File upload (presigned URL generation)
//! - Upload confirmation
//! - File download (presigned URL generation)
//! - File deletion
//! - Quota management

use crate::state::AppState;
use altair_commands::storage::{
    ConfirmUploadInput, ConfirmUploadResponse, DeleteInput, GetUrlInput, GetUrlResponse,
    QuotaResponse, RequestUploadInput, RequestUploadResponse,
};

/// Request a presigned upload URL
///
/// Returns a presigned PUT URL for direct S3 upload. The client uses this URL
/// to upload files directly to storage without going through the backend.
///
/// # Arguments
/// * `state` - Application state containing storage service
/// * `input` - Upload request parameters (filename, mime_type, size_bytes)
///
/// # Returns
/// `RequestUploadResponse` with presigned URL and metadata
///
/// # Errors
/// Returns error if:
/// - Storage service not configured
/// - MIME type not allowed
/// - Quota exceeded
#[tauri::command]
#[specta::specta]
pub async fn storage_request_upload(
    state: tauri::State<'_, AppState>,
    input: RequestUploadInput,
) -> Result<RequestUploadResponse, String> {
    let storage = state
        .storage
        .as_ref()
        .ok_or_else(|| "Storage service not configured".to_string())?;

    altair_commands::storage::storage_request_upload(state.db.client(), storage, input)
        .await
        .map_err(|e| e.to_string())
}

/// Confirm an upload and create attachment record
///
/// After the client uploads to S3 using the presigned URL, this command:
/// 1. Verifies the object exists in S3
/// 2. Calculates SHA-256 checksum
/// 3. Creates attachment record in database
/// 4. Updates user quota
/// 5. Triggers thumbnail generation for images
///
/// # Arguments
/// * `state` - Application state
/// * `input` - Confirmation parameters (storage_key, filename, mime_type)
///
/// # Returns
/// `ConfirmUploadResponse` with attachment metadata
#[tauri::command]
#[specta::specta]
pub async fn storage_confirm_upload(
    state: tauri::State<'_, AppState>,
    input: ConfirmUploadInput,
) -> Result<ConfirmUploadResponse, String> {
    let storage = state
        .storage
        .as_ref()
        .ok_or_else(|| "Storage service not configured".to_string())?;

    altair_commands::storage::storage_confirm_upload(state.db.client(), storage, input)
        .await
        .map_err(|e| e.to_string())
}

/// Get a presigned download URL for an attachment
///
/// Returns a presigned GET URL for downloading a file. The URL is valid for
/// 1 hour. Can optionally request the thumbnail URL instead of the original.
///
/// # Arguments
/// * `state` - Application state
/// * `input` - URL request parameters (attachment_id, thumbnail flag)
///
/// # Returns
/// `GetUrlResponse` with presigned download URL
#[tauri::command]
#[specta::specta]
pub async fn storage_get_url(
    state: tauri::State<'_, AppState>,
    input: GetUrlInput,
) -> Result<GetUrlResponse, String> {
    let storage = state
        .storage
        .as_ref()
        .ok_or_else(|| "Storage service not configured".to_string())?;

    altair_commands::storage::storage_get_url(state.db.client(), storage, input)
        .await
        .map_err(|e| e.to_string())
}

/// Delete an attachment
///
/// Removes the file from S3, deletes the attachment record, and updates quota.
///
/// # Arguments
/// * `state` - Application state
/// * `input` - Delete parameters (attachment_id)
///
/// # Returns
/// Empty result on success
#[tauri::command]
#[specta::specta]
pub async fn storage_delete(
    state: tauri::State<'_, AppState>,
    input: DeleteInput,
) -> Result<(), String> {
    let storage = state
        .storage
        .as_ref()
        .ok_or_else(|| "Storage service not configured".to_string())?;

    altair_commands::storage::storage_delete(state.db.client(), storage, input)
        .await
        .map_err(|e| e.to_string())
}

/// Get user's storage quota information
///
/// Returns the current user's storage usage and limits.
///
/// # Arguments
/// * `state` - Application state
///
/// # Returns
/// `QuotaResponse` with usage and limits
#[tauri::command]
#[specta::specta]
pub async fn storage_get_quota(state: tauri::State<'_, AppState>) -> Result<QuotaResponse, String> {
    altair_commands::storage::storage_get_quota(state.db.client())
        .await
        .map_err(|e| e.to_string())
}

/// Check if storage service is available
///
/// Returns whether the storage service is configured and ready.
/// Useful for UI to conditionally show attachment features.
#[tauri::command]
#[specta::specta]
pub async fn storage_is_available(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    Ok(state.storage.is_some())
}

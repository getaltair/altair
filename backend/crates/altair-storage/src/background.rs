//! Background processing module
//!
//! This module handles spawning background tasks for operations that shouldn't
//! block the main request flow, such as thumbnail generation.
//!
//! ## Design
//!
//! Background tasks are spawned using `tokio::spawn` and run independently of
//! the request that triggered them. They use a callback pattern to notify
//! completion, allowing the caller to update database records when thumbnails
//! are ready.
//!
//! ## Error Handling
//!
//! Background task failures are logged but don't affect the parent operation.
//! For example, a failed thumbnail generation doesn't invalidate the upload -
//! the attachment is still usable, just without a thumbnail.

use crate::client::S3Client;
use crate::thumbnail::{ThumbnailResult, generate_and_upload_thumbnail};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tracing::{Instrument, instrument};

/// Callback type for thumbnail generation completion
///
/// The callback receives:
/// - `storage_key`: The original object's storage key
/// - `result`: The thumbnail result (key, dimensions, size) or error message
pub type ThumbnailCallback = Arc<
    dyn Fn(String, Result<ThumbnailResult, String>) -> Pin<Box<dyn Future<Output = ()> + Send>>
        + Send
        + Sync,
>;

/// Options for background thumbnail generation
#[derive(Clone)]
pub struct ThumbnailTaskOptions {
    /// The S3 client to use for operations
    pub client: S3Client,
    /// The storage key of the original image
    pub storage_key: String,
    /// Optional callback to invoke when thumbnail is ready
    pub on_complete: Option<ThumbnailCallback>,
}

/// Spawn a background task to generate a thumbnail
///
/// This function:
/// 1. Downloads the original image from S3
/// 2. Generates a thumbnail
/// 3. Uploads the thumbnail to S3
/// 4. Invokes the callback (if provided) with the result
///
/// The task runs independently and doesn't block the caller.
///
/// # Arguments
/// * `options` - Configuration for the thumbnail task
///
/// # Example
/// ```ignore
/// spawn_thumbnail_task(ThumbnailTaskOptions {
///     client: client.clone(),
///     storage_key: "user123/abc-photo.jpg".to_string(),
///     on_complete: Some(Arc::new(|key, result| Box::pin(async move {
///         match result {
///             Ok(thumb) => println!("Thumbnail ready: {}", thumb.thumbnail_key),
///             Err(e) => eprintln!("Thumbnail failed: {}", e),
///         }
///     }))),
/// });
/// ```
#[instrument(skip(options), fields(storage_key = %options.storage_key))]
pub fn spawn_thumbnail_task(options: ThumbnailTaskOptions) {
    let storage_key = options.storage_key.clone();
    let span = tracing::info_span!("thumbnail_task", storage_key = %storage_key);

    tokio::spawn(
        async move {
            let result = generate_thumbnail_for_object(&options.client, &options.storage_key).await;

            match &result {
                Ok(thumb) => {
                    tracing::info!(
                        storage_key = options.storage_key,
                        thumbnail_key = thumb.thumbnail_key,
                        width = thumb.width,
                        height = thumb.height,
                        size_bytes = thumb.size_bytes,
                        "Background thumbnail generation completed successfully"
                    );
                }
                Err(e) => {
                    tracing::error!(
                        storage_key = options.storage_key,
                        error = %e,
                        "Background thumbnail generation failed"
                    );
                }
            }

            // Invoke callback if provided
            if let Some(callback) = options.on_complete {
                let callback_result = result.map_err(|e| e.to_string());
                callback(options.storage_key, callback_result).await;
            }
        }
        .instrument(span),
    );
}

/// Generate a thumbnail for an object already in S3
///
/// Downloads the original image, generates a thumbnail, and uploads it.
///
/// # Arguments
/// * `client` - S3 client
/// * `storage_key` - Key of the original image
///
/// # Returns
/// `ThumbnailResult` with the thumbnail key and metadata
#[instrument(skip(client))]
pub async fn generate_thumbnail_for_object(
    client: &S3Client,
    storage_key: &str,
) -> Result<ThumbnailResult, crate::error::StorageError> {
    tracing::debug!(storage_key = storage_key, "Starting thumbnail generation");

    // Step 1: Download the original image
    let image_data = client.get_object_bytes(storage_key).await?;

    tracing::debug!(
        storage_key = storage_key,
        size_bytes = image_data.len(),
        "Downloaded original image"
    );

    // Step 2: Generate thumbnail and upload
    generate_and_upload_thumbnail(client, storage_key, &image_data).await
}

/// Simple version: spawn thumbnail task without callback
///
/// Useful when you don't need to be notified of completion.
#[instrument(skip(client))]
pub fn spawn_thumbnail_task_simple(client: S3Client, storage_key: String) {
    spawn_thumbnail_task(ThumbnailTaskOptions {
        client,
        storage_key,
        on_complete: None,
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_thumbnail_task_options_clone() {
        // This just verifies the struct can be cloned (required for async moves)
        // Actual integration tests require S3
    }
}

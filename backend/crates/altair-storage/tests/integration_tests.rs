//! Integration tests for the storage service
//!
//! These tests verify the complete upload/download flow with a real S3-compatible backend.
//!
//! ## Running Integration Tests
//!
//! Integration tests require a running Minio instance. Set up via Docker:
//!
//! ```bash
//! docker run -d -p 9000:9000 -p 9001:9001 \
//!   -e MINIO_ROOT_USER=minioadmin \
//!   -e MINIO_ROOT_PASSWORD=minioadmin \
//!   minio/minio server /data --console-address ":9001"
//! ```
//!
//! Then run tests with:
//! ```bash
//! STORAGE_ENDPOINT=http://localhost:9000 \
//! STORAGE_BUCKET=altair-test \
//! S3_ACCESS_KEY_ID=minioadmin \
//! S3_SECRET_ACCESS_KEY=minioadmin \
//! cargo test --package altair-storage --test integration_tests
//! ```
//!
//! Tests are skipped if environment variables are not set.

use altair_storage::{
    MediaType, StorageConfig, StorageService, checksum::calculate_checksum,
    mime::validate_mime_type, thumbnail::generate_thumbnail_bytes,
};
use std::env;

/// Check if integration test environment is configured
fn is_integration_enabled() -> bool {
    env::var("STORAGE_ENDPOINT").is_ok()
        && env::var("S3_ACCESS_KEY_ID").is_ok()
        && env::var("S3_SECRET_ACCESS_KEY").is_ok()
}

/// Skip test if integration environment is not configured
macro_rules! skip_if_no_integration {
    () => {
        if !is_integration_enabled() {
            eprintln!("Skipping integration test: STORAGE_ENDPOINT not configured");
            return;
        }
    };
}

/// Get storage config from environment
fn get_test_config() -> Option<StorageConfig> {
    StorageConfig::from_env().ok()
}

/// Create a small test JPEG image
fn create_test_jpeg(width: u32, height: u32) -> Vec<u8> {
    use image::{DynamicImage, Rgb, RgbImage};
    use std::io::Cursor;

    let mut img = RgbImage::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x * 255 / width.max(1)) as u8;
        let g = (y * 255 / height.max(1)) as u8;
        let b = 128;
        *pixel = Rgb([r, g, b]);
    }
    let dynamic = DynamicImage::ImageRgb8(img);
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    dynamic
        .write_to(&mut cursor, image::ImageFormat::Jpeg)
        .expect("Failed to encode test image");
    buffer
}

/// Create a test PNG image
fn create_test_png(width: u32, height: u32) -> Vec<u8> {
    use image::{DynamicImage, Rgba, RgbaImage};
    use std::io::Cursor;

    let mut img = RgbaImage::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x * 255 / width.max(1)) as u8;
        let g = (y * 255 / height.max(1)) as u8;
        *pixel = Rgba([r, g, 128, 255]);
    }
    let dynamic = DynamicImage::ImageRgba8(img);
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    dynamic
        .write_to(&mut cursor, image::ImageFormat::Png)
        .expect("Failed to encode test image");
    buffer
}

// =============================================================================
// TS-001: Upload small image (<1MB)
// =============================================================================

#[tokio::test]
async fn ts_001_upload_small_image() {
    skip_if_no_integration!();

    let config = get_test_config().expect("Config should be available");
    let service = StorageService::new(&config)
        .await
        .expect("Service should initialize");

    // Create small test image (~100KB)
    let image_data = create_test_jpeg(500, 500);
    assert!(image_data.len() < 1024 * 1024, "Test image should be < 1MB");

    let user_id = format!("test-user-{}", uuid::Uuid::new_v4());
    let filename = "test-photo.jpg";

    // Request upload URL
    let upload_request = service
        .request_upload(&user_id, filename, "image/jpeg", image_data.len() as u64)
        .await
        .expect("Should get upload URL");

    assert!(upload_request.presigned.url.starts_with("http"));
    assert!(upload_request.presigned.key.starts_with(&user_id));
    assert_eq!(upload_request.media_type, MediaType::Photo);

    // Upload the file via presigned URL
    let client = reqwest::Client::new();
    let response = client
        .put(&upload_request.presigned.url)
        .header("Content-Type", "image/jpeg")
        .body(image_data.clone())
        .send()
        .await
        .expect("Upload request should succeed");

    assert!(
        response.status().is_success(),
        "Upload should succeed: {}",
        response.status()
    );

    // Confirm the upload
    let confirmation = service
        .confirm_upload(&upload_request.presigned.key, filename, "image/jpeg")
        .await
        .expect("Confirmation should succeed");

    assert_eq!(confirmation.size_bytes, image_data.len() as u64);
    assert_eq!(confirmation.media_type, MediaType::Photo);
    assert!(confirmation.should_generate_thumbnail);

    // Verify checksum matches
    let expected_checksum = calculate_checksum(&image_data);
    assert_eq!(confirmation.checksum, expected_checksum);

    // Clean up
    service
        .delete_object(&upload_request.presigned.key)
        .await
        .expect("Delete should succeed");
}

// =============================================================================
// TS-002: Upload large file (50MB) - streaming checksum
// =============================================================================

#[tokio::test]
async fn ts_002_upload_large_file() {
    skip_if_no_integration!();

    let config = get_test_config().expect("Config should be available");
    let service = StorageService::new(&config)
        .await
        .expect("Service should initialize");

    // Create 50MB of test data
    let large_data = vec![0xABu8; 50 * 1024 * 1024];
    let user_id = format!("test-user-{}", uuid::Uuid::new_v4());
    let filename = "large-file.bin";

    // Request upload URL (using text/plain as it's an allowed type)
    let upload_request = service
        .request_upload(&user_id, filename, "text/plain", large_data.len() as u64)
        .await
        .expect("Should get upload URL");

    // Upload the file
    let client = reqwest::Client::new();
    let response = client
        .put(&upload_request.presigned.url)
        .header("Content-Type", "text/plain")
        .body(large_data.clone())
        .send()
        .await
        .expect("Upload request should succeed");

    assert!(response.status().is_success());

    // Confirm - this tests streaming checksum for >10MB files
    let confirmation = service
        .confirm_upload(&upload_request.presigned.key, filename, "text/plain")
        .await
        .expect("Confirmation should succeed");

    assert_eq!(confirmation.size_bytes, 50 * 1024 * 1024);

    // Verify checksum (calculated in-memory here for comparison)
    let expected_checksum = calculate_checksum(&large_data);
    assert_eq!(confirmation.checksum, expected_checksum);

    // Clean up
    service
        .delete_object(&upload_request.presigned.key)
        .await
        .expect("Delete should succeed");
}

// =============================================================================
// TS-003: Upload with invalid MIME type
// =============================================================================

#[tokio::test]
async fn ts_003_upload_invalid_mime_type() {
    skip_if_no_integration!();

    let config = get_test_config().expect("Config should be available");
    let service = StorageService::new(&config)
        .await
        .expect("Service should initialize");

    let user_id = "test-user";

    // Try to upload with disallowed MIME type
    let result = service
        .request_upload(user_id, "malware.exe", "application/x-executable", 1000)
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, altair_storage::StorageError::InvalidMimeType { .. }),
        "Should be InvalidMimeType error: {:?}",
        err
    );
}

// =============================================================================
// TS-004: Upload exceeding quota (file size limit)
// =============================================================================

#[tokio::test]
async fn ts_004_upload_exceeds_size_limit() {
    skip_if_no_integration!();

    let config = get_test_config().expect("Config should be available");
    let service = StorageService::new(&config)
        .await
        .expect("Service should initialize");

    let user_id = "test-user";

    // Try to request upload for file larger than 100MB limit
    let result = service
        .request_upload(user_id, "huge.jpg", "image/jpeg", 150 * 1024 * 1024) // 150MB
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, altair_storage::StorageError::QuotaExceeded { .. }),
        "Should be QuotaExceeded error: {:?}",
        err
    );
}

// =============================================================================
// TS-005: Confirm upload for non-existent object
// =============================================================================

#[tokio::test]
async fn ts_005_confirm_nonexistent_object() {
    skip_if_no_integration!();

    let config = get_test_config().expect("Config should be available");
    let service = StorageService::new(&config)
        .await
        .expect("Service should initialize");

    // Try to confirm an object that doesn't exist
    let fake_key = format!("fake-user/{}-nonexistent.jpg", uuid::Uuid::new_v4());
    let result = service
        .confirm_upload(&fake_key, "nonexistent.jpg", "image/jpeg")
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, altair_storage::StorageError::ObjectNotFound { .. }),
        "Should be ObjectNotFound error: {:?}",
        err
    );
}

// =============================================================================
// TS-006: Download with expired URL
// =============================================================================

#[tokio::test]
async fn ts_006_expired_url() {
    skip_if_no_integration!();

    let config = get_test_config().expect("Config should be available");
    let service = StorageService::new(&config)
        .await
        .expect("Service should initialize");

    // First, upload a file
    let image_data = create_test_jpeg(100, 100);
    let user_id = format!("test-user-{}", uuid::Uuid::new_v4());
    let filename = "expiry-test.jpg";

    let upload_request = service
        .request_upload(&user_id, filename, "image/jpeg", image_data.len() as u64)
        .await
        .expect("Should get upload URL");

    let client = reqwest::Client::new();
    client
        .put(&upload_request.presigned.url)
        .header("Content-Type", "image/jpeg")
        .body(image_data)
        .send()
        .await
        .expect("Upload should succeed");

    // Generate a download URL with very short expiration (1 second)
    let download = service
        .get_download_url_with_expiration(&upload_request.presigned.key, 1)
        .await
        .expect("Should get download URL");

    // Wait for URL to expire
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Try to download with expired URL
    let response = client
        .get(&download.url)
        .send()
        .await
        .expect("Request should complete");

    // S3 returns 403 Forbidden for expired URLs
    assert!(
        response.status() == 403 || response.status() == 401,
        "Expired URL should return 403/401, got {}",
        response.status()
    );

    // Clean up
    service
        .delete_object(&upload_request.presigned.key)
        .await
        .expect("Delete should succeed");
}

// =============================================================================
// TS-008: Delete attachment cleanup
// =============================================================================

#[tokio::test]
async fn ts_008_delete_cleanup() {
    skip_if_no_integration!();

    let config = get_test_config().expect("Config should be available");
    let service = StorageService::new(&config)
        .await
        .expect("Service should initialize");

    // Upload a file
    let image_data = create_test_jpeg(200, 200);
    let user_id = format!("test-user-{}", uuid::Uuid::new_v4());
    let filename = "to-delete.jpg";

    let upload_request = service
        .request_upload(&user_id, filename, "image/jpeg", image_data.len() as u64)
        .await
        .expect("Should get upload URL");

    let client = reqwest::Client::new();
    client
        .put(&upload_request.presigned.url)
        .header("Content-Type", "image/jpeg")
        .body(image_data)
        .send()
        .await
        .expect("Upload should succeed");

    // Verify object exists
    let exists = service
        .object_exists(&upload_request.presigned.key)
        .await
        .expect("Exists check should work");
    assert!(exists, "Object should exist after upload");

    // Delete the object
    service
        .delete_object(&upload_request.presigned.key)
        .await
        .expect("Delete should succeed");

    // Verify object is gone
    let exists_after = service
        .object_exists(&upload_request.presigned.key)
        .await
        .expect("Exists check should work");
    assert!(!exists_after, "Object should not exist after delete");
}

// =============================================================================
// TS-009: Thumbnail generation for various formats
// =============================================================================

#[tokio::test]
async fn ts_009_thumbnail_formats() {
    // This test doesn't need S3 - it tests local thumbnail generation

    // Test JPEG thumbnail
    let jpeg_data = create_test_jpeg(1000, 1000);
    let jpeg_thumb = generate_thumbnail_bytes(&jpeg_data).expect("JPEG thumbnail should work");
    assert!(!jpeg_thumb.is_empty());
    assert!(
        jpeg_thumb.len() < jpeg_data.len(),
        "Thumbnail should be smaller"
    );

    // Test PNG thumbnail
    let png_data = create_test_png(1000, 1000);
    let png_thumb = generate_thumbnail_bytes(&png_data).expect("PNG thumbnail should work");
    assert!(!png_thumb.is_empty());
    assert!(
        png_thumb.len() < png_data.len(),
        "Thumbnail should be smaller"
    );

    // Verify thumbnail dimensions
    let thumb_img = image::load_from_memory(&jpeg_thumb).expect("Should decode thumbnail");
    assert!(
        thumb_img.width() <= 256 && thumb_img.height() <= 256,
        "Thumbnail should be max 256x256"
    );
}

// =============================================================================
// Additional unit-style tests that don't need S3
// =============================================================================

#[test]
fn test_mime_validation() {
    // Allowed types
    assert!(validate_mime_type("image/jpeg").is_ok());
    assert!(validate_mime_type("image/png").is_ok());
    assert!(validate_mime_type("image/gif").is_ok());
    assert!(validate_mime_type("image/webp").is_ok());
    assert!(validate_mime_type("application/pdf").is_ok());
    assert!(validate_mime_type("text/plain").is_ok());
    assert!(validate_mime_type("text/markdown").is_ok());
    assert!(validate_mime_type("audio/mpeg").is_ok());

    // Disallowed types
    assert!(validate_mime_type("application/javascript").is_err());
    assert!(validate_mime_type("text/html").is_err());
    assert!(validate_mime_type("application/x-executable").is_err());
}

#[test]
fn test_checksum_known_vectors() {
    // SHA-256 of empty string
    assert_eq!(
        calculate_checksum(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );

    // SHA-256 of "hello"
    assert_eq!(
        calculate_checksum(b"hello"),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );

    // SHA-256 of "Hello, World!"
    assert_eq!(
        calculate_checksum(b"Hello, World!"),
        "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
    );
}

#[test]
fn test_storage_config_validation() {
    // Valid config
    let valid = StorageConfig::new(
        "http://localhost:9000",
        "us-east-1",
        "valid-bucket",
        "access-key",
        "secret-key",
    );
    assert!(valid.is_ok());

    // Invalid endpoint scheme
    let invalid_scheme = StorageConfig::new(
        "ftp://localhost:9000",
        "us-east-1",
        "bucket",
        "key",
        "secret",
    );
    assert!(invalid_scheme.is_err());

    // Invalid bucket name
    let invalid_bucket = StorageConfig::new(
        "http://localhost:9000",
        "us-east-1",
        "Invalid_Bucket",
        "key",
        "secret",
    );
    assert!(invalid_bucket.is_err());
}

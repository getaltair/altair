//! Thumbnail generation module
//!
//! This module handles generating thumbnails for image files. Thumbnails are:
//! - Max 256×256 pixels (preserving aspect ratio)
//! - Encoded as JPEG at 80% quality
//! - Uploaded to S3 with `_thumb` suffix in the key
//!
//! Supported input formats: JPEG, PNG, GIF, WebP

use crate::client::S3Client;
use crate::error::{StorageError, StorageResult};
use crate::presigned::generate_thumbnail_key;
use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, ExtendedColorType, ImageFormat, ImageReader};
use std::io::Cursor;
use tracing::instrument;

/// Maximum dimension for thumbnails (width or height)
pub const THUMBNAIL_MAX_DIMENSION: u32 = 256;

/// JPEG quality for thumbnails (0-100)
pub const THUMBNAIL_JPEG_QUALITY: u8 = 80;

/// Result of thumbnail generation
#[derive(Debug, Clone)]
pub struct ThumbnailResult {
    /// The S3 key where the thumbnail was uploaded
    pub thumbnail_key: String,
    /// Width of the thumbnail in pixels
    pub width: u32,
    /// Height of the thumbnail in pixels
    pub height: u32,
    /// Size of the thumbnail in bytes
    pub size_bytes: usize,
}

/// Generate a thumbnail from image bytes
///
/// # Arguments
/// * `image_data` - Raw image bytes (JPEG, PNG, GIF, or WebP)
///
/// # Returns
/// The thumbnail as JPEG bytes
///
/// # Errors
/// - `StorageError::ImageError` if the image cannot be decoded or processed
#[instrument(skip(image_data), fields(data_len = image_data.len()))]
pub fn generate_thumbnail_bytes(image_data: &[u8]) -> StorageResult<Vec<u8>> {
    // Step 1: Decode the image
    let img = decode_image(image_data)?;

    // Step 2: Resize to thumbnail dimensions
    let thumbnail = resize_to_thumbnail(img);

    // Step 3: Encode as JPEG at 80% quality
    let jpeg_bytes = encode_as_jpeg(&thumbnail)?;

    tracing::debug!(
        width = thumbnail.width(),
        height = thumbnail.height(),
        size_bytes = jpeg_bytes.len(),
        "Generated thumbnail"
    );

    Ok(jpeg_bytes)
}

/// Generate a thumbnail and upload it to S3
///
/// # Arguments
/// * `client` - S3 client for uploading
/// * `storage_key` - Original object key (thumbnail key will be derived)
/// * `image_data` - Raw image bytes
///
/// # Returns
/// `ThumbnailResult` with the thumbnail key and metadata
#[instrument(skip(client, image_data), fields(storage_key = storage_key))]
pub async fn generate_and_upload_thumbnail(
    client: &S3Client,
    storage_key: &str,
    image_data: &[u8],
) -> StorageResult<ThumbnailResult> {
    // Generate thumbnail bytes
    let thumbnail_bytes = generate_thumbnail_bytes(image_data)?;

    // Derive thumbnail key
    let thumbnail_key = generate_thumbnail_key(storage_key);

    // Get dimensions from the thumbnail
    let thumbnail_img = decode_image(&thumbnail_bytes)?;
    let width = thumbnail_img.width();
    let height = thumbnail_img.height();
    let size_bytes = thumbnail_bytes.len();

    // Upload to S3
    client
        .put_object(&thumbnail_key, thumbnail_bytes, "image/jpeg")
        .await?;

    tracing::info!(
        storage_key = storage_key,
        thumbnail_key = thumbnail_key,
        width = width,
        height = height,
        size_bytes = size_bytes,
        "Uploaded thumbnail to S3"
    );

    Ok(ThumbnailResult {
        thumbnail_key,
        width,
        height,
        size_bytes,
    })
}

/// Decode image bytes into a DynamicImage
///
/// Supports JPEG, PNG, GIF, and WebP formats.
fn decode_image(data: &[u8]) -> StorageResult<DynamicImage> {
    let cursor = Cursor::new(data);

    let reader = ImageReader::new(cursor)
        .with_guessed_format()
        .map_err(|e| StorageError::ImageError(format!("Failed to guess image format: {}", e)))?;

    let format = reader.format();
    tracing::debug!(format = ?format, "Detected image format");

    // Validate it's a supported format
    match format {
        Some(ImageFormat::Jpeg | ImageFormat::Png | ImageFormat::Gif | ImageFormat::WebP) => {}
        Some(other) => {
            return Err(StorageError::ImageError(format!(
                "Unsupported image format: {:?}",
                other
            )));
        }
        None => {
            return Err(StorageError::ImageError(
                "Could not determine image format".to_string(),
            ));
        }
    }

    reader
        .decode()
        .map_err(|e| StorageError::ImageError(format!("Failed to decode image: {}", e)))
}

/// Resize an image to thumbnail dimensions
///
/// Resizes to fit within THUMBNAIL_MAX_DIMENSION × THUMBNAIL_MAX_DIMENSION
/// while preserving the aspect ratio.
fn resize_to_thumbnail(img: DynamicImage) -> DynamicImage {
    let (orig_width, orig_height) = (img.width(), img.height());

    // If already smaller than max dimension, no resize needed
    if orig_width <= THUMBNAIL_MAX_DIMENSION && orig_height <= THUMBNAIL_MAX_DIMENSION {
        tracing::debug!(
            orig_width = orig_width,
            orig_height = orig_height,
            "Image already within thumbnail size, no resize needed"
        );
        return img;
    }

    // Calculate new dimensions preserving aspect ratio
    let (new_width, new_height) = calculate_thumbnail_dimensions(orig_width, orig_height);

    tracing::debug!(
        orig_width = orig_width,
        orig_height = orig_height,
        new_width = new_width,
        new_height = new_height,
        "Resizing image to thumbnail dimensions"
    );

    // Resize using Lanczos3 filter for high quality
    img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
}

/// Calculate thumbnail dimensions preserving aspect ratio
///
/// Returns (width, height) that fit within THUMBNAIL_MAX_DIMENSION
fn calculate_thumbnail_dimensions(orig_width: u32, orig_height: u32) -> (u32, u32) {
    if orig_width > orig_height {
        // Landscape orientation: width is the limiting factor
        let ratio = orig_height as f32 / orig_width as f32;
        let new_height = (THUMBNAIL_MAX_DIMENSION as f32 * ratio).round() as u32;
        (THUMBNAIL_MAX_DIMENSION, new_height.max(1))
    } else {
        // Portrait or square: height is the limiting factor
        let ratio = orig_width as f32 / orig_height as f32;
        let new_width = (THUMBNAIL_MAX_DIMENSION as f32 * ratio).round() as u32;
        (new_width.max(1), THUMBNAIL_MAX_DIMENSION)
    }
}

/// Encode a DynamicImage as JPEG at the specified quality
fn encode_as_jpeg(img: &DynamicImage) -> StorageResult<Vec<u8>> {
    // Convert to RGB8 for JPEG encoding
    let rgb_img = img.to_rgb8();

    let mut buffer = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, THUMBNAIL_JPEG_QUALITY);

    encoder
        .encode(
            rgb_img.as_raw(),
            img.width(),
            img.height(),
            ExtendedColorType::Rgb8,
        )
        .map_err(|e| StorageError::ImageError(format!("Failed to encode JPEG: {}", e)))?;

    Ok(buffer)
}

/// Check if a MIME type supports thumbnail generation
pub fn supports_thumbnail(mime_type: &str) -> bool {
    matches!(
        mime_type.to_lowercase().as_str(),
        "image/jpeg" | "image/png" | "image/gif" | "image/webp"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a test image with specific dimensions
    fn create_test_image(width: u32, height: u32) -> DynamicImage {
        use image::{Rgb, RgbImage};

        let mut img = RgbImage::new(width, height);

        // Create a simple gradient pattern for testing
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let r = (x * 255 / width.max(1)) as u8;
            let g = (y * 255 / height.max(1)) as u8;
            let b = 128;
            *pixel = Rgb([r, g, b]);
        }

        DynamicImage::ImageRgb8(img)
    }

    /// Helper to encode a DynamicImage as JPEG bytes for testing
    fn to_jpeg_bytes(img: &DynamicImage) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        img.write_to(&mut cursor, ImageFormat::Jpeg)
            .expect("Failed to encode test image");
        buffer
    }

    #[test]
    fn test_calculate_thumbnail_dimensions_landscape() {
        // 3000×2000 should become 256×171
        let (width, height) = calculate_thumbnail_dimensions(3000, 2000);
        assert_eq!(width, 256);
        assert_eq!(height, 171);
    }

    #[test]
    fn test_calculate_thumbnail_dimensions_portrait() {
        // 2000×3000 should become 171×256
        let (width, height) = calculate_thumbnail_dimensions(2000, 3000);
        assert_eq!(width, 171);
        assert_eq!(height, 256);
    }

    #[test]
    fn test_calculate_thumbnail_dimensions_square() {
        // 1000×1000 should become 256×256
        let (width, height) = calculate_thumbnail_dimensions(1000, 1000);
        assert_eq!(width, 256);
        assert_eq!(height, 256);
    }

    #[test]
    fn test_calculate_thumbnail_dimensions_small_image() {
        // 100×100 should stay 100×100 (handled by resize_to_thumbnail)
        // But calculate_thumbnail_dimensions always calculates scaled dimensions
        let (width, height) = calculate_thumbnail_dimensions(100, 100);
        // For a 100×100 image where 100 < 256, the calculation still produces scaled values
        // but resize_to_thumbnail will return early
        assert!(width <= 256);
        assert!(height <= 256);
    }

    #[test]
    fn test_resize_to_thumbnail_preserves_aspect_ratio() {
        let img = create_test_image(3000, 2000);
        let thumbnail = resize_to_thumbnail(img);

        assert_eq!(thumbnail.width(), 256);
        assert_eq!(thumbnail.height(), 171);

        // Verify aspect ratio is preserved (within rounding error)
        let original_ratio = 3000.0 / 2000.0;
        let thumbnail_ratio = thumbnail.width() as f32 / thumbnail.height() as f32;
        assert!((original_ratio - thumbnail_ratio).abs() < 0.01);
    }

    #[test]
    fn test_resize_to_thumbnail_small_image_unchanged() {
        let img = create_test_image(100, 100);
        let thumbnail = resize_to_thumbnail(img);

        // Small images should not be resized
        assert_eq!(thumbnail.width(), 100);
        assert_eq!(thumbnail.height(), 100);
    }

    #[test]
    fn test_generate_thumbnail_bytes() {
        let img = create_test_image(1000, 1000);
        let jpeg_input = to_jpeg_bytes(&img);

        let thumbnail_bytes =
            generate_thumbnail_bytes(&jpeg_input).expect("Should generate thumbnail");

        // Verify output is valid JPEG
        let decoded = decode_image(&thumbnail_bytes).expect("Should decode thumbnail");
        assert_eq!(decoded.width(), 256);
        assert_eq!(decoded.height(), 256);

        // Verify reasonable size
        assert!(
            thumbnail_bytes.len() < 50 * 1024,
            "Thumbnail should be < 50KB"
        );
    }

    #[test]
    fn test_decode_image_jpeg() {
        let img = create_test_image(100, 100);
        let jpeg_bytes = to_jpeg_bytes(&img);

        let decoded = decode_image(&jpeg_bytes);
        assert!(decoded.is_ok());
        assert_eq!(decoded.unwrap().width(), 100);
    }

    #[test]
    fn test_decode_image_invalid() {
        let invalid_data = b"not an image";
        let result = decode_image(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_as_jpeg() {
        let img = create_test_image(100, 100);
        let jpeg_bytes = encode_as_jpeg(&img).expect("Should encode as JPEG");

        assert!(!jpeg_bytes.is_empty());
        // Verify it's valid JPEG by decoding
        let decoded = decode_image(&jpeg_bytes).expect("Should decode encoded JPEG");
        assert_eq!(decoded.width(), 100);
        assert_eq!(decoded.height(), 100);
    }

    #[test]
    fn test_supports_thumbnail() {
        // Supported formats
        assert!(supports_thumbnail("image/jpeg"));
        assert!(supports_thumbnail("image/png"));
        assert!(supports_thumbnail("image/gif"));
        assert!(supports_thumbnail("image/webp"));
        assert!(supports_thumbnail("IMAGE/JPEG")); // Case insensitive

        // Unsupported formats
        assert!(!supports_thumbnail("application/pdf"));
        assert!(!supports_thumbnail("audio/mpeg"));
        assert!(!supports_thumbnail("text/plain"));
        assert!(!supports_thumbnail("video/mp4")); // Video not supported yet
    }

    #[test]
    fn test_thumbnail_result_struct() {
        let result = ThumbnailResult {
            thumbnail_key: "user123/abc-photo_thumb.jpg".to_string(),
            width: 256,
            height: 171,
            size_bytes: 15000,
        };

        assert!(result.thumbnail_key.ends_with("_thumb.jpg"));
        assert_eq!(result.width, 256);
        assert!(result.size_bytes > 0);
    }
}

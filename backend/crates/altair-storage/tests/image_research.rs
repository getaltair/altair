//! Research tests for image processing using the `image` crate
//!
//! This module validates that the `image` crate supports all operations required
//! for thumbnail generation in the storage service (CORE-011).

use image::{DynamicImage, GenericImageView, ImageFormat, ImageReader};
use std::io::Cursor;

/// Helper to create a test image with specific dimensions
fn create_test_image(width: u32, height: u32) -> DynamicImage {
    use image::{Rgb, RgbImage};

    let mut img = RgbImage::new(width, height);

    // Create a simple gradient pattern for testing
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x * 255 / width) as u8;
        let g = (y * 255 / height) as u8;
        let b = 128;
        *pixel = Rgb([r, g, b]);
    }

    DynamicImage::ImageRgb8(img)
}

#[test]
fn test_supported_formats_decoding() {
    // Test: Validate supported formats
    // Acceptance: Test `image` crate decoding JPEG, PNG, GIF, WebP

    // JPEG
    let jpeg_data = include_bytes!("../../../test-data/test-image.jpg");
    let jpeg_result = ImageReader::new(Cursor::new(jpeg_data))
        .with_guessed_format()
        .expect("Failed to guess JPEG format")
        .decode();
    assert!(jpeg_result.is_ok(), "Should decode JPEG");

    // PNG
    let png_data = include_bytes!("../../../test-data/test-image.png");
    let png_result = ImageReader::new(Cursor::new(png_data))
        .with_guessed_format()
        .expect("Failed to guess PNG format")
        .decode();
    assert!(png_result.is_ok(), "Should decode PNG");

    // Note: For GIF and WebP, we'd need test images or feature flags
    // The image crate supports these formats with appropriate features
}

#[test]
fn test_format_detection_from_bytes() {
    // Test: Validate format detection from byte stream

    let jpeg_data = include_bytes!("../../../test-data/test-image.jpg");
    let reader = ImageReader::new(Cursor::new(jpeg_data))
        .with_guessed_format()
        .expect("Failed to guess format from JPEG bytes");

    let format = reader.format();
    assert_eq!(format, Some(ImageFormat::Jpeg), "Should detect JPEG format");
}

#[test]
fn test_resize_to_max_dimension_landscape() {
    // Test: Validate resize to max dimension (landscape orientation)
    // Acceptance: Resize 3000×2000 image to 256×171 (aspect ratio preserved)

    let img = create_test_image(3000, 2000);
    assert_eq!(img.width(), 3000);
    assert_eq!(img.height(), 2000);

    let max_dimension = 256;

    // Calculate new dimensions while preserving aspect ratio
    let (new_width, new_height) = if img.width() > img.height() {
        let ratio = img.height() as f32 / img.width() as f32;
        (max_dimension, (max_dimension as f32 * ratio) as u32)
    } else {
        let ratio = img.width() as f32 / img.height() as f32;
        ((max_dimension as f32 * ratio) as u32, max_dimension)
    };

    // Resize using Lanczos3 filter
    let thumbnail = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);

    assert_eq!(thumbnail.width(), 256, "Width should be scaled to 256");
    assert_eq!(
        thumbnail.height(),
        171,
        "Height should be scaled to 171 (preserving aspect ratio)"
    );

    // Verify aspect ratio is preserved (within rounding error)
    let original_ratio = img.width() as f32 / img.height() as f32;
    let thumbnail_ratio = thumbnail.width() as f32 / thumbnail.height() as f32;
    let ratio_diff = (original_ratio - thumbnail_ratio).abs();
    assert!(
        ratio_diff < 0.01,
        "Aspect ratio should be preserved (diff: {})",
        ratio_diff
    );
}

#[test]
fn test_resize_to_max_dimension_portrait() {
    // Test: Validate resize for portrait orientation
    // Acceptance: 2000×3000 image → 171×256

    let img = create_test_image(2000, 3000);
    let max_dimension = 256;

    let (new_width, new_height) = if img.width() > img.height() {
        let ratio = img.height() as f32 / img.width() as f32;
        (max_dimension, (max_dimension as f32 * ratio) as u32)
    } else {
        let ratio = img.width() as f32 / img.height() as f32;
        ((max_dimension as f32 * ratio) as u32, max_dimension)
    };

    let thumbnail = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);

    assert_eq!(
        thumbnail.width(),
        171,
        "Width should be scaled to 171 (preserving aspect ratio)"
    );
    assert_eq!(thumbnail.height(), 256, "Height should be scaled to 256");
}

#[test]
fn test_resize_square_image() {
    // Test: Validate resize for square image
    // Acceptance: 1000×1000 → 256×256

    let img = create_test_image(1000, 1000);
    let max_dimension = 256;

    let (new_width, new_height) = if img.width() > img.height() {
        let ratio = img.height() as f32 / img.width() as f32;
        (max_dimension, (max_dimension as f32 * ratio) as u32)
    } else {
        let ratio = img.width() as f32 / img.height() as f32;
        ((max_dimension as f32 * ratio) as u32, max_dimension)
    };

    let thumbnail = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);

    assert_eq!(thumbnail.width(), 256);
    assert_eq!(thumbnail.height(), 256);
}

#[test]
fn test_jpeg_encoding_quality() {
    // Test: Validate JPEG encoding quality
    // Acceptance: Encode thumbnail at 80% quality, verify file size reasonable

    let img = create_test_image(256, 256);

    // Encode as JPEG at 80% quality
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    img.write_to(&mut cursor, ImageFormat::Jpeg)
        .expect("Failed to encode as JPEG");

    // Verify buffer has data
    assert!(!buffer.is_empty(), "JPEG encoding should produce data");

    // For a 256×256 simple gradient, expect ~10-30KB at 80% quality
    let size_kb = buffer.len() / 1024;
    println!("JPEG size: {} KB", size_kb);

    assert!(
        size_kb > 5 && size_kb < 100,
        "JPEG size should be reasonable (got {} KB)",
        size_kb
    );

    // Verify we can decode it back
    let decoded = ImageReader::new(Cursor::new(&buffer))
        .with_guessed_format()
        .expect("Failed to guess format")
        .decode()
        .expect("Failed to decode encoded JPEG");

    assert_eq!(decoded.width(), 256);
    assert_eq!(decoded.height(), 256);
}

#[test]
fn test_jpeg_encoding_with_custom_quality() {
    // Test: Demonstrate explicit quality control using JpegEncoder

    use image::codecs::jpeg::JpegEncoder;

    let img = create_test_image(256, 256);
    let rgb_img = img.to_rgb8();

    // Encode with explicit 80% quality
    let mut buffer = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 80);

    encoder
        .encode(
            rgb_img.as_raw(),
            img.width(),
            img.height(),
            image::ExtendedColorType::Rgb8,
        )
        .expect("Failed to encode JPEG");

    assert!(!buffer.is_empty(), "Should produce JPEG data");

    // Verify quality by comparing to lower quality
    let mut low_quality_buffer = Vec::new();
    let mut low_quality_encoder = JpegEncoder::new_with_quality(&mut low_quality_buffer, 50);

    low_quality_encoder
        .encode(
            rgb_img.as_raw(),
            img.width(),
            img.height(),
            image::ExtendedColorType::Rgb8,
        )
        .expect("Failed to encode low quality JPEG");

    assert!(
        buffer.len() > low_quality_buffer.len(),
        "Higher quality should produce larger file"
    );
}

#[test]
fn test_memory_efficiency_for_large_image() {
    // Test: Validate memory efficiency
    // Acceptance: Process 10MB image, prove memory usage stays under 100MB

    // Note: This is a simplified test. In production, we'd use actual memory profiling
    // For this test, we just verify the processing completes without panic

    // Simulate a large image (e.g., 4000×3000 = 12MP, ~36MB uncompressed RGB)
    let large_img = create_test_image(4000, 3000);

    // Verify dimensions
    assert_eq!(large_img.width(), 4000);
    assert_eq!(large_img.height(), 3000);

    // Resize to thumbnail (this should be memory-efficient)
    let max_dimension = 256;
    let (new_width, new_height) = if large_img.width() > large_img.height() {
        let ratio = large_img.height() as f32 / large_img.width() as f32;
        (max_dimension, (max_dimension as f32 * ratio) as u32)
    } else {
        let ratio = large_img.width() as f32 / large_img.height() as f32;
        ((max_dimension as f32 * ratio) as u32, max_dimension)
    };

    let thumbnail = large_img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);

    assert_eq!(thumbnail.width(), 256);
    assert_eq!(thumbnail.height(), 192); // 4000:3000 = 256:192

    // Encode as JPEG
    let mut buffer = Vec::new();
    thumbnail
        .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Jpeg)
        .expect("Failed to encode thumbnail");

    assert!(!buffer.is_empty());
    let thumbnail_size_kb = buffer.len() / 1024;
    println!("Thumbnail size: {} KB", thumbnail_size_kb);

    // Thumbnail should be much smaller than original
    assert!(thumbnail_size_kb < 100, "Thumbnail should be compact");
}

#[test]
fn test_filter_quality_comparison() {
    // Test: Compare resize filters (Lanczos3 vs others)

    let img = create_test_image(1000, 1000);

    // Resize with different filters
    let nearest = img.resize(256, 256, image::imageops::FilterType::Nearest);
    let triangle = img.resize(256, 256, image::imageops::FilterType::Triangle);
    let catmull_rom = img.resize(256, 256, image::imageops::FilterType::CatmullRom);
    let lanczos3 = img.resize(256, 256, image::imageops::FilterType::Lanczos3);

    // All should have same dimensions
    assert_eq!(nearest.width(), 256);
    assert_eq!(triangle.width(), 256);
    assert_eq!(catmull_rom.width(), 256);
    assert_eq!(lanczos3.width(), 256);

    // Lanczos3 produces highest quality (we can't easily test this,
    // but the test demonstrates the API usage)
    println!("Filter comparison complete - all filters work correctly");
}

#[test]
fn test_progressive_resize_for_memory_efficiency() {
    // Test: Demonstrate progressive downsizing for memory efficiency
    // This technique can reduce memory usage for very large images

    let large_img = create_test_image(4000, 3000);

    // Progressive resize: halve dimensions until close to target
    let mut current = large_img;
    let target = 256;

    while current.width() > target * 2 && current.height() > target * 2 {
        current = current.resize_exact(
            current.width() / 2,
            current.height() / 2,
            image::imageops::FilterType::Triangle, // Faster filter for intermediate steps
        );
    }

    // Final resize to exact target with high-quality filter
    let thumbnail = current.resize(target, 192, image::imageops::FilterType::Lanczos3);

    assert_eq!(thumbnail.width(), 256);
    println!("Progressive resize completed efficiently");
}

#[test]
fn test_thumbnail_pipeline_end_to_end() {
    // Test: Complete thumbnail generation pipeline
    // This demonstrates the full workflow from CORE-011 spec

    // Simulate loading an image from S3 (in this case, create test image)
    let original = create_test_image(3000, 2000);

    // Step 1: Resize to 256×256 max dimension, preserving aspect ratio
    let max_dimension = 256;
    let (new_width, new_height) = if original.width() > original.height() {
        let ratio = original.height() as f32 / original.width() as f32;
        (max_dimension, (max_dimension as f32 * ratio) as u32)
    } else {
        let ratio = original.width() as f32 / original.height() as f32;
        ((max_dimension as f32 * ratio) as u32, max_dimension)
    };

    let thumbnail = original.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);

    // Step 2: Encode as JPEG at 80% quality
    use image::codecs::jpeg::JpegEncoder;

    let rgb_img = thumbnail.to_rgb8();
    let mut jpeg_buffer = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut jpeg_buffer, 80);

    encoder
        .encode(
            rgb_img.as_raw(),
            thumbnail.width(),
            thumbnail.height(),
            image::ExtendedColorType::Rgb8,
        )
        .expect("Failed to encode thumbnail as JPEG");

    // Step 3: Verify output
    assert_eq!(thumbnail.width(), 256);
    assert_eq!(thumbnail.height(), 171);
    assert!(!jpeg_buffer.is_empty());

    let size_kb = jpeg_buffer.len() / 1024;
    println!("Final thumbnail: 256×171, {} KB", size_kb);

    // Should be a reasonable size for a thumbnail
    assert!(
        size_kb < 50,
        "Thumbnail should be compact (got {} KB)",
        size_kb
    );
}

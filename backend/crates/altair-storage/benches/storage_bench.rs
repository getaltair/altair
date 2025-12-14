//! Storage service performance benchmarks
//!
//! Benchmarks for:
//! - Presigned URL generation latency (target: <50ms avg for 100 iterations)
//! - Upload confirmation latency for 1MB file (target: <500ms avg)
//! - Thumbnail generation latency for 10MB image (target: <2s avg)

use criterion::{Criterion, criterion_group, criterion_main};

/// Benchmark presigned URL generation
///
/// Measures the time to generate presigned upload and download URLs.
/// Target: Average of 100 iterations <50ms
fn presigned_url_benchmark(c: &mut Criterion) {
    use altair_storage::presigned::{generate_object_key, generate_thumbnail_key};

    c.bench_function("generate_object_key", |b| {
        b.iter(|| {
            let key = generate_object_key("user123", "test-photo.jpg");
            assert!(key.starts_with("user123/"));
            assert!(key.ends_with("-test-photo.jpg"));
        });
    });

    c.bench_function("generate_thumbnail_key", |b| {
        b.iter(|| {
            let thumb_key = generate_thumbnail_key("user123/abc-photo.jpg");
            assert!(thumb_key.ends_with("_thumb.jpg"));
        });
    });
}

/// Benchmark checksum calculation
///
/// Measures in-memory SHA-256 checksum calculation.
fn checksum_benchmark(c: &mut Criterion) {
    use altair_storage::checksum::calculate_checksum;

    // 1KB data
    let data_1kb = vec![0xABu8; 1024];

    // 1MB data
    let data_1mb = vec![0xABu8; 1024 * 1024];

    // 10MB data
    let data_10mb = vec![0xABu8; 10 * 1024 * 1024];

    c.bench_function("checksum_1kb", |b| {
        b.iter(|| {
            let checksum = calculate_checksum(&data_1kb);
            assert_eq!(checksum.len(), 64);
        });
    });

    c.bench_function("checksum_1mb", |b| {
        b.iter(|| {
            let checksum = calculate_checksum(&data_1mb);
            assert_eq!(checksum.len(), 64);
        });
    });

    c.bench_function("checksum_10mb", |b| {
        b.iter(|| {
            let checksum = calculate_checksum(&data_10mb);
            assert_eq!(checksum.len(), 64);
        });
    });
}

/// Benchmark MIME type validation
fn mime_validation_benchmark(c: &mut Criterion) {
    use altair_storage::mime::{classify_media_type, validate_mime_type};

    c.bench_function("validate_mime_type_allowed", |b| {
        b.iter(|| {
            assert!(validate_mime_type("image/jpeg").is_ok());
        });
    });

    c.bench_function("validate_mime_type_disallowed", |b| {
        b.iter(|| {
            assert!(validate_mime_type("application/javascript").is_err());
        });
    });

    c.bench_function("classify_media_type", |b| {
        b.iter(|| {
            use altair_storage::MediaType;
            let mt = classify_media_type("image/jpeg");
            assert_eq!(mt, MediaType::Photo);
        });
    });
}

/// Benchmark thumbnail generation
///
/// Measures time to generate thumbnail from various sized images.
/// Target: <2s for 10MB image
fn thumbnail_benchmark(c: &mut Criterion) {
    use altair_storage::thumbnail::generate_thumbnail_bytes;
    use image::{DynamicImage, Rgb, RgbImage};
    use std::io::Cursor;

    // Helper to create a test image
    fn create_test_image(width: u32, height: u32) -> Vec<u8> {
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

    // 1000x1000 image (~small)
    let img_1k = create_test_image(1000, 1000);

    // 3000x2000 image (~typical photo)
    let img_3k = create_test_image(3000, 2000);

    c.bench_function("thumbnail_1000x1000", |b| {
        b.iter(|| {
            let thumb = generate_thumbnail_bytes(&img_1k).expect("Should generate thumbnail");
            assert!(!thumb.is_empty());
        });
    });

    c.bench_function("thumbnail_3000x2000", |b| {
        b.iter(|| {
            let thumb = generate_thumbnail_bytes(&img_3k).expect("Should generate thumbnail");
            assert!(!thumb.is_empty());
        });
    });
}

/// Benchmark quota calculation
fn quota_benchmark(c: &mut Criterion) {
    use altair_storage::quota::QuotaInfo;

    c.bench_function("quota_info_new", |b| {
        b.iter(|| {
            let info = QuotaInfo::new(1_000_000_000, 5_000_000_000, None);
            assert!(!info.would_exceed(1_000_000));
        });
    });

    c.bench_function("quota_would_exceed_check", |b| {
        let info = QuotaInfo::new(4_900_000_000, 5_000_000_000, None);
        b.iter(|| {
            assert!(info.would_exceed(200_000_000));
            assert!(!info.would_exceed(50_000_000));
        });
    });
}

/// Benchmark config validation
fn config_validation_benchmark(c: &mut Criterion) {
    use altair_storage::StorageConfig;

    c.bench_function("storage_config_new_valid", |b| {
        b.iter(|| {
            let config = StorageConfig::new(
                "http://localhost:9000",
                "us-east-1",
                "test-bucket",
                "access-key",
                "secret-key",
            );
            assert!(config.is_ok());
        });
    });

    c.bench_function("storage_config_new_invalid_bucket", |b| {
        b.iter(|| {
            let config = StorageConfig::new(
                "http://localhost:9000",
                "us-east-1",
                "Invalid_Bucket",
                "access-key",
                "secret-key",
            );
            assert!(config.is_err());
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(10));
    targets =
        presigned_url_benchmark,
        checksum_benchmark,
        mime_validation_benchmark,
        thumbnail_benchmark,
        quota_benchmark,
        config_validation_benchmark
}

criterion_main!(benches);

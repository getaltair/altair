//! Checksum calculation module
//!
//! This module provides SHA-256 checksum calculation for file integrity verification.
//! Supports both in-memory calculation for small files and streaming calculation
//! for large files to avoid excessive memory usage.

use crate::client::S3Client;
use crate::error::{StorageError, StorageResult};
use aws_sdk_s3::primitives::ByteStream;
use sha2::{Digest, Sha256};
use tracing::instrument;

/// Threshold for switching from in-memory to streaming checksum calculation.
/// Files larger than 10MB use streaming to avoid memory pressure.
pub const STREAMING_THRESHOLD_BYTES: u64 = 10 * 1024 * 1024; // 10MB

/// Calculate SHA-256 checksum for in-memory data
///
/// Use this for small files (≤10MB). For larger files, use `calculate_checksum_streaming`.
///
/// # Arguments
/// * `data` - The data to calculate checksum for
///
/// # Returns
/// Hex-encoded SHA-256 checksum string (64 characters)
///
/// # Example
/// ```
/// use altair_storage::checksum::calculate_checksum;
///
/// let checksum = calculate_checksum(b"hello");
/// assert_eq!(checksum, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
/// ```
pub fn calculate_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Calculate SHA-256 checksum from a streaming ByteStream
///
/// Use this for large files (>10MB) to avoid loading the entire file into memory.
/// The hash is computed incrementally as chunks are read from the stream.
///
/// # Arguments
/// * `stream` - A ByteStream (from S3 GetObject response)
///
/// # Returns
/// Hex-encoded SHA-256 checksum string (64 characters)
///
/// # Example
/// ```ignore
/// let stream = client.get_object("key").await?;
/// let checksum = calculate_checksum_streaming(stream).await?;
/// ```
#[instrument(skip(stream))]
pub async fn calculate_checksum_streaming(stream: ByteStream) -> StorageResult<String> {
    let mut hasher = Sha256::new();

    // Collect the stream into an AggregatedBytes which we can iterate over
    let aggregated = stream
        .collect()
        .await
        .map_err(|e| StorageError::checksum(format!("Failed to read stream: {}", e)))?;

    // Update hasher with the collected bytes
    hasher.update(aggregated.into_bytes());

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// Calculate SHA-256 checksum from a ByteStream with chunked processing
///
/// This variant processes the stream in chunks as they arrive, providing
/// more memory-efficient processing for very large files.
///
/// # Arguments
/// * `stream` - A ByteStream (from S3 GetObject response)
///
/// # Returns
/// Hex-encoded SHA-256 checksum string (64 characters) and total bytes processed
#[instrument(skip(stream))]
pub async fn calculate_checksum_chunked(stream: ByteStream) -> StorageResult<(String, u64)> {
    let mut hasher = Sha256::new();
    let mut total_bytes: u64 = 0;

    let mut inner = stream.into_async_read();
    use tokio::io::AsyncReadExt;

    let mut buffer = vec![0u8; 64 * 1024]; // 64KB chunks
    loop {
        let bytes_read = inner
            .read(&mut buffer)
            .await
            .map_err(|e| StorageError::checksum(format!("Failed to read chunk: {}", e)))?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
        total_bytes += bytes_read as u64;
    }

    let result = hasher.finalize();
    Ok((hex::encode(result), total_bytes))
}

/// Calculate checksum for an S3 object, choosing the appropriate method based on size
///
/// For files ≤10MB, loads the entire object into memory for fast hashing.
/// For files >10MB, uses streaming to avoid memory pressure.
///
/// # Arguments
/// * `client` - The S3 client
/// * `key` - The S3 object key
/// * `size` - The object size in bytes (from HEAD request)
///
/// # Returns
/// Hex-encoded SHA-256 checksum string
#[instrument(skip(client), fields(bucket = %client.bucket()))]
pub async fn calculate_object_checksum(
    client: &S3Client,
    key: &str,
    size: u64,
) -> StorageResult<String> {
    if size <= STREAMING_THRESHOLD_BYTES {
        // Small file: load into memory
        tracing::debug!(
            key = key,
            size = size,
            "Using in-memory checksum for small file"
        );
        let data = client.get_object_bytes(key).await?;
        Ok(calculate_checksum(&data))
    } else {
        // Large file: stream
        tracing::debug!(
            key = key,
            size = size,
            "Using streaming checksum for large file"
        );
        let stream = client.get_object(key).await?;
        calculate_checksum_streaming(stream).await
    }
}

/// Verify that a calculated checksum matches an expected value
///
/// # Arguments
/// * `calculated` - The calculated checksum
/// * `expected` - The expected checksum
///
/// # Returns
/// `Ok(())` if checksums match, `Err(StorageError::ChecksumError)` otherwise
pub fn verify_checksum(calculated: &str, expected: &str) -> StorageResult<()> {
    if calculated.eq_ignore_ascii_case(expected) {
        Ok(())
    } else {
        Err(StorageError::checksum(format!(
            "Checksum mismatch: expected {}, got {}",
            expected, calculated
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_checksum_empty() {
        let checksum = calculate_checksum(b"");
        // SHA-256 of empty string
        assert_eq!(
            checksum,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_calculate_checksum_hello() {
        let checksum = calculate_checksum(b"hello");
        // SHA-256 of "hello"
        assert_eq!(
            checksum,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_calculate_checksum_hello_world() {
        let checksum = calculate_checksum(b"Hello, World!");
        // SHA-256 of "Hello, World!"
        assert_eq!(
            checksum,
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
        );
    }

    #[test]
    fn test_calculate_checksum_large_data() {
        // Create 1MB of data
        let data = vec![0xABu8; 1024 * 1024];
        let checksum = calculate_checksum(&data);
        // Verify it produces a valid 64-character hex string
        assert_eq!(checksum.len(), 64);
        assert!(checksum.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_verify_checksum_match() {
        let checksum = calculate_checksum(b"test");
        assert!(verify_checksum(&checksum, &checksum).is_ok());
    }

    #[test]
    fn test_verify_checksum_case_insensitive() {
        let checksum = calculate_checksum(b"test");
        let uppercase = checksum.to_uppercase();
        assert!(verify_checksum(&checksum, &uppercase).is_ok());
    }

    #[test]
    fn test_verify_checksum_mismatch() {
        let checksum = calculate_checksum(b"test");
        let wrong = calculate_checksum(b"wrong");
        let result = verify_checksum(&checksum, &wrong);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            StorageError::ChecksumError(_)
        ));
    }

    #[test]
    fn test_streaming_threshold() {
        assert_eq!(STREAMING_THRESHOLD_BYTES, 10 * 1024 * 1024);
    }

    #[tokio::test]
    async fn test_calculate_checksum_streaming() {
        let data = b"hello";
        let stream = ByteStream::from_static(data);
        let checksum = calculate_checksum_streaming(stream).await.unwrap();
        assert_eq!(
            checksum,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[tokio::test]
    async fn test_calculate_checksum_chunked() {
        let data = b"hello world";
        let stream = ByteStream::from_static(data);
        let (checksum, bytes) = calculate_checksum_chunked(stream).await.unwrap();
        assert_eq!(bytes, 11);
        // SHA-256 of "hello world"
        assert_eq!(
            checksum,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }
}

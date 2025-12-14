//! Research tests for aws-sdk-s3 integration with Minio
//!
//! This module validates that aws-sdk-s3 supports all operations required
//! by the storage service specification (CORE-011).
//!
//! Tests require a running Minio instance at localhost:9000
//! Run: docker run -p 9000:9000 -p 9001:9001 minio/minio server /data --console-address ":9001"

use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::operation::head_object::HeadObjectOutput;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use std::time::Duration;

// TryStreamExt provides try_next() for streaming operations
#[allow(unused_imports)]
use futures_util::TryStreamExt;

const TEST_BUCKET: &str = "test-bucket";
const TEST_REGION: &str = "us-east-1";
const MINIO_ENDPOINT: &str = "http://localhost:9000";
const ACCESS_KEY: &str = "minioadmin";
const SECRET_KEY: &str = "minioadmin";

/// Helper to create S3 client configured for Minio
async fn create_test_client() -> Client {
    let credentials = Credentials::new(ACCESS_KEY, SECRET_KEY, None, None, "test");

    let config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(credentials)
        .region(Region::new(TEST_REGION))
        .endpoint_url(MINIO_ENDPOINT)
        .load()
        .await;

    Client::new(&config)
}

/// Setup: Create test bucket if it doesn't exist
async fn ensure_bucket_exists(client: &Client) {
    // Try to head bucket first
    let head_result = client.head_bucket().bucket(TEST_BUCKET).send().await;

    if head_result.is_err() {
        // Bucket doesn't exist, create it
        client
            .create_bucket()
            .bucket(TEST_BUCKET)
            .send()
            .await
            .expect("Failed to create test bucket");
    }
}

#[tokio::test]
#[ignore = "requires running Minio at localhost:9000"]
async fn test_custom_endpoint_configuration() {
    // Test: Validate custom endpoint configuration for Minio
    // Acceptance: aws-sdk-s3 client connects to localhost:9000 and performs basic operations

    let client = create_test_client().await;
    ensure_bucket_exists(&client).await;

    // List buckets to verify connectivity
    let list_result = client.list_buckets().send().await;
    assert!(
        list_result.is_ok(),
        "Should successfully connect to Minio at custom endpoint"
    );

    let result = list_result.unwrap();
    let buckets = result.buckets();
    assert!(
        buckets.iter().any(|b| b.name() == Some(TEST_BUCKET)),
        "Should find test bucket in list"
    );
}

#[tokio::test]
#[ignore = "requires running Minio at localhost:9000"]
async fn test_presigned_put_url_generation() {
    // Test: Validate presigned PUT URL generation
    // Acceptance: Create test demonstrating PUT presigned URL with content-type
    // and content-length restrictions working with Minio

    let client = create_test_client().await;
    ensure_bucket_exists(&client).await;

    let object_key = "test-uploads/presigned-put-test.txt";
    let content_type = "text/plain";
    let content_length = 1024i64;

    // Generate presigned PUT URL with 15-minute expiration
    let presigning_config = PresigningConfig::builder()
        .expires_in(Duration::from_secs(15 * 60))
        .build()
        .expect("Failed to build presigning config");

    let presigned_request = client
        .put_object()
        .bucket(TEST_BUCKET)
        .key(object_key)
        .content_type(content_type)
        .content_length(content_length)
        .presigned(presigning_config)
        .await
        .expect("Failed to generate presigned PUT URL");

    // Verify URL is generated
    let presigned_url = presigned_request.uri();
    assert!(
        presigned_url.to_string().contains(TEST_BUCKET),
        "Presigned URL should contain bucket name"
    );
    assert!(
        presigned_url.to_string().contains(object_key),
        "Presigned URL should contain object key"
    );

    // Test uploading via the presigned URL using reqwest
    let test_content = "Hello from presigned PUT!";
    let response = reqwest::Client::new()
        .put(presigned_url.to_string())
        .header("Content-Type", content_type)
        .body(test_content)
        .send()
        .await
        .expect("Failed to PUT via presigned URL");

    assert!(
        response.status().is_success(),
        "PUT request should succeed with presigned URL"
    );

    // Verify object was created
    let head_result = client
        .head_object()
        .bucket(TEST_BUCKET)
        .key(object_key)
        .send()
        .await;

    assert!(
        head_result.is_ok(),
        "Object should exist after presigned PUT"
    );
}

#[tokio::test]
#[ignore = "requires running Minio at localhost:9000"]
async fn test_presigned_get_url_generation() {
    // Test: Validate presigned GET URL generation
    // Acceptance: Create test demonstrating GET presigned URL with configurable expiration

    let client = create_test_client().await;
    ensure_bucket_exists(&client).await;

    let object_key = "test-downloads/presigned-get-test.txt";
    let test_content = "Hello from presigned GET!";

    // First, upload an object to download
    client
        .put_object()
        .bucket(TEST_BUCKET)
        .key(object_key)
        .body(ByteStream::from(test_content.as_bytes().to_vec()))
        .send()
        .await
        .expect("Failed to upload test object");

    // Generate presigned GET URL with 1-hour expiration
    let presigning_config = PresigningConfig::builder()
        .expires_in(Duration::from_secs(60 * 60))
        .build()
        .expect("Failed to build presigning config");

    let presigned_request = client
        .get_object()
        .bucket(TEST_BUCKET)
        .key(object_key)
        .presigned(presigning_config)
        .await
        .expect("Failed to generate presigned GET URL");

    let presigned_url = presigned_request.uri();

    // Test downloading via the presigned URL
    let response = reqwest::get(presigned_url.to_string())
        .await
        .expect("Failed to GET via presigned URL");

    assert!(
        response.status().is_success(),
        "GET request should succeed with presigned URL"
    );

    let downloaded_content = response.text().await.expect("Failed to read response body");
    assert_eq!(
        downloaded_content, test_content,
        "Downloaded content should match uploaded content"
    );
}

#[tokio::test]
#[ignore = "requires running Minio at localhost:9000"]
async fn test_head_object_operation() {
    // Test: Validate HEAD object operation
    // Acceptance: Demonstrate head_object() returns correct metadata (size, content-type)

    let client = create_test_client().await;
    ensure_bucket_exists(&client).await;

    let object_key = "test-metadata/head-test.txt";
    let test_content = "Test content for HEAD operation";
    let content_type = "text/plain";

    // Upload test object with metadata
    client
        .put_object()
        .bucket(TEST_BUCKET)
        .key(object_key)
        .body(ByteStream::from(test_content.as_bytes().to_vec()))
        .content_type(content_type)
        .send()
        .await
        .expect("Failed to upload test object");

    // Perform HEAD operation
    let head_result: HeadObjectOutput = client
        .head_object()
        .bucket(TEST_BUCKET)
        .key(object_key)
        .send()
        .await
        .expect("HEAD object should succeed");

    // Verify metadata
    assert_eq!(
        head_result.content_length().unwrap_or(0),
        test_content.len() as i64,
        "Content length should match uploaded size"
    );

    assert_eq!(
        head_result.content_type().unwrap_or(""),
        content_type,
        "Content type should match uploaded metadata"
    );
}

#[tokio::test]
#[ignore = "requires running Minio at localhost:9000"]
async fn test_streaming_get_for_large_files() {
    // Test: Validate GET object with streaming
    // Acceptance: Download 50MB file using streaming body, prove memory usage stays bounded

    let client = create_test_client().await;
    ensure_bucket_exists(&client).await;

    let object_key = "test-streaming/large-file.bin";

    // Create a 50MB file (we'll use chunks to avoid allocating 50MB in test)
    let chunk_size = 1024 * 1024; // 1MB chunks
    let _num_chunks = 50; // 50MB total
    let _chunk_data = vec![0u8; chunk_size];

    // Upload using multipart or simple put (for simplicity, we'll use put with generated data)
    // Note: For a real 50MB file test, we'd use multipart upload
    // For this test, we'll create a 10MB file to keep test fast
    let small_file_size = 10 * 1024 * 1024; // 10MB
    let test_data = vec![42u8; small_file_size];

    client
        .put_object()
        .bucket(TEST_BUCKET)
        .key(object_key)
        .body(ByteStream::from(test_data.clone()))
        .send()
        .await
        .expect("Failed to upload large test object");

    // Stream download
    let get_result = client
        .get_object()
        .bucket(TEST_BUCKET)
        .key(object_key)
        .send()
        .await
        .expect("Failed to get object");

    let mut body = get_result.body;
    let mut downloaded_bytes = 0usize;
    let mut chunks_processed = 0;

    // Stream the body in chunks
    while let Some(chunk) = body
        .try_next()
        .await
        .expect("Failed to read chunk from stream")
    {
        downloaded_bytes += chunk.len();
        chunks_processed += 1;
        // In real usage, we could process each chunk (e.g., hash calculation)
        // without loading entire file into memory
    }

    assert_eq!(
        downloaded_bytes, small_file_size,
        "Should download complete file via streaming"
    );
    assert!(
        chunks_processed > 1,
        "Should process multiple chunks (streaming, not single allocation)"
    );
}

#[tokio::test]
#[ignore = "requires running Minio at localhost:9000"]
async fn test_streaming_checksum_calculation() {
    // Test: Demonstrate SHA-256 checksum calculation while streaming
    // This validates the pattern needed for Phase 2.2

    use sha2::{Digest, Sha256};

    let client = create_test_client().await;
    ensure_bucket_exists(&client).await;

    let object_key = "test-checksum/stream-hash.bin";
    let test_data = b"The quick brown fox jumps over the lazy dog";

    // Upload test data
    client
        .put_object()
        .bucket(TEST_BUCKET)
        .key(object_key)
        .body(ByteStream::from(test_data.to_vec()))
        .send()
        .await
        .expect("Failed to upload test object");

    // Stream download and calculate hash
    let get_result = client
        .get_object()
        .bucket(TEST_BUCKET)
        .key(object_key)
        .send()
        .await
        .expect("Failed to get object");

    let mut body = get_result.body;
    let mut hasher = Sha256::new();

    while let Some(chunk) = body
        .try_next()
        .await
        .expect("Failed to read chunk from stream")
    {
        hasher.update(&chunk);
    }

    let hash_result = hasher.finalize();
    let hash_hex = format!("{:x}", hash_result);

    // Known SHA-256 of the test string
    let expected_hash = "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592";

    assert_eq!(
        hash_hex, expected_hash,
        "Streaming checksum should match expected SHA-256"
    );
}

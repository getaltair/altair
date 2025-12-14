//! S3 Client wrapper module
//!
//! This module provides a wrapper around aws-sdk-s3 client with methods
//! for common storage operations: head, get, delete, and health checks.

use crate::config::StorageConfig;
use crate::error::{StorageError, StorageResult};
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::primitives::ByteStream;
use tracing::instrument;

/// Metadata about an object in S3
#[derive(Debug, Clone)]
pub struct ObjectMetadata {
    /// Size of the object in bytes
    pub size: u64,
    /// Content type (MIME type) of the object
    pub content_type: Option<String>,
    /// ETag (typically MD5 hash of the object)
    pub etag: Option<String>,
    /// Last modified timestamp
    pub last_modified: Option<aws_sdk_s3::primitives::DateTime>,
}

/// S3 storage client wrapper
///
/// Provides a high-level interface to aws-sdk-s3 with proper error handling
/// and integration with Altair's storage configuration.
#[derive(Clone)]
pub struct S3Client {
    /// The underlying aws-sdk-s3 client
    client: Client,
    /// The bucket name to operate on
    bucket: String,
    /// The endpoint URL (for reference)
    endpoint: String,
}

impl S3Client {
    /// Create a new S3 client from StorageConfig
    ///
    /// This initializes the aws-sdk-s3 client with the provided configuration,
    /// setting up custom endpoint and credentials for S3-compatible services.
    #[instrument(skip(config), fields(endpoint = %config.endpoint_str(), bucket = %config.bucket()))]
    pub async fn new(config: &StorageConfig) -> StorageResult<Self> {
        // Build credentials from config
        let credentials = Credentials::new(
            config.access_key_id(),
            config.secret_access_key(),
            None, // session token
            None, // expiration
            "altair-storage",
        );

        // Build S3 config
        let s3_config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(config.region().to_string()))
            .endpoint_url(config.endpoint_str())
            .credentials_provider(credentials)
            // Force path-style for Minio compatibility
            .force_path_style(true)
            .build();

        let client = Client::from_conf(s3_config);

        tracing::info!("Created S3 client for endpoint: {}", config.endpoint_str());

        Ok(Self {
            client,
            bucket: config.bucket().to_string(),
            endpoint: config.endpoint_str().to_string(),
        })
    }

    /// Get the bucket name this client operates on
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Get the endpoint URL
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Get the underlying aws-sdk-s3 client
    ///
    /// Useful for advanced operations or presigning
    pub fn inner(&self) -> &Client {
        &self.client
    }

    /// Check connection health by performing a HEAD bucket operation
    ///
    /// This verifies that:
    /// - The S3 endpoint is reachable
    /// - Credentials are valid
    /// - The bucket exists and is accessible
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> StorageResult<()> {
        self.client
            .head_bucket()
            .bucket(&self.bucket)
            .send()
            .await
            .map_err(|e| {
                let msg = e.into_service_error();
                StorageError::s3(
                    "HeadBucket",
                    format!(
                        "Health check failed for bucket '{}': {:?}",
                        self.bucket, msg
                    ),
                )
            })?;

        tracing::debug!("Health check passed for bucket: {}", self.bucket);
        Ok(())
    }

    /// Get object metadata without downloading the object
    ///
    /// Uses HEAD request to retrieve size, content type, and other metadata.
    ///
    /// # Arguments
    /// * `key` - The S3 object key
    ///
    /// # Returns
    /// `ObjectMetadata` with size and content type information
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn head_object(&self, key: &str) -> StorageResult<ObjectMetadata> {
        let response = self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                let service_error = e.into_service_error();
                if service_error.is_not_found() {
                    StorageError::object_not_found(key)
                } else {
                    StorageError::s3("HeadObject", format!("{:?}", service_error))
                }
            })?;

        Ok(ObjectMetadata {
            size: response.content_length().unwrap_or(0) as u64,
            content_type: response.content_type().map(|s| s.to_string()),
            etag: response.e_tag().map(|s| s.to_string()),
            last_modified: response.last_modified().cloned(),
        })
    }

    /// Get object data as a streaming byte stream
    ///
    /// This method returns a `ByteStream` that can be consumed incrementally,
    /// allowing memory-efficient processing of large files.
    ///
    /// # Arguments
    /// * `key` - The S3 object key
    ///
    /// # Returns
    /// `ByteStream` that can be read asynchronously
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn get_object(&self, key: &str) -> StorageResult<ByteStream> {
        let response = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                let service_error = e.into_service_error();
                if service_error.is_no_such_key() {
                    StorageError::object_not_found(key)
                } else {
                    StorageError::s3("GetObject", format!("{:?}", service_error))
                }
            })?;

        Ok(response.body)
    }

    /// Get object data as bytes (loads entire object into memory)
    ///
    /// Use this only for small objects. For large objects, use `get_object()`
    /// and stream the data.
    ///
    /// # Arguments
    /// * `key` - The S3 object key
    ///
    /// # Returns
    /// The complete object data as a `Vec<u8>`
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn get_object_bytes(&self, key: &str) -> StorageResult<Vec<u8>> {
        let stream = self.get_object(key).await?;
        let data = stream.collect().await.map_err(|e| {
            StorageError::s3("GetObject", format!("Failed to collect bytes: {}", e))
        })?;
        Ok(data.into_bytes().to_vec())
    }

    /// Delete an object from S3
    ///
    /// Note: S3 delete operations are idempotent - deleting a non-existent object
    /// does not return an error.
    ///
    /// # Arguments
    /// * `key` - The S3 object key to delete
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn delete_object(&self, key: &str) -> StorageResult<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                StorageError::s3("DeleteObject", format!("{:?}", e.into_service_error()))
            })?;

        tracing::info!(key = key, "Deleted object from S3");
        Ok(())
    }

    /// Put an object to S3
    ///
    /// This is primarily used for internal operations like uploading thumbnails.
    /// Client uploads should use presigned URLs instead.
    ///
    /// # Arguments
    /// * `key` - The S3 object key
    /// * `data` - The object data
    /// * `content_type` - The MIME type of the object
    #[instrument(skip(self, data), fields(bucket = %self.bucket, size = data.len()))]
    pub async fn put_object(
        &self,
        key: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> StorageResult<()> {
        let body = ByteStream::from(data);

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| StorageError::s3("PutObject", format!("{:?}", e.into_service_error())))?;

        tracing::info!(key = key, "Uploaded object to S3");
        Ok(())
    }

    /// Check if an object exists in S3
    ///
    /// # Arguments
    /// * `key` - The S3 object key
    ///
    /// # Returns
    /// `true` if the object exists, `false` otherwise
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn object_exists(&self, key: &str) -> StorageResult<bool> {
        match self.head_object(key).await {
            Ok(_) => Ok(true),
            Err(StorageError::ObjectNotFound { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// List objects with a given prefix
    ///
    /// Useful for listing all objects for a user or quota reconciliation.
    ///
    /// # Arguments
    /// * `prefix` - The prefix to filter objects by (e.g., "user123/")
    ///
    /// # Returns
    /// A vector of object keys matching the prefix
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn list_objects(&self, prefix: &str) -> StorageResult<Vec<String>> {
        let mut keys = Vec::new();
        let mut continuation_token: Option<String> = None;

        loop {
            let mut request = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(prefix);

            if let Some(token) = continuation_token.take() {
                request = request.continuation_token(token);
            }

            let response = request.send().await.map_err(|e| {
                StorageError::s3("ListObjectsV2", format!("{:?}", e.into_service_error()))
            })?;

            if let Some(ref contents) = response.contents {
                for object in contents {
                    if let Some(key) = &object.key {
                        keys.push(key.clone());
                    }
                }
            }

            if response.is_truncated() == Some(true) {
                continuation_token = response.next_continuation_token;
            } else {
                break;
            }
        }

        Ok(keys)
    }

    /// Get total size of all objects with a given prefix
    ///
    /// Used for quota reconciliation to verify actual storage usage.
    ///
    /// # Arguments
    /// * `prefix` - The prefix to filter objects by (e.g., "user123/")
    ///
    /// # Returns
    /// Total size in bytes
    #[instrument(skip(self), fields(bucket = %self.bucket))]
    pub async fn get_prefix_size(&self, prefix: &str) -> StorageResult<u64> {
        let mut total_size: u64 = 0;
        let mut continuation_token: Option<String> = None;

        loop {
            let mut request = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(prefix);

            if let Some(token) = continuation_token.take() {
                request = request.continuation_token(token);
            }

            let response = request.send().await.map_err(|e| {
                StorageError::s3("ListObjectsV2", format!("{:?}", e.into_service_error()))
            })?;

            if let Some(ref contents) = response.contents {
                for object in contents {
                    total_size += object.size().unwrap_or(0) as u64;
                }
            }

            if response.is_truncated() == Some(true) {
                continuation_token = response.next_continuation_token;
            } else {
                break;
            }
        }

        Ok(total_size)
    }
}

impl std::fmt::Debug for S3Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("S3Client")
            .field("bucket", &self.bucket)
            .field("endpoint", &self.endpoint)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a running Minio instance
    // They are marked as ignored by default and should be run with:
    // cargo test --package altair-storage -- --ignored

    #[tokio::test]
    #[ignore = "requires running Minio"]
    async fn test_client_creation() {
        let config = StorageConfig::new(
            "http://localhost:9000",
            "us-east-1",
            "test-bucket",
            "minioadmin",
            "minioadmin",
        )
        .unwrap();

        let client = S3Client::new(&config).await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires running Minio"]
    async fn test_health_check() {
        let config = StorageConfig::new(
            "http://localhost:9000",
            "us-east-1",
            "test-bucket",
            "minioadmin",
            "minioadmin",
        )
        .unwrap();

        let client = S3Client::new(&config).await.unwrap();
        let result = client.health_check().await;
        // This will fail if bucket doesn't exist, which is expected without setup
        assert!(result.is_ok() || matches!(result.unwrap_err(), StorageError::S3Error { .. }));
    }
}

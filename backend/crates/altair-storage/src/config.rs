//! Storage configuration module
//!
//! This module provides configuration management for S3-compatible storage,
//! including endpoint validation, keychain credential retrieval, and bucket name validation.

use crate::error::{StorageError, StorageResult};
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::env;
use tracing::instrument;
use url::Url;

/// Service name for keyring credential storage
const KEYRING_SERVICE: &str = "altair-storage";

/// Username for keyring entries (we store multiple entries under different keys)
const KEYRING_USER_ACCESS_KEY: &str = "s3_access_key_id";
const KEYRING_USER_SECRET_KEY: &str = "s3_secret_access_key";

/// Regex pattern for valid S3 bucket names
/// See: https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
fn is_valid_bucket_name(name: &str) -> bool {
    // Bucket names must be between 3 and 63 characters long
    if name.len() < 3 || name.len() > 63 {
        return false;
    }

    // Must start with a lowercase letter or number
    let first_char = name.chars().next().unwrap();
    if !first_char.is_ascii_lowercase() && !first_char.is_ascii_digit() {
        return false;
    }

    // Must end with a lowercase letter or number
    let last_char = name.chars().last().unwrap();
    if !last_char.is_ascii_lowercase() && !last_char.is_ascii_digit() {
        return false;
    }

    // Can only contain lowercase letters, numbers, and hyphens
    // Cannot contain consecutive periods or be formatted as an IP address
    let mut prev_char = ' ';
    for c in name.chars() {
        if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != '-' && c != '.' {
            return false;
        }
        // No consecutive periods
        if c == '.' && prev_char == '.' {
            return false;
        }
        prev_char = c;
    }

    // Simple IP address check (not exhaustive, but catches common cases)
    let parts: Vec<&str> = name.split('.').collect();
    if parts.len() == 4 && parts.iter().all(|p| p.parse::<u8>().is_ok()) {
        return false;
    }

    true
}

/// Configuration for S3-compatible storage
///
/// This struct holds all necessary configuration for connecting to an S3-compatible
/// storage service. It supports both embedded Minio and external S3 endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// S3 endpoint URL (e.g., "https://s3.amazonaws.com" or "http://localhost:9000")
    /// Validated to be a proper URL with http or https scheme
    endpoint: Url,

    /// S3 region (e.g., "us-east-1")
    region: String,

    /// S3 bucket name (must be DNS-compliant)
    bucket: String,

    /// Access key ID for S3 authentication
    access_key_id: String,

    /// Secret access key for S3 authentication
    #[serde(skip_serializing)]
    secret_access_key: String,
}

impl StorageConfig {
    /// Create a new StorageConfig with validation
    ///
    /// # Arguments
    /// * `endpoint` - S3 endpoint URL (must be http or https)
    /// * `region` - S3 region
    /// * `bucket` - S3 bucket name (must be DNS-compliant)
    /// * `access_key_id` - Access key ID
    /// * `secret_access_key` - Secret access key
    ///
    /// # Errors
    /// Returns `StorageError::InvalidConfig` if validation fails
    pub fn new(
        endpoint: impl AsRef<str>,
        region: impl Into<String>,
        bucket: impl Into<String>,
        access_key_id: impl Into<String>,
        secret_access_key: impl Into<String>,
    ) -> StorageResult<Self> {
        let endpoint = endpoint.as_ref();
        let region = region.into();
        let bucket = bucket.into();
        let access_key_id = access_key_id.into();
        let secret_access_key = secret_access_key.into();

        let config = Self {
            endpoint: Self::parse_endpoint(endpoint)?,
            region,
            bucket,
            access_key_id,
            secret_access_key,
        };

        config.validate()?;
        Ok(config)
    }

    /// Parse and validate the endpoint URL
    fn parse_endpoint(endpoint: &str) -> StorageResult<Url> {
        let url = Url::parse(endpoint)
            .map_err(|e| StorageError::invalid_config("endpoint", format!("invalid URL: {}", e)))?;

        // Validate scheme is http or https
        match url.scheme() {
            "http" | "https" => Ok(url),
            scheme => Err(StorageError::invalid_config(
                "endpoint",
                format!("scheme must be 'http' or 'https', got '{}'", scheme),
            )),
        }
    }

    /// Validate the configuration
    ///
    /// Checks:
    /// - Endpoint is a valid http/https URL
    /// - Bucket name is DNS-compliant
    /// - Region is not empty
    /// - Credentials are not empty
    #[instrument(skip(self))]
    pub fn validate(&self) -> StorageResult<()> {
        // Validate bucket name
        if !is_valid_bucket_name(&self.bucket) {
            return Err(StorageError::invalid_config(
                "bucket",
                format!(
                    "bucket name '{}' is not DNS-compliant. Must be 3-63 chars, lowercase letters, numbers, and hyphens only",
                    self.bucket
                ),
            ));
        }

        // Validate region is not empty
        if self.region.is_empty() {
            return Err(StorageError::invalid_config(
                "region",
                "region cannot be empty",
            ));
        }

        // Validate credentials are not empty
        if self.access_key_id.is_empty() {
            return Err(StorageError::invalid_config(
                "access_key_id",
                "access key ID cannot be empty",
            ));
        }

        if self.secret_access_key.is_empty() {
            return Err(StorageError::invalid_config(
                "secret_access_key",
                "secret access key cannot be empty",
            ));
        }

        Ok(())
    }

    /// Load configuration from OS keychain
    ///
    /// This method retrieves S3 credentials from the OS keychain/credential manager:
    /// - macOS: Keychain
    /// - Windows: Credential Manager
    /// - Linux: Secret Service (libsecret)
    ///
    /// Environment variables can override keychain values:
    /// - `STORAGE_ENDPOINT` - S3 endpoint URL
    /// - `STORAGE_REGION` - S3 region (default: "us-east-1")
    /// - `STORAGE_BUCKET` - S3 bucket name
    /// - `S3_ACCESS_KEY_ID` - Access key (overrides keychain)
    /// - `S3_SECRET_ACCESS_KEY` - Secret key (overrides keychain)
    ///
    /// # Errors
    /// Returns `StorageError::CredentialsNotFound` if credentials cannot be found
    #[instrument]
    pub fn from_keychain() -> StorageResult<Self> {
        // Try environment variables first (allows override for development)
        let access_key_id = match env::var("S3_ACCESS_KEY_ID") {
            Ok(key) if !key.is_empty() => key,
            _ => Self::get_keychain_value(KEYRING_USER_ACCESS_KEY)?,
        };

        let secret_access_key = match env::var("S3_SECRET_ACCESS_KEY") {
            Ok(key) if !key.is_empty() => key,
            _ => Self::get_keychain_value(KEYRING_USER_SECRET_KEY)?,
        };

        // Get other config from environment (with defaults)
        let endpoint = env::var("STORAGE_ENDPOINT").unwrap_or_else(|_| {
            // Default to embedded Minio endpoint
            "http://localhost:9000".to_string()
        });

        let region = env::var("STORAGE_REGION").unwrap_or_else(|_| "us-east-1".to_string());

        let bucket = env::var("STORAGE_BUCKET").unwrap_or_else(|_| "altair".to_string());

        Self::new(endpoint, region, bucket, access_key_id, secret_access_key)
    }

    /// Get a value from the keychain
    fn get_keychain_value(key: &str) -> StorageResult<String> {
        let entry = Entry::new(KEYRING_SERVICE, key).map_err(|e| {
            StorageError::KeychainError(format!("Failed to create keyring entry: {}", e))
        })?;

        entry.get_password().map_err(|e| match e {
            keyring::Error::NoEntry => StorageError::credentials_not_found(format!(
                "No credential found for key '{}'. Run the storage setup command first.",
                key
            )),
            _ => StorageError::KeychainError(format!("Failed to get password: {}", e)),
        })
    }

    /// Store credentials in the OS keychain
    ///
    /// This should be called during initial setup to securely store S3 credentials.
    ///
    /// # Arguments
    /// * `access_key_id` - The S3 access key ID to store
    /// * `secret_access_key` - The S3 secret access key to store
    #[instrument(skip(access_key_id, secret_access_key))]
    pub fn store_credentials(access_key_id: &str, secret_access_key: &str) -> StorageResult<()> {
        let access_entry = Entry::new(KEYRING_SERVICE, KEYRING_USER_ACCESS_KEY)
            .map_err(|e| StorageError::KeychainError(format!("Failed to create entry: {}", e)))?;

        access_entry.set_password(access_key_id).map_err(|e| {
            StorageError::KeychainError(format!("Failed to store access key: {}", e))
        })?;

        let secret_entry = Entry::new(KEYRING_SERVICE, KEYRING_USER_SECRET_KEY)
            .map_err(|e| StorageError::KeychainError(format!("Failed to create entry: {}", e)))?;

        secret_entry.set_password(secret_access_key).map_err(|e| {
            StorageError::KeychainError(format!("Failed to store secret key: {}", e))
        })?;

        tracing::info!("Stored S3 credentials in system keychain");
        Ok(())
    }

    /// Delete credentials from the OS keychain
    #[instrument]
    pub fn delete_credentials() -> StorageResult<()> {
        let access_entry = Entry::new(KEYRING_SERVICE, KEYRING_USER_ACCESS_KEY)
            .map_err(|e| StorageError::KeychainError(format!("Failed to create entry: {}", e)))?;

        // Ignore NoEntry errors - credentials may not exist
        match access_entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => {}
            Err(e) => {
                return Err(StorageError::KeychainError(format!(
                    "Failed to delete access key: {}",
                    e
                )));
            }
        }

        let secret_entry = Entry::new(KEYRING_SERVICE, KEYRING_USER_SECRET_KEY)
            .map_err(|e| StorageError::KeychainError(format!("Failed to create entry: {}", e)))?;

        match secret_entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => {}
            Err(e) => {
                return Err(StorageError::KeychainError(format!(
                    "Failed to delete secret key: {}",
                    e
                )));
            }
        }

        tracing::info!("Deleted S3 credentials from system keychain");
        Ok(())
    }

    /// Create configuration from environment variables only (no keychain)
    ///
    /// Useful for CI/CD environments or Docker containers where keychain is unavailable.
    #[instrument]
    pub fn from_env() -> StorageResult<Self> {
        let endpoint = env::var("STORAGE_ENDPOINT").map_err(|_| {
            StorageError::credentials_not_found("STORAGE_ENDPOINT environment variable not set")
        })?;

        let region = env::var("STORAGE_REGION").unwrap_or_else(|_| "us-east-1".to_string());

        let bucket = env::var("STORAGE_BUCKET").map_err(|_| {
            StorageError::credentials_not_found("STORAGE_BUCKET environment variable not set")
        })?;

        let access_key_id = env::var("S3_ACCESS_KEY_ID").map_err(|_| {
            StorageError::credentials_not_found("S3_ACCESS_KEY_ID environment variable not set")
        })?;

        let secret_access_key = env::var("S3_SECRET_ACCESS_KEY").map_err(|_| {
            StorageError::credentials_not_found("S3_SECRET_ACCESS_KEY environment variable not set")
        })?;

        Self::new(endpoint, region, bucket, access_key_id, secret_access_key)
    }

    // Getters

    /// Get the S3 endpoint URL
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Get the S3 endpoint as a string
    pub fn endpoint_str(&self) -> &str {
        self.endpoint.as_str()
    }

    /// Get the S3 region
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Get the S3 bucket name
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Get the access key ID
    pub fn access_key_id(&self) -> &str {
        &self.access_key_id
    }

    /// Get the secret access key
    pub fn secret_access_key(&self) -> &str {
        &self.secret_access_key
    }

    /// Check if this config points to localhost (embedded Minio)
    pub fn is_localhost(&self) -> bool {
        self.endpoint
            .host_str()
            .map(|h| h == "localhost" || h == "127.0.0.1")
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bucket_names() {
        assert!(is_valid_bucket_name("my-bucket"));
        assert!(is_valid_bucket_name("my.bucket.name"));
        assert!(is_valid_bucket_name("bucket123"));
        assert!(is_valid_bucket_name("123bucket"));
        assert!(is_valid_bucket_name("a-b"));
    }

    #[test]
    fn test_invalid_bucket_names() {
        // Too short
        assert!(!is_valid_bucket_name("ab"));
        // Too long (64 chars)
        assert!(!is_valid_bucket_name(
            "a123456789012345678901234567890123456789012345678901234567890123"
        ));
        // Uppercase
        assert!(!is_valid_bucket_name("MyBucket"));
        // Starts with hyphen
        assert!(!is_valid_bucket_name("-bucket"));
        // Ends with hyphen
        assert!(!is_valid_bucket_name("bucket-"));
        // Consecutive periods
        assert!(!is_valid_bucket_name("bucket..name"));
        // IP address format
        assert!(!is_valid_bucket_name("192.168.1.1"));
        // Invalid characters
        assert!(!is_valid_bucket_name("bucket_name"));
    }

    #[test]
    fn test_config_creation_valid() {
        let config = StorageConfig::new(
            "http://localhost:9000",
            "us-east-1",
            "altair",
            "access-key",
            "secret-key",
        );

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.endpoint_str(), "http://localhost:9000/");
        assert_eq!(config.region(), "us-east-1");
        assert_eq!(config.bucket(), "altair");
        assert!(config.is_localhost());
    }

    #[test]
    fn test_config_invalid_endpoint_scheme() {
        let config = StorageConfig::new(
            "ftp://localhost:9000",
            "us-east-1",
            "altair",
            "access-key",
            "secret-key",
        );

        assert!(config.is_err());
        let err = config.unwrap_err();
        assert!(matches!(err, StorageError::InvalidConfig { field, .. } if field == "endpoint"));
    }

    #[test]
    fn test_config_invalid_bucket() {
        let config = StorageConfig::new(
            "http://localhost:9000",
            "us-east-1",
            "Invalid_Bucket",
            "access-key",
            "secret-key",
        );

        assert!(config.is_err());
        let err = config.unwrap_err();
        assert!(matches!(err, StorageError::InvalidConfig { field, .. } if field == "bucket"));
    }

    #[test]
    fn test_config_empty_credentials() {
        let config = StorageConfig::new(
            "http://localhost:9000",
            "us-east-1",
            "altair",
            "",
            "secret-key",
        );

        assert!(config.is_err());
        let err = config.unwrap_err();
        assert!(
            matches!(err, StorageError::InvalidConfig { field, .. } if field == "access_key_id")
        );
    }

    #[test]
    fn test_is_localhost() {
        let local_config = StorageConfig::new(
            "http://localhost:9000",
            "us-east-1",
            "altair",
            "key",
            "secret",
        )
        .unwrap();
        assert!(local_config.is_localhost());

        let local_config_127 = StorageConfig::new(
            "http://127.0.0.1:9000",
            "us-east-1",
            "altair",
            "key",
            "secret",
        )
        .unwrap();
        assert!(local_config_127.is_localhost());

        let remote_config = StorageConfig::new(
            "https://s3.amazonaws.com",
            "us-east-1",
            "altair",
            "key",
            "secret",
        )
        .unwrap();
        assert!(!remote_config.is_localhost());
    }

    // Note: Environment variable tests (from_env) are not included here because
    // std::env::set_var is unsafe in Rust 2024 edition and environment variables
    // are shared across threads, making these tests unreliable in parallel test
    // execution. The from_env functionality is tested via integration tests where
    // environment variables can be properly controlled.

    #[test]
    fn test_config_empty_region_rejected() {
        let config = StorageConfig::new(
            "http://localhost:9000",
            "", // empty region
            "altair",
            "access-key",
            "secret-key",
        );

        assert!(config.is_err());
        let err = config.unwrap_err();
        assert!(matches!(err, StorageError::InvalidConfig { field, .. } if field == "region"));
    }

    #[test]
    fn test_config_empty_secret_key_rejected() {
        let config = StorageConfig::new(
            "http://localhost:9000",
            "us-east-1",
            "altair",
            "access-key",
            "", // empty secret key
        );

        assert!(config.is_err());
        let err = config.unwrap_err();
        assert!(
            matches!(err, StorageError::InvalidConfig { field, .. } if field == "secret_access_key")
        );
    }

    #[test]
    fn test_config_invalid_endpoint_url() {
        let config = StorageConfig::new(
            "not-a-valid-url",
            "us-east-1",
            "altair",
            "access-key",
            "secret-key",
        );

        assert!(config.is_err());
        let err = config.unwrap_err();
        assert!(matches!(err, StorageError::InvalidConfig { field, .. } if field == "endpoint"));
    }

    #[test]
    fn test_config_https_endpoint() {
        let config = StorageConfig::new(
            "https://s3.us-west-2.amazonaws.com",
            "us-west-2",
            "my-bucket",
            "AKIA1234567890EXAMPLE",
            "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
        );

        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(!config.is_localhost());
        assert_eq!(config.endpoint_str(), "https://s3.us-west-2.amazonaws.com/");
    }

    #[test]
    fn test_bucket_name_edge_cases() {
        // Minimum valid length (3 chars)
        assert!(is_valid_bucket_name("abc"));

        // Maximum valid length (63 chars)
        assert!(is_valid_bucket_name(
            "a12345678901234567890123456789012345678901234567890123456789012"
        ));

        // Contains periods (valid)
        assert!(is_valid_bucket_name("my.test.bucket"));

        // Starts with number (valid)
        assert!(is_valid_bucket_name("123bucket"));
    }
}

//! Shared traits for Altair components

use async_trait::async_trait;
use crate::Result;

/// Trait for authentication providers (plugin architecture)
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Provider name (e.g., "local", "oauth_google")
    fn name(&self) -> &str;

    /// Authenticate user with credentials
    async fn authenticate(&self, credentials: &str) -> Result<String>;

    /// Validate an existing token
    async fn validate_token(&self, token: &str) -> Result<String>;

    /// Refresh an authentication token
    async fn refresh_token(&self, token: &str) -> Result<String>;

    /// Revoke a token
    async fn revoke_token(&self, token: &str) -> Result<()>;
}

/// Trait for storage providers (S3-compatible backends)
#[async_trait]
pub trait StorageProvider: Send + Sync {
    /// Provider name (e.g., "minio", "backblaze")
    fn name(&self) -> &str;

    /// Upload a file
    async fn upload(&self, key: &str, data: &[u8]) -> Result<String>;

    /// Download a file
    async fn download(&self, key: &str) -> Result<Vec<u8>>;

    /// Delete a file
    async fn delete(&self, key: &str) -> Result<()>;

    /// Check if file exists
    async fn exists(&self, key: &str) -> Result<bool>;

    /// Generate a presigned URL for download
    async fn presigned_url(&self, key: &str, expires_in_seconds: u64) -> Result<String>;
}

/// Trait for AI providers (optional, for embeddings/completions)
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Provider name (e.g., "openai", "anthropic", "local")
    fn name(&self) -> &str;

    /// Generate embeddings for text
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;

    /// Generate text completion
    async fn complete(&self, prompt: &str) -> Result<String>;
}

/// Trait for entities that can be synced
pub trait Syncable {
    /// Get entity ID for sync tracking
    fn sync_id(&self) -> &str;

    /// Get last modified timestamp
    fn last_modified(&self) -> i64;

    /// Get entity type (table name)
    fn entity_type(&self) -> &str;
}

/// Trait for entities that can be searched
pub trait Searchable {
    /// Get searchable text content
    fn searchable_text(&self) -> Vec<String>;

    /// Get entity type for filtering
    fn search_type(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAuthProvider;

    #[async_trait]
    impl AuthProvider for MockAuthProvider {
        fn name(&self) -> &str {
            "mock"
        }

        async fn authenticate(&self, _credentials: &str) -> Result<String> {
            Ok("mock_token".to_string())
        }

        async fn validate_token(&self, _token: &str) -> Result<String> {
            Ok("user:123".to_string())
        }

        async fn refresh_token(&self, _token: &str) -> Result<String> {
            Ok("new_mock_token".to_string())
        }

        async fn revoke_token(&self, _token: &str) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_auth_provider_trait() {
        let provider = MockAuthProvider;
        assert_eq!(provider.name(), "mock");

        let token = provider.authenticate("test").await.unwrap();
        assert_eq!(token, "mock_token");

        let user_id = provider.validate_token(&token).await.unwrap();
        assert_eq!(user_id, "user:123");
    }
}

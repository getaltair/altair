//! Error types for Altair

use thiserror::Error;

/// Altair's main error type
#[derive(Debug, Error)]
pub enum Error {
    /// Database operation failed
    #[error("Database error: {0}")]
    Database(String),

    /// Storage operation failed (S3, file system, etc.)
    #[error("Storage error: {0}")]
    Storage(String),

    /// Authentication or authorization failed
    #[error("Auth error: {0}")]
    Auth(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Entity not found
    #[error("Not found: {entity_type} with id {id}")]
    NotFound { entity_type: String, id: String },

    /// Sync operation failed
    #[error("Sync error: {0}")]
    Sync(String),

    /// Search/embedding operation failed
    #[error("Search error: {0}")]
    Search(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Generic internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Wraps any other error type
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Altair's Result type alias
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a database error
    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }

    /// Create a storage error
    pub fn storage(msg: impl Into<String>) -> Self {
        Self::Storage(msg.into())
    }

    /// Create an auth error
    pub fn auth(msg: impl Into<String>) -> Self {
        Self::Auth(msg.into())
    }

    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Create a not found error
    pub fn not_found(entity_type: impl Into<String>, id: impl Into<String>) -> Self {
        Self::NotFound {
            entity_type: entity_type.into(),
            id: id.into(),
        }
    }

    /// Create a sync error
    pub fn sync(msg: impl Into<String>) -> Self {
        Self::Sync(msg.into())
    }

    /// Create a search error
    pub fn search(msg: impl Into<String>) -> Self {
        Self::Search(msg.into())
    }

    /// Create a serialization error
    pub fn serialization(msg: impl Into<String>) -> Self {
        Self::Serialization(msg.into())
    }

    /// Create an internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

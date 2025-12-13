//! User Credential type - Password storage

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// UserCredential - Stores password hash for local authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredential {
    pub id: Option<Thing>,
    pub user: Thing,           // Reference to user record (unique)
    pub password_hash: String, // Argon2id PHC format hash
    pub updated_at: DateTime<Utc>,
}

// No Default implementation - credentials must always be created explicitly

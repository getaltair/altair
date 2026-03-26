use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Database-backed user record
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    #[allow(dead_code)]
    pub updated_at: DateTime<Utc>,
}

/// Database-backed session record
#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub device_info: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Registration request payload
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    #[validate(length(min = 1, max = 100))]
    pub display_name: String,
}

/// Login request payload
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 128))]
    pub password: String,
}

/// Profile update request payload
#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
}

/// Public-safe user profile (no password_hash)
#[derive(Debug, Clone, Serialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        UserProfile {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            created_at: user.created_at,
        }
    }
}

/// Auth response returned on register/login
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserProfile,
}

/// Response for the PowerSync token endpoint
#[derive(Debug, Serialize)]
pub struct PowerSyncTokenResponse {
    /// Signed JWT for PowerSync authentication
    pub token: String,
    /// PowerSync service URL the client should connect to
    pub powersync_url: String,
}

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
#[allow(dead_code)]
#[derive(Debug, Clone, sqlx::FromRow)]
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
    #[validate(length(min = 8))]
    pub password: String,
    pub display_name: String,
}

/// Login request payload
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
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

/// Auth response returned on register/login
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserProfile,
}

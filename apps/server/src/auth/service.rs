use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{Duration, Utc};
use rand::RngCore;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use super::models::{Session, User};

/// Hash a plaintext password using Argon2id with default parameters
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {e}")))
}

/// Verify a plaintext password against an Argon2id hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return Ok(false),
    };
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Generate a cryptographically random session token
///
/// Returns a tuple of (raw_token_hex, sha256_hash_hex).
/// The raw token is sent to the client; only the hash is stored in the database.
pub fn generate_session_token() -> (String, String) {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    let raw_token = hex::encode(bytes);
    let hash = Sha256::digest(bytes);
    let token_hash = hex::encode(hash);
    (raw_token, token_hash)
}

/// Create a new user with a hashed password
///
/// Returns `AppError::Conflict` if the email is already registered.
pub async fn create_user(
    pool: &PgPool,
    email: &str,
    display_name: &str,
    password: &str,
) -> Result<User, AppError> {
    let password_hash = hash_password(password)?;

    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, display_name, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, email, display_name, password_hash, created_at, updated_at
        "#,
    )
    .bind(email)
    .bind(display_name)
    .bind(&password_hash)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Email already registered".to_string())
        }
        _ => AppError::Database(e),
    })
}

/// Authenticate a user by email and password
///
/// Returns the user if credentials are valid, or `AppError::Unauthorized` otherwise.
pub async fn authenticate(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, display_name, password_hash, created_at, updated_at FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    let user = match user {
        Some(u) => u,
        None => return Err(AppError::Unauthorized("Invalid credentials".to_string())),
    };

    let valid = verify_password(password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    Ok(user)
}

/// Create a new session for a user and return the raw bearer token
pub async fn create_session(
    pool: &PgPool,
    user_id: Uuid,
    ttl_hours: u64,
    device_info: Option<String>,
) -> Result<String, AppError> {
    let (raw_token, token_hash) = generate_session_token();
    let expires_at = Utc::now() + Duration::hours(ttl_hours as i64);

    sqlx::query(
        r#"
        INSERT INTO sessions (user_id, token_hash, expires_at, device_info)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(user_id)
    .bind(&token_hash)
    .bind(expires_at)
    .bind(device_info.as_deref())
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(raw_token)
}

/// Validate a raw bearer token and return the associated user and session
///
/// Decodes the hex token, computes its SHA-256 hash, and looks up
/// the session in the database. Returns `AppError::Unauthorized` if
/// the token is invalid, the session is not found, or the session has expired.
pub async fn validate_session(
    pool: &PgPool,
    raw_token: &str,
) -> Result<(User, Session), AppError> {
    let token_bytes = hex::decode(raw_token)
        .map_err(|_| AppError::Unauthorized("Invalid token format".to_string()))?;
    let hash = Sha256::digest(&token_bytes);
    let token_hash = hex::encode(hash);

    // Look up session by token hash
    let session = sqlx::query_as::<_, Session>(
        "SELECT id, user_id, token_hash, expires_at, device_info, created_at FROM sessions WHERE token_hash = $1",
    )
    .bind(&token_hash)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::Unauthorized("Invalid or expired session".to_string()))?;

    // Check expiry
    if session.expires_at < Utc::now() {
        return Err(AppError::Unauthorized("Session expired".to_string()));
    }

    // Look up user
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, display_name, password_hash, created_at, updated_at FROM users WHERE id = $1",
    )
    .bind(session.user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

    Ok((user, session))
}

/// Delete a session by its token hash, effectively logging the user out
pub async fn invalidate_session(pool: &PgPool, token_hash: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM sessions WHERE token_hash = $1")
        .bind(token_hash)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    Ok(())
}

/// Get all household IDs a user belongs to
pub async fn get_user_household_ids(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Uuid>, AppError> {
    let rows: Vec<(Uuid,)> = sqlx::query_as(
        "SELECT household_id FROM household_memberships WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(rows.into_iter().map(|(id,)| id).collect())
}

/// Get a user by their ID
pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<User, AppError> {
    sqlx::query_as::<_, User>(
        "SELECT id, email, display_name, password_hash, created_at, updated_at FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))
}

/// Update a user's display name
pub async fn update_user_display_name(
    pool: &PgPool,
    user_id: Uuid,
    display_name: &str,
) -> Result<User, AppError> {
    sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET display_name = $1, updated_at = now()
        WHERE id = $2
        RETURNING id, email, display_name, password_hash, created_at, updated_at
        "#,
    )
    .bind(display_name)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))
}

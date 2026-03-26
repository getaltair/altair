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

/// Hash a plaintext password using Argon2 with default parameters
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {e}")))
}

/// Verify a plaintext password against an Argon2 hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("Corrupt password hash in database: {e}")))?;
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
            AppError::Conflict("Registration could not be completed".to_string())
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
        let _ = sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(session.id)
            .execute(pool)
            .await;
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

/// Delete a session by its primary key
pub async fn invalidate_session_by_id(pool: &PgPool, session_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM sessions WHERE id = $1")
        .bind(session_id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        tracing::warn!("Session invalidation by ID matched zero rows");
    }

    Ok(())
}

/// Delete a session by its token hash, effectively logging the user out
#[allow(dead_code)]
pub async fn invalidate_session(pool: &PgPool, token_hash: &str) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM sessions WHERE token_hash = $1")
        .bind(token_hash)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        tracing::warn!("Session invalidation matched zero rows");
    }

    Ok(())
}

/// Get all household IDs a user belongs to
#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    #[test]
    fn hash_and_verify_roundtrip() {
        let password = "correct-horse-battery-staple";
        let hash = hash_password(password).expect("hashing should succeed");

        assert!(
            verify_password(password, &hash).expect("verify should not error"),
            "same password should verify as true"
        );
        assert!(
            !verify_password("wrong-password", &hash).expect("verify should not error"),
            "wrong password should verify as false"
        );
    }

    #[test]
    fn verify_corrupt_hash_returns_error() {
        let result = verify_password("password", "not-a-valid-hash");
        assert!(
            matches!(result, Err(AppError::Internal(_))),
            "corrupt hash should return AppError::Internal"
        );
    }

    #[test]
    fn verify_empty_password_does_not_panic() {
        let hash = hash_password("").expect("hashing empty string should succeed");
        let result = verify_password("", &hash);
        assert!(
            result.is_ok(),
            "verifying empty password should not panic or error"
        );
        assert!(result.unwrap(), "empty password should match its own hash");

        let mismatch = verify_password("not-empty", &hash).expect("verify should not error");
        assert!(!mismatch, "non-empty password should not match empty-password hash");
    }

    #[test]
    fn generate_token_length() {
        let (raw_token, token_hash) = generate_session_token();
        assert_eq!(raw_token.len(), 64, "raw_token hex should be 64 chars (32 bytes)");
        assert_eq!(token_hash.len(), 64, "token_hash hex should be 64 chars (SHA-256)");
    }

    #[test]
    fn generate_token_uniqueness() {
        let (token_a, _) = generate_session_token();
        let (token_b, _) = generate_session_token();
        assert_ne!(token_a, token_b, "two generated tokens should differ");
    }

    #[test]
    fn generate_token_hash_correctness() {
        let (raw_token, token_hash) = generate_session_token();
        let decoded = hex::decode(&raw_token).expect("raw_token should be valid hex");
        let expected_hash = hex::encode(Sha256::digest(&decoded));
        assert_eq!(
            token_hash, expected_hash,
            "token_hash should equal SHA-256 of the raw token bytes"
        );
    }
}

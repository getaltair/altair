use argon2::{
    Argon2, Params,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::Utc;
use hex;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use rand::RngExt;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use super::models::{Claims, TokenResponse};
use crate::error::AppError;

/// Hash a password with Argon2id using OWASP minimum parameters.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let params = Params::new(19 * 1024, 2, 1, None)
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))
}

/// Verify a password against its stored hash.
///
/// P4-003: distinguishes between a corrupt/unparseable stored hash (Internal error
/// with a tracing::error log) and a genuine wrong password (Unauthorized, no log).
/// Previously both cases silently mapped to Unauthorized, hiding DB corruption.
pub fn verify_password(password: &str, hash: &str) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| {
        tracing::error!(
            error = %e,
            "Failed to parse stored password hash — possible DB corruption or migration bug"
        );
        AppError::Internal(anyhow::anyhow!("Failed to parse stored password hash"))
    })?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)
}

/// Issue an RS256 access token (15-minute expiry).
///
/// P4-022: `email` is included in the claims so that client-side JWT decoding
/// (e.g. hooks.server.ts) can populate user context without a separate API call.
pub fn issue_access_token(
    user_id: Uuid,
    email: &str,
    household_ids: Vec<Uuid>,
    enc_key: &EncodingKey,
) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        household_ids,
        iat: now,
        exp: now + 900, // 15 minutes
    };
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some("altair-v1".to_string());
    encode(&header, &claims, enc_key)
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))
}

/// Generate a refresh token: returns (raw_hex, sha256_hash_hex).
pub fn generate_refresh_token() -> (String, String) {
    let mut bytes = [0u8; 32];
    rand::rng().fill(&mut bytes);
    let raw = hex::encode(bytes);
    let hash = hex::encode(Sha256::digest(raw.as_bytes()));
    (raw, hash)
}

/// Store a new refresh token in the database.
pub async fn store_refresh_token(
    pool: &PgPool,
    user_id: Uuid,
    token_hash: &str,
    device_hint: Option<&str>,
) -> Result<(), AppError> {
    let expires_at = Utc::now() + chrono::Duration::days(7);
    sqlx::query(
        "INSERT INTO refresh_tokens (user_id, token_hash, device_hint, expires_at) \
         VALUES ($1, $2, $3, $4)",
    )
    .bind(user_id)
    .bind(token_hash)
    .bind(device_hint)
    .bind(expires_at)
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;
    Ok(())
}

/// Rotate a refresh token: revoke old, issue new pair.
///
/// P4-010: logs a warning when a revoked token is presented (possible replay attack).
/// P4-022: fetches user email from the DB to embed in the new access token claim.
/// P4-023: removed the redundant `user_id`/`household_ids` parameters — the token
/// record already contains `user_id`, eliminating the double DB round-trip in the
/// refresh handler.
pub async fn rotate_refresh_token(
    pool: &PgPool,
    raw_token: &str,
    enc_key: &EncodingKey,
) -> Result<TokenResponse, AppError> {
    let token_hash = hex::encode(Sha256::digest(raw_token.as_bytes()));

    // Look up the token record (includes user_id for downstream queries).
    let record = sqlx::query_as::<_, RefreshTokenRecord>(
        "SELECT id, user_id, revoked_at, expires_at FROM refresh_tokens WHERE token_hash = $1",
    )
    .bind(&token_hash)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?
    .ok_or(AppError::Unauthorized)?;

    // P4-010: log revoked token reuse as a security event before returning 401.
    if record.revoked_at.is_some() {
        tracing::warn!(
            token_hash = %token_hash,
            "Revoked refresh token presented — possible replay attack"
        );
        return Err(AppError::Unauthorized);
    }
    if record.expires_at < Utc::now() {
        return Err(AppError::Unauthorized);
    }

    // Revoke old token.
    sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE id = $1")
        .bind(record.id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

    // P4-022: fetch user email to embed in the new access token.
    let (email,): (String,) =
        sqlx::query_as("SELECT email FROM users WHERE id = $1 AND deleted_at IS NULL")
            .bind(record.user_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

    let household_ids: Vec<Uuid> = vec![]; // placeholder until household sync is wired
    let access_token = issue_access_token(record.user_id, &email, household_ids, enc_key)?;
    let (raw_new, hash_new) = generate_refresh_token();
    store_refresh_token(pool, record.user_id, &hash_new, None).await?;

    Ok(TokenResponse {
        access_token,
        refresh_token: raw_new,
        token_type: "Bearer".to_string(),
    })
}

/// Revoke a refresh token by its raw value.
///
/// P4-009: logs a warning when no matching token is found (token missing or
/// already revoked). Previously, zero-row updates silently returned Ok(()),
/// making logout replay attempts undetectable.
pub async fn revoke_refresh_token(pool: &PgPool, raw_token: &str) -> Result<(), AppError> {
    let token_hash = hex::encode(Sha256::digest(raw_token.as_bytes()));
    let result = sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE token_hash = $1")
        .bind(&token_hash)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

    if result.rows_affected() == 0 {
        tracing::warn!(
            "revoke_refresh_token: no token found for hash — \
             token may already be revoked or never existed (possible replay)"
        );
    }
    Ok(())
}

// Internal helper struct for typed row fetch.
#[derive(sqlx::FromRow)]
struct RefreshTokenRecord {
    id: Uuid,
    user_id: Uuid,
    revoked_at: Option<chrono::DateTime<Utc>>,
    expires_at: chrono::DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Tests (S012-T)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

    fn generate_test_rsa_pem() -> (String, String) {
        use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
        use rsa::rand_core::OsRng;
        let private_key = rsa::RsaPrivateKey::new(&mut OsRng, 2048).unwrap();
        let pub_key = rsa::RsaPublicKey::from(&private_key);
        let priv_pem = private_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .unwrap()
            .to_string();
        let pub_pem = pub_key
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .unwrap();
        (priv_pem, pub_pem)
    }

    #[test]
    fn test_hash_and_verify_password_correct() {
        let hash = hash_password("correct_password_123").unwrap();
        assert!(verify_password("correct_password_123", &hash).is_ok());
    }

    #[test]
    fn test_verify_wrong_password_returns_unauthorized() {
        let hash = hash_password("correct_password_123").unwrap();
        let result = verify_password("wrong_password", &hash);
        assert!(matches!(result, Err(AppError::Unauthorized)));
    }

    #[test]
    fn test_verify_corrupt_hash_returns_internal() {
        // A malformed hash string should return Internal, not Unauthorized (P4-003).
        let result = verify_password("any_password", "not-a-valid-argon2-hash");
        assert!(
            matches!(result, Err(AppError::Internal(_))),
            "corrupt hash must return Internal, not Unauthorized"
        );
    }

    #[test]
    fn test_issue_access_token_round_trips() {
        let (priv_pem, pub_pem) = generate_test_rsa_pem();
        let enc_key = EncodingKey::from_rsa_pem(priv_pem.as_bytes()).unwrap();
        let dec_key = DecodingKey::from_rsa_pem(pub_pem.as_bytes()).unwrap();

        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        let token =
            issue_access_token(user_id, "user@example.com", vec![household_id], &enc_key)
                .unwrap();
        assert!(!token.is_empty());

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_required_spec_claims(&["sub", "exp"]);

        let decoded = decode::<Claims>(&token, &dec_key, &validation).unwrap();
        assert_eq!(decoded.claims.sub, user_id);
        assert_eq!(decoded.claims.email, "user@example.com");
        assert_eq!(decoded.claims.household_ids, vec![household_id]);
    }

    #[test]
    fn test_generate_refresh_token_non_empty_and_differ() {
        let (raw, hash) = generate_refresh_token();
        assert!(!raw.is_empty());
        assert!(!hash.is_empty());
        assert_ne!(raw, hash);
    }
}

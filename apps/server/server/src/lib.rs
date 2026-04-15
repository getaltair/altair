use anyhow::Context;
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as BASE64URL_NOPAD;
use jsonwebtoken::{DecodingKey, EncodingKey};
use rsa::pkcs8::DecodePrivateKey;
use rsa::traits::PublicKeyParts;
use sqlx::PgPool;

pub mod auth;
pub mod config;
pub mod contracts;
pub mod core;
pub mod db;
pub mod error;
pub mod guidance;
pub mod knowledge;
pub mod routes;
pub mod sync;
pub mod tracking;

/// Application state shared across all Axum handlers.
///
/// `enc_key` signs new JWTs; `dec_key` verifies incoming JWTs from clients;
/// `jwks_json` is the pre-serialized JWKS response served to PowerSync.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub enc_key: EncodingKey,
    pub dec_key: DecodingKey,
    /// Pre-serialized JSON string for `GET /api/auth/.well-known/jwks.json`.
    pub jwks_json: String,
    /// Whether to set `Secure` on auth cookies. True in production (APP_ENV=production).
    pub secure_cookies: bool,
}

/// Parse the PEM private key and construct the full AppState.
///
/// Separated from `main` so it can be called in tests without a real DB pool.
pub fn build_app_state(pool: PgPool, pem: &str, secure_cookies: bool) -> anyhow::Result<AppState> {
    // Parse the PKCS#8 PEM private key.
    let private_key = rsa::RsaPrivateKey::from_pkcs8_pem(pem).context(
        "Failed to parse RSA private key from JWT_PRIVATE_KEY — \
         ensure the PEM is PKCS#8 format (openssl genrsa 2048 | openssl pkcs8 -topk8 -nocrypt)",
    )?;

    // Build jsonwebtoken signing key from the PEM bytes directly.
    let enc_key = EncodingKey::from_rsa_pem(pem.as_bytes())
        .context("Failed to construct EncodingKey from RSA PEM")?;

    // Derive the public key and build the DecodingKey from its PEM.
    let public_key = rsa::RsaPublicKey::from(&private_key);
    let pub_pem = {
        use rsa::pkcs8::EncodePublicKey;
        public_key
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .context("Failed to encode RSA public key as PEM")?
    };
    let dec_key = DecodingKey::from_rsa_pem(pub_pem.as_bytes())
        .context("Failed to construct DecodingKey from RSA public key PEM")?;

    // Extract modulus (n) and exponent (e) for the JWKS JSON.
    // RFC 7517 requires base64url (no padding) encoding.
    let n_b64 = BASE64URL_NOPAD.encode(public_key.n().to_bytes_be());
    let e_b64 = BASE64URL_NOPAD.encode(public_key.e().to_bytes_be());

    let jwks = serde_json::json!({
        "keys": [{
            "kty": "RSA",
            "use": "sig",
            "alg": "RS256",
            "kid": "altair-v1",
            "n": n_b64,
            "e": e_b64,
        }]
    });
    let jwks_json = serde_json::to_string(&jwks).context("Failed to serialise JWKS JSON")?;

    Ok(AppState {
        db: pool,
        enc_key,
        dec_key,
        jwks_json,
        secure_cookies,
    })
}

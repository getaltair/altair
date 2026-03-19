//! JWT configuration and token signing for PowerSync authentication.

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use rsa::RsaPrivateKey;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey, LineEnding};
use rsa::traits::PublicKeyParts;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)] // TODO(P4-003): Remove when token endpoint uses JwtError
pub enum JwtError {
	#[error("RSA key generation failed: {0}")]
	KeyGeneration(String),

	#[error("PEM error: {0}")]
	PemError(String),

	#[error("Token signing failed: {0}")]
	SigningError(#[from] jsonwebtoken::errors::Error),

	#[error("File I/O error: {0}")]
	IoError(#[from] std::io::Error),
}

#[derive(Clone)]
#[allow(dead_code)] // TODO(P4-003): Remove when AppState holds JwtConfig
pub struct JwtConfig {
	pub encoding_key: EncodingKey,
	pub kid: String,
	pub audience: String,
	pub expiry_secs: i64,
	pub powersync_url: String,
	pub rsa_n_b64url: String,
	pub rsa_e_b64url: String,
}

#[derive(Debug, Serialize)]
struct PowerSyncClaims {
	sub: String,
	aud: String,
	iat: i64,
	exp: i64,
}

const DEV_KEY_PATH: &str = ".dev-keys/powersync-jwt.pem";

/// Load JWT config from env vars, or generate dev keys if `POWERSYNC_RSA_PRIVATE_KEY` is unset.
///
/// # Panics
///
/// Panics if `POWERSYNC_JWT_AUDIENCE` or `POWERSYNC_URL` is not set.
#[allow(dead_code)] // TODO(P4-003): Remove when AppState initialization calls this
pub fn load_or_generate_jwt_config() -> JwtConfig {
	// Required env vars — panic if missing
	let audience = env::var("POWERSYNC_JWT_AUDIENCE").expect("POWERSYNC_JWT_AUDIENCE must be set");
	let powersync_url = env::var("POWERSYNC_URL").expect("POWERSYNC_URL must be set");

	// Optional env vars with defaults
	let kid = env::var("POWERSYNC_JWT_KID").unwrap_or_else(|_| "altair-dev-1".to_string());
	let expiry_secs: i64 = env::var("POWERSYNC_JWT_EXPIRY_SECS")
		.ok()
		.and_then(|v| v.parse().ok())
		.unwrap_or(3600);

	// Obtain PEM bytes: env var > dev file > generate
	let pem_bytes = match env::var("POWERSYNC_RSA_PRIVATE_KEY") {
		Ok(pem_string) => {
			tracing::info!("Using RSA private key from POWERSYNC_RSA_PRIVATE_KEY env var");
			pem_string.into_bytes()
		}
		Err(_) => load_or_generate_dev_key(),
	};

	// Validate PEM early — fail fast if invalid
	let encoding_key = EncodingKey::from_rsa_pem(&pem_bytes)
		.expect("Invalid RSA PEM key — check POWERSYNC_RSA_PRIVATE_KEY format");

	// Parse private key to extract public key components for JWKS
	let private_key = rsa::RsaPrivateKey::from_pkcs8_pem(
		std::str::from_utf8(&pem_bytes).expect("PEM is not valid UTF-8"),
	)
	.expect("Failed to parse PKCS#8 PEM private key");

	let n_bytes = private_key.n().to_bytes_be();
	let e_bytes = private_key.e().to_bytes_be();
	let rsa_n_b64url = URL_SAFE_NO_PAD.encode(&n_bytes);
	let rsa_e_b64url = URL_SAFE_NO_PAD.encode(&e_bytes);

	tracing::info!(kid = %kid, expiry_secs = %expiry_secs, "JWT config loaded");

	JwtConfig {
		encoding_key,
		kid,
		audience,
		expiry_secs,
		powersync_url,
		rsa_n_b64url,
		rsa_e_b64url,
	}
}

fn load_or_generate_dev_key() -> Vec<u8> {
	let path = Path::new(DEV_KEY_PATH);

	if path.exists() {
		tracing::info!(path = %path.display(), "Loading dev RSA key from file");
		return fs::read(path).expect("Failed to read dev key file");
	}

	tracing::info!("No RSA key found — generating 2048-bit dev key");
	let mut rng = rand::thread_rng();
	let private_key =
		RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate 2048-bit RSA key");

	let pem = private_key
		.to_pkcs8_pem(LineEnding::LF)
		.expect("Failed to encode private key to PKCS#8 PEM");

	// Ensure directory exists
	if let Some(parent) = path.parent() {
		fs::create_dir_all(parent).expect("Failed to create .dev-keys/ directory");
	}

	fs::write(path, pem.as_bytes()).expect("Failed to write dev key to disk");
	tracing::info!(path = %path.display(), "Dev RSA key saved");

	pem.as_bytes().to_vec()
}

/// Sign an RS256 PowerSync JWT for the given user.
///
/// Sets `iat` to 30s in the past as a clock skew buffer.
#[allow(dead_code)] // TODO(P4-003): Remove when token endpoint calls this
pub fn sign_powersync_token(config: &JwtConfig, user_id: &str) -> Result<String, JwtError> {
	let mut header = Header::new(Algorithm::RS256);
	header.kid = Some(config.kid.clone());

	let now = Utc::now().timestamp();
	let claims = PowerSyncClaims {
		sub: user_id.to_string(),
		aud: config.audience.clone(),
		iat: now - 30, // clock skew buffer
		exp: now + config.expiry_secs,
	};

	let token = encode(&header, &claims, &config.encoding_key)?;
	Ok(token)
}

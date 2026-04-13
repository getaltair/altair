use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// P4-002: RegisterRequest contains plaintext password — Debug omitted to prevent
// accidental secret emission in logs, panic messages, and test output.
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub display_name: String,
    pub password: String,
}

impl fmt::Debug for RegisterRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegisterRequest")
            .field("email", &self.email)
            .field("display_name", &self.display_name)
            .field("password", &"[redacted]")
            .finish()
    }
}

// P4-002: LoginRequest contains plaintext password — Debug redacted.
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl fmt::Debug for LoginRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoginRequest")
            .field("email", &self.email)
            .field("password", &"[redacted]")
            .finish()
    }
}

// P4-002: TokenResponse contains live tokens — Debug redacted.
#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

impl fmt::Debug for TokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenResponse")
            .field("access_token", &"[redacted]")
            .field("refresh_token", &"[redacted]")
            .field("token_type", &self.token_type)
            .finish()
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum RegisterResponse {
    Active(TokenResponse),
    Pending { message: String },
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub is_admin: bool,
}

// P4-022: `email` claim added so that hooks.server.ts can populate locals.user.email
// without a separate DB round-trip.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub household_ids: Vec<Uuid>,
    pub iat: i64,
    pub exp: i64,
}

/// Populated by the AuthUser extractor from a valid JWT.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub household_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct JwksResponse {
    pub keys: Vec<JwkKey>,
}

#[derive(Debug, Serialize)]
pub struct JwkKey {
    pub kty: String,
    #[serde(rename = "use")]
    pub use_: String,
    pub alg: String,
    pub kid: String,
    pub n: String,
    pub e: String,
}

// P4-002: RefreshRequest contains a live refresh token — Debug redacted.
#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: Option<String>,
}

impl fmt::Debug for RefreshRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RefreshRequest")
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "[redacted]"),
            )
            .finish()
    }
}

// P4-002: LogoutRequest contains a live refresh token — Debug redacted.
#[derive(Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: Option<String>,
}

impl fmt::Debug for LogoutRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogoutRequest")
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "[redacted]"),
            )
            .finish()
    }
}

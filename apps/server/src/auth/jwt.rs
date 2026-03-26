// PowerSync JWT claims and token generation

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::config::Config;
use crate::error::AppError;

/// JWT claims embedded in PowerSync authentication tokens.
///
/// The `sub` and `user_id` fields both carry the user's UUID so that
/// PowerSync can reference the user in `token_parameters`.
/// `household_ids` provides the list of households the user belongs to
/// for sync-rule bucket filtering.
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerSyncClaims {
    /// Standard JWT subject (user ID as string)
    pub sub: String,
    /// Issued-at timestamp (seconds since UNIX epoch)
    pub iat: u64,
    /// Expiration timestamp (seconds since UNIX epoch, 5 minutes from iat)
    pub exp: u64,
    /// User ID repeated for PowerSync token_parameters access
    pub user_id: String,
    /// Household UUIDs the user belongs to, for sync-rule bucket filtering
    pub household_ids: Vec<String>,
}

/// Generate an HS256-signed JWT for PowerSync authentication.
///
/// The token expires 5 minutes (300 seconds) after issuance.
/// Clients are expected to refresh the token before expiry.
pub fn generate_powersync_token(
    config: &Config,
    user_id: Uuid,
    household_ids: Vec<Uuid>,
) -> Result<String, AppError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock is before UNIX epoch")
        .as_secs();

    let claims = PowerSyncClaims {
        sub: user_id.to_string(),
        iat: now,
        exp: now + 300, // 5 minutes
        user_id: user_id.to_string(),
        household_ids: household_ids.iter().map(|id| id.to_string()).collect(),
    };

    let key = EncodingKey::from_secret(config.jwt_secret.as_bytes());
    let header = Header::new(Algorithm::HS256);

    encode(&header, &claims, &key)
        .map_err(|e| AppError::Internal(format!("Failed to encode PowerSync JWT: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{decode, DecodingKey, Validation};

    fn test_config() -> Config {
        Config {
            database_url: "test".to_string(),
            port: 3000,
            log_level: "info".to_string(),
            environment: "development".to_string(),
            db_min_conn: 5,
            db_max_conn: 20,
            db_timeout_sec: 30,
            session_ttl_hours: 72,
            jwt_secret: "test_jwt_secret_for_powersync".to_string(),
            powersync_url: "http://localhost:8080".to_string(),
        }
    }

    #[test]
    fn test_generate_powersync_token_roundtrip() {
        let config = test_config();
        let user_id = Uuid::new_v4();
        let household_a = Uuid::new_v4();
        let household_b = Uuid::new_v4();
        let household_ids = vec![household_a, household_b];

        let token = generate_powersync_token(&config, user_id, household_ids.clone())
            .expect("token generation should succeed");

        // Decode and validate
        let key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_required_spec_claims(&["sub", "iat", "exp"]);

        let token_data = decode::<PowerSyncClaims>(&token, &key, &validation)
            .expect("token should decode successfully");

        let claims = token_data.claims;
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.user_id, user_id.to_string());
        assert_eq!(claims.exp - claims.iat, 300, "token should expire in 5 minutes");
        assert_eq!(claims.household_ids.len(), 2);
        assert!(claims.household_ids.contains(&household_a.to_string()));
        assert!(claims.household_ids.contains(&household_b.to_string()));
    }

    #[test]
    fn test_generate_powersync_token_empty_households() {
        let config = test_config();
        let user_id = Uuid::new_v4();

        let token = generate_powersync_token(&config, user_id, vec![])
            .expect("token generation should succeed with empty households");

        let key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_required_spec_claims(&["sub", "iat", "exp"]);

        let token_data = decode::<PowerSyncClaims>(&token, &key, &validation)
            .expect("token should decode successfully");

        assert!(
            token_data.claims.household_ids.is_empty(),
            "household_ids should be empty"
        );
    }

    #[test]
    fn test_generate_powersync_token_wrong_key_fails_decode() {
        let config = test_config();
        let user_id = Uuid::new_v4();

        let token = generate_powersync_token(&config, user_id, vec![])
            .expect("token generation should succeed");

        let wrong_key = DecodingKey::from_secret(b"wrong_secret");
        let validation = Validation::new(Algorithm::HS256);

        let result = decode::<PowerSyncClaims>(&token, &wrong_key, &validation);
        assert!(result.is_err(), "decoding with wrong key should fail");
    }
}

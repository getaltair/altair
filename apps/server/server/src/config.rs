use anyhow::Context;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    #[allow(dead_code)]
    pub database_url: String,
    /// RSA private key in PEM format, decoded from the base64-encoded JWT_PRIVATE_KEY env var.
    /// Generate with: openssl genrsa 2048 | base64 -w0
    pub jwt_private_key_pem: String,
    /// Whether to set the `Secure` attribute on auth cookies. Enabled when APP_ENV=production.
    /// Disable for local HTTP dev by setting APP_ENV=development (or omitting the variable).
    pub secure_cookies: bool,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_string());

        let database_url = std::env::var("DATABASE_URL")
            .context("DATABASE_URL environment variable is required")?;

        let jwt_private_key_b64 = std::env::var("JWT_PRIVATE_KEY").context(
            "JWT_PRIVATE_KEY is required — generate with: openssl genrsa 2048 | base64 -w0",
        )?;

        let pem_bytes = BASE64
            .decode(jwt_private_key_b64.trim())
            .context("JWT_PRIVATE_KEY must be a valid base64-encoded RSA PEM string")?;

        let jwt_private_key_pem = String::from_utf8(pem_bytes)
            .context("JWT_PRIVATE_KEY decoded bytes are not valid UTF-8")?;

        let secure_cookies = std::env::var("APP_ENV")
            .map(|v| v == "production")
            .unwrap_or(false);

        Ok(Self {
            bind_addr,
            database_url,
            jwt_private_key_pem,
            secure_cookies,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
    use rsa::{pkcs8::EncodePrivateKey, traits::PublicKeyParts};

    /// Generate a fresh 2048-bit RSA PEM and return it base64-encoded.
    /// Used to set JWT_PRIVATE_KEY in tests without embedding a hardcoded key.
    fn make_test_key_b64() -> String {
        let mut rng = rsa::rand_core::OsRng;
        let private_key =
            rsa::RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate test RSA key");
        let pem = private_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .expect("failed to encode RSA key as PKCS#8 PEM");
        BASE64.encode(pem.as_bytes())
    }

    // SAFETY note for all tests below: serial_test enforces that all tests in
    // this module run sequentially, preventing concurrent env var access.

    #[serial_test::serial]
    #[test]
    fn missing_database_url_returns_error() {
        let saved_db = std::env::var("DATABASE_URL").ok();
        let saved_jwt = std::env::var("JWT_PRIVATE_KEY").ok();
        unsafe {
            std::env::remove_var("DATABASE_URL");
            std::env::set_var("JWT_PRIVATE_KEY", make_test_key_b64());
        }

        let result = Config::from_env();
        assert!(
            result.is_err(),
            "Expected error when DATABASE_URL is missing"
        );

        unsafe {
            match saved_db {
                Some(val) => std::env::set_var("DATABASE_URL", val),
                None => std::env::remove_var("DATABASE_URL"),
            }
            match saved_jwt {
                Some(val) => std::env::set_var("JWT_PRIVATE_KEY", val),
                None => std::env::remove_var("JWT_PRIVATE_KEY"),
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn missing_jwt_private_key_returns_error() {
        let saved_db = std::env::var("DATABASE_URL").ok();
        let saved_jwt = std::env::var("JWT_PRIVATE_KEY").ok();
        unsafe {
            std::env::set_var("DATABASE_URL", "postgres://test:test@localhost/testdb");
            std::env::remove_var("JWT_PRIVATE_KEY");
        }

        let result = Config::from_env();
        assert!(
            result.is_err(),
            "Expected error when JWT_PRIVATE_KEY is missing"
        );

        unsafe {
            match saved_db {
                Some(val) => std::env::set_var("DATABASE_URL", val),
                None => std::env::remove_var("DATABASE_URL"),
            }
            match saved_jwt {
                Some(val) => std::env::set_var("JWT_PRIVATE_KEY", val),
                None => std::env::remove_var("JWT_PRIVATE_KEY"),
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn invalid_base64_jwt_private_key_returns_error() {
        let saved_db = std::env::var("DATABASE_URL").ok();
        let saved_jwt = std::env::var("JWT_PRIVATE_KEY").ok();
        unsafe {
            std::env::set_var("DATABASE_URL", "postgres://test:test@localhost/testdb");
            std::env::set_var("JWT_PRIVATE_KEY", "not-valid-base64!!!");
        }

        let result = Config::from_env();
        assert!(
            result.is_err(),
            "Expected error when JWT_PRIVATE_KEY is invalid base64"
        );

        unsafe {
            match saved_db {
                Some(val) => std::env::set_var("DATABASE_URL", val),
                None => std::env::remove_var("DATABASE_URL"),
            }
            match saved_jwt {
                Some(val) => std::env::set_var("JWT_PRIVATE_KEY", val),
                None => std::env::remove_var("JWT_PRIVATE_KEY"),
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn happy_path_returns_correct_values() {
        let saved_db = std::env::var("DATABASE_URL").ok();
        let saved_bind = std::env::var("BIND_ADDR").ok();
        let saved_jwt = std::env::var("JWT_PRIVATE_KEY").ok();

        let key_b64 = make_test_key_b64();
        unsafe {
            std::env::set_var("DATABASE_URL", "postgres://test:test@localhost/testdb");
            std::env::set_var("BIND_ADDR", "127.0.0.1:9000");
            std::env::set_var("JWT_PRIVATE_KEY", &key_b64);
        }

        let result = Config::from_env();
        assert!(result.is_ok(), "Expected Ok when all env vars are set");
        let config = result.unwrap();
        assert_eq!(config.database_url, "postgres://test:test@localhost/testdb");
        assert_eq!(config.bind_addr, "127.0.0.1:9000");
        assert!(
            config.jwt_private_key_pem.contains("PRIVATE KEY"),
            "PEM should contain PRIVATE KEY header"
        );

        unsafe {
            match saved_db {
                Some(val) => std::env::set_var("DATABASE_URL", val),
                None => std::env::remove_var("DATABASE_URL"),
            }
            match saved_bind {
                Some(val) => std::env::set_var("BIND_ADDR", val),
                None => std::env::remove_var("BIND_ADDR"),
            }
            match saved_jwt {
                Some(val) => std::env::set_var("JWT_PRIVATE_KEY", val),
                None => std::env::remove_var("JWT_PRIVATE_KEY"),
            }
        }
    }

    #[serial_test::serial]
    #[test]
    fn missing_bind_addr_falls_back_to_default() {
        let saved_db = std::env::var("DATABASE_URL").ok();
        let saved_bind = std::env::var("BIND_ADDR").ok();
        let saved_jwt = std::env::var("JWT_PRIVATE_KEY").ok();

        let key_b64 = make_test_key_b64();
        unsafe {
            std::env::set_var("DATABASE_URL", "postgres://test:test@localhost/testdb");
            std::env::remove_var("BIND_ADDR");
            std::env::set_var("JWT_PRIVATE_KEY", &key_b64);
        }

        let result = Config::from_env();
        assert!(result.is_ok(), "Expected Ok when DATABASE_URL is set");
        let config = result.unwrap();
        assert_eq!(config.bind_addr, "0.0.0.0:8000");

        unsafe {
            match saved_db {
                Some(val) => std::env::set_var("DATABASE_URL", val),
                None => std::env::remove_var("DATABASE_URL"),
            }
            match saved_bind {
                Some(val) => std::env::set_var("BIND_ADDR", val),
                None => std::env::remove_var("BIND_ADDR"),
            }
            match saved_jwt {
                Some(val) => std::env::set_var("JWT_PRIVATE_KEY", val),
                None => std::env::remove_var("JWT_PRIVATE_KEY"),
            }
        }
    }

    // --- S007-T: Key parsing and JWT round-trip tests ---

    /// Builds EncodingKey + DecodingKey from a freshly generated RSA PEM and
    /// verifies they can round-trip a JWT with a `sub` claim.
    #[test]
    fn valid_rsa_pem_parses_and_jwt_round_trips() {
        use jsonwebtoken::{
            decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
        };
        use rsa::pkcs8::DecodePrivateKey;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Claims {
            sub: String,
            exp: i64,
        }

        // Generate key
        let private_key = rsa::RsaPrivateKey::new(&mut rsa::rand_core::OsRng, 2048)
            .expect("failed to generate test RSA key");
        let pem = private_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .expect("encode PEM");

        // Build jsonwebtoken keys from the PEM
        let enc_key =
            EncodingKey::from_rsa_pem(pem.as_bytes()).expect("EncodingKey::from_rsa_pem failed");

        // Derive public key for DecodingKey
        let public_key = rsa::RsaPublicKey::from(private_key.as_ref().clone());
        // Encode public key as PEM for DecodingKey
        // (jsonwebtoken accepts SPKI/PKCS#8 public key PEM)
        let pub_pem = {
            use rsa::pkcs8::EncodePublicKey;
            public_key
                .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
                .expect("encode public key PEM")
        };
        let dec_key = DecodingKey::from_rsa_pem(pub_pem.as_bytes())
            .expect("DecodingKey::from_rsa_pem failed");

        // Encode a JWT
        let claims = Claims {
            sub: "test-user-id".to_string(),
            exp: 9999999999,
        };
        let token = encode(&Header::new(Algorithm::RS256), &claims, &enc_key).expect("encode JWT");

        // Decode and verify
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = false;
        let decoded = decode::<Claims>(&token, &dec_key, &validation).expect("decode JWT");
        assert_eq!(decoded.claims.sub, "test-user-id");
    }

    /// Verifies that the JWKS JSON built from an RSA public key contains the
    /// required `kty`, `alg`, and `use` fields.
    #[test]
    fn jwks_json_contains_required_fields() {
        use rsa::traits::PublicKeyParts;

        let private_key = rsa::RsaPrivateKey::new(&mut rsa::rand_core::OsRng, 2048)
            .expect("failed to generate test RSA key");
        let public_key = rsa::RsaPublicKey::from(private_key.as_ref().clone());

        // Build JWKS JSON — same logic used in main.rs AppState construction
        let n_bytes = public_key.n().to_bytes_be();
        let e_bytes = public_key.e().to_bytes_be();
        let n_b64 = BASE64URL_NOPAD.encode(&n_bytes);
        let e_b64 = BASE64URL_NOPAD.encode(&e_bytes);

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
        let jwks_json = serde_json::to_string(&jwks).expect("serialize JWKS");

        assert!(
            jwks_json.contains("\"kty\":\"RSA\""),
            "missing kty:RSA; got: {jwks_json}"
        );
        assert!(
            jwks_json.contains("\"alg\":\"RS256\""),
            "missing alg:RS256; got: {jwks_json}"
        );
        assert!(
            jwks_json.contains("\"use\":\"sig\""),
            "missing use:sig; got: {jwks_json}"
        );
        assert!(
            jwks_json.contains("\"kid\":\"altair-v1\""),
            "missing kid; got: {jwks_json}"
        );
    }

    // Base64url (no padding) engine used in JWKS and in main.rs
    use base64::engine::general_purpose::URL_SAFE_NO_PAD as BASE64URL_NOPAD;
}

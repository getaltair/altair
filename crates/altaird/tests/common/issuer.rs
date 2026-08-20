//! An identity provider that exists only in this process.
//!
//! Every key is generated per test and every token is minted here, so nothing
//! resembling a private key is ever committed and the suite runs offline.
//!
//! **`tests/token_validation.rs` keeps its own, richer provider on purpose.**
//! That lane tests the cache: it needs a provider that can be taken away, one
//! that counts fetches, and one that serves a different key set on the second
//! request. This one serves a fixed document and never changes, because what
//! the daemon's tests need from an issuer is a token that verifies and a token
//! that does not.

#![allow(dead_code)]

use std::net::SocketAddr;
use std::sync::Arc;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use ring::rand::SystemRandom;
use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair, KeyPair};
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

pub const ISSUER: &str = "https://auth.example.test/application/o/altair/";
pub const AUDIENCE: &str = "altair";

/// A signing key the provider publishes and the tests mint with.
pub struct Key {
    kid: String,
    encoding: EncodingKey,
    x: String,
    y: String,
}

impl Key {
    pub fn generate() -> Self {
        let rng = SystemRandom::new();
        let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
            .expect("generate a key pair");
        let pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref(), &rng)
            .expect("read the key pair back");

        // Uncompressed SEC1 point: 0x04 || X || Y, 32 bytes each on P-256.
        let point = pair.public_key().as_ref();
        assert_eq!(point.len(), 65, "unexpected public point encoding");

        Self {
            kid: "test-key".to_owned(),
            encoding: EncodingKey::from_ec_der(pkcs8.as_ref()),
            x: URL_SAFE_NO_PAD.encode(&point[1..33]),
            y: URL_SAFE_NO_PAD.encode(&point[33..65]),
        }
    }

    fn jwk(&self) -> Value {
        json!({
            "kty": "EC",
            "crv": "P-256",
            "alg": "ES256",
            "use": "sig",
            "kid": self.kid,
            "x": self.x,
            "y": self.y,
        })
    }

    fn mint(&self, claims: &Value) -> String {
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(self.kid.clone());
        jsonwebtoken::encode(&header, claims, &self.encoding).expect("mint")
    }

    /// A token that verifies, for a subject, valid for an hour.
    pub fn token_for(&self, subject: &str) -> String {
        let now = now();
        self.mint(&json!({
            "iss": ISSUER, "aud": AUDIENCE, "sub": subject,
            "iat": now, "exp": now + 3600,
        }))
    }

    /// A token that verified yesterday.
    ///
    /// Well past the sixty seconds of clock skew `auth` allows, so this is
    /// expired rather than borderline.
    pub fn expired_token_for(&self, subject: &str) -> String {
        let long_ago = now() - 86_400;
        self.mint(&json!({
            "iss": ISSUER, "aud": AUDIENCE, "sub": subject,
            "iat": long_ago - 3600, "exp": long_ago,
        }))
    }
}

fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("after the epoch")
        .as_secs() as i64
}

/// The provider's JWKS endpoint, on a local port.
pub struct Issuer {
    addr: SocketAddr,
    task: Option<JoinHandle<()>>,
}

impl Issuer {
    /// Serve one key set, for as long as this value is held.
    pub async fn publishing(key: &Key) -> Self {
        let document = json!({ "keys": [key.jwk()] }).to_string();
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let document = Arc::new(document);

        let task = tokio::spawn(async move {
            while let Ok((mut socket, _)) = listener.accept().await {
                let document = Arc::clone(&document);
                tokio::spawn(async move {
                    // Enough of the request to know one arrived. The answer
                    // does not depend on the path.
                    let mut scratch = [0_u8; 1024];
                    let _ = socket.read(&mut scratch).await;
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\
                         Content-Length: {}\r\nConnection: close\r\n\r\n{document}",
                        document.len()
                    );
                    let _ = socket.write_all(response.as_bytes()).await;
                    let _ = socket.shutdown().await;
                });
            }
        });

        Self {
            addr,
            task: Some(task),
        }
    }

    #[must_use]
    pub fn jwks_uri(&self) -> String {
        format!("http://{}/jwks", self.addr)
    }
}

impl Drop for Issuer {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

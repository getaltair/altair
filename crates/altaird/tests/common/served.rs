//! An instance on a port, with a credential it will accept.
//!
//! **Extracted because there are two callers and there will be more.** The
//! served surface is the only place two of the write path's requirements are
//! observable at all — a batch is never all or nothing, and a refusal reveals
//! nothing, which DR-004 extends to the status code — and it is also the only
//! way to prove that a client and this instance actually talk to each other.
//! Standing one up needs a key, a key set served without a network, an
//! authenticator and a listener, and none of that is worth having twice.
//!
//! The provider exists only here. Per DR-005 the instance validates tokens and
//! never issues them, so a test that needs one has to mint it, and the source
//! being a trait is what lets it do so without a network.

#![allow(dead_code)]

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use altair_proto::v1;
use altaird::auth::{Authenticator, IssuerConfig, JwksSource, JwksUnavailable, KeyCache, Member};
use altaird::service::Instance;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use ring::rand::SystemRandom;
use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair, KeyPair};
use serde_json::{Value, json};
use tokio::net::TcpListener;

use super::World;

pub const ISSUER: &str = "https://auth.example.test/application/o/altair/";
pub const AUDIENCE: &str = "altair";

/// A "now" far enough from zero that a token can expire without the arithmetic
/// going negative.
pub const NOW: i64 = 1_780_000_000;

// --- a provider that exists only here ------------------------------------

pub struct Key {
    kid: String,
    encoding: EncodingKey,
    jwk: Value,
}

pub fn key() -> Key {
    let rng = SystemRandom::new();
    let pkcs8 =
        EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).expect("key pair");
    let pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref(), &rng)
        .expect("read back");
    let point = pair.public_key().as_ref();
    let kid = "test".to_owned();
    Key {
        jwk: json!({
            "kty": "EC", "crv": "P-256", "alg": "ES256", "use": "sig", "kid": kid,
            "x": URL_SAFE_NO_PAD.encode(&point[1..33]),
            "y": URL_SAFE_NO_PAD.encode(&point[33..65]),
        }),
        kid,
        encoding: EncodingKey::from_ec_der(pkcs8.as_ref()),
    }
}

impl Key {
    pub fn mint(&self, subject: &str, exp: i64) -> String {
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(self.kid.clone());
        let claims = json!({
            "iss": ISSUER, "aud": AUDIENCE, "sub": subject,
            "iat": exp - 3600, "exp": exp,
        });
        jsonwebtoken::encode(&header, &claims, &self.encoding).expect("mint")
    }
}

type Fetching<'a> = Pin<Box<dyn Future<Output = Result<JwkSet, JwksUnavailable>> + Send + 'a>>;

/// Keys served without a network, which is what the source being a trait is
/// for.
pub struct Keys(JwkSet);

impl JwksSource for Keys {
    fn fetch(&self) -> Fetching<'_> {
        let set = self.0.clone();
        Box::pin(async move { Ok(set) })
    }
}

// --- an instance on a port -----------------------------------------------

pub struct Served {
    pub world: World,
    pub key: Key,
    pub addr: SocketAddr,
}

impl Served {
    pub async fn new() -> Self {
        let world = World::new().await;
        let key = key();
        let keys: JwkSet =
            serde_json::from_value(json!({ "keys": [key.jwk.clone()] })).expect("jwks");

        let auth = Authenticator::new(
            IssuerConfig {
                issuer: ISSUER.into(),
                audience: AUDIENCE.into(),
            },
            KeyCache::new(Arc::new(Keys(keys))),
            world.db.pool.clone(),
        );

        let instance = Instance::new(Arc::new(auth), world.write.clone(), world.capacity.clone());

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        tokio::spawn(async move {
            tonic::transport::Server::builder()
                .add_service(v1::altair_server::AltairServer::new(instance))
                .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
                .await
                .ok();
        });

        Self { world, key, addr }
    }

    pub async fn client(&self) -> v1::altair_client::AltairClient<tonic::transport::Channel> {
        let channel = tonic::transport::Channel::from_shared(format!("http://{}", self.addr))
            .expect("uri")
            .connect()
            .await
            .expect("connect");
        v1::altair_client::AltairClient::new(channel)
    }

    /// A request carrying a credential, or none at all.
    pub fn request<T>(&self, message: T, credential: Option<&str>) -> tonic::Request<T> {
        let mut request = tonic::Request::new(message);
        if let Some(token) = credential {
            request.metadata_mut().insert(
                "authorization",
                format!("Bearer {token}").parse().expect("header"),
            );
        }
        request
    }

    /// Where a client should be pointed.
    #[must_use]
    pub fn url(&self) -> String {
        format!("http://{}", self.addr)
    }

    /// A credential this instance will accept, for `member`.
    pub fn token_for(&self, member: &Member) -> String {
        // Far in the future, so the real clock accepts it.
        self.key.mint(member.subject(), NOW + 10 * 365 * 86_400)
    }

    pub fn token_for_one(&self) -> String {
        self.token_for(&self.world.one)
    }
}

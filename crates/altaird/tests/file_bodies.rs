//! File bodies: `PutBody`, `GetBody`, and the ordering Wave 2.3 exists to
//! keep — bytes before the record, always.
//!
//! The done-when is a kill between the two writes: a body uploaded through
//! `PutBody` with no `Submit` create ever following it. There is no process
//! to actually kill, so the kill is simulated structurally — the two RPCs are
//! independent, and a real client dying between them looks exactly like
//! issuing the first and never the second.

mod common;

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use altair_proto::v1;
use altaird::auth::{Authenticator, IssuerConfig, JwksSource, JwksUnavailable, KeyCache};
use altaird::objects::BodyId;
use altaird::service::Instance;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use common::*;
use futures::StreamExt;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use ring::rand::SystemRandom;
use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair, KeyPair};
use serde_json::{Value, json};
use sqlx::Row;
use tokio::net::TcpListener;
use tonic::Code;
use uuid::Uuid;

const ISSUER: &str = "https://auth.example.test/application/o/altair/";
const AUDIENCE: &str = "altair";

/// A "now" far enough from zero that a token can expire without the
/// arithmetic going negative.
const NOW: i64 = 1_780_000_000;

// --- a provider that exists only here, mirroring submission_call.rs -------

struct Key {
    kid: String,
    encoding: EncodingKey,
    jwk: Value,
}

fn key() -> Key {
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
    fn mint(&self, subject: &str, exp: i64) -> String {
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

struct Keys(JwkSet);

impl JwksSource for Keys {
    fn fetch(&self) -> Fetching<'_> {
        let set = self.0.clone();
        Box::pin(async move { Ok(set) })
    }
}

// --- an instance on a port, mirroring submission_call.rs ------------------

struct Served {
    world: World,
    key: Key,
    addr: SocketAddr,
}

impl Served {
    async fn new() -> Self {
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

        // Reuses the world's own write path rather than building a second
        // one, so PutBody and Submit see the same object store.
        let instance = Instance::new(Arc::new(auth), world.write.clone());

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

    async fn client(&self) -> v1::altair_client::AltairClient<tonic::transport::Channel> {
        let channel = tonic::transport::Channel::from_shared(format!("http://{}", self.addr))
            .expect("uri")
            .connect()
            .await
            .expect("connect");
        v1::altair_client::AltairClient::new(channel)
    }

    fn request<T>(&self, message: T, credential: Option<&str>) -> tonic::Request<T> {
        let mut request = tonic::Request::new(message);
        if let Some(token) = credential {
            request.metadata_mut().insert(
                "authorization",
                format!("Bearer {token}").parse().expect("header"),
            );
        }
        request
    }

    fn token_for_one(&self) -> String {
        self.key
            .mint(self.world.one.subject(), NOW + 10 * 365 * 86_400)
    }

    fn token_for_two(&self) -> String {
        self.key
            .mint(self.world.two.subject(), NOW + 10 * 365 * 86_400)
    }
}

fn intent(action: v1::intent::Action) -> v1::Intent {
    v1::Intent {
        intent_id: Uuid::new_v4().as_bytes().to_vec(),
        action: Some(action),
    }
}

fn file_content(body_id: Uuid, media_type: &str) -> v1::EntityContent {
    v1::EntityContent {
        title: Some("a file".into()),
        specific: Some(v1::entity_content::Specific::File(v1::FileContent {
            body_id: Some(body_id.as_bytes().to_vec()),
            media_type: Some(media_type.into()),
            ..Default::default()
        })),
        ..Default::default()
    }
}

// --- streaming call helpers ------------------------------------------------

async fn put_body(
    served: &Served,
    token: &str,
    id: Uuid,
    bytes: &[u8],
) -> Result<v1::PutBodyAck, tonic::Status> {
    let chunk = v1::BodyChunk {
        body_id: id.as_bytes().to_vec(),
        data: bytes.to_vec(),
    };
    let request = served.request(tokio_stream::iter(vec![chunk]), Some(token));
    Ok(served.client().await.put_body(request).await?.into_inner())
}

async fn get_body_bytes(
    served: &Served,
    token: &str,
    entity_id: Uuid,
) -> Result<Vec<u8>, tonic::Status> {
    let request = served.request(
        v1::BodyRequest {
            entity_id: entity_id.as_bytes().to_vec(),
        },
        Some(token),
    );
    let mut stream = served.client().await.get_body(request).await?.into_inner();
    let mut bytes = Vec::new();
    while let Some(chunk) = stream.message().await? {
        bytes.extend_from_slice(&chunk.data);
    }
    Ok(bytes)
}

async fn file_row(world: &World, id: Uuid) -> Option<(Uuid, Option<String>, Option<i64>)> {
    sqlx::query("SELECT body_id, media_type, byte_size FROM file WHERE entity_id = $1")
        .bind(id)
        .fetch_optional(&world.db.pool)
        .await
        .expect("query")
        .map(|r| {
            (
                r.try_get("body_id").expect("body_id"),
                r.try_get("media_type").expect("media_type"),
                r.try_get("byte_size").expect("byte_size"),
            )
        })
}

async fn stored_bodies(world: &World) -> Vec<BodyId> {
    world
        .write
        .objects()
        .enumerate()
        .map(|entry| entry.expect("enumerate").id)
        .collect()
        .await
}

// --- the happy path ---------------------------------------------------------

#[tokio::test]
async fn putting_a_body_then_creating_the_file_round_trips() {
    let served = Served::new().await;
    let token = served.token_for_one();
    let body_id = Uuid::new_v4();
    let bytes = vec![7u8; 100 * 1024];

    let ack = put_body(&served, &token, body_id, &bytes)
        .await
        .expect("put body");
    assert_eq!(ack.body_id, body_id.as_bytes().to_vec());
    assert_eq!(ack.bytes_received, bytes.len() as u64);

    let entity_id = Uuid::new_v4();
    let acks = served
        .client()
        .await
        .submit(served.request(
            v1::SubmitRequest {
                intents: vec![intent(create_entity(
                    entity_id,
                    file_content(body_id, "text/plain"),
                ))],
            },
            Some(&token),
        ))
        .await
        .expect("submit")
        .into_inner()
        .acknowledgements;
    assert!(
        matches!(
            acks[0].outcome,
            Some(v1::acknowledgement::Outcome::Applied(_))
        ),
        "a body that was uploaded is a body a file may name: {:?}",
        acks[0].outcome
    );

    let row = file_row(&served.world, entity_id)
        .await
        .expect("a file row");
    assert_eq!(row.0, body_id);
    assert_eq!(row.1.as_deref(), Some("text/plain"));
    assert_eq!(row.2, Some(bytes.len() as i64));

    let read_back = get_body_bytes(&served, &token, entity_id)
        .await
        .expect("get body");
    assert_eq!(read_back, bytes);
}

// --- the done-when: a kill between the two writes ---------------------------

#[tokio::test]
async fn a_kill_between_put_body_and_the_create_leaves_collectable_garbage_and_no_broken_entity() {
    let served = Served::new().await;
    let token = served.token_for_one();
    let body_id = Uuid::new_v4();

    put_body(&served, &token, body_id, b"never claimed")
        .await
        .expect("put body");

    // The kill: no Submit create ever follows. A real client dying between
    // the two RPCs looks exactly like this.

    let bodies = stored_bodies(&served.world).await;
    assert!(
        bodies.iter().any(|id| id.as_bytes() == body_id.as_bytes()),
        "the bytes are durable and enumerable — collectable garbage, which is \
         Wave 2.4's to sweep"
    );

    let n: i64 = sqlx::query("SELECT count(*) AS n FROM file")
        .fetch_one(&served.world.db.pool)
        .await
        .expect("count")
        .try_get("n")
        .expect("n");
    assert_eq!(n, 0, "nothing ever pointed at the orphaned bytes");
}

// --- the refusal: naming a body that was never uploaded ---------------------

#[tokio::test]
async fn a_create_naming_an_unuploaded_body_is_refused_and_nothing_is_written() {
    let served = Served::new().await;
    let token = served.token_for_one();
    let entity_id = Uuid::new_v4();
    let never_uploaded = Uuid::new_v4();

    let acks = served
        .client()
        .await
        .submit(served.request(
            v1::SubmitRequest {
                intents: vec![intent(create_entity(
                    entity_id,
                    file_content(never_uploaded, "text/plain"),
                ))],
            },
            Some(&token),
        ))
        .await
        .expect("submit")
        .into_inner()
        .acknowledgements;

    match &acks[0].outcome {
        Some(v1::acknowledgement::Outcome::Refused(r)) => {
            assert_eq!(r.reason, v1::RefusalReason::Malformed as i32);
        }
        other => panic!("expected a refusal, got {other:?}"),
    }

    assert!(file_row(&served.world, entity_id).await.is_none());
    assert!(
        stored_bodies(&served.world).await.is_empty(),
        "nothing was ever written to the object store, so there is nothing to sweep"
    );
}

// --- audience: the same not-found as a nonexistent entity -------------------

#[tokio::test]
async fn get_body_outside_the_requesters_audience_is_the_same_not_found_as_nonexistence() {
    let served = Served::new().await;
    let owner_token = served.token_for_one();
    let outsider_token = served.token_for_two();

    let body_id = Uuid::new_v4();
    put_body(&served, &owner_token, body_id, b"private bytes")
        .await
        .expect("put body");

    let entity_id = Uuid::new_v4();
    let acks = served
        .client()
        .await
        .submit(served.request(
            v1::SubmitRequest {
                intents: vec![intent(create_entity(
                    entity_id,
                    file_content(body_id, "text/plain"),
                ))],
            },
            Some(&owner_token),
        ))
        .await
        .expect("submit")
        .into_inner()
        .acknowledgements;
    assert!(matches!(
        acks[0].outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));

    let against_real = get_body_bytes(&served, &outsider_token, entity_id)
        .await
        .expect_err("private to its author");
    let against_nonexistent = get_body_bytes(&served, &outsider_token, Uuid::new_v4())
        .await
        .expect_err("does not exist at all");

    assert_eq!(against_real.code(), Code::NotFound);
    assert_eq!(against_real.code(), against_nonexistent.code());
    assert_eq!(against_real.message(), against_nonexistent.message());
    assert!(
        against_real.message().is_empty(),
        "any detail would say the thing the single refusal reason exists to avoid saying"
    );
}

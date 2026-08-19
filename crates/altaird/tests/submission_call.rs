//! The submission call, served.
//!
//! Two of Wave 2.1's requirements are observable only here and are not testable
//! from an internal function: **a batch is never all or nothing**, and **a
//! refusal reveals nothing, which DR-004 extends to the status code**. This is
//! also token validation's first caller, which was otherwise built and unused.

mod common;

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use altair_proto::v1;
use altaird::auth::{Authenticator, IssuerConfig, JwksSource, JwksUnavailable, KeyCache};
use altaird::service::Instance;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use common::*;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use ring::rand::SystemRandom;
use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair, KeyPair};
use serde_json::{Value, json};
use tokio::net::TcpListener;
use tonic::Code;
use uuid::Uuid;

const ISSUER: &str = "https://auth.example.test/application/o/altair/";
const AUDIENCE: &str = "altair";

/// A "now" far enough from zero that a token can expire without the arithmetic
/// going negative.
const NOW: i64 = 1_780_000_000;

// --- a provider that exists only here ------------------------------------

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

/// Keys served without a network, which is what the source being a trait is
/// for.
struct Keys(JwkSet);

impl JwksSource for Keys {
    fn fetch(&self) -> Fetching<'_> {
        let set = self.0.clone();
        Box::pin(async move { Ok(set) })
    }
}

// --- an instance on a port -----------------------------------------------

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

        let instance = Instance::new(Arc::new(auth), world.write.clone(), world.db.pool.clone());

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

    /// A request carrying a credential, or none at all.
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
        // Far in the future, so the real clock accepts it.
        self.key
            .mint(self.world.one.subject(), NOW + 10 * 365 * 86_400)
    }
}

fn intent(action: v1::intent::Action) -> v1::Intent {
    v1::Intent {
        intent_id: Uuid::new_v4().as_bytes().to_vec(),
        action: Some(action),
    }
}

fn note(title: &str) -> v1::EntityContent {
    v1::EntityContent {
        title: Some(title.into()),
        specific: Some(v1::entity_content::Specific::Note(
            v1::NoteContent::default(),
        )),
        ..Default::default()
    }
}

fn nowhere() -> altaird::store::ids::EntityId {
    altaird::store::ids::EntityId::from_uuid(Uuid::new_v4())
}

// --- the two things only the wire can show -------------------------------

#[tokio::test]
async fn a_submission_whose_every_intent_is_refused_still_answers_success() {
    let served = Served::new().await;
    let mut client = served.client().await;

    let response = client
        .submit(served.request(
            v1::SubmitRequest {
                intents: vec![
                    intent(edit_entity(nowhere(), 1, note("nowhere"))),
                    intent(edit_entity(nowhere(), 1, note("nowhere either"))),
                ],
            },
            Some(&served.token_for_one()),
        ))
        .await
        .expect("a submission of refusals is a successful submission");

    let acks = response.into_inner().acknowledgements;
    assert_eq!(acks.len(), 2);
    assert!(acks.iter().all(is_not_available));
    assert!(
        acks.iter().all(|a| refused(a).detail.is_empty()),
        "any other status, or any detail, would say at the transport the thing \
         the single refusal reason exists to avoid saying"
    );
}

#[tokio::test]
async fn a_batch_is_never_all_or_nothing() {
    let served = Served::new().await;
    let mut client = served.client().await;

    let first = Uuid::new_v4();
    let last = Uuid::new_v4();

    let acks = client
        .submit(served.request(
            v1::SubmitRequest {
                intents: vec![
                    intent(create_entity(first, note("first"))),
                    intent(edit_entity(nowhere(), 1, note("nowhere"))),
                    intent(create_entity(last, note("last"))),
                ],
            },
            Some(&served.token_for_one()),
        ))
        .await
        .expect("submitted")
        .into_inner()
        .acknowledgements;

    assert_eq!(acks.len(), 3, "one acknowledgement per intent, in order");
    assert!(matches!(
        acks[0].outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));
    assert!(is_not_available(&acks[1]));
    assert!(
        matches!(
            acks[2].outcome,
            Some(v1::acknowledgement::Outcome::Applied(_))
        ),
        "two hundred files where the hundredth fails leaves ninety-nine captured"
    );

    assert_eq!(
        served
            .world
            .title(altaird::store::ids::EntityId::from_uuid(last))
            .await
            .as_deref(),
        Some("last")
    );
}

// --- the credential ------------------------------------------------------

#[tokio::test]
async fn a_valid_token_writes_as_the_member_it_names() {
    let served = Served::new().await;
    let mut client = served.client().await;
    let id = Uuid::new_v4();

    client
        .submit(served.request(
            v1::SubmitRequest {
                intents: vec![intent(create_entity(id, note("mine")))],
            },
            Some(&served.token_for_one()),
        ))
        .await
        .expect("submitted");

    // Member two cannot act on it, so the author was member one.
    let ack = served
        .world
        .submit(
            &served.world.two,
            edit_entity(
                altaird::store::ids::EntityId::from_uuid(id),
                1,
                note("theirs"),
            ),
        )
        .await;
    assert!(is_not_available(&ack));
}

#[tokio::test]
async fn an_absent_credential_reaches_no_query_surface() {
    let served = Served::new().await;
    let mut client = served.client().await;

    let status = client
        .submit(served.request(
            v1::SubmitRequest {
                intents: vec![intent(create_entity(Uuid::new_v4(), note("unbidden")))],
            },
            None,
        ))
        .await
        .expect_err("an unauthenticated request is not served");

    assert_eq!(status.code(), Code::Unauthenticated);
    assert_eq!(
        served.world.changes().await.len(),
        0,
        "unauthenticated reached the write path"
    );
}

#[tokio::test]
async fn an_expired_credential_and_a_forged_one_are_the_same_wait() {
    let served = Served::new().await;
    let mut client = served.client().await;

    let expired = served.key.mint(served.world.one.subject(), NOW - 86_400);
    let forged = key().mint(served.world.one.subject(), NOW + 10 * 365 * 86_400);

    let mut statuses = Vec::new();
    for credential in [expired, forged] {
        let status = client
            .submit(served.request(
                v1::SubmitRequest {
                    intents: vec![intent(create_entity(Uuid::new_v4(), note("held")))],
                },
                Some(&credential),
            ))
            .await
            .expect_err("neither is served");
        statuses.push((status.code(), status.message().to_owned()));
    }

    assert_eq!(
        statuses[0], statuses[1],
        "a forged token and an expired one are literally indistinguishable, and \
         DR-005 defines both as the wait"
    );
    assert_eq!(statuses[0].0, Code::Unauthenticated);
}

// --- the two that are still not served --------------------------------------

#[tokio::test]
async fn the_two_calls_this_build_does_not_serve_say_so_deliberately() {
    let served = Served::new().await;
    let mut client = served.client().await;
    let token = served.token_for_one();

    let codes = vec![
        client
            .query(served.request(v1::QueryRequest::default(), Some(&token)))
            .await
            .expect_err("query")
            .code(),
        client
            .get_health(served.request(v1::HealthRequest::default(), Some(&token)))
            .await
            .expect_err("health")
            .code(),
    ];

    assert!(
        codes.iter().all(|c| *c == Code::Unimplemented),
        "the two are absent on purpose, and waiting will not clear it: {codes:?}"
    );
}

/// `Changes` is served as of Wave 3.2 — `tests/changes.rs` covers its
/// per-member assembly. What is worth asserting here is the same thing
/// `put_body_and_get_body_are_no_longer_among_the_unserved` asserts for those
/// two: a call reaches real handling, not the blanket `Unimplemented` the
/// remaining two still answer with, and an unauthenticated caller still gets
/// nowhere near it.
#[tokio::test]
async fn changes_is_no_longer_among_the_unserved() {
    let served = Served::new().await;
    let mut client = served.client().await;
    let token = served.token_for_one();

    let response = client
        .changes(served.request(v1::ChangesRequest::default(), Some(&token)))
        .await
        .expect("a default request is answerable, not unimplemented")
        .into_inner();
    assert!(matches!(
        response.outcome,
        Some(v1::changes_response::Outcome::Changes(_))
    ));

    let unauthenticated = client
        .changes(served.request(v1::ChangesRequest::default(), None))
        .await
        .expect_err("unauthenticated reaches no query surface")
        .code();
    assert_eq!(unauthenticated, Code::Unauthenticated);
}

/// `PutBody` and `GetBody` are served as of Wave 2.3 — `tests/file_bodies.rs`
/// covers their behaviour. What is still worth asserting here is that a
/// malformed call to either reaches real handling rather than falling through
/// to the blanket `Unimplemented` `Query` and `GetHealth` still answer with.
#[tokio::test]
async fn put_body_and_get_body_are_no_longer_among_the_unserved() {
    let served = Served::new().await;
    let mut client = served.client().await;
    let token = served.token_for_one();

    let put_body = client
        .put_body(served.request(
            tokio_stream::iter(Vec::<v1::BodyChunk>::new()),
            Some(&token),
        ))
        .await
        .expect_err("an empty upload is malformed, not unserved")
        .code();
    let get_body = client
        .get_body(served.request(v1::BodyRequest::default(), Some(&token)))
        .await
        .expect_err("a default request names no entity")
        .code();

    assert_eq!(put_body, Code::InvalidArgument);
    assert_eq!(get_body, Code::InvalidArgument);
}

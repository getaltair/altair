//! `GetHealth`, served without a member and answering honestly with the
//! derivation worker entirely absent (Wave 5).
//!
//! The done-when this lane owes: health answers with that worker absent
//! entirely. Every field here is computed from something already in the
//! store or the object store — nothing depends on `derivation_queue`, which
//! does not run in this build.

mod common;

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use altair_proto::v1;
use altaird::auth::{Authenticator, IssuerConfig, JwksSource, JwksUnavailable, KeyCache};
use altaird::service::Instance;
use common::*;
use jsonwebtoken::jwk::JwkSet;
use tokio::net::TcpListener;
use uuid::Uuid;

const ISSUER: &str = "https://auth.example.test/application/o/altair/";
const AUDIENCE: &str = "altair";

type Fetching<'a> = Pin<Box<dyn Future<Output = Result<JwkSet, JwksUnavailable>> + Send + 'a>>;

/// Holds no key, because `GetHealth` needs [`Instance::new`] to be given an
/// [`Authenticator`] but must never actually consult it — that is the whole
/// point of this lane. An empty set is enough to prove that: any call that
/// somehow did reach `Instance::member` would fail to find a signer and every
/// test here would fail on that instead of on the assertion it wrote.
struct EmptyKeys;

impl JwksSource for EmptyKeys {
    fn fetch(&self) -> Fetching<'_> {
        Box::pin(async { Ok(JwkSet { keys: Vec::new() }) })
    }
}

// --- an instance on a port, mirroring submission_call.rs -------------------

struct Served {
    world: World,
    addr: SocketAddr,
}

impl Served {
    async fn new() -> Self {
        let world = World::new().await;

        let auth = Authenticator::new(
            IssuerConfig {
                issuer: ISSUER.into(),
                audience: AUDIENCE.into(),
            },
            KeyCache::new(Arc::new(EmptyKeys)),
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

        Self { world, addr }
    }

    async fn client(&self) -> v1::altair_client::AltairClient<tonic::transport::Channel> {
        let channel = tonic::transport::Channel::from_shared(format!("http://{}", self.addr))
            .expect("uri")
            .connect()
            .await
            .expect("connect");
        v1::altair_client::AltairClient::new(channel)
    }

    async fn health(&self) -> v1::HealthResponse {
        self.client()
            .await
            .get_health(tonic::Request::new(v1::HealthRequest::default()))
            .await
            .expect("health answers")
            .into_inner()
    }
}

#[tokio::test]
async fn health_answers_with_no_credential_at_all() {
    let served = Served::new().await;

    // No "authorization" metadata at all, unlike every other call in this
    // suite — an infra probe cannot carry a bearer token, and this is the one
    // RPC that must not need one.
    let response = served
        .client()
        .await
        .get_health(tonic::Request::new(v1::HealthRequest::default()))
        .await;

    assert!(
        response.is_ok(),
        "GetHealth must answer without a member: {response:?}"
    );
}

#[tokio::test]
async fn the_object_store_answers_reachable_when_it_is() {
    let served = Served::new().await;
    let health = served.health().await;
    assert!(health.object_store_reachable);
}

#[tokio::test]
async fn no_inference_boundary_exists_yet_and_says_so() {
    let served = Served::new().await;
    let health = served.health().await;
    assert!(!health.bi_encoder_present);
    assert!(!health.cross_encoder_present);
}

#[tokio::test]
async fn derivation_outstanding_counts_active_entities_with_no_embedding() {
    let served = Served::new().await;

    let before = served.health().await.derivation_outstanding;
    served.world.note(&served.world.one, "captured").await;
    let after = served.health().await.derivation_outstanding;

    assert_eq!(
        after,
        before + 1,
        "nothing writes `embedding` in this build, so a new active entity is \
         outstanding work the moment it exists"
    );
}

#[tokio::test]
async fn intents_refused_counts_refusals_and_only_refusals() {
    let served = Served::new().await;

    let before = served.health().await.intents_refused;

    // A base counter against an entity that does not exist: refused, not
    // applied, and not a fault either.
    let nowhere = altaird::store::ids::EntityId::from_uuid(Uuid::new_v4());
    served
        .world
        .submit(
            &served.world.one,
            edit_entity(
                nowhere,
                1,
                v1::EntityContent {
                    title: Some("nowhere".into()),
                    specific: Some(v1::entity_content::Specific::Note(
                        v1::NoteContent::default(),
                    )),
                    ..Default::default()
                },
            ),
        )
        .await;

    let after = served.health().await.intents_refused;
    assert_eq!(after, before + 1);

    // An applied intent must not move this count.
    served.world.note(&served.world.one, "applied fine").await;
    let still = served.health().await.intents_refused;
    assert_eq!(still, after);
}

#[tokio::test]
async fn storage_bytes_free_reports_real_headroom() {
    let served = Served::new().await;
    let health = served.health().await;
    assert!(
        health.storage_bytes_free > 0,
        "the object root is a fresh temp directory on a real filesystem"
    );
}

//! A real daemon on a real socket, with a real client in front of it.
//!
//! Everything else in `tests/` reaches the instance by calling a function.
//! This is the harness for what could not be tested that way: configuration,
//! preconditions, the credential layer, the statuses that cross a wire, and
//! shutdown. It stands up [`altaird::daemon::start`] over a branched test
//! database and a temporary object root, and hands back a client that talks
//! gRPC.
//!
//! The channel is **lazy** on purpose. A client built with `connect()` fails
//! at construction when nothing is listening, which is not what a client does
//! in life and would make "an unreachable instance" untestable as a condition
//! a call meets. Lazily, a call against a dead address comes back as a
//! `Status` — the same shape an answered call's failure has — which is exactly
//! the comparison the wave has to make.

#![allow(dead_code)]

use std::net::SocketAddr;
use std::sync::Arc;

use altair_proto::v1;
use altaird::daemon::{Config, Daemon};
use altaird::testing::TestDb;
use tempfile::TempDir;
use tonic::transport::Channel;
use uuid::Uuid;

use super::issuer::{AUDIENCE, ISSUER, Issuer, Key};

pub type Client = v1::altair_client::AltairClient<Channel>;

/// A household, an issuer, and an instance serving them.
pub struct Running {
    pub daemon: Daemon,
    pub db: Arc<TestDb>,
    pub key: Key,
    pub household: Uuid,
    /// The membership identity behind the subject `one`.
    pub one: Uuid,
    /// The membership identity behind the subject `two`.
    pub two: Uuid,
    /// Held for its lifetime. The daemon opens its own object store over this
    /// directory, so it must outlive the daemon.
    object_root: TempDir,
    /// Held for its lifetime: the key cache fetches from it lazily, so it has
    /// to still be serving when the first authenticated call arrives.
    issuer: Issuer,
}

impl Running {
    pub async fn new() -> Self {
        Self::with(|config| config).await
    }

    /// The same, with the configuration altered before the daemon sees it.
    ///
    /// For the tests that ask what happens when a precondition is not met.
    pub async fn with(alter: impl FnOnce(Config) -> Config) -> Self {
        match Self::try_with(alter).await {
            Ok(running) => running,
            Err(e) => panic!("the daemon did not start: {e:#}"),
        }
    }

    /// The same again, answering rather than panicking, for the tests whose
    /// subject is a start that must fail.
    pub async fn try_with(alter: impl FnOnce(Config) -> Config) -> anyhow::Result<Self> {
        let db = TestDb::new().await;
        let household = Uuid::new_v4();
        sqlx::query("INSERT INTO household (id, name, created_at) VALUES ($1, $2, now())")
            .bind(household)
            .bind("test")
            .execute(&db.pool)
            .await
            .expect("household");
        let one = seed_member(&db.pool, household, "one").await;
        let two = seed_member(&db.pool, household, "two").await;

        let key = Key::generate();
        let issuer = Issuer::publishing(&key).await;
        let object_root = TempDir::new().expect("temp object root");

        let config = alter(Config {
            database_url: db.url(),
            object_root: object_root
                .path()
                .to_str()
                .expect("a temporary directory with a usable name")
                .to_owned(),
            issuer: ISSUER.to_owned(),
            audience: AUDIENCE.to_owned(),
            jwks_uri: issuer.jwks_uri(),
            // Zero: the harness asks the operating system for a port and reads
            // back which one it got, so parallel tests never collide.
            listen: "127.0.0.1:0".parse().expect("an address"),
        });

        let daemon = altaird::daemon::start(config).await?;

        Ok(Self {
            daemon,
            db,
            key,
            household,
            one,
            two,
            object_root,
            issuer,
        })
    }

    #[must_use]
    pub fn address(&self) -> SocketAddr {
        self.daemon.address()
    }

    /// A client pointed at this instance. See the module docs on laziness.
    #[must_use]
    pub fn client(&self) -> Client {
        client_for(self.address())
    }

    /// A credential for a subject, valid for an hour.
    #[must_use]
    pub fn token(&self, subject: &str) -> String {
        self.key.token_for(subject)
    }

    /// A request carrying a credential, or carrying none at all.
    pub fn request<T>(&self, message: T, credential: Option<&str>) -> tonic::Request<T> {
        request(message, credential)
    }

    /// Stop, and answer with whatever shutting down produced.
    pub async fn stop(self) -> anyhow::Result<()> {
        self.daemon.stop().await
    }
}

/// A client pointed anywhere, including at a port nothing is listening on.
#[must_use]
pub fn client_for(address: SocketAddr) -> Client {
    let channel = Channel::from_shared(format!("http://{address}"))
        .expect("uri")
        .connect_lazy();
    v1::altair_client::AltairClient::new(channel)
}

pub fn request<T>(message: T, credential: Option<&str>) -> tonic::Request<T> {
    let mut request = tonic::Request::new(message);
    if let Some(token) = credential {
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {token}").parse().expect("a header value"),
        );
    }
    request
}

async fn seed_member(pool: &sqlx::PgPool, household: Uuid, subject: &str) -> Uuid {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO membership (id, household_id, subject, display_name, joined_at) \
         VALUES ($1, $2, $3, $3, now())",
    )
    .bind(id)
    .bind(household)
    .bind(subject)
    .execute(pool)
    .await
    .expect("membership");
    id
}

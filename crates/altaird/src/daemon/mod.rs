//! `altaird`, as a process.
//!
//! Waves 1 through 3 produced libraries: a store, an object store, a write
//! path, a read path, a served service. Nothing had ever spoken gRPC over a
//! socket. This module is the composition — configuration, preconditions, the
//! server, the edge that resolves a credential, the shape background work will
//! attach to, and shutdown — and it deliberately adds no behaviour of its own.
//! Every question a request can ask was already answerable before this
//! existed; what was missing was a process to ask it of.
//!
//! # Startup, in order
//!
//! 1. **Configuration**, all of it, or a refusal naming every value that was
//!    missing ([`config`]).
//! 2. **Bind**, before anything expensive. An address already in use is the
//!    commonest way a start fails and it costs nothing to find out first.
//! 3. **Connect and migrate** the structured store ([`crate::store::connect`]).
//! 4. **Check the extensions** the schema depends on
//!    ([`crate::store::preflight`]).
//! 5. **Open the object store and prove it writable** ([`preflight`]).
//! 6. **Serve.**
//!
//! Steps 3 to 5 are the preconditions, and a failure in any of them is a
//! process that does not start. Nothing here contacts the identity provider —
//! [`preflight`] says why.
//!
//! # Shutdown
//!
//! One signal stops the server and every background task
//! ([`tasks`]). The server stops accepting and drains what is in flight; the
//! tasks are asked and joined with a deadline; the pool is closed last. An
//! intent that was acknowledged is durable because the acknowledgement and the
//! write it acknowledges commit in one transaction — see [`serve`].

pub mod config;
pub mod logging;
pub mod preflight;
pub mod serve;
pub mod tasks;

use std::net::SocketAddr;
use std::sync::Arc;

use sqlx::PgPool;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use crate::auth::{Authenticator, HttpJwks, IssuerConfig, KeyCache};
use crate::objects::{FilesystemObjectStore, ObjectStore, StorageCapacity};
use crate::service::Instance;
use crate::store;
use crate::write::WritePath;

pub use config::Config;
pub use tasks::{Shutdown, Tasks};

/// A running instance: what it is listening on, and how to stop it.
pub struct Daemon {
    address: SocketAddr,
    tasks: Tasks,
    serving: JoinHandle<Result<(), tonic::transport::Error>>,
    pool: PgPool,
}

impl Daemon {
    /// Where it is actually listening, which is not necessarily what was asked
    /// for: a configuration naming port zero gets one assigned.
    #[must_use]
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// Stop accepting, let what is in flight finish, stop the background
    /// tasks, and close the pool.
    ///
    /// # Errors
    ///
    /// If the server itself failed. A request that failed is not one of those,
    /// and neither is a background task that overran — that is reported by
    /// name in the log and does not make shutting down an error.
    pub async fn stop(self) -> anyhow::Result<()> {
        tracing::info!("shutting down");

        // `tasks.stop()` sends the one signal, which is also the server's, so
        // both are driven together rather than in sequence — waiting for the
        // server first would mean the signal had not been sent yet and nothing
        // would ever stop. What is ordered is what comes after: the pool is
        // closed only once both have finished with it.
        let (served, overran) = tokio::join!(self.serving, self.tasks.stop());

        for name in overran {
            tracing::warn!(task = name, "a background task did not stop in time");
        }

        // Last. Every user of it has finished by now.
        self.pool.close().await;

        match served {
            Ok(result) => result.map_err(Into::into),
            // The serving task itself panicked, which is not something a
            // request can cause and not something to be quiet about.
            Err(e) => Err(anyhow::anyhow!("the server task did not finish: {e}")),
        }
    }
}

/// Configure from the environment, serve, and stop on a signal.
///
/// **Builds its own runtime**, rather than being an `async fn` behind a macro
/// in `main.rs`. How many threads the instance runs on is the daemon's
/// decision, and it belongs beside the rest of the daemon's decisions.
///
/// # Errors
///
/// If the configuration is incomplete, a precondition fails, or the server
/// does.
pub fn run() -> anyhow::Result<()> {
    // Before anything that might want to say something.
    logging::install().map_err(|e| anyhow::anyhow!("could not install logging: {e}"))?;

    let config = Config::from_env()?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    runtime.block_on(async move {
        let daemon = start(config).await?;
        wait_for_a_signal().await;
        daemon.stop().await
    })
}

/// Check every precondition, then serve.
///
/// Returns once the socket is bound and the service is mounted, so a caller
/// that connects to [`Daemon::address`] immediately afterwards will be
/// answered.
///
/// # Errors
///
/// If the address cannot be bound, the store cannot be reached or migrated, an
/// extension is missing, or the object store will not take bytes.
pub async fn start(config: Config) -> anyhow::Result<Daemon> {
    // First, and deliberately. Binding is the cheapest thing that fails and
    // "address already in use" is the commonest reason a start does.
    let listener = TcpListener::bind(config.listen)
        .await
        .map_err(|e| anyhow::anyhow!("could not listen on {}: {e}", config.listen))?;
    let address = listener.local_addr()?;

    let pool = store::connect(&config.database_url).await?;
    tracing::info!("the structured store is connected and migrated");

    let missing = store::preflight::missing_extensions(&pool).await?;
    if !missing.is_empty() {
        anyhow::bail!(
            "the structured store is missing {}. The instance will not start without them: \
             a query that needs one would fail at retrieval time instead, which is later, \
             quieter, and in front of a person.",
            missing.join(" and ")
        );
    }

    let objects = Arc::new(FilesystemObjectStore::open(&config.object_root).await?);
    preflight::object_store(objects.as_ref()).await?;
    tracing::info!("the object store took a body and gave it back");

    let capacity: Arc<dyn StorageCapacity> = Arc::clone(&objects) as Arc<dyn StorageCapacity>;
    let store: Arc<dyn ObjectStore> = objects;

    let authenticator = Arc::new(Authenticator::new(
        IssuerConfig {
            issuer: config.issuer.clone(),
            audience: config.audience.clone(),
        },
        // No fetch happens here. The first request that needs a key is what
        // fills the cache, which is why a provider that is still booting does
        // not stop the instance from starting.
        KeyCache::new(Arc::new(HttpJwks::new(&config.jwks_uri)?)),
        pool.clone(),
    ));

    let instance = Instance::new(WritePath::new(pool.clone(), store), capacity);

    // Nothing is spawned yet. Wave 2.4's reclamation and Wave 5's derivation
    // worker attach here; `tasks` holds the decisions they will need.
    let tasks = Tasks::new();
    let shutdown = tasks.signal();

    let serving = tokio::spawn(serve::serve(
        listener,
        Arc::clone(&authenticator),
        instance,
        shutdown,
    ));

    tracing::info!(%address, "serving");

    Ok(Daemon {
        address,
        tasks,
        serving,
        pool,
    })
}

/// Whichever of the two ways an operator or an init system asks for a stop.
///
/// `SIGTERM` is what a container runtime sends and `SIGINT` is what a terminal
/// sends; treating them differently would mean a daemon that drains cleanly
/// when a person stops it and not when its supervisor does.
async fn wait_for_a_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        let mut terminate = match signal(SignalKind::terminate()) {
            Ok(s) => s,
            Err(e) => {
                tracing::error!("could not listen for SIGTERM: {e}");
                return;
            }
        };
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = terminate.recv() => {}
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}

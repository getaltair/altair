//! The gRPC server: one composition, used by the daemon and by every test
//! that needs an instance on a socket.
//!
//! **There is one way to mount this service, and this is it.** The auth layer
//! is not optional and not a parameter: `Identify` goes on here, so a caller
//! cannot assemble an instance that serves requests without resolving a
//! credential first. A test that stood up its own `Server::builder()` would be
//! testing a composition the daemon does not run, and would have quietly opted
//! out of the layer that makes "nothing below the edge sees a token" true.
//!
//! # Shutdown
//!
//! The signal is a [`Shutdown`], the same value the background tasks hold, so
//! "we are stopping" is one fact rather than two that have to be kept in step.
//! tonic stops accepting connections when it resolves and returns once every
//! request already in flight has finished. That is what makes an acknowledged
//! intent durable across a shutdown: the write is inside a transaction that
//! either commits before the server returns or rolls back with the process,
//! and the acknowledgement is written in that same transaction, so there is no
//! ordering in which a client is told "applied" for something the store does
//! not hold.

use std::sync::Arc;

use tokio::net::TcpListener;
use tonic::transport::server::TcpIncoming;

use altair_proto::v1;

use crate::auth::{Authenticator, Identify};
use crate::daemon::tasks::Shutdown;
use crate::service::Instance;

/// Serve until `shutdown` resolves and every request in flight has finished.
///
/// Takes an already-bound listener rather than an address, so a caller that
/// asked for port zero can find out which port it got before anything connects
/// — and so binding fails before the pool, the migrations and the preflight,
/// rather than after them.
///
/// # Errors
///
/// If the server itself fails. A request failing is not one of those.
pub async fn serve(
    listener: TcpListener,
    authenticator: Arc<Authenticator>,
    instance: Instance,
    mut shutdown: Shutdown,
) -> Result<(), tonic::transport::Error> {
    tonic::transport::Server::builder()
        .layer(Identify::new(authenticator))
        .add_service(v1::altair_server::AltairServer::new(instance))
        .serve_with_incoming_shutdown(TcpIncoming::from(listener), async move {
            shutdown.requested().await;
        })
        .await
}

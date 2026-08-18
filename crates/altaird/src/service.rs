//! The public interface, served.
//!
//! # Three calls are served and three are not, on purpose
//!
//! Wave 2.1 stood up `Submit` end to end; Wave 2.3 adds `PutBody` and
//! `GetBody`. `Query`, `Changes`, and `GetHealth` still answer `unimplemented`,
//! and will until their own waves. Two of the write path's requirements are
//! observable only through `Submit` and are not testable from an internal
//! function:
//!
//! - **A submission is never all or nothing.** The answer carries one
//!   acknowledgement per intent, in the order submitted, and an intent that was
//!   refused does not affect the one after it.
//! - **A refusal reveals nothing, and DR-004 extends that to the status code.**
//!   A submission whose every intent was refused answers `Ok`, with the
//!   refusals inside it. Any other status would say, at the transport, the
//!   thing the single refusal reason exists to avoid saying.
//!
//! `tests/submission_call.rs` asserts both, and asserts that the remaining
//! three are deliberately absent rather than forgotten. `tests/file_bodies.rs`
//! covers `PutBody` and `GetBody`.
//!
//! # What a status means here
//!
//! The substrate divides conditions into a **wait**, which the ordinary path
//! clears by continuing to run, and a **fault**, which it does not.
//!
//! - `Unauthenticated` — a wait. An absent, expired, forged, or unknown
//!   credential all produce it, indistinguishably, and DR-005 says the client
//!   holds silently. It is not a fault and must not be signalled as one.
//! - `Unavailable` — a wait. The store could not be reached, nothing was
//!   acknowledged, and the outbox holds. To the person this is the same as the
//!   instance being unreachable, which is why it is the same outcome.
//! - `NotFound` — a wait, on `GetBody` only. Indistinguishable from an
//!   entity outside the requester's audience, for the same reason a refusal
//!   never says which of "does not exist" and "not yours to see" is true.
//! - `Unimplemented` — a fault. Waiting will not clear it. That is the honest
//!   answer for the three calls this item does not serve.
//!
//! **An intent being refused is none of these.** It is `Ok`, inside the
//! response, which is what makes a batch partial.

use std::pin::Pin;
use std::sync::Arc;

use futures::StreamExt;
use tonic::{Request, Response, Status};

use altair_proto::v1;

use crate::auth::{Authentication, Authenticator, Member, bearer_token};
use crate::objects::{self, BodyId, ByteSource};
use crate::write::{BodyLookup, WritePath, content};

/// The instance, as callers reach it.
pub struct Instance {
    auth: Arc<Authenticator>,
    write: WritePath,
}

impl Instance {
    #[must_use]
    pub fn new(auth: Arc<Authenticator>, write: WritePath) -> Self {
        Self { auth, write }
    }

    /// Resolve the credential on a request, or say nothing about why not.
    ///
    /// Unauthenticated reaches no query surface: a caller of this has a
    /// `Member` or has a `Status`, and there is no third thing to hold.
    async fn member<T>(&self, request: &Request<T>) -> Result<Member, Status> {
        let header = request
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok());
        match self.auth.authenticate(bearer_token(header)).await {
            // The store was unavailable while resolving a membership. A wait.
            Err(_) => Err(Status::unavailable("")),
            Ok(Authentication::Unauthenticated) => Err(Status::unauthenticated("")),
            Ok(Authentication::Member(m)) => Ok(m),
        }
    }
}

/// The three calls this build does not serve: `Query`, `Changes`, `GetHealth`.
///
/// One message rather than three, because a caller has nothing to do
/// differently for any of them and the difference would only invite one to
/// try.
const NOT_YET: &str = "this call is not served by this build";

#[tonic::async_trait]
impl v1::altair_server::Altair for Instance {
    async fn submit(
        &self,
        request: Request<v1::SubmitRequest>,
    ) -> Result<Response<v1::SubmitResponse>, Status> {
        let member = self.member(&request).await?;
        let intents = request.into_inner().intents;

        match self.write.submit(&member, &intents).await {
            Ok(acknowledgements) => Ok(Response::new(v1::SubmitResponse { acknowledgements })),
            // The instance failed, so nothing was acknowledged and the caller's
            // outbox holds. Silent: the detail would describe the store to
            // somebody who cannot act on it.
            Err(_) => Err(Status::unavailable("")),
        }
    }

    async fn query(
        &self,
        _request: Request<v1::QueryRequest>,
    ) -> Result<Response<v1::QueryResponse>, Status> {
        Err(Status::unimplemented(NOT_YET))
    }

    async fn changes(
        &self,
        _request: Request<v1::ChangesRequest>,
    ) -> Result<Response<v1::ChangesResponse>, Status> {
        Err(Status::unimplemented(NOT_YET))
    }

    async fn get_health(
        &self,
        _request: Request<v1::HealthRequest>,
    ) -> Result<Response<v1::HealthResponse>, Status> {
        Err(Status::unimplemented(NOT_YET))
    }

    async fn put_body(
        &self,
        request: Request<tonic::Streaming<v1::BodyChunk>>,
    ) -> Result<Response<v1::PutBodyAck>, Status> {
        // A body upload still needs a real member, even though nothing about
        // *which* member is recorded anywhere in the object store — it has no
        // notion of ownership, which lives entirely in the entity that later
        // names the body.
        let _member = self.member(&request).await?;
        let mut stream = request.into_inner();

        let Some(first) = stream.message().await? else {
            return Err(Status::invalid_argument(
                "a body upload carries at least one chunk",
            ));
        };
        let id = BodyId::try_from(first.body_id.as_slice())
            .map_err(|_| Status::invalid_argument("a body identity is 16 bytes"))?;

        let source = adapt_upload(id, first.data, stream);

        match self.write.put_body(id, source).await {
            Ok(bytes_received) => Ok(Response::new(v1::PutBodyAck {
                body_id: id.as_bytes().to_vec(),
                bytes_received,
            })),
            Err(objects::Error::Source(e)) if e.downcast_ref::<BodyIdMismatch>().is_some() => Err(
                Status::invalid_argument("every chunk of an upload names the same body"),
            ),
            // The client's own stream broke before the body was complete.
            // Not the instance's fault, and not "unavailable" either — the
            // store answered nothing at all, because it was never asked to.
            Err(objects::Error::Source(_)) => Err(Status::aborted("")),
            Err(objects::Error::Unavailable(_)) => Err(Status::unavailable("")),
            Err(objects::Error::NoSuchBody) => unreachable!("put never answers NoSuchBody"),
        }
    }

    type GetBodyStream =
        Pin<Box<dyn tokio_stream::Stream<Item = Result<v1::BodyChunk, Status>> + Send>>;

    async fn get_body(
        &self,
        request: Request<v1::BodyRequest>,
    ) -> Result<Response<Self::GetBodyStream>, Status> {
        let member = self.member(&request).await?;
        let entity_id = content::identifier(&request.into_inner().entity_id)
            .map_err(|_| Status::invalid_argument("an entity identity is 16 bytes"))?;

        match self.write.get_body(&member, entity_id).await {
            // No such entity, not visible, not a file, or a file naming no
            // body — indistinguishable from outside, the same way audience
            // refusal and nonexistence never differ anywhere else.
            Ok(BodyLookup::NotFound) => Err(Status::not_found("")),
            // The entity is there and visible, but the object store cannot
            // produce it. DR-003: a different statement from missing, and to
            // the person it reads as a wait rather than a refusal.
            Ok(BodyLookup::Unavailable) => Err(Status::unavailable("")),
            Ok(BodyLookup::Found(id, body)) => {
                let body_id = id.as_bytes().to_vec();
                let stream = body.into_chunks().map(move |chunk| {
                    chunk
                        .map(|data| v1::BodyChunk {
                            body_id: body_id.clone(),
                            data,
                        })
                        .map_err(|_| Status::unavailable(""))
                });
                Ok(Response::new(Box::pin(stream)))
            }
            Err(_) => Err(Status::unavailable("")),
        }
    }
}

/// A later chunk of an upload named a different body than the first did.
///
/// Carried through [`objects::Error::Source`] so the byte source can report
/// it without a second error channel; downcast back out in [`Instance::put_body`]
/// to tell it apart from an ordinary broken transport.
#[derive(Debug)]
struct BodyIdMismatch;

impl std::fmt::Display for BodyIdMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a later chunk named a different body")
    }
}

impl std::error::Error for BodyIdMismatch {}

/// Adapt an incoming chunk stream into the shape [`objects::ObjectStore::put`]
/// wants: the first chunk's data, then every later chunk's, validating that
/// none of them names a different body than the first did.
fn adapt_upload(
    id: BodyId,
    first_data: Vec<u8>,
    rest: tonic::Streaming<v1::BodyChunk>,
) -> ByteSource {
    let head = futures::stream::once(async move { Ok(first_data) });
    let tail = futures::stream::unfold(rest, move |mut rest| async move {
        match rest.message().await {
            Ok(Some(chunk)) => {
                if !chunk.body_id.is_empty() && chunk.body_id.as_slice() != id.as_bytes().as_slice()
                {
                    let err: objects::BoxError = Box::new(BodyIdMismatch);
                    return Some((Err(err), rest));
                }
                Some((Ok(chunk.data), rest))
            }
            Ok(None) => None,
            Err(status) => {
                let err: objects::BoxError = Box::new(status);
                Some((Err(err), rest))
            }
        }
    });
    Box::pin(head.chain(tail))
}

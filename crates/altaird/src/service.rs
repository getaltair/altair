//! The public interface, served.
//!
//! # One call is served and five are not, on purpose
//!
//! Wave 2.1 stands up `Submit` end to end and answers the other five
//! `unimplemented`. Two of the write path's requirements are observable only
//! here and are not testable from an internal function:
//!
//! - **A submission is never all or nothing.** The answer carries one
//!   acknowledgement per intent, in the order submitted, and an intent that was
//!   refused does not affect the one after it.
//! - **A refusal reveals nothing, and DR-004 extends that to the status code.**
//!   A submission whose every intent was refused answers `Ok`, with the
//!   refusals inside it. Any other status would say, at the transport, the
//!   thing the single refusal reason exists to avoid saying.
//!
//! `tests/submission_call.rs` asserts both, and asserts that the five are
//! deliberately absent rather than forgotten.
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
//! - `Unimplemented` — a fault. Waiting will not clear it. That is the honest
//!   answer for the five calls this item does not serve.
//!
//! **An intent being refused is none of these.** It is `Ok`, inside the
//! response, which is what makes a batch partial.

use std::pin::Pin;
use std::sync::Arc;

use tonic::{Request, Response, Status};

use altair_proto::v1;

use crate::auth::{Authentication, Authenticator, Member, bearer_token};
use crate::write::WritePath;

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

/// The five calls Wave 2.1 does not serve.
///
/// One message rather than five, because a caller has nothing to do differently
/// for any of them and the difference would only invite one to try.
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
        _request: Request<tonic::Streaming<v1::BodyChunk>>,
    ) -> Result<Response<v1::PutBodyAck>, Status> {
        Err(Status::unimplemented(NOT_YET))
    }

    type GetBodyStream =
        Pin<Box<dyn tokio_stream::Stream<Item = Result<v1::BodyChunk, Status>> + Send>>;

    async fn get_body(
        &self,
        _request: Request<v1::BodyRequest>,
    ) -> Result<Response<Self::GetBodyStream>, Status> {
        Err(Status::unimplemented(NOT_YET))
    }
}

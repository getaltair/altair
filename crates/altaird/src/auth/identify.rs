//! The one place a token is read.
//!
//! Wave 1 Lane C stopped at a verified subject claim and a resolved
//! membership. This is where that resolution happens **once, at the edge**, so
//! that everything underneath is handed an identity instead of a credential.
//!
//! # Nothing below this ever sees a token
//!
//! [`Identify`] is a tower layer wrapped around the whole gRPC service. For
//! every request it:
//!
//! 1. takes the `authorization` header value,
//! 2. resolves it through [`Authenticator::authenticate`],
//! 3. **removes the header**, and
//! 4. puts an [`Identity`] in the request's extensions.
//!
//! Step 3 is the load-bearing one and it happens on every path, including the
//! ones where nothing resolved. The served surface cannot read a credential
//! because by the time it runs there is not one to read, and
//! `tests/one_credential.rs` fails if any source outside this module names the
//! header or the token at all. That is a structural guarantee rather than a
//! convention: a future call that wanted to do its own thing with the token
//! would have to delete a test to get it.
//!
//! # Why a layer and not a `tonic::service::Interceptor`
//!
//! An interceptor is synchronous. Resolving a subject to a membership is a
//! query against the structured store, so it has to await, and an interceptor
//! could only do it by blocking a runtime thread or by handing the token
//! onwards for somebody else to resolve — which is the one thing this must not
//! do.
//!
//! # Why this does not refuse
//!
//! The layer resolves and records; it never turns a request away. Two reasons,
//! and the second is the important one:
//!
//! * `GetHealth` is served without a member on purpose (see
//!   [`crate::service`]): an infra probe carries no bearer token and is
//!   exactly the caller it exists for. A layer that rejected unauthenticated
//!   requests would make the instance unmonitorable.
//! * **What an absent identity means is the call's to say, not the edge's.**
//!   Unauthenticated is a *wait* (DR-005) and the store being unreachable
//!   while resolving is also a wait, but they are different waits with
//!   different statuses, and the mapping from an internal outcome to what
//!   crosses the wire belongs in one place. That place is `service.rs`.

use std::sync::Arc;
use std::task::{Context, Poll};

use tower_layer::Layer;
use tower_service::Service;

use super::{Authentication, Authenticator, Member, bearer_token};

/// The header a credential arrives in. Named here and nowhere else.
const CREDENTIAL_HEADER: &str = "authorization";

/// Who is asking, as far as anything below the edge is concerned.
///
/// The three variants are the three things the edge can conclude, and each has
/// exactly one meaning downstream. There is no fourth, and in particular there
/// is no "a token was presented but" — see [`super`] on why a forged token and
/// an expired one are the same value.
#[derive(Clone, Debug)]
pub enum Identity {
    /// A signature verified and the subject named a living membership.
    Member(Member),
    /// No usable credential. A wait.
    Unauthenticated,
    /// The store could not be reached while resolving one. Also a wait, and a
    /// different one: it says nothing about the credential.
    Unavailable,
}

/// Resolves a credential into an [`Identity`], once, for everything below.
#[derive(Clone)]
pub struct Identify {
    authenticator: Arc<Authenticator>,
}

impl Identify {
    #[must_use]
    pub fn new(authenticator: Arc<Authenticator>) -> Self {
        Self { authenticator }
    }
}

impl<S> Layer<S> for Identify {
    type Service = Identifying<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Identifying {
            inner,
            authenticator: Arc::clone(&self.authenticator),
        }
    }
}

/// [`Identify`] wrapped round one service.
#[derive(Clone)]
pub struct Identifying<S> {
    inner: S,
    authenticator: Arc<Authenticator>,
}

impl<S, B> Service<http::Request<B>> for Identifying<S>
where
    S: Service<http::Request<B>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: http::Request<B>) -> Self::Future {
        // The clone is the documented way to hold a tower service across an
        // await: `self` is the one that was made ready, and the clone is the
        // one that gets called.
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let authenticator = Arc::clone(&self.authenticator);

        Box::pin(async move {
            // Removed rather than read, and removed before anything is
            // awaited, so there is no window in which the request carries both
            // a credential and an identity.
            let presented = request.headers_mut().remove(CREDENTIAL_HEADER);
            let credential = presented.as_ref().and_then(|v| v.to_str().ok());

            let identity = match authenticator.authenticate(bearer_token(credential)).await {
                Ok(Authentication::Member(member)) => Identity::Member(member),
                Ok(Authentication::Unauthenticated) => Identity::Unauthenticated,
                Err(_) => Identity::Unavailable,
            };
            // Dropped explicitly. It has already been removed from the
            // request; this is the last reference in the process, and it goes
            // before the call rather than at the end of the scope.
            drop(presented);

            request.extensions_mut().insert(identity);
            inner.call(request).await
        })
    }
}

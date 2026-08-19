//! The credential this device presents, and where it comes from.
//!
//! # What is settled, and what is deliberately not
//!
//! DR-005 settles the shape: Authentik issues, the instance validates, and
//! **this client never authenticates anybody**. An absent or expired token is a
//! typed outcome carried by the protocol rather than a page of HTML the client
//! has to recognise, which is what makes "an unreachable instance and an
//! expired session are the same wait" implementable at all.
//!
//! It deliberately leaves **which flow each client uses** open, calling it a
//! platform decision. So this module does not choose one. What it does is hold
//! the client's side of whatever flow is eventually chosen:
//!
//! - the credential is **durable**, so a device that has been away for months
//!   still has one when it comes back;
//! - it is **re-read on every attempt** rather than captured once at startup,
//!   so a credential refreshed by anything else is picked up without this
//!   client being restarted, and without a person;
//! - an unauthenticated answer is **a wait**. Nothing is signalled, nothing is
//!   dropped, and there is no login page anywhere in this client — not because
//!   one was left out, but because there is nowhere one could go.
//!
//! Requiring a human before an outbox can drain would put a barrier in front of
//! exactly the return the product exists to make free.

use crate::device::Store;

/// The credential, read fresh each time it is asked for.
pub struct Session<'a> {
    store: &'a Store,
}

impl<'a> Session<'a> {
    #[must_use]
    pub const fn new(store: &'a Store) -> Self {
        Self { store }
    }

    /// What to present right now.
    ///
    /// **Read rather than remembered.** A client that captured its token at
    /// startup would keep presenting a stale one until somebody restarted it,
    /// which is a person doing the refreshing.
    #[must_use]
    pub fn credential(&self) -> Option<String> {
        self.store.credential().ok().flatten()
    }

    /// The `authorization` header value, where there is a credential.
    ///
    /// **The `Bearer` scheme is not decoration**: the instance reads this
    /// header through `bearer_token`, which requires it and answers nothing at
    /// all without it.
    #[must_use]
    pub fn authorization(&self) -> Option<String> {
        self.credential().map(|token| format!("Bearer {token}"))
    }
}

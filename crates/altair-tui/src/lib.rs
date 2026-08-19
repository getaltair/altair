//! The terminal client.
//!
//! Wave 4 builds this in six stages over one store: the device store and the
//! outbox first (4.1), then the replica face and the screens (4.2). What is
//! here now is 4.1 — everything needed to accept a capture locally and get it
//! to an instance, and nothing else.
//!
//! # The two things worth knowing before reading further
//!
//! **Acceptance is local.** Nothing about the network or the token
//! participates in telling a person their capture is theirs. [`device`] writes
//! it durably and answers; [`sender`] is a separate concern that runs later and
//! elsewhere, and it can be stopped for a week without changing what the person
//! was told.
//!
//! **The store holds encoded messages.** An entity is kept as its encoded
//! `EntityContent`, an outbox item as its encoded `Intent`. The contract is
//! designed to grow fields, so a client schema mirroring it would migrate every
//! time it did.

/// Behind a feature, so the shipped client does not link the harness that
/// judges it. What it drives is not behind one.
#[cfg(feature = "conformance")]
pub mod adapter;
pub mod app;
pub mod capture;
pub mod config;
pub mod device;
pub mod editor;
pub mod replica;
pub mod sender;
pub mod session;
pub mod signals;
pub mod ui;
pub mod wire;

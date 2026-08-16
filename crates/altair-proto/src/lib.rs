//! The public interface, generated from `proto/altair/v1/altair.proto`.
//!
//! The contract is accepted and its field numbers are permanent (DR-004), so
//! this crate generates and re-exports; it never adapts. Everything a client
//! or the instance needs from the wire lives under [`v1`].

// Nothing here is written by hand, so lints that ask for a different shape have
// no one to address. Boxing a large oneof variant, for one, is prost's call and
// not ours.
#[allow(clippy::all, clippy::pedantic, rustdoc::all)]
pub mod v1 {
    tonic::include_proto!("altair.v1");
}

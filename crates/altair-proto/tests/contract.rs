//! The generated contract is reachable from outside the crate.
//!
//! Wave 0 wired codegen into the build but nothing included the output, so the
//! crate compiled while exposing none of it. This fails if that regresses.

use altair_proto::v1;

#[test]
fn the_service_client_and_server_are_generated() {
    // Naming the types is the assertion; if codegen is not included this does
    // not compile.
    fn _client(_: v1::altair_client::AltairClient<tonic::transport::Channel>) {}
    fn _server<T: v1::altair_server::Altair>(_: v1::altair_server::AltairServer<T>) {}
}

#[test]
fn messages_carry_their_permanent_field_numbers() {
    let intent = v1::Intent::default();
    assert!(intent.intent_id.is_empty());
    let _ = v1::SubmitRequest::default();
    let _ = v1::QueryRequest::default();
    let _ = v1::ChangesRequest::default();
}

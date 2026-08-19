//! What each thing the instance can do looks like from inside the client, and
//! which of them is a wait.
//!
//! **This is the test to read first if silence and signalling go strange.**
//! The rule is that waiting is silent and faults signal, and the whole of it
//! rests on telling a torn connection from an answer. Both arrive as gRPC code
//! `Unknown`, so the code cannot be the discriminator, and the one that works —
//! a status tonic synthesised from a transport failure carries the underlying
//! error as its source, one the instance actually sent does not — is a property
//! of a library rather than of the contract. Nothing would fail loudly if a
//! future tonic stopped doing it: the client would go quiet about faults, or
//! start signalling every dropped connection. So it is asserted here.

use altair_conformance::instance::{Behaviour, FakeInstance};
use altair_proto::v1;
use altair_tui::wire::{self, Condition};

/// Make one submission against an instance behaving this way, and say what
/// came back.
async fn submit_under(behaviour: Behaviour) -> Result<(), tonic::Status> {
    let instance = FakeInstance::start().await;
    instance.set_behaviour(behaviour);
    let channel = tonic::transport::Endpoint::from_shared(instance.url())
        .expect("the fake instance's address")
        .connect_lazy();
    let mut client = v1::altair_client::AltairClient::new(channel);
    let composed = wire::compose_creation(&altair_tui::capture::Captured::Bare);
    client
        .submit(v1::SubmitRequest {
            intents: vec![composed.intent],
        })
        .await
        .map(|_| ())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_instance_that_accepts_answers() {
    assert!(
        submit_under(Behaviour::Accept).await.is_ok(),
        "the fake instance accepts, so this is the harness and not the client"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_unreachable_instance_is_a_wait() {
    let status = submit_under(Behaviour::Unreachable)
        .await
        .expect_err("an unreachable instance cannot answer");
    assert!(
        std::error::Error::source(&status).is_some(),
        "a transport failure carries its cause, and that is what makes it recognisable"
    );
    assert_eq!(wire::classify(&status), Condition::Wait);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_expired_session_is_a_wait() {
    let status = submit_under(Behaviour::Unauthenticated)
        .await
        .expect_err("the instance rejects the token");
    // DR-005: the client refreshes without a person. An expired session and an
    // unreachable instance are the same wait, and neither is ever a login page.
    assert_eq!(status.code(), tonic::Code::Unauthenticated);
    assert_eq!(wire::classify(&status), Condition::Wait);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_connection_torn_down_is_a_wait() {
    for behaviour in [Behaviour::DropBeforeCommit, Behaviour::DropAfterCommit] {
        let status = submit_under(behaviour.clone())
            .await
            .expect_err("the connection went away");
        assert_eq!(
            status.code(),
            tonic::Code::Unknown,
            "{behaviour:?} arrives under the same code an unrecognised answer does, \
             which is exactly why the code cannot be the discriminator"
        );
        assert_eq!(
            wire::classify(&status),
            Condition::Wait,
            "{behaviour:?} is a dropped connection, and retrying is what clears it"
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn something_the_client_cannot_classify_is_a_fault() {
    let status = submit_under(Behaviour::UnrecognisedCondition)
        .await
        .expect_err("the instance answers with a condition");
    assert_eq!(status.code(), tonic::Code::Unknown);
    assert!(
        std::error::Error::source(&status).is_none(),
        "the instance sent this, so there is no transport error under it"
    );
    assert_eq!(
        wire::classify(&status),
        Condition::Fault,
        "a condition not known to be self clearing is a fault: a wrong signal \
         costs attention and a wrong silence costs data"
    );
}

//! The sender against the fake instance, without a client process in the way.
//!
//! The conformance suite drives the whole client and observes two boundaries.
//! What it cannot see is anything the fake instance records but never reads —
//! and the credential is exactly that. So it is checked here.

use std::sync::Arc;
use std::time::Duration;

use altair_conformance::instance::{Behaviour, FakeInstance};
use altair_tui::capture::Captured;
use altair_tui::device::Store;
use altair_tui::sender::Sender;
use altair_tui::signals::Signals;
use altair_tui::wire;

/// Accept one note into a fresh store, and answer where the store is.
fn a_store_with_one_capture() -> tempfile::TempDir {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let mut store = Store::open(directory.path()).expect("open");
    store.set_credential("a-token").expect("credential");
    let composed = wire::compose_creation(&Captured::Note {
        title: "A".to_string(),
        body: String::new(),
    });
    store
        .accept_creation(
            &composed.entity_id,
            &composed.content,
            None,
            &composed.intent,
        )
        .expect("accept");
    directory
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_credential_carries_the_bearer_scheme() {
    let instance = FakeInstance::start().await;
    instance.set_behaviour(Behaviour::Accept);
    let directory = a_store_with_one_capture();

    let sender =
        Sender::new(directory.path(), &instance.url(), Arc::new(Signals::new())).expect("sender");
    let running = tokio::spawn(sender.run());

    assert!(
        instance.wait_for_intents(1, Duration::from_secs(5)).await,
        "the capture must reach the instance"
    );
    running.abort();

    let submissions = instance.log().submissions;
    assert_eq!(
        submissions[0].token.as_deref(),
        Some("Bearer a-token"),
        "the instance reads this header through `bearer_token`, which requires \
         the scheme and answers nothing at all without it"
    );
}

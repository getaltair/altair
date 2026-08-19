//! One entity, the whole way through, against a real instance.
//!
//! **The canary, and it is deliberately one entity rather than a suite.** The
//! outbox was developed against a fake instance, which is the right place to
//! drive refusals, stalls and torn connections from. What a fake cannot prove
//! is that the client and this instance actually talk to each other: that the
//! credential carries the scheme the instance reads, that an intent this client
//! composes is one this instance applies, and that what comes back through the
//! change stream is something the client can hold. Every one of those is a
//! place where two correct-looking halves disagree.
//!
//! This is where they meet, so it takes a database and stays on Linux.
//!
//! The path: captured in the client while nothing is running, held locally,
//! accepted and shown, replayed when the instance appears, returned through
//! `Changes`, landed in the replica, and drawn on the captures list — without
//! the person asking for any of it.

mod common;

use std::sync::Arc;
use std::time::Duration;

use altair_tui::capture::Captured;
use altair_tui::device::Store;
use altair_tui::replica::Replica;
use altair_tui::sender::Sender;
use altair_tui::signals::Signals;
use altair_tui::wire;
use common::served::Served;

/// Long enough for many passes of a client that retries every hundred
/// milliseconds and polls every five hundred.
const SETTLE: Duration = Duration::from_secs(15);

/// Wait for something to become true of the store, and say whether it did.
async fn until(store: &Store, condition: impl Fn(&Store) -> bool) -> bool {
    let deadline = tokio::time::Instant::now() + SETTLE;
    loop {
        if condition(store) {
            return true;
        }
        if tokio::time::Instant::now() >= deadline {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn one_capture_reaches_the_instance_and_comes_back() {
    let served = Served::new().await;
    let token = served.token_for_one();
    let device = tempfile::tempdir().expect("a device directory");

    // --- the person captures, and nothing is running ----------------------
    //
    // No sender, no replica, no connection. Acceptance is local and durable
    // before the person is told, and neither credentials nor connectivity
    // participate — so this is the whole of it.
    let mut store = Store::open(device.path()).expect("open the device store");
    let composed = wire::compose_creation(&Captured::Note {
        title: "the canary".to_string(),
        body: "captured before anything was reachable".to_string(),
    });
    store
        .accept_creation(
            &composed.entity_id,
            &composed.content,
            None,
            &composed.intent,
        )
        .expect("acceptance is local");

    let shown = store.captures().expect("the captures list");
    assert_eq!(shown.len(), 1, "it is shown the moment it is accepted");
    assert_eq!(shown[0].content.title.as_deref(), Some("the canary"));
    assert!(
        shown[0].pending,
        "and nothing has taken it yet, which is the only thing that makes the \
         local view outrank the instance's"
    );

    // --- the instance appears ---------------------------------------------
    let signals = Arc::new(Signals::new());
    let sender = Sender::new(
        device.path(),
        &served.url(),
        token.clone(),
        Arc::clone(&signals),
    )
    .expect("a sender");
    let replica =
        Replica::new(device.path(), &served.url(), token, Arc::clone(&signals)).expect("a replica");
    let sending = tokio::spawn(sender.run());
    let reading = tokio::spawn(replica.run());

    // Nobody asks for anything from here on. The row arrives because the
    // ordinary path is running, which is the whole claim.
    //
    // **Waited on the instance's own copy, not on `held`.** `held` falls back
    // to what this device wrote when the change stream has not delivered
    // anything, so waiting on it would go green the moment the intent was
    // acknowledged and would prove nothing about the read path at all.
    let arrived = until(&store, |store| {
        store
            .instance_copy(&composed.entity_id)
            .ok()
            .flatten()
            .is_some()
    })
    .await;
    sending.abort();
    reading.abort();

    assert!(
        arrived,
        "the intent must reach the instance and the entity must come back \
         through the change stream, with nobody asking"
    );

    // --- what the household holds -----------------------------------------
    let stored = served
        .world
        .title(altaird::store::ids::EntityId::from_uuid(
            uuid::Uuid::from_slice(&composed.entity_id).expect("16 bytes"),
        ))
        .await;
    assert_eq!(
        stored.as_deref(),
        Some("the canary"),
        "the instance holds what the person captured, under the identity the \
         client assigned — a client never learns an identity from the instance"
    );

    // --- what came back ---------------------------------------------------
    let copy = store
        .instance_copy(&composed.entity_id)
        .expect("read")
        .expect("the change stream delivered it");
    assert_eq!(
        copy.author_member_id,
        served.world.one.membership_id().as_bytes().to_vec(),
        "carrying an author, which only the instance can set: the client never \
         sent one and could not have"
    );
    assert!(
        copy.counter > 0,
        "and the counter the write was answered with"
    );
    assert!(
        store.position().expect("position") > 0,
        "and the client has moved along the change sequence"
    );

    // --- what the person sees ---------------------------------------------
    let held = store
        .held(&composed.entity_id)
        .expect("read")
        .expect("still present");
    assert_eq!(
        held.content.title.as_deref(),
        Some("the canary"),
        "and the person's row is the instance's now, not the device's copy of it"
    );
    assert!(!held.pending, "with nothing left on its way");
    assert_eq!(
        held.lifecycle,
        altair_proto::v1::LifecycleState::Active,
        "and the instance says it stands"
    );

    let shown = store.captures().expect("the captures list");
    assert_eq!(shown.len(), 1, "one entity, not two");
    assert_eq!(shown[0].entity_id, composed.entity_id);

    assert!(
        signals.showing().is_empty(),
        "and none of that was worth telling the person about: {:?}",
        signals.showing()
    );
}

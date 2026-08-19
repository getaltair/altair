//! `conformance/scenarios.md`, executed.
//!
//! One test per scenario, named for its id. The id literal in each
//! `scenario!` invocation is what `tests/coverage.rs` reads back to prove the
//! document and this file name the same thirty five scenarios.
//!
//! **These fail, and that is the deliverable of Wave 1.5.** The outbox is
//! Wave 4.1's work. Until it exists, [`NullClient`] is what these run against
//! and it implements nothing, so every scenario drives its steps to completion
//! and then fails on the behaviour it is about. Read the crate's `README.md`
//! before deciding one of them is broken.

#![cfg(feature = "run-conformance")]

use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use std::path::PathBuf;

use altair_conformance::client::{
    Action, CaptureContent, ClientEnvironment, ClientError, ClientProcess, ClientUnderTest, Offers,
    Reply, Signal, SignalKind,
};
use altair_conformance::instance::{self, Behaviour};
use altair_conformance::process::ChildClient;
use altair_conformance::world::{QUIET, SETTLE, Seen, World, assert_accepted, expect_entity};
use altair_conformance::{Outcome, scenario, skip_unless};

/// **The line Wave 4.1 changed.** It used to name `NullClient`, a stub that
/// implemented nothing, and every scenario failed on the behaviour it was
/// about. It now names the terminal client's own binary, in its adapter mode.
fn client_under_test() -> Arc<dyn ClientUnderTest> {
    Arc::new(TerminalClient)
}

/// The terminal client, run as the harness runs any client: its own process,
/// speaking the channel on stdin and stdout.
///
/// **The adapter is a mode of that binary and not a lookalike.** A separate
/// adapter would be a second implementation of local acceptance and this suite
/// would be judging something shaped like the client rather than the client.
struct TerminalClient;

impl ClientUnderTest for TerminalClient {
    fn name(&self) -> &str {
        "altair terminal client (adapter mode)"
    }

    fn offers(&self) -> Offers {
        Offers {
            // **The one thing above the floor this client does not offer, and
            // the reason is the harness rather than the client.** A3's
            // condition is a device that has never been signed in, and every
            // client this harness launches is handed a token in its
            // environment. There is no launch this client could read as "never
            // bound", so the scenario cannot arise here and is skipped rather
            // than faked. A client whose binding is a separate act — the
            // Android one will be — offers it and runs it.
            unbound_capture: false,
            offline_edit: true,
            file_capture: true,
        }
    }

    fn launch(
        &self,
        environment: &ClientEnvironment,
    ) -> Result<Box<dyn ClientProcess>, ClientError> {
        let child = ChildClient::spawn(
            &client_binary(),
            &["--conformance-adapter".to_string()],
            environment,
        )?;
        Ok(Box::new(child))
    }
}

/// Where the client binary is.
///
/// Cargo only sets `CARGO_BIN_EXE_*` for binaries of the package under test,
/// and the client is deliberately a different package — it depends on the
/// generated contract and not on this crate. So the path is derived from this
/// test binary's own: `target/<profile>/deps/scenarios-<hash>` sits two levels
/// under the directory the client is built into. `mise run conformance` builds
/// it first; a missing file here means that step was skipped.
fn client_binary() -> PathBuf {
    let mut directory = std::env::current_exe().expect("this test binary has a path");
    directory.pop();
    if directory.ends_with("deps") {
        directory.pop();
    }
    let binary = directory.join(format!("altair{}", std::env::consts::EXE_SUFFIX));
    assert!(
        binary.exists(),
        "the client binary is not at {}. Build it first: \n  \
         cargo build -p altair-tui --features conformance --bin altair\n\
         or run the whole thing with `mise run conformance`.",
        binary.display()
    );
    binary
}

async fn world() -> World {
    World::new(client_under_test()).await
}

/// F9 says three weeks. A suite cannot wait three weeks, so it waits long
/// enough to cover many retry cycles of any sane client and asserts that the
/// standing statement did not change and that nothing repeated. What this does
/// not exercise is anything keyed to a real calendar.
const LONG_ABSENCE: Duration = Duration::from_millis(2000);

// ---------------------------------------------------------------------------
// Polling the person's boundary. The instance's own waits live on FakeInstance.
// ---------------------------------------------------------------------------

async fn until_showing<F>(world: &mut World, patience: Duration, condition: F) -> bool
where
    F: Fn(&[Signal]) -> bool,
{
    let deadline = Instant::now() + patience;
    loop {
        let signals = world.showing();
        if condition(&signals) {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

async fn until_seen<F>(world: &mut World, label: &str, patience: Duration, condition: F) -> bool
where
    F: Fn(&Seen) -> bool,
{
    let deadline = Instant::now() + patience;
    loop {
        let seen = expect_entity(&world.open(label));
        if condition(&seen) {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

fn faults(signals: &[Signal]) -> Vec<&Signal> {
    signals
        .iter()
        .filter(|signal| signal.kind == SignalKind::Fault)
        .collect()
}

// ---------------------------------------------------------------------------
// A. Acceptance
// ---------------------------------------------------------------------------

scenario!(
    "A1",
    a1_capture_with_the_instance_unreachable_is_accepted,
    {
        let mut world = world().await;
        world.instance.set_behaviour(Behaviour::Unreachable);
        world.launch();

        let reply = world.capture("A");
        assert_accepted(&reply, "A");

        let seen = expect_entity(&world.open("A"));
        assert!(
            seen.present,
            "an accepted capture is readable on the client immediately"
        );
        assert_eq!(
            seen.title.as_deref(),
            Some("A"),
            "and it is the thing the person captured"
        );

        let signals = world.showing();
        assert!(
            signals.is_empty(),
            "nothing about the unreachability may be shown at the moment of capture, and the client showed {signals:?}"
        );

        Outcome::Passed
    }
);

scenario!("A2", a2_capture_with_an_expired_session_is_accepted, {
    let mut world = world().await;
    // Per DR-005 this is a wait, carried by the protocol rather than inferred.
    world.instance.set_behaviour(Behaviour::Unauthenticated);
    world.launch();

    let reply = world.capture("A");
    assert_accepted(&reply, "A");

    let seen = expect_entity(&world.open("A"));
    assert!(
        seen.present,
        "acceptance is identical to A1 in every observable respect"
    );
    assert_eq!(seen.title.as_deref(), Some("A"));

    let signals = world.showing();
    assert!(
        signals.is_empty(),
        "an expired session is the same wait as an unreachable instance, and the client showed {signals:?}"
    );

    Outcome::Passed
});

scenario!("A3", a3_capture_with_no_household_binding_is_accepted, {
    let mut world = world().await;
    skip_unless!(
        world.offers().unbound_capture,
        "this client requires binding before capture, which is conforming"
    );
    world.instance.set_behaviour(Behaviour::Accept);
    world.launch();

    let reply = world.capture("A");
    assert_accepted(&reply, "A");
    let seen = expect_entity(&world.open("A"));
    assert!(seen.present, "an unbound capture is accepted and durable");

    // No author, so nothing can be attributed and nothing sends.
    assert!(
        world
            .instance
            .stayed_true(QUIET, |i| i.intents().is_empty())
            .await,
        "an unbound capture has no author, so it cannot have sent before binding"
    );

    let member = uuid::Uuid::new_v4();
    world.bind(member.as_bytes());
    assert!(
        world.instance.wait_for_intents(1, SETTLE).await,
        "on binding, the author is set and the intent sends"
    );
    let arrivals = world.instance.intents();
    assert!(
        arrivals
            .iter()
            .any(|arrival| instance::title_of(&arrival.intent).as_deref() == Some("A")),
        "what sends on binding is the capture that was waiting"
    );

    Outcome::Passed
});

scenario!(
    "A4",
    a4_acceptance_is_refused_when_local_storage_cannot_hold_the_capture,
    {
        let mut world = world().await;
        world.instance.set_behaviour(Behaviour::Unreachable);
        skip_unless!(
            world.make_state_unwritable(),
            "this platform's harness cannot make the state directory unwritable"
        );
        world.launch();

        let reply = world.capture("A");
        match &reply {
            Reply::Refused { condition } => assert!(
                !condition.trim().is_empty(),
                "the condition is stated at the moment of the attempt, and this refusal states nothing"
            ),
            other => panic!(
                "local storage cannot hold the capture, so the client must refuse and state why; it answered {other:?}"
            ),
        }

        let seen = expect_entity(&world.open("A"));
        assert!(
            !seen.present,
            "the entity must not appear anywhere as though it were held"
        );

        Outcome::Passed
    }
);

scenario!(
    "A5",
    a5_an_entity_with_only_an_identity_a_type_and_a_timestamp_is_accepted,
    {
        let mut world = world().await;
        world.instance.set_behaviour(Behaviour::Accept);
        world.launch();

        let reply = world.capture_content("A", CaptureContent::Bare);
        assert_accepted(&reply, "A");

        assert!(
            world.instance.wait_for_intents(1, SETTLE).await,
            "and it sends"
        );
        let arrivals = world.instance.intents();
        let create = arrivals
            .iter()
            .find(|arrival| instance::is_create(&arrival.intent))
            .expect("a create must arrive");
        let entity =
            instance::create_entity_of(&create.intent).expect("the create names an entity");

        assert!(!entity.entity_id.is_empty(), "an identity");
        assert!(entity.created_at.is_some(), "a timestamp");
        let content = entity
            .content
            .as_ref()
            .expect("a type is the tag of content.specific");
        assert!(content.specific.is_some(), "a type");
        assert!(
            content.title.is_none(),
            "no field is required at any point on this path, and the client invented a title"
        );

        Outcome::Passed
    }
);

// ---------------------------------------------------------------------------
// B. Durability
// ---------------------------------------------------------------------------

scenario!("B1", b1_an_accepted_capture_survives_a_kill, {
    let mut world = world().await;
    world.instance.set_behaviour(Behaviour::Unreachable);
    world.launch();
    world.accept("A");

    world.restart_process();

    let seen = expect_entity(&world.open("A"));
    assert!(
        seen.present,
        "the entity is present after a kill without warning"
    );
    assert!(
        world.instance.intents().is_empty(),
        "the instance was unreachable throughout, so the entity is still unsent"
    );

    world.instance.set_behaviour(Behaviour::Accept);
    assert!(
        world.instance.wait_for_intents(1, SETTLE).await,
        "and it sends when the instance becomes reachable"
    );

    Outcome::Passed
});

scenario!(
    "B2",
    b2_an_accepted_capture_survives_a_restart_of_the_device,
    {
        let mut world = world().await;
        world.instance.set_behaviour(Behaviour::Unreachable);
        world.launch();
        world.accept("A");

        // A device restart loses everything ephemeral, not only the process. The
        // runtime directory goes; the state directory stays.
        world.restart_device();

        let seen = expect_entity(&world.open("A"));
        assert!(
            seen.present,
            "the entity is present after the device restarts"
        );
        assert!(world.instance.intents().is_empty(), "and is still unsent");

        world.instance.set_behaviour(Behaviour::Accept);
        assert!(
            world.instance.wait_for_intents(1, SETTLE).await,
            "and it sends when the instance becomes reachable"
        );

        Outcome::Passed
    }
);

scenario!(
    "B3",
    b3_the_queue_survives_alongside_the_entities_it_references,
    {
        let mut world = world().await;
        world.instance.set_behaviour(Behaviour::Unreachable);
        world.launch();
        for label in ["A", "B", "C"] {
            world.accept(label);
        }

        world.restart_process();

        let mut local = Vec::new();
        for label in ["A", "B", "C"] {
            let seen = expect_entity(&world.open(label));
            assert!(
                seen.present,
                "every entity an intent refers to is present after restart: {label}"
            );
            local.push(seen.entity_id);
        }

        world.instance.set_behaviour(Behaviour::Accept);
        assert!(
            world.instance.wait_for_intents(3, SETTLE).await,
            "every intent is present after restart and sends"
        );

        let arrived: HashSet<Vec<u8>> = world
            .instance
            .intents()
            .iter()
            .filter_map(|arrival| instance::entity_id_of(&arrival.intent))
            .collect();
        for (label, id) in ["A", "B", "C"].iter().zip(local) {
            assert!(
                arrived.contains(&id),
                "neither exists without the other: {label} is readable on the client but no intent named it"
            );
        }

        Outcome::Passed
    }
);

scenario!(
    "B4",
    b4_acceptance_is_shown_only_once_the_capture_is_durable,
    {
        // Killed at the moment acceptance is shown, on repeated runs: the harness
        // kills as soon as it reads the acceptance and before doing anything else.
        for run in 1..=10 {
            let mut world = world().await;
            world.instance.set_behaviour(Behaviour::Unreachable);
            world.launch();

            let reply = world.capture("A");
            world.kill();
            assert_accepted(&reply, "A");

            world.launch();
            let seen = expect_entity(&world.open("A"));
            assert!(
                seen.present,
                "run {run}: acceptance was shown before a kill immediately afterwards would have left the entity present"
            );
        }

        Outcome::Passed
    }
);

scenario!("B5", b5_a_body_is_durable_before_its_capture_is_accepted, {
    let mut world = world().await;
    skip_unless!(
        world.offers().file_capture,
        "this client does not capture files"
    );
    world.instance.set_behaviour(Behaviour::Unreachable);
    world.launch();

    let bytes = b"the bytes acceptance has to cover".to_vec();
    let reply = world.capture_content(
        "F",
        CaptureContent::File {
            media_type: "text/plain".to_string(),
            bytes: bytes.clone(),
        },
    );
    world.kill();
    assert_accepted(&reply, "F");

    world.launch();
    let seen = expect_entity(&world.open("F"));
    assert!(
        seen.present,
        "a kill immediately after acceptance leaves the intent present"
    );
    assert_eq!(
        seen.bytes.as_ref(),
        Some(&bytes),
        "and leaves the bytes present"
    );

    world.instance.set_behaviour(Behaviour::Accept);
    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| !i.bodies().is_empty())
            .await,
        "and both reach the instance"
    );
    assert_eq!(
        world.instance.bodies()[0].bytes,
        bytes,
        "the bytes acceptance covered are the bytes that arrive"
    );

    Outcome::Passed
});

// ---------------------------------------------------------------------------
// C. Ordering
// ---------------------------------------------------------------------------

scenario!("C1", c1_a_create_precedes_its_own_edits, {
    let mut world = world().await;
    skip_unless!(
        world.offers().offline_edit,
        "this client does not edit while offline"
    );
    world.instance.set_behaviour(Behaviour::Unreachable);
    world.launch();
    world.accept("A");
    world.edit("A", "A/1");
    world.edit("A", "A/2");

    world.instance.set_behaviour(Behaviour::Accept);
    assert!(
        world.instance.wait_for_intents(3, SETTLE).await,
        "all three intents send"
    );

    let arrivals = world.instance.intents();
    let create = arrivals
        .iter()
        .find(|arrival| instance::is_create(&arrival.intent))
        .expect("the create must arrive");
    let subject = instance::entity_id_of(&create.intent).expect("the create names an entity");
    let position = |title: &str| -> usize {
        arrivals
            .iter()
            .find(|arrival| {
                instance::is_edit(&arrival.intent)
                    && instance::entity_id_of(&arrival.intent).as_ref() == Some(&subject)
                    && instance::title_of(&arrival.intent).as_deref() == Some(title)
            })
            .unwrap_or_else(|| panic!("the edit carrying {title} must arrive"))
            .order
    };

    let first = position("A/1");
    let second = position("A/2");
    assert!(
        create.order < first,
        "the instance receives the create before either edit"
    );
    assert!(first < second, "and the first edit before the second");

    Outcome::Passed
});

scenario!("C2", c2_ordering_is_per_entity_not_global, {
    let mut world = world().await;
    skip_unless!(
        world.offers().offline_edit,
        "this client does not edit while offline"
    );
    world.instance.set_behaviour(Behaviour::Unreachable);
    world.launch();
    world.accept("A");
    world.accept("B");
    world.edit("A", "A/1");

    world.instance.set_behaviour(Behaviour::Accept);
    assert!(
        world.instance.wait_for_intents(3, SETTLE).await,
        "all three intents send"
    );

    let arrivals = world.instance.intents();
    let create = arrivals
        .iter()
        .find(|arrival| {
            instance::is_create(&arrival.intent)
                && instance::title_of(&arrival.intent).as_deref() == Some("A")
        })
        .expect("A's create must arrive");
    let subject = instance::entity_id_of(&create.intent).expect("A's create names an entity");
    let edit = arrivals
        .iter()
        .find(|arrival| {
            instance::is_edit(&arrival.intent)
                && instance::entity_id_of(&arrival.intent).as_ref() == Some(&subject)
        })
        .expect("A's edit must arrive");

    assert!(create.order < edit.order, "A's create precedes A's edit");
    // Nothing is asserted about where B's create falls, deliberately: this
    // scenario exists to say that nothing is required of it.

    Outcome::Passed
});

scenario!("C3", c3_a_body_precedes_the_intent_that_refers_to_it, {
    let mut world = world().await;
    skip_unless!(
        world.offers().file_capture,
        "this client does not capture files"
    );
    world.instance.set_behaviour(Behaviour::Accept);
    world.launch();

    let reply = world.capture_content(
        "F",
        CaptureContent::File {
            media_type: "text/plain".to_string(),
            bytes: b"body".to_vec(),
        },
    );
    assert_accepted(&reply, "F");

    let arrived = world
        .instance
        .wait_until(SETTLE, |i| {
            !i.bodies().is_empty()
                && i.intents()
                    .iter()
                    .any(|arrival| instance::body_id_of(&arrival.intent).is_some())
        })
        .await;
    assert!(
        arrived,
        "both the body and the intent naming it must reach the instance"
    );

    let body = world.instance.bodies().remove(0);
    let intents = world.instance.intents();
    let naming = intents
        .iter()
        .find(|arrival| instance::body_id_of(&arrival.intent) == Some(body.body_id.clone()))
        .expect("an intent must name the body that was uploaded");

    assert!(
        body.seq < naming.seq,
        "the body stream completes before the intent naming that body arrives"
    );

    Outcome::Passed
});

scenario!(
    "C4",
    c4_a_queued_edit_is_sent_against_the_acknowledged_counter,
    {
        let mut world = world().await;
        skip_unless!(
            world.offers().offline_edit,
            "this client does not edit while offline"
        );
        world.instance.set_behaviour(Behaviour::Unreachable);
        world.launch();
        world.accept("A");
        world.edit("A", "A/1");
        world.edit("A", "A/2");

        world.instance.set_behaviour(Behaviour::Accept);
        assert!(
            world.instance.wait_for_intents(3, SETTLE).await,
            "all three intents send"
        );

        let arrivals = world.instance.intents();
        let create = arrivals
            .iter()
            .find(|arrival| instance::is_create(&arrival.intent))
            .expect("the create must arrive");
        let subject = instance::entity_id_of(&create.intent).expect("the create names an entity");

        // Walk this entity's writes in arrival order. Each edit must declare
        // the counter the instance answered with for the write before it. A
        // client that coalesces its two edits into one intent passes this too:
        // what fails is sending a counter it has already been told is
        // superseded.
        let mut acknowledged = create
            .applied_counter()
            .expect("the create is applied, so it is answered with a counter");

        let mut edits = 0;
        for arrival in arrivals
            .iter()
            .filter(|arrival| instance::entity_id_of(&arrival.intent).as_ref() == Some(&subject))
            .filter(|arrival| instance::is_edit(&arrival.intent))
        {
            let base = instance::base_counter_of(&arrival.intent)
                .expect("an edit declares the counter it was based on");
            assert_eq!(
                base, acknowledged,
                "an edit for A arrived against counter {base}, and the instance \
                 had already answered {acknowledged} for the write before it. \
                 Both edits were composed offline against the same counter, so \
                 the client owes the rebase; sending as composed makes the \
                 person conflict with themselves."
            );
            acknowledged = arrival
                .applied_counter()
                .expect("the edit is applied, so it is answered with a counter");
            edits += 1;
        }

        assert!(edits > 0, "at least one edit for A must arrive");

        Outcome::Passed
    }
);

// ---------------------------------------------------------------------------
// D. Idempotence
// ---------------------------------------------------------------------------

scenario!(
    "D1",
    d1_a_lost_acknowledgement_produces_one_entity_not_two,
    {
        let mut world = world().await;
        world.instance.set_behaviour(Behaviour::DropAfterCommit);
        world.launch();
        world.accept("A");

        assert!(
            world.instance.wait_for_intents(1, SETTLE).await,
            "the intent reaches the instance"
        );
        let target = world.instance.intents()[0].intent.intent_id.clone();

        world.instance.set_behaviour(Behaviour::Accept);
        let landed = world
            .instance
            .wait_until(SETTLE, |i| {
                i.intents()
                    .iter()
                    .any(|arrival| arrival.intent.intent_id == target && arrival.answered)
            })
            .await;
        assert!(landed, "the client retries after a lost acknowledgement");

        let arrivals = world.instance.intents();
        let same: Vec<_> = arrivals
            .iter()
            .filter(|arrival| arrival.intent.intent_id == target)
            .collect();
        assert!(
            same.len() >= 2,
            "the retry is a resubmission of the same intent"
        );
        assert!(
            same.iter().all(|arrival| arrival.intent == same[0].intent),
            "it resubmits the same intent identity, unchanged"
        );
        assert_eq!(
            world.instance.committed_entities().len(),
            1,
            "the instance produces one entity"
        );

        let seen = same.len();
        assert!(
            world
                .instance
                .stayed_true(QUIET, |i| i
                    .intents()
                    .iter()
                    .filter(|arrival| arrival.intent.intent_id == target)
                    .count()
                    <= seen)
                .await,
            "the client accepts the returned acknowledgement as final"
        );

        Outcome::Passed
    }
);

scenario!("D2", d2_an_intent_identity_is_generated_once, {
    let mut world = world().await;
    world.instance.set_behaviour(Behaviour::DropBeforeCommit);
    world.launch();
    world.accept("A");

    assert!(
        world.instance.wait_for_intents(2, SETTLE).await,
        "the client retries"
    );
    world.restart_process();
    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| i.intents().len() >= 3)
            .await,
        "and keeps retrying across a restart"
    );

    world.instance.set_behaviour(Behaviour::Accept);
    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| i.intents().iter().any(|a| a.answered))
            .await,
        "and eventually lands"
    );

    let identities: HashSet<Vec<u8>> = world
        .instance
        .intents()
        .iter()
        .map(|arrival| arrival.intent.intent_id.clone())
        .collect();
    assert_eq!(
        identities.len(),
        1,
        "its identity is the value it was given when it was created, across every retry and restart"
    );

    Outcome::Passed
});

scenario!(
    "D3",
    d3_replaying_an_intent_the_instance_already_holds_is_not_a_fault,
    {
        let mut world = world().await;
        world.instance.set_behaviour(Behaviour::DropAfterCommit);
        world.launch();
        world.accept("A");

        assert!(
            world.instance.wait_for_intents(1, SETTLE).await,
            "the instance takes the intent"
        );
        let target = world.instance.intents()[0].intent.intent_id.clone();
        let original = world.instance.intents()[0]
            .acknowledgement
            .clone()
            .expect("the instance committed, so it holds an acknowledgement");

        world.instance.set_behaviour(Behaviour::Accept);
        let landed = world
            .instance
            .wait_until(SETTLE, |i| {
                i.intents()
                    .iter()
                    .any(|arrival| arrival.intent.intent_id == target && arrival.answered)
            })
            .await;
        assert!(landed, "the client submits it again");

        let arrivals = world.instance.intents();
        let replayed = arrivals
            .iter()
            .rev()
            .find(|arrival| arrival.intent.intent_id == target && arrival.answered)
            .expect("the replay was answered");
        assert_eq!(
            replayed.acknowledgement.as_ref(),
            Some(&original),
            "the instance returns the original acknowledgement unchanged"
        );

        let signals = world.showing();
        assert!(
            signals.is_empty(),
            "the client shows no fault, and it showed {signals:?}"
        );
        assert_eq!(
            world.instance.committed_entities().len(),
            1,
            "and no duplicate"
        );

        Outcome::Passed
    }
);

scenario!("D4", d4_replay_after_an_absence_produces_no_duplicates, {
    let mut world = world().await;
    world.instance.set_behaviour(Behaviour::DropAfterCommit);
    world.launch();
    let labels = ["A", "B", "C", "D", "E"];
    for label in labels {
        world.accept(label);
    }

    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| i.committed_entities().len() == labels.len())
            .await,
        "some of its intents arrived, and their acknowledgements did not"
    );

    // The absence.
    world.kill();
    world.instance.set_behaviour(Behaviour::Accept);
    world.launch();

    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| {
                let held: HashSet<_> = i
                    .intents()
                    .iter()
                    .filter(|arrival| arrival.answered)
                    .map(|arrival| arrival.intent.intent_id.clone())
                    .collect();
                held.len() == labels.len()
            })
            .await,
        "it returns and replays its whole outbox"
    );
    assert_eq!(
        world.instance.committed_entities().len(),
        labels.len(),
        "the household holds one of each entity"
    );

    Outcome::Passed
});

// ---------------------------------------------------------------------------
// E. Non-blocking
// ---------------------------------------------------------------------------

scenario!("E1", e1_a_stalled_send_does_not_gate_the_interface, {
    let mut world = world().await;
    world.instance.set_behaviour(Behaviour::Unreachable);
    world.launch();

    let (reply, baseline) = world.act_timed(&Action::Capture {
        label: "base".to_string(),
        content: CaptureContent::Note {
            title: "base".to_string(),
            body: String::new(),
        },
    });
    assert_accepted(&reply, "base");

    world.instance.set_behaviour(Behaviour::Stall);
    assert!(
        world.instance.wait_for_intents(1, SETTLE).await,
        "a send must be in progress"
    );

    let (reply, stalled) = world.act_timed(&Action::Capture {
        label: "during".to_string(),
        content: CaptureContent::Note {
            title: "during".to_string(),
            body: String::new(),
        },
    });
    assert_accepted(&reply, "during");
    assert!(
        stalled <= baseline + Duration::from_millis(500),
        "acceptance happens at the same speed as it would with nothing in flight: {baseline:?} with nothing in flight, {stalled:?} with a send hanging"
    );

    let (reply, latency) = world.act_timed(&Action::Open {
        label: "base".to_string(),
    });
    let seen = expect_entity(&reply);
    assert!(
        seen.present && latency <= Duration::from_millis(500),
        "nothing in the client is unavailable while the send hangs: reading took {latency:?}"
    );

    Outcome::Passed
});

scenario!("E2", e2_a_faulted_item_does_not_stop_unrelated_items, {
    let mut world = world().await;
    world
        .instance
        .set_behaviour(Behaviour::RefuseTitled(vec!["A".to_string()]));
    world.launch();
    world.accept("A");
    world.accept("B");

    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| i
                .intents()
                .iter()
                .any(|arrival| arrival.refused()))
            .await,
        "an intent for entity A must be refused"
    );
    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| i
                .intents()
                .iter()
                .any(|arrival| arrival.applied()
                    && instance::title_of(&arrival.intent).as_deref() == Some("B")))
            .await,
        "B's intents send"
    );

    Outcome::Passed
});

scenario!(
    "E3",
    e3_a_faulted_item_stops_later_intents_for_the_same_entity,
    {
        let mut world = world().await;
        skip_unless!(
            world.offers().offline_edit,
            "this client does not edit while offline"
        );
        world
            .instance
            .set_behaviour(Behaviour::RefuseTitled(vec!["A".to_string()]));
        world.launch();
        world.accept("A");
        world.edit("A", "A/1");

        assert!(
            world
                .instance
                .wait_until(SETTLE, |i| i
                    .intents()
                    .iter()
                    .any(|arrival| arrival.refused()))
                .await,
            "a create for entity A must be refused"
        );

        assert!(
            world
                .instance
                .stayed_true(QUIET, |i| !i.intents().iter().any(
                    |arrival| instance::title_of(&arrival.intent).as_deref() == Some("A/1")
                ))
                .await,
            "the edit waiting behind the refused create is not sent"
        );

        // Retention is observed where it can be: the person's edit is still theirs.
        // What the outbox holds internally is not a conformance concern.
        let seen = expect_entity(&world.open("A"));
        assert!(seen.present, "and it is retained rather than discarded");
        assert_eq!(
            seen.title.as_deref(),
            Some("A/1"),
            "the person's edit is still there"
        );

        Outcome::Passed
    }
);

scenario!("E4", e4_a_refused_intent_is_retained_and_not_retried, {
    let mut world = world().await;
    world
        .instance
        .set_behaviour(Behaviour::RefuseTitled(vec!["A".to_string()]));
    world.launch();
    world.accept("A");

    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| i
                .intents()
                .iter()
                .any(|arrival| arrival.refused()))
            .await,
        "the intent must be refused"
    );
    let target = world
        .instance
        .intents()
        .into_iter()
        .find(instance::IntentArrival::refused)
        .expect("a refused arrival")
        .intent
        .intent_id;

    assert!(
        world
            .instance
            .stayed_true(QUIET, |i| i
                .intents()
                .iter()
                .filter(|arrival| arrival.intent.intent_id == target)
                .count()
                == 1)
            .await,
        "the client does not resubmit a refused intent on its own"
    );

    let seen = expect_entity(&world.open("A"));
    assert!(
        seen.present,
        "nothing is dropped, and nothing is errored away"
    );

    Outcome::Passed
});

// ---------------------------------------------------------------------------
// F. Silence and signalling
// ---------------------------------------------------------------------------

scenario!("F1", f1_depth_is_never_reported, {
    let mut world = world().await;
    world.instance.set_behaviour(Behaviour::Unreachable);
    world.launch();
    for index in 0..40 {
        world.accept(&format!("N{index}"));
    }

    let signals = world.showing();
    assert!(
        signals.is_empty(),
        "no badge, count, banner, or other indication of how many is permitted, and the client showed {signals:?}"
    );

    // Including after an absence.
    world.restart_process();
    let signals = world.showing();
    assert!(
        signals.is_empty(),
        "this holds at every depth, including after an absence: {signals:?}"
    );
    assert!(
        !signals
            .iter()
            .any(|signal| signal.quantity == Some(40) || signal.text.contains("40")),
        "and nothing may say forty"
    );

    Outcome::Passed
});

scenario!("F2", f2_an_unreachable_instance_is_silent, {
    let mut world = world().await;
    world.instance.set_behaviour(Behaviour::Unreachable);
    world.launch();
    for label in ["A", "B", "C"] {
        world.accept(label);
    }

    assert!(
        until_showing(&mut world, QUIET, <[Signal]>::is_empty).await,
        "an unreachable instance is a wait, and nothing is signalled"
    );

    Outcome::Passed
});

scenario!("F3", f3_an_expired_session_is_silent, {
    let mut world = world().await;
    world.instance.set_behaviour(Behaviour::Unauthenticated);
    world.launch();
    for label in ["A", "B", "C"] {
        world.accept(label);
    }

    assert!(
        world.instance.wait_for_intents(1, SETTLE).await,
        "the client keeps trying rather than waiting to be told"
    );
    let signals = world.showing();
    assert!(
        signals.is_empty(),
        "an expired session is a wait, not a fault: {signals:?}"
    );

    // And it clears by the ordinary path continuing to run, with no person in
    // it: the harness does nothing between here and the assertion.
    world.instance.set_behaviour(Behaviour::Accept);
    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| i.committed_entities().len() == 3)
            .await,
        "the client refreshes without a person"
    );
    let signals = world.showing();
    assert!(
        signals.is_empty(),
        "and nothing was signalled on the way: {signals:?}"
    );

    Outcome::Passed
});

scenario!("F4", f4_a_refusal_is_signalled, {
    let mut world = world().await;
    world
        .instance
        .set_behaviour(Behaviour::RefuseTitled(vec!["A".to_string()]));
    world.launch();
    world.accept("A");

    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| i
                .intents()
                .iter()
                .any(|arrival| arrival.refused()))
            .await,
        "the instance must refuse the intent"
    );
    assert!(
        until_showing(&mut world, SETTLE, |signals| !faults(signals).is_empty()).await,
        "the client signals a fault"
    );

    Outcome::Passed
});

scenario!(
    "F5",
    f5_a_local_durability_failure_is_signalled_at_the_moment_of_the_attempt,
    {
        let mut world = world().await;
        world.instance.set_behaviour(Behaviour::Accept);
        skip_unless!(
            world.make_state_unwritable(),
            "this platform's harness cannot make the state directory unwritable"
        );
        world.launch();

        // The moment of the attempt: the answer to the capture itself, not
        // something that turns up later.
        let reply = world.capture("A");
        match &reply {
            Reply::Refused { condition } => {
                assert!(!condition.trim().is_empty(), "the condition is stated");
            }
            other => panic!(
                "the one condition that reaches the guarantee rather than delaying it must be said plainly at the moment of the attempt; the client answered {other:?}"
            ),
        }
        assert!(
            world
                .instance
                .stayed_true(QUIET, |i| i.intents().is_empty())
                .await,
            "nothing was accepted, so nothing may send"
        );

        Outcome::Passed
    }
);

scenario!("F6", f6_an_unrecognised_condition_is_signalled, {
    let mut world = world().await;
    world
        .instance
        .set_behaviour(Behaviour::UnrecognisedCondition);
    world.launch();
    world.accept("A");

    assert!(
        world.instance.wait_for_intents(1, SETTLE).await,
        "the client must reach the instance"
    );
    assert!(
        until_showing(&mut world, SETTLE, |signals| !faults(signals).is_empty()).await,
        "a condition the client cannot classify as self clearing is treated as a fault and signalled"
    );

    Outcome::Passed
});

scenario!(
    "F7",
    f7_a_signal_may_quantify_the_fault_and_never_the_backlog,
    {
        let mut world = world().await;
        let refused: Vec<String> = (0..3).map(|index| format!("R{index}")).collect();
        world
            .instance
            .set_behaviour(Behaviour::RefuseTitled(refused.clone()));
        world.launch();
        for label in &refused {
            world.accept(label);
        }
        assert!(
            world
                .instance
                .wait_until(SETTLE, |i| i
                    .intents()
                    .iter()
                    .filter(|a| a.refused())
                    .count()
                    == 3)
                .await,
            "three intents were refused"
        );

        // And forty are waiting.
        world.instance.set_behaviour(Behaviour::Unreachable);
        for index in 0..40 {
            world.accept(&format!("W{index}"));
        }

        // **Waited for, the way F4 waits for the very same claim.** What is
        // synchronised on above is the *instance's* log, and the instance
        // records a refusal before the answer carrying it has been serialised,
        // let alone read — so a client that has not been told yet is not a
        // client hiding a fault, it is a client that does not know. Asserting
        // on the person's surface the instant the instance decided was asking
        // it to be telepathic, and it failed about one run in four.
        //
        // Nothing this scenario is about has been relaxed. The forty are
        // already captured when the wait begins, so the backlog is live for
        // every assertion below it.
        assert!(
            until_showing(&mut world, SETTLE, |signals| !signals.is_empty()).await,
            "the refusals are still a fault, and a fault is signalled"
        );
        let signals = world.showing();
        for signal in &signals {
            assert!(
                matches!(signal.quantity, None | Some(3)),
                "a signal may say three and no other number, and this one says {:?}",
                signal.quantity
            );
            // Only checkable by convention: a person facing statement is text, so
            // the harness can look for the forbidden number in it and no more.
            assert!(
                !signal.text.contains("40") && !signal.text.contains("43"),
                "and nothing may say forty: {}",
                signal.text
            );
        }

        Outcome::Passed
    }
);

scenario!("F8", f8_a_signal_clears_when_its_condition_clears, {
    let mut world = world().await;
    world
        .instance
        .set_behaviour(Behaviour::UnrecognisedCondition);
    world.launch();
    world.accept("A");

    assert!(
        until_showing(&mut world, SETTLE, |signals| !faults(signals).is_empty()).await,
        "a fault must be signalled first"
    );

    world.instance.set_behaviour(Behaviour::Accept);
    // Nothing here acknowledges or dismisses anything. There is no action in
    // the harness's vocabulary that could.
    assert!(
        until_showing(&mut world, SETTLE, |signals| faults(signals).is_empty()).await,
        "the signal goes when its condition ceases, with nothing for the person to dismiss"
    );

    Outcome::Passed
});

scenario!("F9", f9_a_signal_does_not_escalate_repeat_or_age, {
    let mut world = world().await;
    world
        .instance
        .set_behaviour(Behaviour::UnrecognisedCondition);
    world.launch();
    world.accept("A");

    assert!(
        until_showing(&mut world, SETTLE, |signals| faults(signals).len() == 1).await,
        "a fault must be signalled first"
    );
    let first = world.showing();
    world.drain_events();

    // Three weeks, approximated: long enough for many retry cycles.
    tokio::time::sleep(LONG_ABSENCE).await;

    let later = world.showing();
    assert_eq!(
        later.len(),
        1,
        "when the person returns they find one current statement, and found {later:?}"
    );
    assert_eq!(
        later, first,
        "nothing says how long it has been true, and the statement did not escalate"
    );
    let events = world.drain_events();
    assert!(
        events.is_empty(),
        "and nothing has repeated in the interval: {events:?}"
    );

    Outcome::Passed
});

// ---------------------------------------------------------------------------
// G. Outcomes the instance returns
// ---------------------------------------------------------------------------

scenario!(
    "G1",
    g1_partial_success_leaves_the_successful_part_captured,
    {
        let mut world = world().await;
        world.instance.set_behaviour(Behaviour::Unreachable);
        world.launch();
        for index in 0..200 {
            world.accept(&format!("N{index}"));
        }

        world.instance.set_behaviour(Behaviour::RefuseNth(100));
        assert!(
            world
                .instance
                .wait_until(SETTLE, |i| i.intents().len() >= 200)
                .await,
            "two hundred intents are submitted"
        );

        let arrivals = world.instance.intents();
        let applied: Vec<_> = arrivals
            .iter()
            .filter(|arrival| arrival.applied())
            .collect();
        assert!(
            applied.len() >= 99,
            "the hundredth is refused and ninety nine are applied; {} were",
            applied.len()
        );
        assert_eq!(
            arrivals.iter().filter(|arrival| arrival.refused()).count(),
            1,
            "one refusal, and the submission is not treated as one unit"
        );

        let applied_ids: Vec<Vec<u8>> = applied
            .iter()
            .map(|arrival| arrival.intent.intent_id.clone())
            .collect();
        let unique: HashSet<Vec<u8>> = applied_ids.iter().cloned().collect();
        assert_eq!(
            unique.len(),
            applied_ids.len(),
            "the client handles each acknowledgement on its own terms: an applied intent is not sent again"
        );

        Outcome::Passed
    }
);

scenario!("G2", g2_a_retained_conflict_is_not_a_failure, {
    let mut world = world().await;
    skip_unless!(
        world.offers().offline_edit,
        "this client does not edit while offline"
    );
    world.instance.set_behaviour(Behaviour::ApplyWithConflict);
    world.launch();
    world.accept("A");
    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| i
                .intents()
                .iter()
                .any(|a| a.applied() && a.answered))
            .await,
        "the create must land before there is anything to edit"
    );

    world.edit("A", "A/1");
    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| i
                .intents()
                .iter()
                .any(|arrival| arrival.answered && arrival.conflict().is_some()))
            .await,
        "an edit is applied and its acknowledgement carries a retained conflict"
    );

    let target = world
        .instance
        .intents()
        .into_iter()
        .find(|arrival| arrival.conflict().is_some())
        .expect("the conflicted acknowledgement")
        .intent
        .intent_id;

    let signals = world.showing();
    assert!(
        signals.is_empty(),
        "nothing is signalled as a fault, and the client showed {signals:?}"
    );
    assert!(
        world
            .instance
            .stayed_true(QUIET, |i| i
                .intents()
                .iter()
                .filter(|arrival| arrival.intent.intent_id == target)
                .count()
                == 1)
            .await,
        "the client treats the intent as done and nothing is retried"
    );

    Outcome::Passed
});

scenario!("G3", g3_a_recreation_is_carried_not_lost, {
    let mut world = world().await;
    skip_unless!(
        world.offers().offline_edit,
        "this client does not edit while offline"
    );
    world.instance.set_behaviour(Behaviour::RecreateOnEdit);
    world.launch();
    world.accept("A");
    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| i.intents().iter().any(|a| a.applied()))
            .await,
        "the create must land before there is anything to edit"
    );
    let original = expect_entity(&world.open("A")).entity_id;

    world.edit("A", "A/1");
    assert!(
        world
            .instance
            .wait_until(SETTLE, |i| i
                .intents()
                .iter()
                .any(|a| a.recreated().is_some()))
            .await,
        "the acknowledgement names a new identity"
    );
    let recreated = world
        .instance
        .intents()
        .into_iter()
        .find_map(|arrival| arrival.recreated())
        .expect("a recreation");
    assert_ne!(
        recreated.new_entity_id, original,
        "the new identity is a new identity"
    );

    let adopted = recreated.new_entity_id.clone();
    assert!(
        until_seen(&mut world, "A", SETTLE, |seen| seen.entity_id == adopted).await,
        "the client's local entity adopts the new identity"
    );
    let seen = expect_entity(&world.open("A"));
    assert!(seen.present, "and the person's work is present under it");
    assert_eq!(seen.title.as_deref(), Some("A/1"), "all of it");

    let signals = world.showing();
    assert!(
        signals.is_empty(),
        "this is not signalled as a fault, because nothing was lost: {signals:?}"
    );

    Outcome::Passed
});

scenario!("G4", g4_a_refusal_reveals_nothing_about_existence, {
    async fn signals_for(detail: &str) -> Vec<Signal> {
        let mut world = World::new(client_under_test()).await;
        world.instance.set_behaviour(Behaviour::RefuseNotAvailable {
            detail: detail.to_string(),
        });
        world.launch();
        world.accept("A");
        assert!(
            world
                .instance
                .wait_until(SETTLE, |i| i
                    .intents()
                    .iter()
                    .any(instance::IntentArrival::refused))
                .await,
            "the intent must be refused as not available"
        );
        assert!(
            until_showing(&mut world, SETTLE, |signals| !signals.is_empty()).await,
            "the refusal must reach the person"
        );
        world.showing()
    }

    // The detail is free text for a person reading a log and is never parsed
    // by a client. Nothing anywhere distinguishes absent from invisible.
    let absent = signals_for("entity does not exist").await;
    let invisible = signals_for("entity is not visible to this member").await;
    assert_eq!(
        absent, invisible,
        "the client shows the same thing whether the entity is absent or invisible to this member"
    );

    Outcome::Passed
});

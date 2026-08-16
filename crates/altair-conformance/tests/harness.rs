//! The harness itself works.
//!
//! Ungated on purpose: these run in the default `cargo test --workspace` and
//! stay green. A red conformance suite is only worth anything if the red is
//! about the client and not about the harness, so the pieces the scenarios
//! lean on are exercised here — the fake instance's four behaviours, the
//! process channel, and the difference between a skip and a pass.

use std::sync::Arc;
use std::time::Duration;

use altair_conformance::client::{
    Action, CaptureContent, ClientEnvironment, ClientUnderTest, Offers, Reply,
};
use altair_conformance::instance::{Behaviour, FakeInstance};
use altair_conformance::null::NullClient;
use altair_conformance::world::World;
use altair_conformance::{Outcome, report};
use altair_proto::v1;

fn null_client() -> Arc<dyn ClientUnderTest> {
    Arc::new(NullClient::at(env!("CARGO_BIN_EXE_altair-null-client")))
}

fn an_intent() -> v1::Intent {
    v1::Intent {
        intent_id: uuid::Uuid::new_v4().as_bytes().to_vec(),
        action: Some(v1::intent::Action::Create(v1::Create {
            subject: Some(v1::create::Subject::Entity(v1::CreateEntity {
                entity_id: uuid::Uuid::new_v4().as_bytes().to_vec(),
                created_at: Some(prost_types::Timestamp {
                    seconds: 0,
                    nanos: 0,
                }),
                capture_method: "conformance-harness".to_string(),
                content: Some(v1::EntityContent {
                    title: Some("harness".to_string()),
                    specific: Some(v1::entity_content::Specific::Note(
                        v1::NoteContent::default(),
                    )),
                    ..v1::EntityContent::default()
                }),
            })),
        })),
    }
}

async fn submit(url: &str, intent: v1::Intent) -> Result<v1::SubmitResponse, tonic::Status> {
    let mut client = v1::altair_client::AltairClient::connect(url.to_string())
        .await
        .map_err(|error| tonic::Status::unavailable(error.to_string()))?;
    let response = client
        .submit(v1::SubmitRequest {
            intents: vec![intent],
        })
        .await?;
    Ok(response.into_inner())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_fake_instance_accepts_and_records_what_arrived() {
    let instance = FakeInstance::start().await;
    instance.set_behaviour(Behaviour::Accept);
    let response = submit(&instance.url(), an_intent())
        .await
        .expect("accepted");
    assert_eq!(response.acknowledgements.len(), 1);
    let arrivals = instance.intents();
    assert_eq!(arrivals.len(), 1);
    assert!(arrivals[0].applied());
    assert!(arrivals[0].answered);
    assert_eq!(instance.committed_entities().len(), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_fake_instance_refuses_on_command() {
    let instance = FakeInstance::start().await;
    instance.set_behaviour(Behaviour::RefuseTitled(vec!["harness".to_string()]));
    let response = submit(&instance.url(), an_intent())
        .await
        .expect("answered");
    assert!(matches!(
        response.acknowledgements[0].outcome,
        Some(v1::acknowledgement::Outcome::Refused(_))
    ));
    assert!(instance.committed_entities().is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_fake_instance_can_be_unreachable() {
    let instance = FakeInstance::start().await;
    instance.set_behaviour(Behaviour::Unreachable);
    let outcome = submit(&instance.url(), an_intent()).await;
    assert!(
        outcome.is_err(),
        "nothing connects while the instance is unreachable"
    );
    assert!(instance.intents().is_empty(), "and nothing reaches it");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_fake_instance_can_stall() {
    let instance = FakeInstance::start().await;
    instance.set_behaviour(Behaviour::Stall);
    let waited = tokio::time::timeout(
        Duration::from_millis(750),
        submit(&instance.url(), an_intent()),
    )
    .await;
    assert!(waited.is_err(), "the instance never responds");
    assert_eq!(
        instance.intents().len(),
        1,
        "though the submission did arrive"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_fake_instance_can_drop_after_committing() {
    let instance = FakeInstance::start().await;
    instance.set_behaviour(Behaviour::DropAfterCommit);
    let outcome =
        tokio::time::timeout(Duration::from_secs(5), submit(&instance.url(), an_intent())).await;
    assert!(
        matches!(outcome, Ok(Err(_))),
        "the connection drops before the acknowledgement"
    );
    assert_eq!(
        instance.committed_entities().len(),
        1,
        "after the instance committed"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_fake_instance_returns_a_held_acknowledgement_unchanged() {
    let instance = FakeInstance::start().await;
    instance.set_behaviour(Behaviour::Accept);
    let intent = an_intent();
    let first = submit(&instance.url(), intent.clone())
        .await
        .expect("accepted");
    let second = submit(&instance.url(), intent)
        .await
        .expect("accepted again");
    assert_eq!(first.acknowledgements, second.acknowledgements);
    assert_eq!(
        instance.committed_entities().len(),
        1,
        "one entity, not two"
    );
}

/// C1 and C2 read ordering out of a flat list, so that list has to be flat
/// across submissions and not only within one.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_arrival_order_is_flat_across_submissions() {
    let instance = FakeInstance::start().await;
    instance.set_behaviour(Behaviour::Accept);
    let mut client = v1::altair_client::AltairClient::connect(instance.url())
        .await
        .expect("connect");
    for _ in 0..2 {
        client
            .submit(v1::SubmitRequest {
                intents: vec![an_intent(), an_intent()],
            })
            .await
            .expect("accepted");
    }
    let orders: Vec<usize> = instance
        .intents()
        .iter()
        .map(|arrival| arrival.order)
        .collect();
    assert_eq!(orders, vec![0, 1, 2, 3]);
}

/// G1 leans on the nth intent being counted across the whole run.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn refuse_nth_refuses_exactly_the_nth_intent_to_arrive() {
    let instance = FakeInstance::start().await;
    instance.set_behaviour(Behaviour::RefuseNth(3));
    let mut client = v1::altair_client::AltairClient::connect(instance.url())
        .await
        .expect("connect");
    for _ in 0..4 {
        client
            .submit(v1::SubmitRequest {
                intents: vec![an_intent()],
            })
            .await
            .expect("answered");
    }
    let refused: Vec<usize> = instance
        .intents()
        .iter()
        .filter(|arrival| arrival.refused())
        .map(|a| a.order)
        .collect();
    assert_eq!(refused, vec![2], "the third to arrive, counting from one");
}

/// C3 compares a body's position in the arrival order with an intent's, so the
/// body has to take its position when the stream completes.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_body_takes_its_place_in_the_arrival_order_when_its_stream_completes() {
    let instance = FakeInstance::start().await;
    instance.set_behaviour(Behaviour::Accept);
    let mut client = v1::altair_client::AltairClient::connect(instance.url())
        .await
        .expect("connect");
    let body_id = uuid::Uuid::new_v4().as_bytes().to_vec();
    let chunks = vec![
        v1::BodyChunk {
            body_id: body_id.clone(),
            data: b"one".to_vec(),
        },
        v1::BodyChunk {
            body_id: Vec::new(),
            data: b"two".to_vec(),
        },
    ];
    let ack = client
        .put_body(tokio_stream::iter(chunks))
        .await
        .expect("the body is accepted")
        .into_inner();
    assert_eq!(ack.bytes_received, 6);
    client
        .submit(v1::SubmitRequest {
            intents: vec![an_intent()],
        })
        .await
        .expect("accepted");

    let body = &instance.bodies()[0];
    assert_eq!(body.body_id, body_id);
    assert_eq!(body.bytes, b"onetwo");
    assert!(
        body.seq < instance.intents()[0].seq,
        "the completed body precedes the later intent"
    );
}

/// G2 and G3 read their verdicts off the acknowledgement, so those readers have
/// to see what the behaviours put there.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn conflict_and_recreation_acknowledgements_are_readable() {
    let entity_id = uuid::Uuid::new_v4().as_bytes().to_vec();
    let edit = |title: &str| v1::Intent {
        intent_id: uuid::Uuid::new_v4().as_bytes().to_vec(),
        action: Some(v1::intent::Action::Edit(v1::Edit {
            subject: Some(v1::edit::Subject::Entity(v1::EditEntity {
                entity_id: entity_id.clone(),
                base_counter: 1,
                content: Some(v1::EntityContent {
                    title: Some(title.to_string()),
                    ..v1::EntityContent::default()
                }),
            })),
        })),
    };

    let conflicting = FakeInstance::start().await;
    conflicting.set_behaviour(Behaviour::ApplyWithConflict);
    submit(&conflicting.url(), edit("one"))
        .await
        .expect("applied");
    assert!(conflicting.intents()[0].conflict().is_some());
    assert!(conflicting.intents()[0].applied());

    let recreating = FakeInstance::start().await;
    recreating.set_behaviour(Behaviour::RecreateOnEdit);
    submit(&recreating.url(), edit("one"))
        .await
        .expect("recreated");
    let recreated = recreating.intents()[0].recreated().expect("a recreation");
    assert_eq!(recreated.original_entity_id, entity_id);
    assert_ne!(recreated.new_entity_id, entity_id);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_client_process_answers_on_the_channel() {
    let instance = FakeInstance::start().await;
    let root = std::env::temp_dir().join(format!("altair-harness-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(root.join("state")).expect("state dir");
    std::fs::create_dir_all(root.join("runtime")).expect("runtime dir");
    let environment = ClientEnvironment {
        instance_url: instance.url(),
        state_dir: root.join("state"),
        runtime_dir: root.join("runtime"),
        token: "harness".to_string(),
    };

    let mut client = null_client().launch(&environment).expect("launch");
    let reply = client
        .act(&Action::Capture {
            label: "A".to_string(),
            content: CaptureContent::Bare,
        })
        .expect("an answer");
    assert_eq!(
        reply,
        Reply::Nothing,
        "the Wave 1 client under test implements nothing"
    );
    assert_eq!(
        client.act(&Action::Showing).expect("an answer"),
        Reply::Showing {
            signals: Vec::new()
        }
    );
    client.kill();
    assert!(
        client.act(&Action::Showing).is_err(),
        "a killed client is gone"
    );

    let _ = std::fs::remove_dir_all(&root);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_kill_and_a_device_restart_differ_in_what_survives() {
    let mut world = World::new(null_client()).await;
    world.launch();
    world.restart_process();
    world.restart_device();
    // Reaching here means both restart paths launch a fresh process against
    // the same durable state. What the two do differently to the ephemeral
    // directory is B2's whole subject.
    assert_eq!(world.offers(), Offers::everything());
}

#[test]
fn a_capture_only_client_offers_less_and_therefore_skips_more() {
    let narrow = NullClient::at("unused").offering(Offers::capture_only());
    assert!(
        !narrow.offers().offline_edit,
        "A3, B5, C1, C3, E3, G2 and G3 skip against this client"
    );
    assert!(!narrow.offers().unbound_capture);
    assert!(!narrow.offers().file_capture);
}

#[test]
fn a_skip_is_reported_differently_from_a_pass() {
    let ledger = std::env::temp_dir().join(format!("altair-ledger-{}", uuid::Uuid::new_v4()));
    // SAFETY: single-threaded test, and the variable is removed before it ends.
    unsafe { std::env::set_var("ALTAIR_CONFORMANCE_LEDGER", &ledger) };
    report("A3", "a3_example", &Outcome::Passed);
    report(
        "B5",
        "b5_example",
        &Outcome::Skipped("this client does not capture files".to_string()),
    );
    unsafe { std::env::remove_var("ALTAIR_CONFORMANCE_LEDGER") };

    let written = std::fs::read_to_string(&ledger).expect("the ledger was written");
    let _ = std::fs::remove_file(&ledger);
    assert!(written.contains("A3\tPASSED\ta3_example"));
    assert!(written.contains("B5\tSKIPPED\tb5_example\tthis client does not capture files"));
}

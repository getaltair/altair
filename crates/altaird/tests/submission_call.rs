//! The submission call, served.
//!
//! Two of Wave 2.1's requirements are observable only here and are not testable
//! from an internal function: **a batch is never all or nothing**, and **a
//! refusal reveals nothing, which DR-004 extends to the status code**. This is
//! also token validation's first caller, which was otherwise built and unused.

mod common;

use altair_proto::v1;
use common::served::{NOW, Served, key};
use common::*;
use tonic::Code;
use uuid::Uuid;

fn intent(action: v1::intent::Action) -> v1::Intent {
    v1::Intent {
        intent_id: Uuid::new_v4().as_bytes().to_vec(),
        action: Some(action),
    }
}

fn note(title: &str) -> v1::EntityContent {
    v1::EntityContent {
        title: Some(title.into()),
        specific: Some(v1::entity_content::Specific::Note(
            v1::NoteContent::default(),
        )),
        ..Default::default()
    }
}

fn nowhere() -> altaird::store::ids::EntityId {
    altaird::store::ids::EntityId::from_uuid(Uuid::new_v4())
}

// --- the two things only the wire can show -------------------------------

#[tokio::test]
async fn a_submission_whose_every_intent_is_refused_still_answers_success() {
    let served = Served::new().await;
    let mut client = served.client().await;

    let response = client
        .submit(served.request(
            v1::SubmitRequest {
                intents: vec![
                    intent(edit_entity(nowhere(), 1, note("nowhere"))),
                    intent(edit_entity(nowhere(), 1, note("nowhere either"))),
                ],
            },
            Some(&served.token_for_one()),
        ))
        .await
        .expect("a submission of refusals is a successful submission");

    let acks = response.into_inner().acknowledgements;
    assert_eq!(acks.len(), 2);
    assert!(acks.iter().all(is_not_available));
    assert!(
        acks.iter().all(|a| refused(a).detail.is_empty()),
        "any other status, or any detail, would say at the transport the thing \
         the single refusal reason exists to avoid saying"
    );
}

#[tokio::test]
async fn a_batch_is_never_all_or_nothing() {
    let served = Served::new().await;
    let mut client = served.client().await;

    let first = Uuid::new_v4();
    let last = Uuid::new_v4();

    let acks = client
        .submit(served.request(
            v1::SubmitRequest {
                intents: vec![
                    intent(create_entity(first, note("first"))),
                    intent(edit_entity(nowhere(), 1, note("nowhere"))),
                    intent(create_entity(last, note("last"))),
                ],
            },
            Some(&served.token_for_one()),
        ))
        .await
        .expect("submitted")
        .into_inner()
        .acknowledgements;

    assert_eq!(acks.len(), 3, "one acknowledgement per intent, in order");
    assert!(matches!(
        acks[0].outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));
    assert!(is_not_available(&acks[1]));
    assert!(
        matches!(
            acks[2].outcome,
            Some(v1::acknowledgement::Outcome::Applied(_))
        ),
        "two hundred files where the hundredth fails leaves ninety-nine captured"
    );

    assert_eq!(
        served
            .world
            .title(altaird::store::ids::EntityId::from_uuid(last))
            .await
            .as_deref(),
        Some("last")
    );
}

// --- the credential ------------------------------------------------------

#[tokio::test]
async fn a_valid_token_writes_as_the_member_it_names() {
    let served = Served::new().await;
    let mut client = served.client().await;
    let id = Uuid::new_v4();

    client
        .submit(served.request(
            v1::SubmitRequest {
                intents: vec![intent(create_entity(id, note("mine")))],
            },
            Some(&served.token_for_one()),
        ))
        .await
        .expect("submitted");

    // Member two cannot act on it, so the author was member one.
    let ack = served
        .world
        .submit(
            &served.world.two,
            edit_entity(
                altaird::store::ids::EntityId::from_uuid(id),
                1,
                note("theirs"),
            ),
        )
        .await;
    assert!(is_not_available(&ack));
}

#[tokio::test]
async fn an_absent_credential_reaches_no_query_surface() {
    let served = Served::new().await;
    let mut client = served.client().await;

    let status = client
        .submit(served.request(
            v1::SubmitRequest {
                intents: vec![intent(create_entity(Uuid::new_v4(), note("unbidden")))],
            },
            None,
        ))
        .await
        .expect_err("an unauthenticated request is not served");

    assert_eq!(status.code(), Code::Unauthenticated);
    assert_eq!(
        served.world.changes().await.len(),
        0,
        "unauthenticated reached the write path"
    );
}

#[tokio::test]
async fn an_expired_credential_and_a_forged_one_are_the_same_wait() {
    let served = Served::new().await;
    let mut client = served.client().await;

    let expired = served.key.mint(served.world.one.subject(), NOW - 86_400);
    let forged = key().mint(served.world.one.subject(), NOW + 10 * 365 * 86_400);

    let mut statuses = Vec::new();
    for credential in [expired, forged] {
        let status = client
            .submit(served.request(
                v1::SubmitRequest {
                    intents: vec![intent(create_entity(Uuid::new_v4(), note("held")))],
                },
                Some(&credential),
            ))
            .await
            .expect_err("neither is served");
        statuses.push((status.code(), status.message().to_owned()));
    }

    assert_eq!(
        statuses[0], statuses[1],
        "a forged token and an expired one are literally indistinguishable, and \
         DR-005 defines both as the wait"
    );
    assert_eq!(statuses[0].0, Code::Unauthenticated);
}

// --- every call is now served -----------------------------------------------
//
// Wave 3.1 served `Query`, Wave 3.2 served `Changes`, Wave 3.3 served
// `GetHealth`. Nothing on this service answers `Unimplemented` any longer, so
// there is no "still not served" assertion left to make here — only that each
// newly-served call reaches real handling rather than falling through to a
// blanket refusal.

/// `Query` is served as of Wave 3.1. What is worth asserting here is that it
/// reaches real handling — a malformed `container_id` is refused as
/// malformed, and an ordinary request answers rather than falling through to
/// a blanket `Unimplemented`.
#[tokio::test]
async fn query_is_no_longer_among_the_unserved() {
    let served = Served::new().await;
    let mut client = served.client().await;
    let token = served.token_for_one();

    let malformed = client
        .query(served.request(
            v1::QueryRequest {
                container_id: Some(vec![1, 2, 3]),
                ..Default::default()
            },
            Some(&token),
        ))
        .await
        .expect_err("a container identity is 16 bytes")
        .code();
    assert_eq!(malformed, Code::InvalidArgument);

    let ok = client
        .query(served.request(v1::QueryRequest::default(), Some(&token)))
        .await
        .expect("a default query answers rather than refusing");
    assert_eq!(
        ok.into_inner().results.len(),
        0,
        "empty text matches nothing, which is an ordinary empty answer"
    );
}

/// `Changes` is served as of Wave 3.2 — `tests/changes.rs` covers its
/// per-member assembly. What is worth asserting here is the same thing
/// `query_is_no_longer_among_the_unserved` asserts for `Query`: the call
/// reaches real handling, not a blanket `Unimplemented`, and an
/// unauthenticated caller still gets nowhere near it.
#[tokio::test]
async fn changes_is_no_longer_among_the_unserved() {
    let served = Served::new().await;
    let mut client = served.client().await;
    let token = served.token_for_one();

    let response = client
        .changes(served.request(v1::ChangesRequest::default(), Some(&token)))
        .await
        .expect("a default request is answerable, not unimplemented")
        .into_inner();
    assert!(matches!(
        response.outcome,
        Some(v1::changes_response::Outcome::Changes(_))
    ));

    let unauthenticated = client
        .changes(served.request(v1::ChangesRequest::default(), None))
        .await
        .expect_err("unauthenticated reaches no query surface")
        .code();
    assert_eq!(unauthenticated, Code::Unauthenticated);
}

/// `PutBody` and `GetBody` are served as of Wave 2.3, `GetHealth` as of Wave
/// 3.3 — `tests/file_bodies.rs` and `tests/health.rs` cover their behaviour.
/// What is still worth asserting here is that a malformed call to either of
/// the first two reaches real handling rather than falling through to a
/// blanket `Unimplemented`.
#[tokio::test]
async fn put_body_and_get_body_are_no_longer_among_the_unserved() {
    let served = Served::new().await;
    let mut client = served.client().await;
    let token = served.token_for_one();

    let put_body = client
        .put_body(served.request(
            tokio_stream::iter(Vec::<v1::BodyChunk>::new()),
            Some(&token),
        ))
        .await
        .expect_err("an empty upload is malformed, not unserved")
        .code();
    let get_body = client
        .get_body(served.request(v1::BodyRequest::default(), Some(&token)))
        .await
        .expect_err("a default request names no entity")
        .code();

    assert_eq!(put_body, Code::InvalidArgument);
    assert_eq!(get_body, Code::InvalidArgument);
}

//! The device store, without a network and without a database.
//!
//! Everything the conformance suite checks through a running client is checked
//! here at the store's own boundary, where a failure names the rule rather than
//! the scenario.

use altair_proto::v1;
use altair_tui::capture::Captured;
use altair_tui::device::Store;
use altair_tui::wire;

fn store() -> (tempfile::TempDir, Store) {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let store = Store::open(directory.path()).expect("open");
    (directory, store)
}

/// Accept a note titled `title`, and answer what it was given.
fn accept(store: &mut Store, title: &str) -> Vec<u8> {
    let composed = wire::compose_creation(&Captured::Note {
        title: title.to_string(),
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
    composed.entity_id
}

fn queue_edit(store: &mut Store, entity_id: &[u8], title: &str) -> Vec<u8> {
    let (patch, intent) = wire::compose_title_edit(entity_id, title);
    store.accept_edit(entity_id, &patch, &intent).expect("edit");
    intent.intent_id
}

#[test]
fn what_was_accepted_is_readable_immediately() {
    let (_directory, mut store) = store();
    let entity_id = accept(&mut store, "A");
    let held = store.held(&entity_id).expect("read").expect("present");
    assert_eq!(held.title(), Some("A"));
}

#[test]
fn a_capture_outlives_the_handle_that_took_it() {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let entity_id = {
        let mut store = Store::open(directory.path()).expect("open");
        accept(&mut store, "A")
    };
    // Reopening is what a restart is, from the store's side.
    let store = Store::open(directory.path()).expect("reopen");
    assert!(
        store.held(&entity_id).expect("read").is_some(),
        "acceptance is durable, so it survives the process that took it"
    );
    assert_eq!(
        store.next_to_send(10).expect("outbox").len(),
        1,
        "and so does the intent that will carry it"
    );
}

#[test]
fn a_bare_capture_invents_nothing() {
    let (_directory, mut store) = store();
    let composed = wire::compose_creation(&Captured::Bare);
    store
        .accept_creation(
            &composed.entity_id,
            &composed.content,
            None,
            &composed.intent,
        )
        .expect("accept");
    let held = store
        .held(&composed.entity_id)
        .expect("read")
        .expect("present");
    assert_eq!(held.title(), None, "no field is required on this path");
    assert!(
        held.content.specific.is_some(),
        "a type is the tag of content.specific, and it is the one thing there is"
    );
}

#[test]
fn at_most_one_intent_per_entity_is_offered() {
    let (_directory, mut store) = store();
    let a = accept(&mut store, "A");
    let b = accept(&mut store, "B");
    queue_edit(&mut store, &a, "A/1");
    queue_edit(&mut store, &a, "A/2");

    let batch = store.next_to_send(10).expect("outbox");
    assert_eq!(
        batch.len(),
        2,
        "one for A and one for B, and never two for A: this is what makes a \
         create reach the instance before its own edits"
    );
    assert_eq!(batch[0].entity_id, a);
    assert_eq!(batch[1].entity_id, b);
    assert!(
        matches!(batch[0].intent.action, Some(v1::intent::Action::Create(_))),
        "and the one offered for A is the oldest, which is the create"
    );
}

#[test]
fn an_edit_is_offered_against_the_counter_the_instance_answered_with() {
    let (_directory, mut store) = store();
    let a = accept(&mut store, "A");
    queue_edit(&mut store, &a, "A/1");
    queue_edit(&mut store, &a, "A/2");

    // Both edits were composed while the instance was unreachable, so both
    // were composed against the counter the client last knew, which was none.
    let create = store.next_to_send(10).expect("outbox").remove(0);
    store.applied(&create.intent_id, &a, 1).expect("applied");

    let first = store.next_to_send(10).expect("outbox").remove(0);
    assert_eq!(
        first.acknowledged_counter, 1,
        "the first edit goes out against what the create was answered with"
    );
    store.applied(&first.intent_id, &a, 2).expect("applied");

    let second = store.next_to_send(10).expect("outbox").remove(0);
    assert_eq!(
        second.acknowledged_counter, 2,
        "and the second against what the first was answered with, rather than \
         against the counter it was composed against — otherwise the person \
         conflicts with themselves over a sequence they performed in order"
    );
}

#[test]
fn an_applied_intent_leaves_the_outbox_and_is_not_offered_again() {
    let (_directory, mut store) = store();
    let a = accept(&mut store, "A");
    let create = store.next_to_send(10).expect("outbox").remove(0);
    store.applied(&create.intent_id, &a, 1).expect("applied");
    assert!(
        store.next_to_send(10).expect("outbox").is_empty(),
        "nothing applied is ever sent a second time"
    );
    assert!(
        store.held(&a).expect("read").is_some(),
        "and the entity is still the person's"
    );
}

#[test]
fn a_refusal_is_retained_and_holds_what_is_behind_it() {
    let (_directory, mut store) = store();
    let a = accept(&mut store, "A");
    let b = accept(&mut store, "B");
    queue_edit(&mut store, &a, "A/1");

    let create = store.next_to_send(10).expect("outbox").remove(0);
    store.refused(&create.intent_id, &a).expect("refused");

    let batch = store.next_to_send(10).expect("outbox");
    assert_eq!(batch.len(), 1, "B is unrelated and still sends");
    assert_eq!(batch[0].entity_id, b);
    assert_eq!(
        store.refused_count().expect("count"),
        1,
        "the refused intent stays: nothing is dropped and nothing is errored away"
    );
    let held = store.held(&a).expect("read").expect("present");
    assert_eq!(
        held.title(),
        Some("A/1"),
        "and the person's edit is still theirs, even though it will not send"
    );
}

#[test]
fn a_recreation_moves_the_identity_and_everything_still_queued() {
    let (_directory, mut store) = store();
    let original = accept(&mut store, "A");
    let create = store.next_to_send(10).expect("outbox").remove(0);
    store.applied(&create.intent_id, &original, 1).expect("ok");

    queue_edit(&mut store, &original, "A/1");
    queue_edit(&mut store, &original, "A/2");
    let first = store.next_to_send(10).expect("outbox").remove(0);

    let new = wire::fresh_id();
    store
        .recreated(&first.intent_id, &original, &new, 1)
        .expect("recreated");

    let held = store.held(&new).expect("read").expect("present");
    assert_eq!(
        held.entity_id, new,
        "the local entity adopts the new identity"
    );
    assert_eq!(
        held.title(),
        Some("A/2"),
        "and all of the person's work is under it — both edits were accepted \
         locally, so what is held is the later one whether or not either has sent"
    );
    assert!(
        store.held(&original).expect("read").is_some(),
        "anything still holding the old identity finds it, because the client \
         keeps the trail rather than making what it held unreachable"
    );

    let queued = store.next_to_send(10).expect("outbox");
    assert_eq!(queued.len(), 1);
    assert_eq!(
        queued[0].entity_id, new,
        "and what was still queued names the identity that now exists"
    );
    match &queued[0].intent.action {
        Some(v1::intent::Action::Edit(v1::Edit {
            subject: Some(v1::edit::Subject::Entity(entity)),
        })) => assert_eq!(
            entity.entity_id, new,
            "inside the encoded intent too, not only in the column beside it"
        ),
        other => panic!("expected a queued edit and found {other:?}"),
    }
}

#[test]
fn a_body_is_held_until_it_reaches_the_instance() {
    let (_directory, mut store) = store();
    let bytes = b"the bytes acceptance has to cover".to_vec();
    let composed = wire::compose_creation(&Captured::File {
        media_type: "text/plain".to_string(),
        bytes: bytes.clone(),
    });
    let (body_id, body_bytes) = composed.body.clone().expect("a file has a body");
    store
        .accept_creation(
            &composed.entity_id,
            &composed.content,
            Some((&body_id, &body_bytes)),
            &composed.intent,
        )
        .expect("accept");

    let held = store
        .held(&composed.entity_id)
        .expect("read")
        .expect("present");
    assert_eq!(
        held.bytes.as_ref(),
        Some(&bytes),
        "acceptance covers the bytes, so they are here the moment it is shown"
    );
    assert_eq!(
        store.unsent_body(&body_id).expect("read"),
        Some(bytes.clone()),
        "and they are outstanding until they have gone"
    );
    store.body_sent(&body_id).expect("sent");
    assert_eq!(
        store.unsent_body(&body_id).expect("read"),
        None,
        "sending it once is enough"
    );
    assert_eq!(
        store
            .held(&composed.entity_id)
            .expect("read")
            .expect("present")
            .bytes
            .as_ref(),
        Some(&bytes),
        "and the person still has their file"
    );
}

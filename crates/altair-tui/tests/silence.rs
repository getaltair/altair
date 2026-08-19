//! What the client never says.
//!
//! **Waiting is silent and faults signal**, and the difference is easy to erode
//! one helpful addition at a time. Both of these were violations found by
//! running the client rather than by reading it: the faults screen listed
//! everything with an intent outstanding — under a heading counting it — and
//! every row on the captures list wore a "not sent" marker, which is an
//! indication of how many as soon as there is more than one.

use altair_tui::app::compose;
use altair_tui::capture::Captured;
use altair_tui::device::Store;
use altair_tui::wire;

fn store_with(unsent: usize) -> (tempfile::TempDir, Store) {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let mut store = Store::open(directory.path()).expect("open");
    for index in 0..unsent {
        let composed = wire::compose_creation(&Captured::Note {
            title: format!("capture {index}"),
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
    }
    (directory, store)
}

#[test]
fn a_backlog_reaches_no_screen() {
    let (_directory, store) = store_with(40);
    let held = store.captures().expect("captures");
    assert_eq!(held.len(), 40, "all forty are the person's");

    let groups = compose::captures(&held, 0);
    for group in &groups {
        assert!(
            !group.label.chars().any(|c| c.is_ascii_digit()) || !group.label.contains("40"),
            "a section label counted the backlog: {}",
            group.label
        );
        for row in &group.rows {
            assert!(
                row.note.is_empty(),
                "a row says {:?} about not having been sent. Forty rows saying \
                 it is an indication of how many, which is the one thing that \
                 may never reach a person.",
                row.note
            );
        }
    }
}

#[test]
fn nothing_unsent_is_a_fault() {
    let (_directory, store) = store_with(40);
    assert!(
        store.refused_entities().expect("refused").is_empty(),
        "forty things on their way is a wait, and a wait is silent. Only what \
         the instance actually refused belongs on the faults screen."
    );
    assert_eq!(
        store.refused_count().expect("count"),
        0,
        "and the one number this client states is refusals, not depth"
    );
}

#[test]
fn a_refusal_is_the_only_thing_that_becomes_a_fault() {
    let (_directory, mut store) = store_with(3);
    let outstanding = store.next_to_send(10).expect("outbox");
    assert_eq!(outstanding.len(), 3);

    store
        .refused(&outstanding[0].intent_id, &outstanding[0].entity_id)
        .expect("refused");

    let refused = store.refused_entities().expect("refused");
    assert_eq!(
        refused.len(),
        1,
        "one was refused and two are still on their way; only the refusal is a \
         fault"
    );
    assert_eq!(store.refused_count().expect("count"), 1);
    assert!(
        refused[0].pending,
        "and it is still the person's — nothing was dropped and nothing was \
         errored away"
    );
}

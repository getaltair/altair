//! `Changes`: one member's own stream, assembled from the sequence.
//!
//! Every scenario here checks the same two things the architecture names as
//! the point of the whole exercise: an entity's own audience decides whether
//! its changes reach a member, and a narrowing (or an erasure, which is the
//! strongest narrowing there is) arrives as a disappearance rather than
//! silence. A relation has no audience of its own, so its own set of
//! scenarios checks the decision this lane made instead: both endpoints or
//! nothing.

mod common;

use altair_proto::v1;
use altaird::read::changes::{self, Outcome};
use altaird::store::ids::MemberId;
use common::*;
use sqlx::Row;
use uuid::Uuid;

fn member_id(m: &altaird::auth::Member) -> MemberId {
    MemberId::for_test(m.membership_id())
}

fn answered(outcome: Outcome) -> v1::ChangeSet {
    match outcome {
        Outcome::Answered(set) => set,
        Outcome::Unanswerable => panic!("expected an answer, got unanswerable"),
    }
}

fn entity_id_of(c: &v1::Change) -> Option<Uuid> {
    match &c.what {
        Some(v1::change::What::Entity(e)) => {
            Some(Uuid::from_slice(&e.entity_id).expect("16 bytes"))
        }
        _ => None,
    }
}

fn gone_entity_id_of(c: &v1::Change) -> Option<Uuid> {
    match &c.what {
        Some(v1::change::What::GoneEntityId(b)) => Some(Uuid::from_slice(b).expect("16 bytes")),
        _ => None,
    }
}

fn relation_id_of(c: &v1::Change) -> Option<Uuid> {
    match &c.what {
        Some(v1::change::What::Relation(r)) => {
            Some(Uuid::from_slice(&r.relation_id).expect("16 bytes"))
        }
        _ => None,
    }
}

fn gone_relation_id_of(c: &v1::Change) -> Option<Uuid> {
    match &c.what {
        Some(v1::change::What::GoneRelationId(b)) => Some(Uuid::from_slice(b).expect("16 bytes")),
        _ => None,
    }
}

async fn global_latest(w: &World) -> i64 {
    sqlx::query("SELECT next_position - 1 AS latest FROM change_position")
        .fetch_one(&w.db.pool)
        .await
        .expect("change_position")
        .try_get("latest")
        .expect("latest")
}

// --- entities --------------------------------------------------------------

#[tokio::test]
async fn an_entity_in_the_audience_arrives_as_entity() {
    let w = World::new().await;
    let id = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("shared".into()),
                audience_member_ids: vec![w.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;

    let set = answered(
        changes::assemble(&w.db.pool, member_id(&w.two), None)
            .await
            .expect("assemble"),
    );
    assert_eq!(set.changes.len(), 1);
    assert_eq!(entity_id_of(&set.changes[0]), Some(id.as_uuid()));
}

#[tokio::test]
async fn an_entity_not_in_the_audience_never_arrives() {
    let w = World::new().await;
    w.create(
        &w.one,
        v1::entity_content::Specific::Note(v1::NoteContent::default()),
        v1::EntityContent {
            title: Some("private".into()),
            ..Default::default()
        },
    )
    .await;

    let set = answered(
        changes::assemble(&w.db.pool, member_id(&w.two), None)
            .await
            .expect("assemble"),
    );
    assert!(set.changes.is_empty());
}

#[tokio::test]
async fn narrowing_an_audience_away_from_a_member_arrives_as_gone() {
    let w = World::new().await;
    let id = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("shared then not".into()),
                audience_member_ids: vec![w.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;

    // A client that was watching sees the creation before the narrowing
    // happens — the ordinary case, and the one the architecture's own
    // sequence diagram describes: creation, then disappearance.
    let caught_up = answered(
        changes::assemble(&w.db.pool, member_id(&w.two), None)
            .await
            .expect("assemble"),
    );
    assert_eq!(caught_up.changes.len(), 1);
    assert_eq!(entity_id_of(&caught_up.changes[0]), Some(id.as_uuid()));

    let counter = w.counter(id).await;
    let ack = w
        .submit(
            &w.one,
            edit_entity(
                id,
                counter as u64,
                v1::EntityContent {
                    cleared: vec![5], // audience_member_ids field number
                    ..Default::default()
                },
            ),
        )
        .await;
    assert!(matches!(
        ack.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));

    let next = answered(
        changes::assemble(&w.db.pool, member_id(&w.two), caught_up.position)
            .await
            .expect("assemble"),
    );
    assert_eq!(next.changes.len(), 1);
    assert_eq!(gone_entity_id_of(&next.changes[0]), Some(id.as_uuid()));
}

#[tokio::test]
async fn a_client_that_falls_behind_both_writes_is_never_shown_content_it_cannot_currently_see() {
    // The other half of the same guarantee: a client that was away for both
    // the creation and the narrowing must never receive the entity's content
    // at all, because by the time it asks, it cannot see the entity. Both of
    // this entity's rows are relevant to the member's stream — the row-level
    // audience snapshot says so — but content is gated on *current*
    // visibility, so both collapse to the same disappearance rather than
    // leaking a title nobody may show this member any more.
    let w = World::new().await;
    let id = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("shared then not".into()),
                audience_member_ids: vec![w.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;
    let counter = w.counter(id).await;
    w.submit(
        &w.one,
        edit_entity(
            id,
            counter as u64,
            v1::EntityContent {
                cleared: vec![5],
                ..Default::default()
            },
        ),
    )
    .await;

    let set = answered(
        changes::assemble(&w.db.pool, member_id(&w.two), None)
            .await
            .expect("assemble"),
    );
    assert_eq!(set.changes.len(), 2);
    for change in &set.changes {
        assert_eq!(entity_id_of(change), None, "content must never appear here");
        assert_eq!(gone_entity_id_of(change), Some(id.as_uuid()));
    }
}

#[tokio::test]
async fn erasure_reaches_a_member_who_could_see_it_as_gone() {
    let w = World::new().await;
    let id = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("erased later".into()),
                audience_member_ids: vec![w.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;
    let caught_up = answered(
        changes::assemble(&w.db.pool, member_id(&w.two), None)
            .await
            .expect("assemble"),
    );
    assert_eq!(caught_up.changes.len(), 1);
    assert_eq!(entity_id_of(&caught_up.changes[0]), Some(id.as_uuid()));

    let ack = w.submit(&w.one, erase(&[id])).await;
    assert!(matches!(
        ack.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));

    let next = answered(
        changes::assemble(&w.db.pool, member_id(&w.two), caught_up.position)
            .await
            .expect("assemble"),
    );
    assert_eq!(next.changes.len(), 1);
    assert_eq!(gone_entity_id_of(&next.changes[0]), Some(id.as_uuid()));
}

#[tokio::test]
async fn an_author_sees_their_own_private_entity() {
    let w = World::new().await;
    let id = w.note(&w.one, "mine").await;

    let set = answered(
        changes::assemble(&w.db.pool, member_id(&w.one), None)
            .await
            .expect("assemble"),
    );
    assert_eq!(set.changes.len(), 1);
    assert_eq!(entity_id_of(&set.changes[0]), Some(id.as_uuid()));
}

// --- relations ---------------------------------------------------------------

#[tokio::test]
async fn a_relation_reaches_a_member_who_can_see_both_ends() {
    let w = World::new().await;
    let shared = v1::EntityContent {
        audience_member_ids: vec![w.two.membership_id().as_bytes().to_vec()],
        ..Default::default()
    };
    let a = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("a".into()),
                ..shared.clone()
            },
        )
        .await;
    let b = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("b".into()),
                ..shared
            },
        )
        .await;
    let ack = w
        .submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(a, b)),
        )
        .await;
    let rel_id = relation_id(&ack);

    let set = answered(
        changes::assemble(&w.db.pool, member_id(&w.two), None)
            .await
            .expect("assemble"),
    );
    let relation_change = set
        .changes
        .iter()
        .find(|c| relation_id_of(c) == Some(rel_id));
    assert!(
        relation_change.is_some(),
        "expected the relation among {:?}",
        set.changes.iter().map(|c| &c.what).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn a_relation_is_withheld_when_one_end_is_private() {
    let w = World::new().await;
    let visible = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("visible".into()),
                audience_member_ids: vec![w.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;
    let private = w.note(&w.one, "private").await;
    let ack = w
        .submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(visible, private)),
        )
        .await;
    let rel_id = relation_id(&ack);

    let set = answered(
        changes::assemble(&w.db.pool, member_id(&w.two), None)
            .await
            .expect("assemble"),
    );
    assert!(
        !set.changes
            .iter()
            .any(|c| relation_id_of(c) == Some(rel_id) || gone_relation_id_of(c) == Some(rel_id)),
        "a relation naming an entity this member cannot see must never be shown, in either \
         direction: {:?}",
        set.changes.iter().map(|c| &c.what).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn removing_a_relation_reaches_a_member_who_can_see_both_ends_as_gone() {
    let w = World::new().await;
    let shared = v1::EntityContent {
        audience_member_ids: vec![w.two.membership_id().as_bytes().to_vec()],
        ..Default::default()
    };
    let a = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("a".into()),
                ..shared.clone()
            },
        )
        .await;
    let b = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("b".into()),
                ..shared
            },
        )
        .await;
    let ack = w
        .submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(a, b)),
        )
        .await;
    let rel_id = relation_id(&ack);
    w.submit(&w.one, remove(&[], &[rel_id])).await;

    let set = answered(
        changes::assemble(&w.db.pool, member_id(&w.two), None)
            .await
            .expect("assemble"),
    );
    assert!(
        set.changes
            .iter()
            .any(|c| gone_relation_id_of(c) == Some(rel_id)),
        "expected the removal among {:?}",
        set.changes.iter().map(|c| &c.what).collect::<Vec<_>>()
    );
}

// --- the horizon -------------------------------------------------------------

#[tokio::test]
async fn a_position_before_the_earliest_retained_is_unanswerable() {
    let w = World::new().await;
    w.note(&w.one, "one").await;
    w.note(&w.one, "two").await;
    w.note(&w.one, "three").await;

    // Nothing has trimmed the sequence yet — reclamation is Wave 2.4 and does
    // not exist — so this stands in for what it will one day leave behind:
    // the earliest position the table still holds moving forward of what a
    // returning client remembers.
    sqlx::query("DELETE FROM change WHERE position < (SELECT max(position) FROM change)")
        .execute(&w.db.pool)
        .await
        .expect("simulate reclamation");

    let outcome = changes::assemble(
        &w.db.pool,
        member_id(&w.one),
        Some(v1::Position { value: 0 }),
    )
    .await
    .expect("assemble");
    assert!(matches!(outcome, Outcome::Unanswerable));
}

#[tokio::test]
async fn an_empty_sequence_is_always_answerable() {
    let w = World::new().await;
    let set = answered(
        changes::assemble(&w.db.pool, member_id(&w.one), None)
            .await
            .expect("assemble"),
    );
    assert!(set.changes.is_empty());
    assert_eq!(set.position, Some(v1::Position { value: 0 }));
    assert!(!set.more);
}

#[tokio::test]
async fn a_page_that_is_not_full_reports_the_sequences_own_latest_position() {
    let w = World::new().await;
    w.note(&w.one, "one").await;
    w.note(&w.one, "two").await;

    let set = answered(
        changes::assemble(&w.db.pool, member_id(&w.one), None)
            .await
            .expect("assemble"),
    );
    assert!(!set.more);
    assert_eq!(
        set.position,
        Some(v1::Position {
            value: global_latest(&w).await as u64
        })
    );
}

// --- the horizon check and the page read must agree ------------------------
//
// Both come out of one statement now (see read/changes.rs's own comment on
// why), specifically so a concurrent write or reclamation cannot land
// between "how far back can I answer" and "here is the page" and make the
// two disagree. That is not something a test can force a race into without
// controlling statement-level timing, which nothing here does — but the
// invariant the bug would have broken is checkable directly: nothing this
// function returns should ever claim to cover a change beyond its own
// reported cursor.

#[tokio::test]
async fn the_reported_cursor_never_falls_behind_a_change_it_returned() {
    let w = World::new().await;
    // More than one page's worth, so `more` is true and the cursor comes
    // from the boundary this page actually scanned rather than simply
    // echoing the sequence's own latest position.
    for i in 0..205 {
        w.note(&w.one, &format!("note {i}")).await;
    }

    let set = answered(
        changes::assemble(&w.db.pool, member_id(&w.one), None)
            .await
            .expect("assemble"),
    );
    assert!(set.more, "205 notes must not all fit in one page");
    let cursor = set.position.expect("position").value;
    for change in &set.changes {
        let position = change.position.as_ref().expect("position").value;
        assert!(
            position <= cursor,
            "a change at {position} was returned but the reported cursor was only {cursor} — \
             a client resuming from that cursor would never see it"
        );
    }
    assert!(
        (cursor as i64) < global_latest(&w).await,
        "more of the sequence exists past this page"
    );
}

// --- a position outside anything this store could have issued --------------

#[tokio::test]
async fn a_position_that_does_not_fit_the_stores_own_representation_never_panics() {
    let w = World::new().await;
    w.note(&w.one, "one").await;

    // The store's own sequence is `bigint` (`i64`); the wire's `Position` is
    // a bare `uint64`. Nothing stops a client — buggy or otherwise — from
    // sending a value this instance could never have issued, including
    // exactly `i64::MAX`, which is the one value that overflows the `+ 1`
    // the horizon check does internally.
    for value in [u64::MAX, i64::MAX as u64 + 1, i64::MAX as u64] {
        let outcome =
            changes::assemble(&w.db.pool, member_id(&w.one), Some(v1::Position { value }))
                .await
                .expect("an out-of-range position is not a store fault");
        let set = answered(outcome);
        assert!(
            set.changes.is_empty(),
            "nothing real exists after a position beyond anything this store ever issued"
        );
    }
}

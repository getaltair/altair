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

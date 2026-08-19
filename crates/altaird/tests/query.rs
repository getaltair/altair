//! The literal retrieval arm, served through `WritePath::query`.
//!
//! `store/search.rs` is built entirely on `CandidateQuery`, the same builder
//! `tests/audience.rs` and `tests/changes.rs` check against the write and the
//! change stream. This file checks the same property against the third
//! consumer: that a private entity never surfaces as a candidate for a member
//! outside its audience, alongside the scoping and ordering behaviour that is
//! particular to this arm.

mod common;

use altaird::store::entity::EntityType;
use altaird::store::search::{Domain, Scope};
use common::*;

const LIMIT: i64 = 20;

#[tokio::test]
async fn a_private_entity_never_appears_in_another_members_results() {
    let w = World::new().await;

    // No audience_member_ids set: private to the author, by default, at
    // creation.
    w.note(&w.one, "a secret about kobolds").await;

    let theirs = w
        .write
        .query(&w.two, "kobolds", &Scope::default(), LIMIT)
        .await
        .expect("query answers");
    assert!(
        theirs.is_empty(),
        "member two must not see member one's private note"
    );

    // The same text, from the author, proves the query text does match —
    // an empty result above is audience scoping, not a text that fails to
    // hit the index.
    let ours = w
        .write
        .query(&w.one, "kobolds", &Scope::default(), LIMIT)
        .await
        .expect("query answers");
    assert_eq!(ours.len(), 1);
}

#[tokio::test]
async fn sharing_widens_who_a_result_reaches() {
    let w = World::new().await;
    w.create(
        &w.one,
        altair_proto::v1::entity_content::Specific::Note(altair_proto::v1::NoteContent::default()),
        altair_proto::v1::EntityContent {
            title: Some("a shared plan for the gnoll camp".into()),
            audience_member_ids: vec![w.two.membership_id().as_bytes().to_vec()],
            ..Default::default()
        },
    )
    .await;

    let theirs = w
        .write
        .query(&w.two, "gnoll", &Scope::default(), LIMIT)
        .await
        .expect("query answers");
    assert_eq!(theirs.len(), 1);
}

#[tokio::test]
async fn a_real_full_text_match_returns_the_matching_entity() {
    let w = World::new().await;
    let id = w.note(&w.one, "the lighthouse keeper's ledger").await;
    w.note(&w.one, "an entirely unrelated grocery list").await;

    let results = w
        .write
        .query(&w.one, "lighthouse", &Scope::default(), LIMIT)
        .await
        .expect("query answers");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].entity.id, id);
}

#[tokio::test]
async fn domain_scoping_excludes_a_match_outside_the_domain() {
    let w = World::new().await;
    // A note (Knowledge) matching the text.
    w.note(&w.one, "the wyvern nest survey").await;

    let scope = Scope {
        domain: Some(Domain::Tracking),
        ..Default::default()
    };
    let results = w
        .write
        .query(&w.one, "wyvern", &scope, LIMIT)
        .await
        .expect("query answers");
    assert!(
        results.is_empty(),
        "a note must not surface when the query is scoped to Tracking"
    );
}

#[tokio::test]
async fn entity_type_scoping_narrows_to_exactly_that_type() {
    let w = World::new().await;
    let id = w.note(&w.one, "the wyvern nest survey").await;

    let scope = Scope {
        entity_type: Some(EntityType::Note),
        ..Default::default()
    };
    let results = w
        .write
        .query(&w.one, "wyvern", &scope, LIMIT)
        .await
        .expect("query answers");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].entity.id, id);

    let scope = Scope {
        entity_type: Some(EntityType::Item),
        ..Default::default()
    };
    let results = w
        .write
        .query(&w.one, "wyvern", &scope, LIMIT)
        .await
        .expect("query answers");
    assert!(results.is_empty());
}

#[tokio::test]
async fn container_scoping_admits_the_container_and_its_members_only() {
    let w = World::new().await;
    let category = w
        .create(
            &w.one,
            altair_proto::v1::entity_content::Specific::Category(
                altair_proto::v1::CategoryContent::default(),
            ),
            altair_proto::v1::EntityContent {
                title: Some("the wyvern archive".into()),
                ..Default::default()
            },
        )
        .await;
    let filed = w
        .create(
            &w.one,
            altair_proto::v1::entity_content::Specific::Note(
                altair_proto::v1::NoteContent::default(),
            ),
            altair_proto::v1::EntityContent {
                title: Some("wyvern migration notes".into()),
                category_id: Some(category.as_uuid().as_bytes().to_vec()),
                ..Default::default()
            },
        )
        .await;
    // Same matching text, filed nowhere.
    w.note(&w.one, "wyvern nesting habits, unfiled").await;

    let scope = Scope {
        container: Some(category),
        ..Default::default()
    };
    let results = w
        .write
        .query(&w.one, "wyvern", &scope, LIMIT)
        .await
        .expect("query answers");

    let ids: Vec<_> = results.iter().map(|r| r.entity.id).collect();
    assert!(
        ids.contains(&filed),
        "the entity filed under the container must be a candidate"
    );
    assert!(
        ids.contains(&category),
        "the container satisfies `e.id = container` on its own account, and \
         its own title also matches the query text"
    );
    assert_eq!(
        ids.len(),
        2,
        "the unfiled entity sharing the same text must not be a candidate"
    );
}

#[tokio::test]
async fn the_tiebreak_orders_equal_scores_by_id() {
    let w = World::new().await;
    let a = w.note(&w.one, "duplicate phrasing").await;
    let b = w.note(&w.one, "duplicate phrasing").await;

    let results = w
        .write
        .query(&w.one, "duplicate phrasing", &Scope::default(), LIMIT)
        .await
        .expect("query answers");

    assert_eq!(results.len(), 2);
    assert!(
        (results[0].score - results[1].score).abs() < f64::EPSILON,
        "both notes share the same title text and should score identically"
    );
    let mut expected = [a.as_uuid(), b.as_uuid()];
    expected.sort();
    assert_eq!(
        [
            results[0].entity.id.as_uuid(),
            results[1].entity.id.as_uuid()
        ],
        expected,
        "equal scores must break the tie by id, ascending"
    );
}

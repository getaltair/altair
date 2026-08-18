//! The intent spine: identity, the counter, conflict, the change sequence, and
//! audience.
//!
//! Most of the schema's *checks this file cannot make, which the write path
//! owes* list is here. The lines this item does not own — block recomputation
//! and identity matching, the graduation half of bulk state, ladder position,
//! and the horizon — belong to 2.2 and 2.4 and are not asserted.

mod common;

use altair_proto::v1;
use common::*;
use uuid::Uuid;

fn note_content(title: &str) -> v1::EntityContent {
    v1::EntityContent {
        title: Some(title.into()),
        specific: Some(v1::entity_content::Specific::Note(
            v1::NoteContent::default(),
        )),
        ..Default::default()
    }
}

// --- identity and replay -------------------------------------------------

#[tokio::test]
async fn a_create_lands_with_a_counter_of_one() {
    let w = World::new().await;
    let id = w.note(&w.one, "a thought").await;
    assert_eq!(w.counter(id).await, 1);
    assert_eq!(w.title(id).await.as_deref(), Some("a thought"));
}

#[tokio::test]
async fn a_replayed_intent_returns_the_first_acknowledgement_and_produces_no_second_effect() {
    let w = World::new().await;
    let intent = Uuid::new_v4();
    let entity = Uuid::new_v4();

    let first = w
        .submit_as(&w.one, intent, create_entity(entity, note_content("once")))
        .await;
    let second = w
        .submit_as(&w.one, intent, create_entity(entity, note_content("once")))
        .await;

    assert_eq!(first, second, "a replay is answered with what was answered");
    assert_eq!(
        w.changes().await.len(),
        1,
        "a replay produced a second change entry, so it produced a second effect"
    );
}

#[tokio::test]
async fn a_replayed_edit_does_not_advance_the_counter_again() {
    let w = World::new().await;
    let id = w.note(&w.one, "first").await;
    let intent = Uuid::new_v4();

    let first = w
        .submit_as(&w.one, intent, edit_entity(id, 1, note_content("second")))
        .await;
    assert_eq!(w.counter(id).await, 2);

    let second = w
        .submit_as(&w.one, intent, edit_entity(id, 1, note_content("third")))
        .await;
    assert_eq!(first, second);
    assert_eq!(w.counter(id).await, 2, "the replay wrote again");
    assert_eq!(w.title(id).await.as_deref(), Some("second"));
}

#[tokio::test]
async fn the_acknowledgement_is_held_for_a_refusal_too() {
    let w = World::new().await;
    let intent = Uuid::new_v4();
    let missing = altaird::store::ids::EntityId::from_uuid(Uuid::new_v4());

    let first = w
        .submit_as(&w.one, intent, edit_entity(missing, 1, note_content("x")))
        .await;
    assert!(is_not_available(&first));

    let second = w
        .submit_as(&w.one, intent, edit_entity(missing, 1, note_content("x")))
        .await;
    assert_eq!(first, second, "a refusal is an answer and is replayed too");
}

#[tokio::test]
async fn an_intent_identity_held_by_another_member_is_nothing() {
    let w = World::new().await;
    let intent = Uuid::new_v4();
    w.submit_as(
        &w.one,
        intent,
        create_entity(Uuid::new_v4(), note_content("mine")),
    )
    .await;

    let theirs = w
        .submit_as(
            &w.two,
            intent,
            create_entity(Uuid::new_v4(), note_content("theirs")),
        )
        .await;
    assert!(
        is_not_available(&theirs),
        "an intent identity is a client's assertion, and the held answer is not \
         another member's to receive"
    );
}

// --- the counter and conflict -------------------------------------------

/// A note member two can also see, which is what makes a concurrent edit
/// possible at all.
async fn shared_note(w: &World, title: &str) -> altaird::store::ids::EntityId {
    w.create(
        &w.one,
        v1::entity_content::Specific::Note(v1::NoteContent::default()),
        v1::EntityContent {
            title: Some(title.into()),
            audience_member_ids: vec![w.two.membership_id().as_bytes().to_vec()],
            ..Default::default()
        },
    )
    .await
}

#[tokio::test]
async fn a_current_base_applies_with_no_conflict() {
    let w = World::new().await;
    let id = shared_note(&w, "start").await;
    let ack = w
        .submit(&w.one, edit_entity(id, 1, note_content("next")))
        .await;
    assert!(applied(&ack).conflict.is_none());
    assert_eq!(applied(&ack).counter, 2);
}

#[tokio::test]
async fn a_stale_base_is_never_a_rejection() {
    let w = World::new().await;
    let id = shared_note(&w, "start").await;
    w.submit(&w.two, edit_entity(id, 1, note_content("theirs")))
        .await;

    // Base 1, three counters behind by the time it lands.
    w.submit(&w.two, edit_entity(id, 2, note_content("theirs again")))
        .await;
    let ack = w
        .submit(&w.one, edit_entity(id, 1, note_content("mine")))
        .await;

    assert!(
        matches!(ack.outcome, Some(v1::acknowledgement::Outcome::Applied(_))),
        "a stale base was rejected, which is the familiar pattern and is wrong here"
    );
    assert_eq!(w.title(id).await.as_deref(), Some("mine"));
}

#[tokio::test]
async fn a_stale_base_touching_a_different_part_applies_without_conflict() {
    let w = World::new().await;
    let id = shared_note(&w, "start").await;

    // Two moves the title.
    w.submit(&w.two, edit_entity(id, 1, note_content("theirs")))
        .await;

    // One, still on base 1, writes a date. Disjoint parts.
    let ack = w
        .submit(
            &w.one,
            edit_entity(
                id,
                1,
                v1::EntityContent {
                    dates: vec![v1::LabelledDate {
                        label: "due".into(),
                        when: Some(timestamp(chrono::Utc::now())),
                        bring_forward: true,
                    }],
                    ..Default::default()
                },
            ),
        )
        .await;

    assert!(
        applied(&ack).conflict.is_none(),
        "disjoint parts are not a conflict, which is the whole point of scoping \
         a write to the part that changed"
    );
    assert!(w.conflicts(id).await.is_empty());
    assert_eq!(w.title(id).await.as_deref(), Some("theirs"));
}

#[tokio::test]
async fn a_stale_base_touching_the_same_part_retains_both_values() {
    let w = World::new().await;
    let id = shared_note(&w, "start").await;
    w.submit(&w.two, edit_entity(id, 1, note_content("theirs")))
        .await;

    let ack = w
        .submit(&w.one, edit_entity(id, 1, note_content("mine")))
        .await;
    let a = applied(&ack);

    let conflict = a
        .conflict
        .as_ref()
        .expect("the same part moved and this is a conflict");
    assert_eq!(conflict.content_fields, vec![1], "the title is field 1");

    // The write still applied. This is not a failure and nothing is sent back.
    assert_eq!(w.title(id).await.as_deref(), Some("mine"));

    let held = w.conflicts(id).await;
    assert_eq!(held.len(), 1);
    assert_eq!(held[0].field.as_deref(), Some("title"));
    assert_eq!(held[0].mine.as_deref(), Some("mine"));
    assert_eq!(held[0].theirs.as_deref(), Some("theirs"));
    assert_eq!(held[0].mine_member, Some(w.one.membership_id()));
    assert_eq!(
        held[0].theirs_member,
        Some(w.two.membership_id()),
        "whose edit each value was is a fact, and it is the fact that changes \
         the decision a person is making"
    );
}

#[tokio::test]
async fn writes_producing_the_same_value_are_not_divergent() {
    let w = World::new().await;
    let id = shared_note(&w, "start").await;
    w.submit(&w.two, edit_entity(id, 1, note_content("agreed")))
        .await;

    // One is stale, touches the same part, and picks the same value. This is
    // the likely case when two people resolve one conflict.
    let ack = w
        .submit(&w.one, edit_entity(id, 1, note_content("agreed")))
        .await;

    assert!(applied(&ack).conflict.is_none());
    assert!(
        w.conflicts(id).await.is_empty(),
        "two people agreeing produced a conflict"
    );
}

// --- the change sequence -------------------------------------------------

#[tokio::test]
async fn every_write_gets_an_entry_and_positions_follow_commit_order() {
    let w = World::new().await;
    let id = w.note(&w.one, "one").await;
    w.submit(&w.one, edit_entity(id, 1, note_content("two")))
        .await;
    w.submit(&w.one, edit_entity(id, 2, note_content("three")))
        .await;

    let changes = w.changes().await;
    assert_eq!(changes.len(), 3);
    assert_eq!(
        changes.iter().map(|c| c.position).collect::<Vec<_>>(),
        vec![1, 2, 3],
        "positions are allocated from one row inside the writing transaction, \
         so they are dense and in commit order"
    );
    assert!(changes.iter().all(|c| c.kind == "entity_written"));
}

#[tokio::test]
async fn a_narrowing_records_who_could_see_it_before() {
    let w = World::new().await;
    let id = shared_note(&w, "shared").await;

    // Clear the audience: private to the author again.
    w.submit(
        &w.one,
        edit_entity(
            id,
            1,
            v1::EntityContent {
                cleared: vec![5],
                ..Default::default()
            },
        ),
    )
    .await;

    let changes = w.changes().await;
    let last = changes.last().expect("a change");
    assert_eq!(
        last.audience_before.as_deref(),
        Some(&[w.two.membership_id()][..]),
        "a narrowing has to reach the member who lost sight of the entity, and \
         the current audience no longer names them"
    );
    assert_eq!(last.audience_after.as_deref(), Some(&[][..]));
}

#[tokio::test]
async fn a_refused_intent_leaves_no_gap_in_the_sequence() {
    let w = World::new().await;
    w.note(&w.one, "one").await;
    let missing = altaird::store::ids::EntityId::from_uuid(Uuid::new_v4());
    w.submit(&w.one, edit_entity(missing, 1, note_content("x")))
        .await;
    w.note(&w.one, "two").await;

    let positions: Vec<i64> = w.changes().await.iter().map(|c| c.position).collect();
    assert_eq!(
        positions,
        vec![1, 2],
        "a rolled-back write must leave no gap, or a client cannot tell a hole \
         it must wait for from one that will never be filled"
    );
}

// --- audience ------------------------------------------------------------

#[tokio::test]
async fn an_entity_this_member_cannot_see_is_refused_exactly_as_a_missing_one() {
    let w = World::new().await;
    let theirs = w.note(&w.two, "private").await;
    let missing = altaird::store::ids::EntityId::from_uuid(Uuid::new_v4());

    let invisible = w
        .submit(&w.one, edit_entity(theirs, 1, note_content("x")))
        .await;
    let absent = w
        .submit(&w.one, edit_entity(missing, 1, note_content("x")))
        .await;

    assert_eq!(
        refused(&invisible),
        refused(&absent),
        "nothing distinguishes an entity that does not exist from one the \
         submitting member cannot see"
    );
    assert!(
        refused(&invisible).detail.is_empty(),
        "the detail is the field nobody thinks of as carrying meaning, which is \
         where a disclosure would hide"
    );
}

#[tokio::test]
async fn every_member_in_an_audience_must_answer_to_a_membership() {
    let w = World::new().await;
    let ack = w
        .submit(
            &w.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    title: Some("shared with nobody".into()),
                    audience_member_ids: vec![Uuid::new_v4().as_bytes().to_vec()],
                    specific: Some(v1::entity_content::Specific::Note(
                        v1::NoteContent::default(),
                    )),
                    ..Default::default()
                },
            ),
        )
        .await;
    assert!(
        is_not_available(&ack),
        "a foreign key cannot reach inside an array, so the write path owes this"
    );
}

#[tokio::test]
async fn an_assignment_must_answer_to_a_membership() {
    let w = World::new().await;
    let ack = w
        .submit(
            &w.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    assigned_member_ids: vec![Uuid::new_v4().as_bytes().to_vec()],
                    specific: Some(v1::entity_content::Specific::Note(
                        v1::NoteContent::default(),
                    )),
                    ..Default::default()
                },
            ),
        )
        .await;
    assert!(is_not_available(&ack));
}

// --- the shape of a well-formed intent -----------------------------------

#[tokio::test]
async fn an_edit_may_not_change_the_type() {
    let w = World::new().await;
    let id = w.note(&w.one, "a note").await;
    let ack = w
        .submit(
            &w.one,
            edit_entity(
                id,
                1,
                v1::EntityContent {
                    specific: Some(v1::entity_content::Specific::Item(
                        v1::ItemContent::default(),
                    )),
                    ..Default::default()
                },
            ),
        )
        .await;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
}

#[tokio::test]
async fn a_field_both_present_and_cleared_is_malformed() {
    let w = World::new().await;
    let id = w.note(&w.one, "a note").await;
    let ack = w
        .submit(
            &w.one,
            edit_entity(
                id,
                1,
                v1::EntityContent {
                    title: Some("set".into()),
                    cleared: vec![1],
                    ..Default::default()
                },
            ),
        )
        .await;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
}

#[tokio::test]
async fn a_cleared_number_naming_no_field_is_malformed() {
    let w = World::new().await;
    let id = w.note(&w.one, "a note").await;
    // 6 is reserved and 100 is `cleared` itself.
    for number in [6u32, 100, 999] {
        let ack = w
            .submit(
                &w.one,
                edit_entity(
                    id,
                    1,
                    v1::EntityContent {
                        cleared: vec![number],
                        ..Default::default()
                    },
                ),
            )
            .await;
        assert_eq!(
            refused(&ack).reason,
            v1::RefusalReason::Malformed as i32,
            "clearing field {number} should not be readable as an instruction"
        );
    }
}

#[tokio::test]
async fn an_identifier_of_the_wrong_length_is_malformed() {
    let w = World::new().await;
    let ack = w
        .submit(
            &w.one,
            v1::intent::Action::Create(v1::Create {
                subject: Some(v1::create::Subject::Entity(v1::CreateEntity {
                    entity_id: vec![1, 2, 3],
                    created_at: None,
                    capture_method: "test".into(),
                    content: Some(note_content("short")),
                })),
            }),
        )
        .await;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
}

#[tokio::test]
async fn content_with_no_type_is_malformed() {
    let w = World::new().await;
    let ack = w
        .submit(
            &w.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    title: Some("typeless".into()),
                    ..Default::default()
                },
            ),
        )
        .await;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
}

#[tokio::test]
async fn the_types_with_no_table_are_refused_for_a_reason_that_expires() {
    use v1::entity_content::Specific;
    let w = World::new().await;
    let cases = [
        Specific::Routine(v1::RoutineContent::default()),
        Specific::FocusSession(v1::FocusSessionContent::default()),
        Specific::CheckIn(v1::CheckInContent::default()),
        Specific::File(v1::FileContent::default()),
    ];
    for specific in cases {
        let ack = w
            .submit(
                &w.one,
                create_entity(
                    Uuid::new_v4(),
                    v1::EntityContent {
                        specific: Some(specific),
                        ..Default::default()
                    },
                ),
            )
            .await;
        assert_eq!(
            refused(&ack).reason,
            v1::RefusalReason::Malformed as i32,
            "refused, and with a detail that says why rather than nothing"
        );
        assert!(!refused(&ack).detail.is_empty());
    }
}

#[tokio::test]
async fn every_type_the_store_has_a_table_for_creates_a_row_with_defaults() {
    use v1::entity_content::Specific;
    let w = World::new().await;
    let cases = [
        (
            Specific::Campaign(v1::CampaignContent::default()),
            "campaign",
        ),
        (Specific::Arc(v1::ArcContent::default()), "arc"),
        (Specific::Quest(v1::QuestContent::default()), "quest"),
        (Specific::Item(v1::ItemContent::default()), "item"),
        (
            Specific::Location(v1::LocationContent::default()),
            "location",
        ),
        (
            Specific::Category(v1::CategoryContent::default()),
            "category",
        ),
    ];
    for (specific, table) in cases {
        let id = w
            .create(&w.one, specific, v1::EntityContent::default())
            .await;
        assert_eq!(
            w.rows_for(table, "entity_id", id).await,
            1,
            "{table} has no row, so the entity is half-formed"
        );
    }
    // A note and a shopping list have no table: a body is its blocks in order
    // and there is nothing else.
    let note = w.note(&w.one, "no table").await;
    assert_eq!(w.counter(note).await, 1);
}

// --- containers ----------------------------------------------------------

async fn a_category(w: &World) -> altaird::store::ids::EntityId {
    w.create(
        &w.one,
        v1::entity_content::Specific::Category(v1::CategoryContent::default()),
        v1::EntityContent {
            title: Some("work".into()),
            ..Default::default()
        },
    )
    .await
}

#[tokio::test]
async fn entering_a_container_places_the_entity_at_the_end() {
    let w = World::new().await;
    let category = a_category(&w).await;

    let mut placed = Vec::new();
    for n in 0..3 {
        let id = w
            .create(
                &w.one,
                v1::entity_content::Specific::Note(v1::NoteContent::default()),
                v1::EntityContent {
                    title: Some(format!("note {n}")),
                    category_id: Some(category.as_uuid().as_bytes().to_vec()),
                    ..Default::default()
                },
            )
            .await;
        placed.push(id);
    }

    for (n, id) in placed.iter().enumerate() {
        assert_eq!(
            w.category_position(*id).await,
            Some(n as i32),
            "position is assigned by the instance, which appends"
        );
    }
}

#[tokio::test]
async fn leaving_a_container_forgets_the_position() {
    let w = World::new().await;
    let category = a_category(&w).await;
    let id = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                category_id: Some(category.as_uuid().as_bytes().to_vec()),
                ..Default::default()
            },
        )
        .await;
    assert_eq!(w.category_position(id).await, Some(0));

    w.submit(
        &w.one,
        edit_entity(
            id,
            1,
            v1::EntityContent {
                cleared: vec![3],
                ..Default::default()
            },
        ),
    )
    .await;
    assert_eq!(w.category_position(id).await, None);
}

#[tokio::test]
async fn an_explicit_position_is_refused_while_nothing_reorders() {
    let w = World::new().await;
    let category = a_category(&w).await;
    let ack = w
        .submit(
            &w.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    category_id: Some(category.as_uuid().as_bytes().to_vec()),
                    category_position: Some(0),
                    specific: Some(v1::entity_content::Specific::Note(
                        v1::NoteContent::default(),
                    )),
                    ..Default::default()
                },
            ),
        )
        .await;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
}

#[tokio::test]
async fn a_container_that_is_not_a_category_is_nothing() {
    let w = World::new().await;
    let not_a_category = w.note(&w.one, "not a container").await;
    let ack = w
        .submit(
            &w.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    category_id: Some(not_a_category.as_uuid().as_bytes().to_vec()),
                    specific: Some(v1::entity_content::Specific::Note(
                        v1::NoteContent::default(),
                    )),
                    ..Default::default()
                },
            ),
        )
        .await;
    assert!(is_not_available(&ack));
}

#[tokio::test]
async fn a_categorys_creation_default_acts_once_and_an_explicit_audience_outranks_it() {
    let w = World::new().await;
    let category = w
        .create(
            &w.one,
            v1::entity_content::Specific::Category(v1::CategoryContent::default()),
            v1::EntityContent {
                title: Some("shared".into()),
                ..Default::default()
            },
        )
        .await;
    sqlx::query("UPDATE category SET creation_default_audience = $2 WHERE entity_id = $1")
        .bind(category.as_uuid())
        .bind(vec![w.two.membership_id()])
        .execute(&w.db.pool)
        .await
        .expect("default");

    let inherited = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                category_id: Some(category.as_uuid().as_bytes().to_vec()),
                ..Default::default()
            },
        )
        .await;
    // Member two can act on it, which is the only observation that matters.
    let ack = w
        .submit(&w.two, edit_entity(inherited, 1, note_content("theirs")))
        .await;
    assert!(matches!(
        ack.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));

    let stated = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                category_id: Some(category.as_uuid().as_bytes().to_vec()),
                cleared: vec![5],
                ..Default::default()
            },
        )
        .await;
    let ack = w
        .submit(&w.two, edit_entity(stated, 1, note_content("theirs")))
        .await;
    assert!(
        is_not_available(&ack),
        "an explicit audience outranks the category's creation default"
    );
}

// --- the batch -----------------------------------------------------------

#[tokio::test]
async fn a_batch_answers_every_intent_and_a_refusal_does_not_stop_the_next() {
    let w = World::new().await;
    let missing = Uuid::new_v4();
    let first = Uuid::new_v4();
    let last = Uuid::new_v4();

    let acks = w
        .write
        .submit(
            &w.one,
            &[
                v1::Intent {
                    intent_id: Uuid::new_v4().as_bytes().to_vec(),
                    action: Some(create_entity(first, note_content("first"))),
                },
                v1::Intent {
                    intent_id: Uuid::new_v4().as_bytes().to_vec(),
                    action: Some(edit_entity(
                        altaird::store::ids::EntityId::from_uuid(missing),
                        1,
                        note_content("nowhere"),
                    )),
                },
                v1::Intent {
                    intent_id: Uuid::new_v4().as_bytes().to_vec(),
                    action: Some(create_entity(last, note_content("last"))),
                },
            ],
        )
        .await
        .expect("the store was reachable");

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
        "the intent after a refusal was not applied, so the batch was all or nothing"
    );

    assert_eq!(
        w.title(altaird::store::ids::EntityId::from_uuid(last))
            .await
            .as_deref(),
        Some("last")
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_writes_produce_dense_positions_and_a_total_container_order() {
    // The claim `changes` makes is that positions are allocated inside the
    // writing transaction, so commit order and position order are the same and
    // no position is skipped or shared. It also claims that holding the
    // sequence row first is what makes `max + 1` safe for a container position
    // without a second lock. Neither is worth documenting without watching them
    // hold under real concurrency.
    let w = std::sync::Arc::new(World::new().await);
    let category = a_category(&w).await;
    let before = w.changes().await.len() as i64;

    let mut running = Vec::new();
    for n in 0..16 {
        let w = w.clone();
        running.push(tokio::spawn(async move {
            w.create(
                &w.one,
                v1::entity_content::Specific::Note(v1::NoteContent::default()),
                v1::EntityContent {
                    title: Some(format!("note {n}")),
                    category_id: Some(category.as_uuid().as_bytes().to_vec()),
                    ..Default::default()
                },
            )
            .await
        }));
    }
    let mut placed = Vec::new();
    for handle in running {
        placed.push(handle.await.expect("a create"));
    }

    let positions: Vec<i64> = w.changes().await.iter().map(|c| c.position).collect();
    assert_eq!(
        positions,
        (1..=before + 16).collect::<Vec<_>>(),
        "positions must be dense and unique, or a poller cannot tell a hole it \
         must wait for from one that will never be filled"
    );

    let mut order: Vec<i32> = Vec::new();
    for id in &placed {
        order.push(w.category_position(*id).await.expect("a position"));
    }
    order.sort_unstable();
    assert_eq!(
        order,
        (0..16).collect::<Vec<_>>(),
        "two entities shared a position, so the container's order is not total"
    );
}

// --- what a cross-model review found -------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn two_submissions_of_one_intent_at_once_both_get_the_acknowledgement() {
    // The promise is that resubmitting an intent returns the acknowledgement
    // the instance already issued, unchanged, rather than producing a second
    // effect. Sequential replay is the easy half. The half that was wrong: two
    // submissions of the same identity in flight together both find it absent,
    // both work, and the loser used to answer with a store fault — the instance
    // saying it is down while holding the answer.
    //
    // A retry after a stalled or dropped submission is exactly this shape, and
    // it is what the outbox does by design.
    let w = std::sync::Arc::new(World::new().await);
    let id = w.note(&w.one, "first").await;
    let intent = Uuid::new_v4();

    let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(2));
    let mut running = Vec::new();
    for _ in 0..2 {
        let w = w.clone();
        let barrier = barrier.clone();
        running.push(tokio::spawn(async move {
            barrier.wait().await;
            w.submit_as(&w.one, intent, edit_entity(id, 1, note_content("second")))
                .await
        }));
    }
    let mut acks = Vec::new();
    for handle in running {
        acks.push(handle.await.expect("neither call is a fault"));
    }

    assert_eq!(
        acks[0], acks[1],
        "both callers are owed the acknowledgement the instance issued"
    );
    assert_eq!(w.counter(id).await, 2, "the effect happened once");
    assert_eq!(
        w.changes().await.len(),
        2,
        "one create and one edit, not one create and two edits"
    );
}

#[tokio::test]
async fn holding_an_acknowledgement_a_second_time_says_so_rather_than_failing() {
    // The mechanism the test above exercises end to end, asserted directly,
    // because the refusal path holds its acknowledgement in a second
    // transaction that never takes the sequence lock — so its race is real and
    // cannot be forced from outside the instance. What can be asserted is what
    // the spine relies on: the second hold reports that it lost, instead of
    // raising a primary-key violation that reaches a caller as the instance
    // being down while it is holding their answer.
    use altaird::store::begin_write;
    use altaird::write::intent;
    use altaird::write::outcome::Outcome;

    let w = World::new().await;
    let id = Uuid::new_v4();
    let member = w.one.membership_id();
    let at = chrono::Utc::now();
    let outcome = Outcome::not_available();

    let mut tx = begin_write(&w.db.pool).await.expect("a transaction");
    assert!(
        intent::hold(&mut tx, id, member, at, &outcome)
            .await
            .expect("held")
    );
    tx.commit().await.expect("committed");

    let mut tx = begin_write(&w.db.pool).await.expect("a transaction");
    assert!(
        !intent::hold(&mut tx, id, member, at, &outcome)
            .await
            .expect("a second hold is an answer, not a store error"),
        "the loser has to be told it lost, or it cannot return the \
         acknowledgement the instance already issued"
    );
    tx.rollback().await.expect("rolled back");
}

#[tokio::test]
async fn a_create_naming_an_invisible_identity_is_refused_and_never_faults() {
    // The lookup cannot tell "not there" from "there and not yours" — that is
    // what the audience predicate is for — so the insert has to, without ever
    // reaching the error channel. A store fault where an unused identifier
    // would have succeeded is an oracle for whether something is there,
    // arriving through the one channel nobody thinks of as an answer.
    let w = World::new().await;
    let theirs = w.note(&w.two, "private").await;

    let taken = w
        .write
        .submit(
            &w.one,
            &[v1::Intent {
                intent_id: Uuid::new_v4().as_bytes().to_vec(),
                action: Some(create_entity(theirs.as_uuid(), note_content("mine now"))),
            }],
        )
        .await
        .expect("an identifier somebody else holds is an answer, not a fault");

    assert!(is_not_available(&taken[0]));
    assert_eq!(
        w.title(theirs).await.as_deref(),
        Some("private"),
        "and it did not overwrite what was there"
    );
}

#[tokio::test]
async fn a_timestamp_that_cannot_be_read_is_refused_wherever_it_arrives() {
    // Two callers parse the same wire shape, and they used to disagree: a
    // labelled date refused a garbled instant while a create's `created_at`
    // silently substituted the instance's clock. A client with a broken clock
    // conversion was then told its capture had been accepted carrying a time it
    // never sent, which is the quietest way to be wrong about when something
    // happened.
    let w = World::new().await;

    let unreadable = [
        // Nanos that are not a fraction of a second.
        altair_proto::prost_types::Timestamp {
            seconds: 1_780_000_000,
            nanos: -1,
        },
        // Seconds naming no representable instant.
        altair_proto::prost_types::Timestamp {
            seconds: i64::MAX,
            nanos: 0,
        },
    ];

    for stamp in unreadable {
        let ack = w
            .submit(
                &w.one,
                v1::intent::Action::Create(v1::Create {
                    subject: Some(v1::create::Subject::Entity(v1::CreateEntity {
                        entity_id: Uuid::new_v4().as_bytes().to_vec(),
                        created_at: Some(stamp),
                        capture_method: "test".into(),
                        content: Some(note_content("when?")),
                    })),
                }),
            )
            .await;
        assert_eq!(
            refused(&ack).reason,
            v1::RefusalReason::Malformed as i32,
            "{stamp:?} was absorbed rather than refused"
        );

        let id = w.note(&w.one, "a note").await;
        let ack = w
            .submit(
                &w.one,
                edit_entity(
                    id,
                    1,
                    v1::EntityContent {
                        dates: vec![v1::LabelledDate {
                            label: "due".into(),
                            when: Some(stamp),
                            bring_forward: false,
                        }],
                        ..Default::default()
                    },
                ),
            )
            .await;
        assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
    }

    // An absent created_at is ordinary, not malformed: it means the instance's
    // own clock, which is what a capture with nothing but an identity gets.
    let id = w.note(&w.one, "no stated time").await;
    assert_eq!(w.counter(id).await, 1);
}

#[tokio::test]
async fn a_base_counter_this_instance_could_not_have_issued_is_refused() {
    // Clamping it was worse than it looks. A base larger than any counter the
    // store can hold reads as current against every entity, so the write skips
    // conflict detection entirely — silently, and only for the input nobody
    // should be sending.
    let w = World::new().await;
    let id = shared_note(&w, "start").await;
    w.submit(&w.two, edit_entity(id, 1, note_content("theirs")))
        .await;

    let ack = w
        .submit(&w.one, edit_entity(id, u64::MAX, note_content("mine")))
        .await;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
    assert_eq!(
        w.title(id).await.as_deref(),
        Some("theirs"),
        "and it did not apply while skipping the conflict it should have found"
    );
}

//! Removal, grouped removal, restoration, erasure, and recreation.
//!
//! **The intent is to retain.** Anything that becomes permanent does so
//! visibly, predictably, and never as a side effect of something else, and
//! these are the assertions that make that a property of the instance rather
//! than of a document.

mod common;

use altair_proto::v1;
use altaird::store::ids::EntityId;
use common::*;
use sqlx::Row;
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

// --- removal and restoration --------------------------------------------

#[tokio::test]
async fn removal_is_a_change_carrying_a_lifecycle_and_not_an_absence() {
    let w = World::new().await;
    let id = w.note(&w.one, "regrettable").await;
    w.submit(&w.one, remove(&[id], &[])).await;

    assert_eq!(w.lifecycle(id).await, "deleted");
    let changes = w.changes().await;
    assert_eq!(
        changes.last().expect("a change").kind,
        "entity_written",
        "a removal is a change carrying a lifecycle state, not a disappearance"
    );
}

#[tokio::test]
async fn a_deletion_of_several_is_remembered_as_one_act() {
    let w = World::new().await;
    let a = w.note(&w.one, "a").await;
    let b = w.note(&w.one, "b").await;
    let c = w.note(&w.one, "c").await;

    w.submit(&w.one, remove(&[a, b], &[])).await;
    w.submit(&w.one, remove(&[c], &[])).await;

    // Restoring any of them can bring back the rest of that act, and only that
    // act: c was removed separately and is not dragged back.
    let ack = w.submit(&w.one, restore(a, true)).await;
    assert_eq!(applied(&ack).entity_ids.len(), 2);
    assert_eq!(w.lifecycle(a).await, "active");
    assert_eq!(w.lifecycle(b).await, "active");
    assert_eq!(
        w.lifecycle(c).await,
        "deleted",
        "restoring a container must not drag back entities deleted separately"
    );
}

#[tokio::test]
async fn restoring_one_on_its_own_leaves_the_rest_of_the_act_alone() {
    let w = World::new().await;
    let a = w.note(&w.one, "a").await;
    let b = w.note(&w.one, "b").await;
    w.submit(&w.one, remove(&[a, b], &[])).await;

    w.submit(&w.one, restore(a, false)).await;
    assert_eq!(w.lifecycle(a).await, "active");
    assert_eq!(w.lifecycle(b).await, "deleted");
}

#[tokio::test]
async fn a_removal_that_cannot_land_has_already_happened() {
    let w = World::new().await;
    let theirs = w.note(&w.two, "not mine").await;
    let missing = EntityId::from_uuid(Uuid::new_v4());
    let mine = w.note(&w.one, "mine").await;
    w.submit(&w.one, remove(&[mine], &[])).await;

    // An identifier that is invisible, one that does not exist, and one already
    // removed all converge on the end state the person asked for.
    let ack = w
        .submit(&w.one, remove(&[theirs, missing, mine], &[]))
        .await;
    assert!(
        matches!(ack.outcome, Some(v1::acknowledgement::Outcome::Applied(_))),
        "reporting this as a failure would say only that the thing they wanted \
         is the thing that occurred"
    );
    assert!(applied(&ack).entity_ids.is_empty());
    assert_eq!(w.lifecycle(theirs).await, "active");
}

#[tokio::test]
async fn restoring_places_the_entity_at_the_end_of_its_container_again() {
    let w = World::new().await;
    let category = w
        .create(
            &w.one,
            v1::entity_content::Specific::Category(v1::CategoryContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    let in_category = |title: &str| v1::EntityContent {
        title: Some(title.into()),
        category_id: Some(category.as_uuid().as_bytes().to_vec()),
        ..Default::default()
    };

    let first = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            in_category("first"),
        )
        .await;
    w.create(
        &w.one,
        v1::entity_content::Specific::Note(v1::NoteContent::default()),
        in_category("second"),
    )
    .await;

    assert_eq!(w.category_position(first).await, Some(0));
    w.submit(&w.one, remove(&[first], &[])).await;
    w.submit(&w.one, restore(first, false)).await;
    assert_eq!(
        w.category_position(first).await,
        Some(2),
        "entering a container places the entity at the end, and that holds for \
         a restoration exactly as it does for a create or a move"
    );
}

#[tokio::test]
async fn restoring_something_active_is_the_state_that_is_already_true() {
    let w = World::new().await;
    let id = w.note(&w.one, "here").await;
    let ack = w.submit(&w.one, restore(id, false)).await;
    assert!(matches!(
        ack.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));
    assert_eq!(w.counter(id).await, 1, "nothing was written");
}

#[tokio::test]
async fn restoring_something_this_member_cannot_see_is_nothing() {
    let w = World::new().await;
    let theirs = w.note(&w.two, "private").await;
    w.submit(&w.two, remove(&[theirs], &[])).await;
    let ack = w.submit(&w.one, restore(theirs, true)).await;
    assert!(is_not_available(&ack));
}

// --- erasure -------------------------------------------------------------

#[tokio::test]
async fn erasure_strips_content_and_leaves_a_tombstone() {
    let w = World::new().await;
    let id = w.note(&w.one, "sensitive").await;
    w.submit(&w.one, erase(&[id])).await;

    assert_eq!(w.lifecycle(id).await, "erased");
    assert_eq!(w.title(id).await, None, "the title is content");

    let row =
        sqlx::query("SELECT search_text, capture_method, deleted_at FROM entity WHERE id = $1")
            .bind(id.as_uuid())
            .fetch_one(&w.db.pool)
            .await
            .expect("the row survives as a tombstone");
    assert_eq!(row.try_get::<String, _>("search_text").unwrap(), "");
    assert_eq!(row.try_get::<String, _>("capture_method").unwrap(), "");
    assert!(
        row.try_get::<Option<chrono::DateTime<chrono::Utc>>, _>("deleted_at")
            .unwrap()
            .is_none()
    );

    let changes = w.changes().await;
    assert_eq!(changes.last().expect("a change").kind, "entity_gone");
}

#[tokio::test]
async fn erasure_removes_every_dependent_row_explicitly() {
    let w = World::new().await;
    let id = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("everything".into()),
                dates: vec![v1::LabelledDate {
                    label: "due".into(),
                    when: Some(timestamp(chrono::Utc::now())),
                    bring_forward: true,
                }],
                assigned_member_ids: vec![w.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;

    // The rows Wave 2.1 does not yet write, put there directly, because the
    // schema's cascades hang off a delete of the entity row and erasure never
    // performs one.
    let block = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO block (id, entity_id, body_type, position, text) \
         VALUES ($1, $2, 'note', 0, 'a paragraph')",
    )
    .bind(block)
    .bind(id.as_uuid())
    .execute(&w.db.pool)
    .await
    .expect("block");

    sqlx::query("INSERT INTO entity_property_value (entity_id, name, value) VALUES ($1, 'k', 'v')")
        .bind(id.as_uuid())
        .execute(&w.db.pool)
        .await
        .expect("property");

    sqlx::query(
        "INSERT INTO entity_version (id, entity_id, captured_at, body) \
         VALUES ($1, $2, now(), 'old')",
    )
    .bind(Uuid::new_v4())
    .bind(id.as_uuid())
    .execute(&w.db.pool)
    .await
    .expect("version");

    sqlx::query(
        "INSERT INTO derived_text (entity_id, content, producer, produced_at) \
         VALUES ($1, 'extracted', 'test', now())",
    )
    .bind(id.as_uuid())
    .execute(&w.db.pool)
    .await
    .expect("derived text");

    let vector = format!("[{}]", vec!["0"; 1024].join(","));
    sqlx::query(
        "INSERT INTO embedding (id, entity_id, model, produced_at, vector) \
         VALUES ($1, $2, 'test', now(), $3::vector)",
    )
    .bind(Uuid::new_v4())
    .bind(id.as_uuid())
    .bind(&vector)
    .execute(&w.db.pool)
    .await
    .expect("embedding");

    sqlx::query(
        "INSERT INTO derivation_queue (entity_id, kind, enqueued_at) \
         VALUES ($1, 'embedding', now())",
    )
    .bind(id.as_uuid())
    .execute(&w.db.pool)
    .await
    .expect("queue");

    sqlx::query(
        "INSERT INTO conflict (id, entity_id, field_name, mine_value, theirs_value, detected_at) \
         VALUES ($1, $2, 'title', 'a', 'b', now())",
    )
    .bind(Uuid::new_v4())
    .bind(id.as_uuid())
    .execute(&w.db.pool)
    .await
    .expect("conflict");

    w.submit(&w.one, erase(&[id])).await;

    for (table, column) in [
        ("block", "entity_id"),
        ("entity_date", "entity_id"),
        ("entity_assignment", "entity_id"),
        ("entity_property_value", "entity_id"),
        ("entity_version", "entity_id"),
        ("derived_text", "entity_id"),
        ("embedding", "entity_id"),
        ("derivation_queue", "entity_id"),
        ("conflict", "entity_id"),
        ("entity_part_counter", "entity_id"),
    ] {
        assert_eq!(
            w.rows_for(table, column, id).await,
            0,
            "{table} still holds a row for an erased entity, and its cascade \
             never fires because erasure does not delete the entity row"
        );
    }
}

#[tokio::test]
async fn erasing_an_item_removes_its_event_records() {
    let w = World::new().await;
    let id = w
        .create(
            &w.one,
            v1::entity_content::Specific::Item(v1::ItemContent::default()),
            v1::EntityContent {
                title: Some("flour".into()),
                ..Default::default()
            },
        )
        .await;
    sqlx::query(
        "INSERT INTO event_record (id, item_id, kind, amount, occurred_at, recorded_at) \
         VALUES ($1, $2, 'purchase', 1, now(), now())",
    )
    .bind(Uuid::new_v4())
    .bind(id.as_uuid())
    .execute(&w.db.pool)
    .await
    .expect("event record");

    w.submit(&w.one, erase(&[id])).await;
    assert_eq!(w.rows_for("event_record", "item_id", id).await, 0);
    assert_eq!(
        w.rows_for("item", "entity_id", id).await,
        0,
        "the side table is content too"
    );
}

#[tokio::test]
async fn an_erased_category_holds_nothing() {
    let w = World::new().await;
    let category = w
        .create(
            &w.one,
            v1::entity_content::Specific::Category(v1::CategoryContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    let inside = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("filed".into()),
                category_id: Some(category.as_uuid().as_bytes().to_vec()),
                ..Default::default()
            },
        )
        .await;

    w.submit(&w.one, erase(&[category])).await;
    assert_eq!(
        w.category_position(inside).await,
        None,
        "an entity whose category is gone becomes uncategorised, which is a \
         valid state requiring no repair"
    );
    assert_eq!(
        w.counter(inside).await,
        2,
        "leaving the container is an accepted write, so a client has to learn it"
    );
}

#[tokio::test]
async fn erasure_converges_like_a_removal() {
    let w = World::new().await;
    let theirs = w.note(&w.two, "not mine").await;
    let ack = w.submit(&w.one, erase(&[theirs])).await;
    assert!(matches!(
        ack.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));
    assert!(applied(&ack).entity_ids.is_empty());
    assert_eq!(w.lifecycle(theirs).await, "active");
}

// --- recreation ----------------------------------------------------------

#[tokio::test]
async fn an_edit_arriving_for_an_erased_entity_is_a_recreation() {
    let w = World::new().await;
    let id = w.note(&w.one, "original").await;
    w.submit(&w.one, erase(&[id])).await;

    let ack = w
        .submit(&w.one, edit_entity(id, 1, note_content("offline work")))
        .await;
    let r = recreated(&ack);
    assert_eq!(r.original_entity_id, id.as_uuid().as_bytes().to_vec());
    assert_ne!(r.new_entity_id, r.original_entity_id, "the identity is new");
    assert_eq!(r.counter, 1);

    let new = EntityId::from_uuid(Uuid::from_bytes(
        r.new_entity_id.clone().try_into().expect("16 bytes"),
    ));
    assert_eq!(w.title(new).await.as_deref(), Some("offline work"));
    assert_eq!(
        w.lifecycle(id).await,
        "erased",
        "the erasure stands, and so does the person's work"
    );
}

#[tokio::test]
async fn a_recreation_is_private_to_the_author_whatever_the_audience_was() {
    let w = World::new().await;
    let id = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("was shared".into()),
                audience_member_ids: vec![w.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;
    w.submit(&w.one, erase(&[id])).await;

    // The offline device still believes the entity was shared and says so.
    let ack = w
        .submit(
            &w.one,
            edit_entity(
                id,
                1,
                v1::EntityContent {
                    title: Some("still working on it".into()),
                    audience_member_ids: vec![w.two.membership_id().as_bytes().to_vec()],
                    ..Default::default()
                },
            ),
        )
        .await;
    let new = EntityId::from_uuid(Uuid::from_bytes(
        recreated(&ack)
            .new_entity_id
            .clone()
            .try_into()
            .expect("16 bytes"),
    ));

    let seen_by_two = w
        .submit(&w.two, edit_entity(new, 1, note_content("mine now")))
        .await;
    assert!(
        is_not_available(&seen_by_two),
        "a device that was offline recreating it with a household audience \
         would re-expose the thing erasure exists to remove"
    );
}

#[tokio::test]
async fn a_create_naming_an_erased_identity_is_nothing() {
    let w = World::new().await;
    let id = w.note(&w.one, "gone").await;
    w.submit(&w.one, erase(&[id])).await;

    let ack = w
        .submit(&w.one, create_entity(id.as_uuid(), note_content("reused")))
        .await;
    assert!(
        is_not_available(&ack),
        "a tombstone is not a free identity: anything that once pointed at it \
         would silently reattach"
    );
}

#[tokio::test]
async fn the_same_entity_submitted_twice_produces_one_entity() {
    let w = World::new().await;
    let id = Uuid::new_v4();
    w.submit(&w.one, create_entity(id, note_content("once")))
        .await;
    let again = w
        .submit(&w.one, create_entity(id, note_content("twice")))
        .await;

    assert!(matches!(
        again.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));
    assert_eq!(
        w.title(EntityId::from_uuid(id)).await.as_deref(),
        Some("once"),
        "the second create is the same entity, not a second one and not an edit"
    );
    assert_eq!(w.changes().await.len(), 1);
}

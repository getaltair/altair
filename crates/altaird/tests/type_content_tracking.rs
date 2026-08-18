//! Tracking's type content: an item, a location, and what a template does not
//! own.
//!
//! **LANE: Tracking.** The spine's own suite is `type_content.rs` and proves a
//! type-specific part reaches every mechanism the shared model reaches. This one
//! proves the three things about this domain that a plausible implementation
//! gets wrong:
//!
//! - **An amount and the time it was asserted move in one statement.** The
//!   table refuses the state in between, so a sequence of two writes fails
//!   whichever order it is in.
//! - **The assertion time is the amount's own.** An unrelated edit must not
//!   advance it, or a count from March looks fresh — which the PRD says is worse
//!   than showing nothing.
//! - **A loop in nested locations is refused where the schema cannot see it.**
//!   Self-reference passes for free and proves nothing; two hops and three are
//!   the cases that matter.
//!
//! And two the scope decides rather than this lane: property values survive a
//! template being detached, keyed by the names they had, and a shopping list's
//! entries are still refused with the reason they are deferred.

mod common;

use altair_proto::v1;
use altaird::store::begin_write;
use altaird::store::entity::EntityType;
use altaird::store::ids::{EntityId, MemberId};
use altaird::write::entity::Ctx;
use altaird::write::specific::{self, Detachment};
use chrono::Utc;
use common::*;
use sqlx::Row;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Building the two messages
// ---------------------------------------------------------------------------

fn item(c: v1::ItemContent) -> v1::entity_content::Specific {
    v1::entity_content::Specific::Item(c)
}

fn location(c: v1::LocationContent) -> v1::entity_content::Specific {
    v1::entity_content::Specific::Location(c)
}

fn decimal(units: i64, nanos: i32) -> v1::Decimal {
    v1::Decimal { units, nanos }
}

fn property(name: &str, value: &str) -> v1::PropertyValue {
    v1::PropertyValue {
        name: name.into(),
        value: value.into(),
    }
}

fn id_bytes(id: EntityId) -> Vec<u8> {
    id.as_uuid().as_bytes().to_vec()
}

/// An edit carrying one type message and nothing else.
fn edit_specific(
    id: EntityId,
    base: i64,
    specific: v1::entity_content::Specific,
) -> v1::intent::Action {
    edit_entity(
        id,
        base as u64,
        v1::EntityContent {
            specific: Some(specific),
            ..Default::default()
        },
    )
}

// ---------------------------------------------------------------------------
// Reading the store back
// ---------------------------------------------------------------------------

/// One item row, as text, so a numeric keeps the precision it was written with.
struct ItemRow {
    amount: Option<String>,
    unit: Option<String>,
    asserted_at: Option<chrono::DateTime<chrono::Utc>>,
    location: Option<Uuid>,
    template: Option<Uuid>,
}

async fn item_row(world: &World, id: EntityId) -> ItemRow {
    let r = sqlx::query(
        "SELECT asserted_amount::text AS amount, unit, asserted_at, location_id, template_id \
         FROM item WHERE entity_id = $1",
    )
    .bind(id.as_uuid())
    .fetch_one(&world.db.pool)
    .await
    .expect("item row");
    ItemRow {
        amount: r.try_get("amount").unwrap(),
        unit: r.try_get("unit").unwrap(),
        asserted_at: r.try_get("asserted_at").unwrap(),
        location: r.try_get("location_id").unwrap(),
        template: r.try_get("template_id").unwrap(),
    }
}

async fn location_row(world: &World, id: EntityId) -> (Option<Uuid>, Option<Uuid>) {
    let r = sqlx::query(
        "SELECT parent_location_id AS p, template_id AS t FROM location WHERE entity_id = $1",
    )
    .bind(id.as_uuid())
    .fetch_one(&world.db.pool)
    .await
    .expect("location row");
    (r.try_get("p").unwrap(), r.try_get("t").unwrap())
}

/// Every property value on an entity, by name.
async fn properties_of(world: &World, id: EntityId) -> Vec<(String, Option<String>)> {
    sqlx::query("SELECT name, value FROM entity_property_value WHERE entity_id = $1 ORDER BY name")
        .bind(id.as_uuid())
        .fetch_all(&world.db.pool)
        .await
        .expect("property values")
        .iter()
        .map(|r| (r.try_get("name").unwrap(), r.try_get("value").unwrap()))
        .collect()
}

async fn search_text(world: &World, id: EntityId) -> String {
    sqlx::query("SELECT search_text AS s FROM entity WHERE id = $1")
        .bind(id.as_uuid())
        .fetch_one(&world.db.pool)
        .await
        .expect("entity")
        .try_get("s")
        .expect("search_text")
}

/// The counters `entity_part_counter` recorded, by part name.
async fn moved_parts(world: &World, id: EntityId) -> Vec<(String, i64)> {
    sqlx::query(
        "SELECT field_name, counter FROM entity_part_counter \
         WHERE entity_id = $1 AND field_name IS NOT NULL ORDER BY field_name",
    )
    .bind(id.as_uuid())
    .fetch_all(&world.db.pool)
    .await
    .expect("part counters")
    .iter()
    .map(|r| {
        (
            r.try_get("field_name").unwrap(),
            r.try_get("counter").unwrap(),
        )
    })
    .collect()
}

/// A template with the property names it contributes.
///
/// Declared by hand: nothing on the wire creates a template yet, and a template
/// contributes names rather than content, so what matters below is only that the
/// row exists to be followed and detached.
async fn declare_template(world: &World, name: &str, properties: &[(&str, &str)]) -> Uuid {
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO template (id, name, created_at) VALUES ($1, $2, now())")
        .bind(id)
        .bind(name)
        .execute(&world.db.pool)
        .await
        .expect("template");
    for (ordinal, (property, kind)) in properties.iter().enumerate() {
        sqlx::query(
            "INSERT INTO template_property (template_id, name, kind, ordinal) \
             VALUES ($1, $2, $3::property_kind, $4)",
        )
        .bind(id)
        .bind(property)
        .bind(kind)
        .bind(ordinal as i32)
        .execute(&world.db.pool)
        .await
        .expect("template property");
    }
    id
}

async fn a_location(world: &World, member: &altaird::auth::Member, title: &str) -> EntityId {
    world
        .create(
            member,
            location(v1::LocationContent::default()),
            v1::EntityContent {
                title: Some(title.into()),
                ..Default::default()
            },
        )
        .await
}

// ---------------------------------------------------------------------------
// An item's fields, there and back
// ---------------------------------------------------------------------------

#[tokio::test]
async fn every_item_field_reaches_the_store_on_creation() {
    let world = World::new().await;
    let shelf = a_location(&world, &world.one, "the shelf").await;
    let template = declare_template(&world, "filament", &[("diameter", "text")]).await;

    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(3, 250_000_000)),
                unit: Some("rolls".into()),
                location_id: Some(id_bytes(shelf)),
                template_id: Some(template.as_bytes().to_vec()),
                property_values: vec![property("diameter", "1.75mm"), property("material", "PLA")],
                ..Default::default()
            }),
            v1::EntityContent {
                title: Some("black filament".into()),
                ..Default::default()
            },
        )
        .await;

    let row = item_row(&world, id).await;
    assert_eq!(row.amount.as_deref(), Some("3.250000000"));
    assert_eq!(row.unit.as_deref(), Some("rolls"));
    assert_eq!(row.location, Some(shelf.as_uuid()));
    assert_eq!(row.template, Some(template));
    assert!(row.asserted_at.is_some());
    assert_eq!(
        properties_of(&world, id).await,
        vec![
            ("diameter".to_owned(), Some("1.75mm".to_owned())),
            ("material".to_owned(), Some("PLA".to_owned())),
        ]
    );
}

#[tokio::test]
async fn every_item_field_moves_on_an_edit() {
    let world = World::new().await;
    let first = a_location(&world, &world.one, "the shelf").await;
    let second = a_location(&world, &world.one, "the garage").await;
    let template = declare_template(&world, "filament", &[]).await;

    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(1, 0)),
                unit: Some("roll".into()),
                location_id: Some(id_bytes(first)),
                property_values: vec![property("material", "PLA")],
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let base = world.counter(id).await;

    world
        .submit(
            &world.one,
            edit_specific(
                id,
                base,
                item(v1::ItemContent {
                    asserted_amount: Some(decimal(12, 500_000_000)),
                    unit: Some("rolls".into()),
                    location_id: Some(id_bytes(second)),
                    template_id: Some(template.as_bytes().to_vec()),
                    property_values: vec![property("material", "PETG")],
                    ..Default::default()
                }),
            ),
        )
        .await;

    let row = item_row(&world, id).await;
    assert_eq!(row.amount.as_deref(), Some("12.500000000"));
    assert_eq!(row.unit.as_deref(), Some("rolls"));
    assert_eq!(row.location, Some(second.as_uuid()));
    assert_eq!(row.template, Some(template));
    assert_eq!(
        properties_of(&world, id).await,
        vec![("material".to_owned(), Some("PETG".to_owned()))]
    );
}

/// Clearing empties every one of them, and the store holds nothing rather than
/// an empty string.
#[tokio::test]
async fn clearing_every_item_field_empties_it() {
    let world = World::new().await;
    let shelf = a_location(&world, &world.one, "the shelf").await;
    let template = declare_template(&world, "filament", &[]).await;

    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(3, 0)),
                unit: Some("rolls".into()),
                location_id: Some(id_bytes(shelf)),
                template_id: Some(template.as_bytes().to_vec()),
                property_values: vec![property("material", "PLA")],
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let base = world.counter(id).await;

    // Every field but the assertion time, which is not one a client states.
    world
        .submit(
            &world.one,
            edit_specific(
                id,
                base,
                item(v1::ItemContent {
                    cleared: vec![1, 2, 4, 5, 6],
                    ..Default::default()
                }),
            ),
        )
        .await;

    let row = item_row(&world, id).await;
    assert_eq!(row.amount, None);
    assert_eq!(row.unit, None);
    assert_eq!(row.location, None);
    assert_eq!(row.template, None);
    assert_eq!(row.asserted_at, None, "the time goes with the amount");
    assert!(properties_of(&world, id).await.is_empty());
}

/// **No required fields.** An item with a name and nothing else is a complete
/// item, and so is one with no name at all.
#[tokio::test]
async fn an_item_with_a_name_and_nothing_else_is_complete() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            item(v1::ItemContent::default()),
            v1::EntityContent {
                title: Some("the good screwdriver".into()),
                ..Default::default()
            },
        )
        .await;
    let row = item_row(&world, id).await;
    assert_eq!(row.amount, None);
    assert_eq!(row.asserted_at, None);
    assert_eq!(row.unit, None);
    assert_eq!(row.location, None);
}

// ---------------------------------------------------------------------------
// The amount and its own timestamp
// ---------------------------------------------------------------------------

/// **The pair, written in one statement.**
///
/// `CHECK ((asserted_amount IS NULL) = (asserted_at IS NULL))` refuses the state
/// in between, so an implementation that wrote the two columns in sequence would
/// fail whichever order it chose — the amount alone leaves the time null, and
/// the time alone leaves the amount null. Both of these submissions would have
/// been a store fault rather than an acknowledgement, so the assertion is that
/// they landed at all as much as it is the values.
#[tokio::test]
async fn an_amount_and_its_time_are_written_together_or_the_table_refuses_it() {
    let world = World::new().await;

    // Setting: both arrive.
    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(3, 0)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let row = item_row(&world, id).await;
    assert_eq!(row.amount.as_deref(), Some("3.000000000"));
    assert!(row.asserted_at.is_some());

    // Clearing: both leave.
    let base = world.counter(id).await;
    world
        .submit(
            &world.one,
            edit_specific(
                id,
                base,
                item(v1::ItemContent {
                    cleared: vec![1],
                    ..Default::default()
                }),
            ),
        )
        .await;
    let row = item_row(&world, id).await;
    assert_eq!(row.amount, None);
    assert_eq!(row.asserted_at, None);
}

/// **The requirement most easily broken by a plausible implementation.**
///
/// The assertion time is the amount's own and not a general modified time.
/// Renaming an item or correcting its notes does not confirm what is on the
/// shelf, and a timestamp that moves when those happen makes a stale count look
/// fresh — which the PRD calls worse than showing nothing.
#[tokio::test]
async fn an_unrelated_edit_leaves_the_assertion_time_alone() {
    let world = World::new().await;
    let shelf = a_location(&world, &world.one, "the shelf").await;
    let template = declare_template(&world, "filament", &[]).await;

    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(3, 0)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let asserted = item_row(&world, id).await.asserted_at.expect("a time");

    // Everything an item has that is not its amount, one edit at a time.
    let unrelated = [
        v1::EntityContent {
            title: Some("black filament".into()),
            ..Default::default()
        },
        v1::EntityContent {
            specific: Some(item(v1::ItemContent {
                unit: Some("rolls".into()),
                ..Default::default()
            })),
            ..Default::default()
        },
        v1::EntityContent {
            specific: Some(item(v1::ItemContent {
                location_id: Some(id_bytes(shelf)),
                ..Default::default()
            })),
            ..Default::default()
        },
        v1::EntityContent {
            specific: Some(item(v1::ItemContent {
                template_id: Some(template.as_bytes().to_vec()),
                ..Default::default()
            })),
            ..Default::default()
        },
        v1::EntityContent {
            specific: Some(item(v1::ItemContent {
                property_values: vec![property("material", "PLA")],
                ..Default::default()
            })),
            ..Default::default()
        },
    ];

    for content in unrelated {
        let base = world.counter(id).await;
        world
            .submit(&world.one, edit_entity(id, base as u64, content))
            .await;
        assert_eq!(
            item_row(&world, id).await.asserted_at,
            Some(asserted),
            "an edit that is not the amount moved the time the amount was asserted"
        );
    }

    assert_eq!(
        item_row(&world, id).await.amount.as_deref(),
        Some("3.000000000"),
        "and the amount itself is untouched"
    );
}

/// The other half of the same rule. *I looked again, still three* is a real act,
/// and it is how the freshness of a count is recorded.
#[tokio::test]
async fn re_asserting_the_same_amount_moves_the_time() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(3, 0)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let first = item_row(&world, id).await.asserted_at.expect("a time");

    let base = world.counter(id).await;
    world
        .submit(
            &world.one,
            edit_specific(
                id,
                base,
                item(v1::ItemContent {
                    asserted_amount: Some(decimal(3, 0)),
                    ..Default::default()
                }),
            ),
        )
        .await;

    let row = item_row(&world, id).await;
    assert_eq!(row.amount.as_deref(), Some("3.000000000"));
    assert!(
        row.asserted_at.expect("a time") > first,
        "counting again is an assertion, even when the answer is the same"
    );
}

/// **A client's own assertion time is honoured, not replaced.**
///
/// The wire carries `asserted_at` so a client can say when the person looked,
/// and acceptance is local and durable with the outbox replaying later — so a
/// count taken at nine and reaching the instance at six must read nine. Wave 2.1
/// settled the same question for a create's `created_at`; substituting the
/// instance's clock tells a client its capture landed with a time it never sent.
#[tokio::test]
async fn a_client_stated_assertion_time_is_what_the_store_keeps() {
    let world = World::new().await;
    let stated = "2026-03-04T09:15:00Z"
        .parse::<chrono::DateTime<chrono::Utc>>()
        .unwrap();

    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(3, 0)),
                asserted_at: Some(timestamp(stated)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;

    let row = item_row(&world, id).await;
    assert_eq!(row.amount.as_deref(), Some("3.000000000"));
    assert_eq!(
        row.asserted_at,
        Some(stated),
        "the instance substituted its own clock for a time the client sent"
    );
}

/// And on an edit, which is the path a replayed outbox actually takes.
#[tokio::test]
async fn a_client_stated_assertion_time_is_honoured_on_an_edit_too() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(1, 0)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let base = world.counter(id).await;
    let stated = "2026-03-04T09:15:00Z"
        .parse::<chrono::DateTime<chrono::Utc>>()
        .unwrap();

    world
        .submit(
            &world.one,
            edit_specific(
                id,
                base,
                item(v1::ItemContent {
                    asserted_amount: Some(decimal(7, 0)),
                    asserted_at: Some(timestamp(stated)),
                    ..Default::default()
                }),
            ),
        )
        .await;

    let row = item_row(&world, id).await;
    assert_eq!(row.amount.as_deref(), Some("7.000000000"));
    assert_eq!(row.asserted_at, Some(stated));
}

/// **Stating the time alone still works, when there is an amount for it to
/// belong to.** *I looked again, still three* — the count is unchanged and only
/// the freshness moved.
#[tokio::test]
async fn an_assertion_time_may_be_restated_alone_when_an_amount_is_already_there() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(3, 0)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let base = world.counter(id).await;
    let looked_again = "2026-05-01T18:00:00Z"
        .parse::<chrono::DateTime<chrono::Utc>>()
        .unwrap();

    world
        .submit(
            &world.one,
            edit_specific(
                id,
                base,
                item(v1::ItemContent {
                    asserted_at: Some(timestamp(looked_again)),
                    ..Default::default()
                }),
            ),
        )
        .await;

    let row = item_row(&world, id).await;
    assert_eq!(row.asserted_at, Some(looked_again));
    assert_eq!(
        row.amount.as_deref(),
        Some("3.000000000"),
        "restating the time must not disturb the count"
    );
}

/// **The refusal that remains, and it is about the row rather than the field.**
///
/// The table's check refuses a time with no amount beside it. All three of these
/// name that row: a time on an item that holds no amount, a time in the same
/// message that clears the amount, and clearing the time while an amount stands.
/// Each is malformed rather than a constraint violation that takes the whole
/// transaction down with a message nobody can act on.
#[tokio::test]
async fn an_assertion_time_with_no_amount_to_belong_to_is_malformed() {
    let world = World::new().await;
    let at = timestamp(chrono::Utc::now());

    // On a fresh item that states no amount.
    let ack = world
        .submit(
            &world.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    specific: Some(item(v1::ItemContent {
                        asserted_at: Some(at),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            ),
        )
        .await;
    let r = refused(&ack);
    assert_eq!(r.reason, v1::RefusalReason::Malformed as i32, "{r:?}");
    assert!(r.detail.contains("belongs to an amount"), "{}", r.detail);

    // Clearing the amount and stating a time in one message.
    let ack = world
        .submit(
            &world.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    specific: Some(item(v1::ItemContent {
                        asserted_at: Some(at),
                        cleared: vec![1],
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            ),
        )
        .await;
    assert_eq!(
        refused(&ack).reason,
        v1::RefusalReason::Malformed as i32,
        "clearing the amount while stating a time was accepted"
    );

    // And clearing the time out from under an amount that stands.
    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(3, 0)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let base = world.counter(id).await;
    let ack = world
        .submit(
            &world.one,
            edit_specific(
                id,
                base,
                item(v1::ItemContent {
                    cleared: vec![3],
                    ..Default::default()
                }),
            ),
        )
        .await;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
    // The refusal left the row alone.
    assert!(item_row(&world, id).await.asserted_at.is_some());
}

/// **The stated time reaches the store in one statement with the amount, and
/// records its movement exactly once.**
///
/// The client states field 3 and the instance also moves it as the amount's
/// companion, so the same part is touched twice in one write. Provenance upserts
/// and `detect` drops a comparison where arriving equals stored, so the result
/// must be one counter row and no phantom conflict — this is the test that says
/// the redundancy is harmless rather than assuming it.
#[tokio::test]
async fn a_stated_time_and_its_companion_record_one_movement_and_no_conflict() {
    let world = World::new().await;
    let stated = "2026-03-04T09:15:00Z"
        .parse::<chrono::DateTime<chrono::Utc>>()
        .unwrap();
    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(3, 0)),
                asserted_at: Some(timestamp(stated)),
                ..Default::default()
            }),
            v1::EntityContent {
                audience_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;

    let moved = moved_parts(&world, id).await;
    let times: Vec<_> = moved.iter().filter(|(n, _)| n == "specific.3").collect();
    assert_eq!(
        times.len(),
        1,
        "the assertion time recorded twice: {moved:?}"
    );
    assert!(world.conflicts(id).await.is_empty());
}

/// Both halves of the pair record their movement.
///
/// `entity_part_counter` is per part and conflict detection asks which parts
/// moved between two counter values. A part that does not record its movement is
/// invisible to that, and the failure is silent.
#[tokio::test]
async fn the_amount_and_its_time_each_record_the_counter_they_moved_at() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(3, 0)),
                unit: Some("tins".into()),
                property_values: vec![property("brand", "Heinz")],
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;

    let moved = moved_parts(&world, id).await;
    for part in ["specific.1", "specific.2", "specific.3", "specific.6"] {
        assert!(
            moved
                .iter()
                .any(|(name, counter)| name == part && *counter == 1),
            "{part} recorded no movement, so nothing can conflict on it: {moved:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// The number a household's stock is carried in
// ---------------------------------------------------------------------------

/// Binary floating point is the wrong place for this, so nothing rounds on the
/// way through the store either.
#[tokio::test]
async fn an_amount_keeps_its_precision_through_the_store() {
    let world = World::new().await;
    for (units, nanos, text) in [
        (0_i64, 1_i32, "0.000000001"),
        (1, 750_000_000, "1.750000000"),
        (999_999_999, 999_999_999, "999999999.999999999"),
        (0, 0, "0.000000000"),
    ] {
        let id = world
            .create(
                &world.one,
                item(v1::ItemContent {
                    asserted_amount: Some(decimal(units, nanos)),
                    ..Default::default()
                }),
                v1::EntityContent::default(),
            )
            .await;
        assert_eq!(item_row(&world, id).await.amount.as_deref(), Some(text));
    }
}

/// **Not clamped.** A garbled number is a message that cannot be read, and
/// absorbing one would store something the client never sent — the same failure
/// as the base counter that was clamped and silently skipped conflict detection.
#[tokio::test]
async fn an_amount_out_of_range_is_refused_rather_than_clamped() {
    let world = World::new().await;
    for (units, nanos) in [(0_i64, 1_000_000_000_i32), (1, -1), (-1, 1)] {
        let ack = world
            .submit(
                &world.one,
                create_entity(
                    Uuid::new_v4(),
                    v1::EntityContent {
                        specific: Some(item(v1::ItemContent {
                            asserted_amount: Some(decimal(units, nanos)),
                            ..Default::default()
                        })),
                        ..Default::default()
                    },
                ),
            )
            .await;
        let r = refused(&ack);
        assert_eq!(
            r.reason,
            v1::RefusalReason::Malformed as i32,
            "{units} units and {nanos} nanos was absorbed: {r:?}"
        );
        assert!(r.detail.contains("decimal"), "{}", r.detail);
    }
}

// ---------------------------------------------------------------------------
// An item's location is a typed foreign key
// ---------------------------------------------------------------------------

/// The column is `FOREIGN KEY (location_id, location_type) REFERENCES entity`,
/// so pointing at something that is not a location is a refusal rather than a
/// cast — and rather than a constraint violation taking the transaction down.
#[tokio::test]
async fn an_items_location_must_actually_be_a_location() {
    let world = World::new().await;
    let note = world.note(&world.one, "not a place").await;

    let ack = world
        .submit(
            &world.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    specific: Some(item(v1::ItemContent {
                        location_id: Some(id_bytes(note)),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            ),
        )
        .await;
    assert!(is_not_available(&ack), "{:?}", ack.outcome);
}

/// **Refusal on audience and refusal on nonexistence are the same nothing.** A
/// location the member cannot see, one that does not exist, and one that is not
/// a location all answer identically — reason and detail alike.
#[tokio::test]
async fn a_location_that_is_hidden_and_one_that_does_not_exist_are_indistinguishable() {
    let world = World::new().await;
    // Private to two, so one cannot see it.
    let hidden = a_location(&world, &world.two, "the other shelf").await;
    let never = EntityId::from_uuid(Uuid::new_v4());
    let note = world.note(&world.one, "not a place").await;

    let mut answers = Vec::new();
    for target in [hidden, never, note] {
        let ack = world
            .submit(
                &world.one,
                create_entity(
                    Uuid::new_v4(),
                    v1::EntityContent {
                        specific: Some(item(v1::ItemContent {
                            location_id: Some(id_bytes(target)),
                            ..Default::default()
                        })),
                        ..Default::default()
                    },
                ),
            )
            .await;
        let r = refused(&ack);
        answers.push((r.reason, r.detail.clone()));
    }

    assert_eq!(answers[0].0, v1::RefusalReason::NotAvailable as i32);
    assert!(
        answers.windows(2).all(|w| w[0] == w[1]),
        "the three refusals differ, so one of them answers a question about what exists: \
         {answers:?}"
    );
}

// ---------------------------------------------------------------------------
// A location's fields, there and back
// ---------------------------------------------------------------------------

/// **The guarantee the spine's stub list used to carry, for this type.**
///
/// That list asserted a type refused rather than accepting a write and storing
/// nothing. Once a type is built the claim inverts, and the honest form of it is
/// this: every field the message addressed is in the row afterwards. Asserting
/// the stored value rather than the wording of a refusal is what makes it a
/// guarantee rather than a description.
#[tokio::test]
async fn every_location_field_reaches_the_store_on_creation() {
    let world = World::new().await;
    let cupboard = a_location(&world, &world.one, "the cupboard").await;
    let template = declare_template(&world, "hosting account", &[("renews", "date")]).await;

    let id = world
        .create(
            &world.one,
            location(v1::LocationContent {
                parent_location_id: Some(id_bytes(cupboard)),
                template_id: Some(template.as_bytes().to_vec()),
                property_values: vec![property("shelf", "third"), property("depth", "40cm")],
                ..Default::default()
            }),
            v1::EntityContent {
                title: Some("the shelf".into()),
                ..Default::default()
            },
        )
        .await;

    assert_eq!(
        location_row(&world, id).await,
        (Some(cupboard.as_uuid()), Some(template))
    );
    assert_eq!(
        properties_of(&world, id).await,
        vec![
            ("depth".to_owned(), Some("40cm".to_owned())),
            ("shelf".to_owned(), Some("third".to_owned())),
        ]
    );
}

#[tokio::test]
async fn every_location_field_moves_on_an_edit() {
    let world = World::new().await;
    let cupboard = a_location(&world, &world.one, "the cupboard").await;
    let garage = a_location(&world, &world.one, "the garage").await;
    let template = declare_template(&world, "hosting account", &[]).await;

    let id = world
        .create(
            &world.one,
            location(v1::LocationContent {
                parent_location_id: Some(id_bytes(cupboard)),
                property_values: vec![property("shelf", "third")],
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let base = world.counter(id).await;

    world
        .submit(
            &world.one,
            edit_specific(
                id,
                base,
                location(v1::LocationContent {
                    parent_location_id: Some(id_bytes(garage)),
                    template_id: Some(template.as_bytes().to_vec()),
                    property_values: vec![property("shelf", "second")],
                    ..Default::default()
                }),
            ),
        )
        .await;

    assert_eq!(
        location_row(&world, id).await,
        (Some(garage.as_uuid()), Some(template))
    );
    assert_eq!(
        properties_of(&world, id).await,
        vec![("shelf".to_owned(), Some("second".to_owned()))]
    );
}

#[tokio::test]
async fn clearing_every_location_field_empties_it() {
    let world = World::new().await;
    let cupboard = a_location(&world, &world.one, "the cupboard").await;
    let template = declare_template(&world, "hosting account", &[]).await;

    let id = world
        .create(
            &world.one,
            location(v1::LocationContent {
                parent_location_id: Some(id_bytes(cupboard)),
                template_id: Some(template.as_bytes().to_vec()),
                property_values: vec![property("shelf", "third")],
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let base = world.counter(id).await;

    world
        .submit(
            &world.one,
            edit_specific(
                id,
                base,
                location(v1::LocationContent {
                    cleared: vec![1, 2, 3],
                    ..Default::default()
                }),
            ),
        )
        .await;

    assert_eq!(location_row(&world, id).await, (None, None));
    assert!(properties_of(&world, id).await.is_empty());
}

/// **No required fields**, on this type too. A location with a name and nothing
/// else is a complete location, and the row holds nothing rather than a default
/// somebody invented for it.
#[tokio::test]
async fn a_location_with_a_name_and_nothing_else_is_complete() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            location(v1::LocationContent::default()),
            v1::EntityContent {
                title: Some("the drawer".into()),
                ..Default::default()
            },
        )
        .await;
    assert_eq!(location_row(&world, id).await, (None, None));
    assert!(properties_of(&world, id).await.is_empty());
}

/// A location's parts record their movement, so conflict detection can see them.
/// The same silent failure as the item's, on the type that has no companion.
#[tokio::test]
async fn a_locations_parts_record_the_counter_they_moved_at() {
    let world = World::new().await;
    let cupboard = a_location(&world, &world.one, "the cupboard").await;
    let template = declare_template(&world, "hosting account", &[]).await;

    let id = world
        .create(
            &world.one,
            location(v1::LocationContent {
                parent_location_id: Some(id_bytes(cupboard)),
                template_id: Some(template.as_bytes().to_vec()),
                property_values: vec![property("shelf", "third")],
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;

    let moved = moved_parts(&world, id).await;
    for part in ["specific.1", "specific.2", "specific.3"] {
        assert!(
            moved
                .iter()
                .any(|(name, counter)| name == part && *counter == 1),
            "{part} recorded no movement, so nothing can conflict on it: {moved:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Nesting, and the loop the schema cannot see
// ---------------------------------------------------------------------------

/// Refused through the write path, which is the caller migration one's gap (h)
/// was waiting for.
async fn nest(world: &World, child: EntityId, parent: EntityId) -> v1::Acknowledgement {
    let base = world.counter(child).await;
    world
        .submit(
            &world.one,
            edit_specific(
                child,
                base,
                location(v1::LocationContent {
                    parent_location_id: Some(id_bytes(parent)),
                    ..Default::default()
                }),
            ),
        )
        .await
}

fn is_a_loop(ack: &v1::Acknowledgement) -> bool {
    match &ack.outcome {
        Some(v1::acknowledgement::Outcome::Refused(r)) => {
            r.reason == v1::RefusalReason::Malformed as i32
                && r.detail.contains("nested inside itself")
        }
        _ => false,
    }
}

/// Self-reference passes for free — the schema already refuses it — so it is
/// here only to show it still does through this path.
#[tokio::test]
async fn a_location_cannot_be_its_own_parent() {
    let world = World::new().await;
    let a = a_location(&world, &world.one, "the cupboard").await;
    let ack = nest(&world, a, a).await;
    assert!(is_a_loop(&ack), "{:?}", ack.outcome);
}

/// **The case that matters.** Two hops, which no constraint can see.
#[tokio::test]
async fn a_two_hop_loop_in_nested_locations_is_refused() {
    let world = World::new().await;
    let cupboard = a_location(&world, &world.one, "the cupboard").await;
    let shelf = a_location(&world, &world.one, "the shelf").await;

    // The shelf goes in the cupboard, and then the cupboard is put on the shelf.
    assert!(matches!(
        nest(&world, shelf, cupboard).await.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));
    let ack = nest(&world, cupboard, shelf).await;
    assert!(is_a_loop(&ack), "{:?}", ack.outcome);
    assert_eq!(
        location_row(&world, cupboard).await.0,
        None,
        "the refusal left nothing behind"
    );
}

#[tokio::test]
async fn a_longer_loop_in_nested_locations_is_refused() {
    let world = World::new().await;
    let house = a_location(&world, &world.one, "the house").await;
    let room = a_location(&world, &world.one, "the room").await;
    let cupboard = a_location(&world, &world.one, "the cupboard").await;
    let shelf = a_location(&world, &world.one, "the shelf").await;

    for (child, parent) in [(room, house), (cupboard, room), (shelf, cupboard)] {
        assert!(matches!(
            nest(&world, child, parent).await.outcome,
            Some(v1::acknowledgement::Outcome::Applied(_))
        ));
    }

    // Four hops back up to the top.
    let ack = nest(&world, house, shelf).await;
    assert!(is_a_loop(&ack), "{:?}", ack.outcome);
}

/// **Nesting exists and is never required**, and the depth is nowhere bounded.
/// A flat set of locations is complete rather than degraded.
#[tokio::test]
async fn nesting_is_optional_and_its_depth_is_not_bounded() {
    let world = World::new().await;
    let flat = a_location(&world, &world.one, "the drawer").await;
    assert_eq!(location_row(&world, flat).await.0, None);

    let mut previous: Option<EntityId> = None;
    for depth in 0..8 {
        let here = a_location(&world, &world.one, &format!("level {depth}")).await;
        if let Some(parent) = previous {
            assert!(
                matches!(
                    nest(&world, here, parent).await.outcome,
                    Some(v1::acknowledgement::Outcome::Applied(_))
                ),
                "nesting stopped being allowed at depth {depth}"
            );
            assert_eq!(location_row(&world, here).await.0, Some(parent.as_uuid()));
        }
        previous = Some(here);
    }
}

/// A parent the member cannot see answers before anything about ancestry is
/// read, so the cycle refusal — which says more than nothing — is never reachable
/// for a location they were not entitled to know about.
#[tokio::test]
async fn a_parent_the_member_cannot_see_is_the_same_nothing() {
    let world = World::new().await;
    let hidden = a_location(&world, &world.two, "theirs").await;
    let mine = a_location(&world, &world.one, "mine").await;

    let base = world.counter(mine).await;
    let ack = world
        .submit(
            &world.one,
            edit_specific(
                mine,
                base,
                location(v1::LocationContent {
                    parent_location_id: Some(id_bytes(hidden)),
                    ..Default::default()
                }),
            ),
        )
        .await;
    assert!(is_not_available(&ack), "{:?}", ack.outcome);
}

// ---------------------------------------------------------------------------
// Templates and property values
// ---------------------------------------------------------------------------

/// **Detaching is always available and never destructive.** The entity stops
/// following the template and keeps every value it holds, with the names it had
/// at that moment — which is why the store keys a value by name rather than by a
/// reference to the template's property.
#[tokio::test]
async fn a_property_value_survives_its_template_being_detached() {
    let world = World::new().await;
    let template = declare_template(
        &world,
        "filament",
        &[("diameter", "text"), ("material", "text")],
    )
    .await;

    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                template_id: Some(template.as_bytes().to_vec()),
                property_values: vec![property("diameter", "1.75mm"), property("material", "PLA")],
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;

    let base = world.counter(id).await;
    world
        .submit(
            &world.one,
            edit_specific(
                id,
                base,
                item(v1::ItemContent {
                    cleared: vec![5],
                    ..Default::default()
                }),
            ),
        )
        .await;

    assert_eq!(item_row(&world, id).await.template, None);
    assert_eq!(
        properties_of(&world, id).await,
        vec![
            ("diameter".to_owned(), Some("1.75mm".to_owned())),
            ("material".to_owned(), Some("PLA".to_owned())),
        ],
        "detaching a template took the values with it"
    );
}

/// **Following is live, not a copy**, so a value is not re-keyed when the
/// template's property is renamed. It keeps the name it had, which is the whole
/// reason for keying by name.
#[tokio::test]
async fn a_stored_value_keeps_the_name_it_had_when_the_template_renames_its_property() {
    let world = World::new().await;
    let template = declare_template(&world, "filament", &[("material", "text")]).await;
    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                template_id: Some(template.as_bytes().to_vec()),
                property_values: vec![property("material", "PLA")],
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;

    sqlx::query("UPDATE template_property SET name = $2 WHERE template_id = $1 AND name = $3")
        .bind(template)
        .bind("polymer")
        .bind("material")
        .execute(&world.db.pool)
        .await
        .expect("rename");

    assert_eq!(
        properties_of(&world, id).await,
        vec![("material".to_owned(), Some("PLA".to_owned()))]
    );
}

/// One shared set reaches items *and* locations. A hosting account is a place a
/// virtual machine lives and also a thing with credentials and a renewal date,
/// and whether it is tracked as an item or a location is the person's call.
#[tokio::test]
async fn a_template_reaches_a_location_too() {
    let world = World::new().await;
    let template = declare_template(&world, "hosting account", &[("renews", "date")]).await;

    let id = world
        .create(
            &world.one,
            location(v1::LocationContent {
                template_id: Some(template.as_bytes().to_vec()),
                property_values: vec![property("account", "vps-1")],
                ..Default::default()
            }),
            v1::EntityContent {
                title: Some("the VPS".into()),
                ..Default::default()
            },
        )
        .await;

    assert_eq!(location_row(&world, id).await.1, Some(template));
    assert_eq!(
        properties_of(&world, id).await,
        vec![("account".to_owned(), Some("vps-1".to_owned()))]
    );
}

/// A template that was never declared is a refusal, not a foreign-key violation
/// that takes the transaction down with a message nobody can act on.
#[tokio::test]
async fn a_template_that_does_not_exist_is_refused() {
    let world = World::new().await;
    let ack = world
        .submit(
            &world.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    specific: Some(item(v1::ItemContent {
                        template_id: Some(Uuid::new_v4().as_bytes().to_vec()),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            ),
        )
        .await;
    assert!(is_not_available(&ack), "{:?}", ack.outcome);
}

/// **A kind is not a constraint.** No lengths, no ranges, no patterns, and
/// nothing required. A number property holding *about a dozen* is a real answer,
/// because an approximate amount is a real answer.
#[tokio::test]
async fn a_property_kind_constrains_nothing() {
    let world = World::new().await;
    let template = declare_template(
        &world,
        "stock",
        &[("count", "number"), ("bought", "date"), ("open", "boolean")],
    )
    .await;

    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                template_id: Some(template.as_bytes().to_vec()),
                property_values: vec![
                    property("count", "about a dozen"),
                    property("bought", "some time last spring"),
                    property("open", "half"),
                    // And a name the template never declared, which is not the
                    // template's business either.
                    property("undeclared", ""),
                ],
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;

    assert_eq!(
        properties_of(&world, id).await,
        vec![
            (
                "bought".to_owned(),
                Some("some time last spring".to_owned())
            ),
            ("count".to_owned(), Some("about a dozen".to_owned())),
            ("open".to_owned(), Some("half".to_owned())),
            ("undeclared".to_owned(), Some(String::new())),
        ]
    );
}

/// The list is the value, so it is replaced whole rather than merged. Anything
/// nested is replaced whole; the proto says so of a routine's occurrences and
/// the reasoning is the same.
#[tokio::test]
async fn property_values_are_replaced_whole_rather_than_merged() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                property_values: vec![property("a", "1"), property("b", "2")],
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let base = world.counter(id).await;

    world
        .submit(
            &world.one,
            edit_specific(
                id,
                base,
                item(v1::ItemContent {
                    property_values: vec![property("b", "two"), property("c", "3")],
                    ..Default::default()
                }),
            ),
        )
        .await;

    assert_eq!(
        properties_of(&world, id).await,
        vec![
            ("b".to_owned(), Some("two".to_owned())),
            ("c".to_owned(), Some("3".to_owned())),
        ]
    );
}

/// The store keys a value by name, so one name twice is two instructions with no
/// reading that honours both — and the store's own upsert would silently keep
/// whichever came last.
#[tokio::test]
async fn one_property_name_given_two_values_is_malformed() {
    let world = World::new().await;
    for values in [
        vec![property("material", "PLA"), property("material", "PETG")],
        vec![property("", "nameless")],
    ] {
        let ack = world
            .submit(
                &world.one,
                create_entity(
                    Uuid::new_v4(),
                    v1::EntityContent {
                        specific: Some(item(v1::ItemContent {
                            property_values: values,
                            ..Default::default()
                        })),
                        ..Default::default()
                    },
                ),
            )
            .await;
        let r = refused(&ack);
        assert_eq!(r.reason, v1::RefusalReason::Malformed as i32, "{r:?}");
        assert!(r.detail.contains("keyed by its name"), "{}", r.detail);
    }
}

// ---------------------------------------------------------------------------
// Conflict, on the parts this lane added
// ---------------------------------------------------------------------------

async fn shared_item(world: &World, content: v1::ItemContent) -> EntityId {
    world
        .create(
            &world.one,
            item(content),
            v1::EntityContent {
                audience_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await
}

#[tokio::test]
async fn a_same_part_conflict_on_an_amount_retains_both_values() {
    let world = World::new().await;
    let id = shared_item(
        &world,
        v1::ItemContent {
            asserted_amount: Some(decimal(3, 0)),
            ..Default::default()
        },
    )
    .await;
    let base = world.counter(id).await;

    for (member, amount) in [(&world.one, decimal(2, 0)), (&world.two, decimal(5, 0))] {
        world
            .submit(
                member,
                edit_specific(
                    id,
                    base,
                    item(v1::ItemContent {
                        asserted_amount: Some(amount),
                        ..Default::default()
                    }),
                ),
            )
            .await;
    }

    let conflicts = world.conflicts(id).await;
    let amount = conflicts
        .iter()
        .find(|c| c.field.as_deref() == Some("specific.1"))
        .expect("two people counted the same shelf and got different answers");
    assert_eq!(amount.mine.as_deref(), Some("5.000000000"));
    assert_eq!(amount.theirs.as_deref(), Some("2.000000000"));
    assert_eq!(amount.mine_member, Some(world.two.membership_id()));
    assert_eq!(
        item_row(&world, id).await.amount.as_deref(),
        Some("5.000000000")
    );
}

/// **Writes producing the same value are not divergent.** Two people counting
/// the same shelf and agreeing is the likely case, and forming a conflict out of
/// agreement is the machinery working against its own purpose.
#[tokio::test]
async fn two_members_asserting_the_same_amount_do_not_conflict_on_it() {
    let world = World::new().await;
    let id = shared_item(&world, v1::ItemContent::default()).await;
    let base = world.counter(id).await;

    for member in [&world.one, &world.two] {
        world
            .submit(
                member,
                edit_specific(
                    id,
                    base,
                    item(v1::ItemContent {
                        asserted_amount: Some(decimal(4, 0)),
                        unit: Some("tins".into()),
                        ..Default::default()
                    }),
                ),
            )
            .await;
    }

    let conflicts = world.conflicts(id).await;
    assert!(
        !conflicts
            .iter()
            .any(|c| c.field.as_deref() == Some("specific.1")),
        "agreement about the amount was recorded as a disagreement: {:?}",
        conflicts
            .iter()
            .map(|c| c.field.clone())
            .collect::<Vec<_>>()
    );
    assert!(
        !conflicts
            .iter()
            .any(|c| c.field.as_deref() == Some("specific.2")),
        "agreement about the unit was recorded as a disagreement"
    );
    assert_eq!(
        item_row(&world, id).await.amount.as_deref(),
        Some("4.000000000")
    );
}

/// **A stale base is never a rejection**, and different parts merge with nobody
/// involved. One counts the tins, the other names the unit.
#[tokio::test]
async fn a_stale_base_over_different_item_parts_merges_and_is_never_a_rejection() {
    let world = World::new().await;
    let id = shared_item(&world, v1::ItemContent::default()).await;
    let base = world.counter(id).await;

    world
        .submit(
            &world.one,
            edit_specific(
                id,
                base,
                item(v1::ItemContent {
                    unit: Some("tins".into()),
                    ..Default::default()
                }),
            ),
        )
        .await;
    let ack = world
        .submit(
            &world.two,
            edit_specific(
                id,
                base,
                item(v1::ItemContent {
                    asserted_amount: Some(decimal(6, 0)),
                    ..Default::default()
                }),
            ),
        )
        .await;

    assert!(
        applied(&ack).conflict.is_none(),
        "different parts merge; a stale base alone is not a conflict and never a rejection"
    );
    let row = item_row(&world, id).await;
    assert_eq!(row.unit.as_deref(), Some("tins"));
    assert_eq!(row.amount.as_deref(), Some("6.000000000"));
}

// ---------------------------------------------------------------------------
// What Tracking contributes to searchable text
// ---------------------------------------------------------------------------

/// The person's own words, so a just-captured item is findable by them before
/// any derivation runs — which is what makes the literal arm a permanent arm
/// rather than a fallback.
#[tokio::test]
async fn an_items_unit_and_property_values_reach_its_searchable_text() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            item(v1::ItemContent {
                asserted_amount: Some(decimal(3, 0)),
                unit: Some("rolls".into()),
                property_values: vec![property("material", "PLA")],
                ..Default::default()
            }),
            v1::EntityContent {
                title: Some("black filament".into()),
                ..Default::default()
            },
        )
        .await;

    let text = search_text(&world, id).await;
    for word in ["black filament", "rolls", "material", "PLA"] {
        assert!(text.contains(word), "{word} is not in {text:?}");
    }
    // Not the amount, which is a value rather than a word, and not the
    // assertion time.
    assert!(
        !text.contains("3.0"),
        "the amount reached search text: {text:?}"
    );
}

#[tokio::test]
async fn a_locations_property_values_reach_its_searchable_text() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            location(v1::LocationContent {
                property_values: vec![property("renews", "March")],
                ..Default::default()
            }),
            v1::EntityContent {
                title: Some("the VPS".into()),
                ..Default::default()
            },
        )
        .await;
    let text = search_text(&world, id).await;
    assert!(
        text.contains("the VPS") && text.contains("renews") && text.contains("March"),
        "{text:?}"
    );
}

// ---------------------------------------------------------------------------
// The refusal that is not this lane's to lift
// ---------------------------------------------------------------------------

/// `altair-v0-scope.md` defers shopping lists deliberately rather than
/// incidentally, and it governs the implementation plan where the two disagree.
/// Building item and location content does not touch that, and this is the test
/// that says so.
#[tokio::test]
async fn a_shopping_lists_entries_are_still_refused_with_the_reason() {
    let world = World::new().await;
    let ack = world
        .submit(
            &world.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    specific: Some(v1::entity_content::Specific::ShoppingList(
                        v1::ShoppingListContent {
                            body: Some("milk\nbread".into()),
                            cleared: Vec::new(),
                        },
                    )),
                    ..Default::default()
                },
            ),
        )
        .await;
    let r = refused(&ack);
    assert_eq!(r.reason, v1::RefusalReason::Malformed as i32, "{r:?}");
    assert!(
        r.detail.contains("deferred in v0") && r.detail.contains("anchor granularity"),
        "{}",
        r.detail
    );

    // And the list itself is still a thing that can be created.
    world
        .create(
            &world.one,
            v1::entity_content::Specific::ShoppingList(v1::ShoppingListContent::default()),
            v1::EntityContent {
                title: Some("the weekly shop".into()),
                ..Default::default()
            },
        )
        .await;
}

// ---------------------------------------------------------------------------
// What an erased location lets go of
// ---------------------------------------------------------------------------
//
// **Reached directly, because nothing calls it yet.** The seam belongs to this
// wave and the call belongs in the erase path, which this lane does not own —
// the same split `type_content.rs` used for `would_cycle` before either
// container had a caller. When the call is wired,
// `an_erase_does_not_yet_release_what_a_location_held` below is the test that
// has to be inverted, and it is written to say so.

/// Run the release against a location, outside any write of its own.
async fn detach(world: &World, container: EntityId, kind: EntityType) -> Detachment {
    let mut tx = begin_write(&world.db.pool).await.expect("a transaction");
    let mut ctx = Ctx {
        tx: &mut tx,
        member: MemberId::for_test(world.one.membership_id()),
        at: Utc::now(),
    };
    let answer = specific::detach_contained(&mut ctx, container, kind)
        .await
        .unwrap_or_else(|_| panic!("the store was reachable"));
    tx.commit().await.expect("commit");
    answer
}

fn released(answer: &Detachment) -> Vec<EntityId> {
    match answer {
        Detachment::Released(ids) => {
            let mut ids = ids.clone();
            ids.sort_by_key(|i| i.as_uuid());
            ids
        }
        other => panic!("expected a release, got {other:?}"),
    }
}

/// **Both kinds of thing a location holds, not just the nested one.**
///
/// The child-location case is the one that gets remembered, because the
/// container and the contained are the same type. The items shelved in it are
/// easier to forget and are the larger set, and both point back by identity.
#[tokio::test]
async fn an_erased_location_releases_its_children_and_its_items() {
    let world = World::new().await;
    let cupboard = a_location(&world, &world.one, "the cupboard").await;
    let shelf = a_location(&world, &world.one, "the shelf").await;
    let elsewhere = a_location(&world, &world.one, "the garage").await;

    assert!(matches!(
        nest(&world, shelf, cupboard).await.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));

    let tin = world
        .create(
            &world.one,
            item(v1::ItemContent {
                location_id: Some(id_bytes(cupboard)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    // An item somewhere else, which must be left exactly where it is.
    let other_tin = world
        .create(
            &world.one,
            item(v1::ItemContent {
                location_id: Some(id_bytes(elsewhere)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;

    let answer = detach(&world, cupboard, EntityType::Location).await;
    let mut expected = vec![shelf, tin];
    expected.sort_by_key(|i| i.as_uuid());
    assert_eq!(released(&answer), expected);

    // The shelf is now a top-level location, which is a complete state.
    assert_eq!(location_row(&world, shelf).await.0, None);
    // The tin has no location, which is also complete.
    assert_eq!(item_row(&world, tin).await.location, None);
    // And nothing else moved.
    assert_eq!(
        item_row(&world, other_tin).await.location,
        Some(elsewhere.as_uuid())
    );
}

/// A released entity's counter advances, because losing a container is an
/// accepted write to it. A client holding the old counter has to learn the
/// container went away.
///
/// **The bump is `store::entity::detached_from_container`'s, not the seam's** —
/// it happens on the same statement that reads the audience a change entry
/// needs, and that statement may only live in the store layer. So this goes
/// through the erase path rather than calling the seam directly.
#[tokio::test]
async fn releasing_an_entity_advances_its_counter() {
    let world = World::new().await;
    let cupboard = a_location(&world, &world.one, "the cupboard").await;
    let shelf = a_location(&world, &world.one, "the shelf").await;
    nest(&world, shelf, cupboard).await;

    let before = world.counter(shelf).await;
    world.submit(&world.one, erase(&[cupboard])).await;
    assert_eq!(world.counter(shelf).await, before + 1);
}

/// An empty container releases nothing, and that is an answer rather than an
/// absence — which is why it is a different variant from the one a lane that
/// has not built its release returns.
#[tokio::test]
async fn an_empty_location_releases_nothing_and_says_so() {
    let world = World::new().await;
    let empty = a_location(&world, &world.one, "the empty drawer").await;
    assert_eq!(
        detach(&world, empty, EntityType::Location).await,
        Detachment::Released(Vec::new())
    );
}

/// An item is a leaf, and a shopping list's entries are blocks of its own body
/// rather than entities pointing at it.
#[tokio::test]
async fn a_type_that_contains_nothing_says_it_has_no_container() {
    let world = World::new().await;
    let tin = world
        .create(
            &world.one,
            item(v1::ItemContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(
        detach(&world, tin, EntityType::Item).await,
        Detachment::NoContainer
    );

    let list = world
        .create(
            &world.one,
            v1::entity_content::Specific::ShoppingList(v1::ShoppingListContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(
        detach(&world, list, EntityType::ShoppingList).await,
        Detachment::NoContainer
    );
}

/// **The distinction the third variant exists for.** Guidance's three types are
/// containers whose release is not built, and they must not answer the way an
/// empty container answers — otherwise wiring the call would leave every arc and
/// quest pointing at a tombstone with the suite green.
///
/// **LANE: Guidance.** When that lane builds the release, this test changes to
/// assert a `Released`, and its failure here is the reminder.
#[tokio::test]
async fn guidances_containers_report_their_release_is_not_built() {
    let world = World::new().await;
    for (kind, specific) in [
        (
            EntityType::Campaign,
            v1::entity_content::Specific::Campaign(v1::CampaignContent::default()),
        ),
        (
            EntityType::Arc,
            v1::entity_content::Specific::Arc(v1::ArcContent::default()),
        ),
        (
            EntityType::Quest,
            v1::entity_content::Specific::Quest(v1::QuestContent::default()),
        ),
    ] {
        let id = world
            .create(&world.one, specific, v1::EntityContent::default())
            .await;
        assert_eq!(
            detach(&world, id, kind).await,
            Detachment::NotBuilt,
            "{kind:?} answered as though it held nothing"
        );
    }
}

/// A category's release is the shared model's and Wave 2.1 already built it —
/// the erase path calls `uncategorise_all` directly, so nothing is owed on this
/// seam. A note and a file hold nothing at all.
#[tokio::test]
async fn the_types_whose_release_is_not_this_seams_owe_it_nothing() {
    let world = World::new().await;
    let category = world
        .create(
            &world.one,
            v1::entity_content::Specific::Category(v1::CategoryContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(
        detach(&world, category, EntityType::Category).await,
        Detachment::NoContainer
    );

    let note = world.note(&world.one, "holds nothing").await;
    assert_eq!(
        detach(&world, note, EntityType::Note).await,
        Detachment::NoContainer
    );
}

/// **Wired.** An erased location releases what it held, and the change sequence
/// learns about each one.
///
/// This is the test that was a characterisation of the unwired state one commit
/// ago; the assertions inverted when the call landed, which is exactly what it
/// was written to make visible.
#[tokio::test]
async fn an_erase_releases_what_a_location_held_and_says_so_in_the_change_sequence() {
    let world = World::new().await;
    let cupboard = a_location(&world, &world.one, "the cupboard").await;
    let shelf = a_location(&world, &world.one, "the shelf").await;
    nest(&world, shelf, cupboard).await;
    let tin = world
        .create(
            &world.one,
            item(v1::ItemContent {
                location_id: Some(id_bytes(cupboard)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;

    world.submit(&world.one, erase(&[cupboard])).await;
    assert_eq!(world.lifecycle(cupboard).await, "erased");

    // Neither points at the tombstone. Both states are complete rather than
    // damaged: a top-level location, and an item with no location.
    assert_eq!(location_row(&world, shelf).await.0, None);
    assert_eq!(item_row(&world, tin).await.location, None);

    // **A change nobody learns about is the hole this closes.** Each released
    // entity gets an entry of its own, the way an uncategorised one does.
    let changes = world.changes().await;
    for released in [shelf, tin] {
        assert!(
            changes
                .iter()
                .any(|c| c.entity == Some(released.as_uuid()) && c.kind == "entity_written"),
            "{released:?} lost its container with no change entry"
        );
    }
}

/// An erased location that held nothing releases nothing, and the erase is
/// unaffected.
#[tokio::test]
async fn erasing_an_empty_location_releases_nothing() {
    let world = World::new().await;
    let empty = a_location(&world, &world.one, "the empty drawer").await;
    world.submit(&world.one, erase(&[empty])).await;
    assert_eq!(world.lifecycle(empty).await, "erased");
}

/// **The release reaches entities the erasing member cannot see.**
///
/// Deliberately unscoped, for the reason `uncategorise_all` gives: the container
/// is gone for everybody, so leaving the rows one member cannot see still
/// pointing at it would keep a dangling reference alive precisely where nobody
/// can find it.
#[tokio::test]
async fn the_release_is_unscoped_because_the_container_is_gone_for_everybody() {
    let world = World::new().await;
    // One owns the cupboard and can erase it; two shelves inside it are two's
    // own and private, so one cannot see them.
    let cupboard = a_location(&world, &world.one, "the shared cupboard").await;
    let theirs = world
        .create(
            &world.two,
            location(v1::LocationContent::default()),
            v1::EntityContent {
                title: Some("their shelf".into()),
                ..Default::default()
            },
        )
        .await;
    // Placed by its own member, so the nesting is legitimate.
    sqlx::query("UPDATE location SET parent_location_id = $2 WHERE entity_id = $1")
        .bind(theirs.as_uuid())
        .bind(cupboard.as_uuid())
        .execute(&world.db.pool)
        .await
        .expect("nest");

    world.submit(&world.one, erase(&[cupboard])).await;

    assert_eq!(
        location_row(&world, theirs).await.0,
        None,
        "a child the erasing member cannot see kept pointing at the tombstone"
    );
}

/// **Where the companion's value actually matters: the conflict comparison.**
///
/// A client states the time, and the instance also moves that part as the
/// amount's companion. Both sides of the conflict render through the same code,
/// so the companion must carry *the client's* instant — if it carried the
/// instance's clock instead, this write would touch the part twice with two
/// different arriving values and retain a value the client never sent.
///
/// Two members, one stale base, both stating their own times.
#[tokio::test]
async fn a_conflict_on_a_client_stated_time_retains_the_time_the_client_sent() {
    let world = World::new().await;
    let id = shared_item(
        &world,
        v1::ItemContent {
            asserted_amount: Some(decimal(1, 0)),
            ..Default::default()
        },
    )
    .await;
    let base = world.counter(id).await;
    let theirs = "2026-03-04T09:00:00Z"
        .parse::<chrono::DateTime<chrono::Utc>>()
        .unwrap();
    let mine = "2026-03-05T17:30:00Z"
        .parse::<chrono::DateTime<chrono::Utc>>()
        .unwrap();

    for (member, amount, at) in [
        (&world.one, decimal(2, 0), theirs),
        (&world.two, decimal(5, 0), mine),
    ] {
        world
            .submit(
                member,
                edit_specific(
                    id,
                    base,
                    item(v1::ItemContent {
                        asserted_amount: Some(amount),
                        asserted_at: Some(timestamp(at)),
                        ..Default::default()
                    }),
                ),
            )
            .await;
    }

    let conflicts = world.conflicts(id).await;
    let time = conflicts
        .iter()
        .find(|c| c.field.as_deref() == Some("specific.3"))
        .expect("two members asserted at different times from one base");
    // Exactly one row for this part, and both sides are times a client sent.
    assert_eq!(
        conflicts
            .iter()
            .filter(|c| c.field.as_deref() == Some("specific.3"))
            .count(),
        1
    );
    assert_eq!(time.mine.as_deref(), Some(mine.to_rfc3339().as_str()));
    assert_eq!(time.theirs.as_deref(), Some(theirs.to_rfc3339().as_str()));
    assert_eq!(item_row(&world, id).await.asserted_at, Some(mine));
}

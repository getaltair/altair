//! One invariant over `entity_part_counter`, applied to every kind of part.
//!
//! # Why this suite exists
//!
//! Wave 2.2 produced four bugs in this one table, and every one of them was
//! silent:
//!
//! - a part **recorded that did not move** — a write submitting a value that was
//!   already there took ownership of the part, so a later conflict named a
//!   member who had authored nothing;
//! - a part that **moved and was not recorded** — `uncategorise_all` cleared a
//!   category and its position and advanced the counter, and the table said
//!   nothing happened;
//! - the same again on the type-specific release, where an erased container let
//!   go of what it held;
//! - and **the member**, which is the third face of the first: `member_id` is
//!   overwritten by whoever wrote last, and it crosses the wire in
//!   `conflict.theirs_member_id` for a client to render.
//!
//! Each was found by reading code. None was caught by a test, because every
//! check on this table was incidental to some other assertion — a test about
//! conflicts that happened to look at a counter. **This suite makes the table
//! the subject.**
//!
//! # The invariant
//!
//! > **The set of parts a write records is exactly the set whose stored value
//! > changed, and each is attributed to the member who changed it.**
//!
//! Both halves matter and they fail in opposite directions. A part recorded
//! without moving makes an unrelated later edit look like an overlap; a part
//! that moves without being recorded makes a real overlap invisible, and *both
//! values retained on a same-part conflict* is a standing constraint.
//!
//! # How the expected set is derived, and why not the obvious way
//!
//! **Never from the write path.** The tempting implementation asks
//! `write::entity::current` what a part holds, which is the same code the write
//! path uses to decide what it recorded — an assertion against itself, green
//! whatever either does. This wave already produced one test that passed with
//! the mechanism it was testing deliberately broken, and the lesson generalises.
//!
//! So the expected set comes from **the store's own columns, read as text**,
//! through the field-to-column mapping each type declares in
//! [`altaird::write::specific`]. Snapshot every part before a write, snapshot
//! after, and the parts whose text differs are the parts that moved. Nothing in
//! that path consults provenance, and nothing renders a value the way the write
//! path renders it — [`snapshot`] reads `entity`, `block` and the side tables
//! directly.
//!
//! The rendering here need not match the write path's. It only has to be
//! *stable*, because the only question asked of it is whether two readings of
//! the same column differ.
//!
//! # Why the whole-set invariant is asserted on edits, not on creates
//!
//! **It does not hold on a create, and that is correct rather than a bug.**
//! Measured, not assumed — a create that clears a title and states nothing else
//! gives:
//!
//! ```text
//! changed:  ["Content(Bulk)"]
//! recorded: ["Content(Title)"]
//! ```
//!
//! Both directions diverge, and each for a good reason. `bulk` changed because
//! the row went from absent to the column's `false`, which nobody wrote — the
//! entity came into existence, and recording provenance for every column that
//! took a default would say a member edited things they never addressed.
//! `title` was recorded because a create records what it applied without
//! comparing, there being no prior value to compare against.
//!
//! Neither is reachable as a fault. Provenance written at counter 1 cannot
//! produce a spurious conflict, because conflict detection runs only when an
//! edit's base counter is *below* the entity's, and nothing can hold a base
//! below the counter a create issues.
//!
//! So the whole-set invariant belongs to the edit path and to the consequential
//! writes, where a prior value exists and provenance is actually read. **A
//! create is not exempt from scrutiny** — it is asserted per part instead, where
//! a specific claim is meaningful: see
//! `a_companion_for_an_empty_sibling_is_not_recorded_on_creation`, which is the
//! one path where a type handing back a companion it did not move reaches the
//! store unfiltered.
//!
//! # Two guards on the suite itself
//!
//! - **Every exercise must write something.** A write path that refused
//!   everything would satisfy "no part was wrongly recorded" perfectly, and an
//!   exercise whose intent drifts into refusal would go on passing while testing
//!   nothing. [`Exercise::run`] refuses an empty change set unless the exercise
//!   says it expects one.
//! - **The mapping is walked, not listed.** Type-specific parts come from
//!   `specific::fields`, so a type gaining a field is covered here without
//!   anyone remembering to add it. Listing them would drift exactly as the two
//!   callers of `would_cycle` drifted.

mod common;

use std::collections::{HashMap, HashSet};

use altair_proto::v1;
use altaird::store::audience::AUDIENCE_COLUMN;
use altaird::store::entity::EntityType;
use altaird::store::ids::EntityId;
use altaird::write::parts::{ALL_CONTENT_PARTS, ContentPart, Part, PartKey};
use altaird::write::specific::{self, Held};
use common::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Reading what the store actually holds
// ---------------------------------------------------------------------------

/// The separators a composite value is rendered with here.
///
/// The same two the write path uses, but that is a convenience rather than a
/// requirement: nothing compares a value produced here against one produced
/// there. What matters is that two readings of an unchanged column produce the
/// same string.
const WITHIN: &str = "\u{1f}";
const BETWEEN: &str = "\u{1e}";

async fn scalar(pool: &PgPool, sql: String, id: Uuid) -> Option<String> {
    sqlx::query(sqlx::AssertSqlSafe(sql))
        .bind(id)
        .fetch_optional(pool)
        .await
        .expect("query")
        .and_then(|r| r.try_get::<Option<String>, _>("v").expect("v"))
}

/// The raw column behind each shared-model part.
///
/// A test may name the audience column and query `entity` directly, and
/// `tests/one_predicate.rs` exempts test sources deliberately — a suite that
/// could only read the store through the audience-scoped surface would be
/// checking the predicate against itself.
fn content_sql(part: ContentPart) -> String {
    match part {
        ContentPart::Title => "SELECT title AS v FROM entity WHERE id = $1".into(),
        ContentPart::Category => "SELECT category_id::text AS v FROM entity WHERE id = $1".into(),
        ContentPart::CategoryPosition => {
            "SELECT category_position::text AS v FROM entity WHERE id = $1".into()
        }
        ContentPart::Bulk => "SELECT bulk::text AS v FROM entity WHERE id = $1".into(),
        ContentPart::Audience => format!(
            "SELECT nullif(array_to_string(ARRAY(\
               SELECT unnest({AUDIENCE_COLUMN}) ORDER BY 1), ','), '') AS v \
             FROM entity WHERE id = $1"
        ),
        ContentPart::Dates => format!(
            "SELECT string_agg(label || '{WITHIN}' || occurs_at::text || '{WITHIN}' || \
               bring_forward::text, '{BETWEEN}' ORDER BY ordinal) AS v \
             FROM entity_date WHERE entity_id = $1"
        ),
        ContentPart::Assignments => "SELECT string_agg(member_id::text, ',' ORDER BY \
             member_id::text) AS v FROM entity_assignment WHERE entity_id = $1"
            .into(),
    }
}

/// Every part of one entity, and the text the store holds for it.
///
/// `None` means the part holds nothing — including the case where the entity or
/// its side-table row does not exist, which is what makes a snapshot taken
/// before a create meaningful.
///
/// **Type-specific parts are walked from the declaration**, so a type gaining a
/// field is covered without anyone adding a line here.
async fn snapshot(pool: &PgPool, id: EntityId, kind: EntityType) -> HashMap<Part, Option<String>> {
    let uuid = id.as_uuid();
    let mut out = HashMap::new();

    for part in ALL_CONTENT_PARTS {
        out.insert(
            Part::Content(*part),
            scalar(pool, content_sql(*part), uuid).await,
        );
    }

    let table = specific::side_table(kind);
    for field in specific::fields(kind) {
        let value = match (field.held, table) {
            (Held::Column(column), Some(table)) => {
                scalar(
                    pool,
                    format!("SELECT {column}::text AS v FROM {table} WHERE entity_id = $1"),
                    uuid,
                )
                .await
            }
            (Held::Rows(rows), _) => {
                scalar(
                    pool,
                    format!(
                        "SELECT string_agg(name || '{WITHIN}' || coalesce(value, ''), \
                         '{BETWEEN}' ORDER BY name) AS v FROM {rows} WHERE entity_id = $1"
                    ),
                    uuid,
                )
                .await
            }
            // A body is not one part. Its blocks are, and they are collected
            // below by identity rather than by field number.
            (Held::Body, _) | (Held::NotServed(_), _) | (Held::Column(_), None) => continue,
        };
        out.insert(Part::Specific(field.number), value);
    }

    let blocks = sqlx::query("SELECT id, text FROM block WHERE entity_id = $1")
        .bind(uuid)
        .fetch_all(pool)
        .await
        .expect("blocks");
    for b in &blocks {
        out.insert(
            Part::Block(b.try_get("id").expect("id")),
            Some(b.try_get::<String, _>("text").expect("text")),
        );
    }

    out
}

/// The parts whose stored text differs between two snapshots.
///
/// A part present in one snapshot and absent from the other is a block that was
/// created or removed, and that is a change.
fn changed(
    before: &HashMap<Part, Option<String>>,
    after: &HashMap<Part, Option<String>>,
) -> HashSet<Part> {
    let mut parts: HashSet<Part> = HashSet::new();
    for key in before.keys().chain(after.keys()) {
        let was = before.get(key).cloned().flatten();
        let is = after.get(key).cloned().flatten();
        if was != is {
            parts.insert(key.clone());
        }
    }
    parts
}

/// Parts in a stable order, for a message a person can read.
///
/// `Part` is hashed rather than ordered, so a set has no order of its own and an
/// assertion message would otherwise vary between runs.
fn listed(parts: &HashSet<Part>) -> Vec<String> {
    let mut out: Vec<String> = parts.iter().map(|p| format!("{p:?}")).collect();
    out.sort();
    out
}

/// What `entity_part_counter` says moved at exactly this counter.
async fn recorded_at(pool: &PgPool, id: EntityId, counter: i64) -> HashSet<Part> {
    let rows = sqlx::query(
        "SELECT field_name, block_id FROM entity_part_counter \
         WHERE entity_id = $1 AND counter = $2",
    )
    .bind(id.as_uuid())
    .bind(counter)
    .fetch_all(pool)
    .await
    .expect("part counters");

    rows.iter()
        .map(|r| {
            let field: Option<String> = r.try_get("field_name").expect("field_name");
            let block: Option<Uuid> = r.try_get("block_id").expect("block_id");
            let key = match (field, block) {
                (Some(name), None) => PartKey::Field(name),
                (None, Some(id)) => PartKey::Block(id),
                other => panic!("a part row set both or neither: {other:?}"),
            };
            Part::from_key(&key).expect("a part name this build knows")
        })
        .collect()
}

/// Who `entity_part_counter` says last moved a part.
async fn owner_of(pool: &PgPool, id: EntityId, part: &Part) -> Option<Uuid> {
    let (field, block) = match part.key() {
        PartKey::Field(name) => (Some(name), None),
        PartKey::Block(id) => (None, Some(id)),
    };
    sqlx::query(
        "SELECT member_id FROM entity_part_counter \
         WHERE entity_id = $1 AND field_name IS NOT DISTINCT FROM $2 \
           AND block_id IS NOT DISTINCT FROM $3",
    )
    .bind(id.as_uuid())
    .bind(field)
    .bind(block)
    .fetch_optional(pool)
    .await
    .expect("query")
    .and_then(|r| r.try_get("member_id").expect("member_id"))
}

async fn counter_of(pool: &PgPool, id: EntityId) -> i64 {
    sqlx::query("SELECT counter FROM entity WHERE id = $1")
        .bind(id.as_uuid())
        .fetch_one(pool)
        .await
        .expect("entity")
        .try_get("counter")
        .expect("counter")
}

// ---------------------------------------------------------------------------
// The invariant, applied
// ---------------------------------------------------------------------------

/// One write, checked against the invariant.
///
/// The entity watched need not be the entity written: the upward state climb
/// and both container releases move parts on *other* entities, and those are
/// exactly the writes where "a part moved and nothing recorded it" lived.
struct Watch {
    entity: EntityId,
    kind: EntityType,
    before: HashMap<Part, Option<String>>,
    counter_before: i64,
}

impl Watch {
    async fn open(world: &World, entity: EntityId, kind: EntityType) -> Self {
        Self {
            entity,
            kind,
            before: snapshot(&world.db.pool, entity, kind).await,
            counter_before: counter_of(&world.db.pool, entity).await,
        }
    }

    /// Check the invariant, and return the parts that moved.
    ///
    /// `expect_change` is the second guard on the suite: an exercise that stops
    /// writing anything — because a refusal crept in, or because the shape it
    /// exercised moved — would otherwise satisfy the invariant trivially and go
    /// on passing.
    async fn close(self, world: &World, what: &str) -> HashSet<Part> {
        let after = snapshot(&world.db.pool, self.entity, self.kind).await;
        let moved = changed(&self.before, &after);
        let counter = counter_of(&world.db.pool, self.entity).await;

        assert!(
            !moved.is_empty(),
            "{what}: nothing about {:?} changed, so this exercise is asserting the \
             invariant against an empty set and would pass however the write path behaved",
            self.entity
        );
        assert!(
            counter > self.counter_before,
            "{what}: parts of {:?} moved ({:?}) and its counter did not, so no \
             client can learn the write happened",
            self.entity,
            listed(&moved)
        );

        let recorded = recorded_at(&world.db.pool, self.entity, counter).await;
        assert_eq!(
            recorded,
            moved,
            "{what}: the parts recorded at counter {counter} are not the parts whose \
             stored value changed.\n  recorded but unchanged: {:?}\n  changed but \
             unrecorded: {:?}",
            listed(&recorded.difference(&moved).cloned().collect()),
            listed(&moved.difference(&recorded).cloned().collect())
        );
        moved
    }

    /// The other half: a write that changed nothing about this entity must
    /// record nothing about it either.
    async fn close_expecting_no_change(self, world: &World, what: &str) {
        let after = snapshot(&world.db.pool, self.entity, self.kind).await;
        let moved = changed(&self.before, &after);
        assert!(
            moved.is_empty(),
            "{what}: expected nothing to change, but {:?} did",
            listed(&moved)
        );
        let counter = counter_of(&world.db.pool, self.entity).await;
        let recorded = recorded_at(&world.db.pool, self.entity, counter).await;
        assert!(
            recorded.is_empty(),
            "{what}: nothing about {:?} changed, yet {:?} were recorded as having \
             moved at counter {counter}. A later edit will read that as an overlap.",
            self.entity,
            listed(&recorded)
        );
    }
}

// ---------------------------------------------------------------------------
// Shared-model parts
// ---------------------------------------------------------------------------

fn note() -> v1::entity_content::Specific {
    v1::entity_content::Specific::Note(v1::NoteContent::default())
}

fn campaign(state: Option<v1::GuidanceState>) -> v1::entity_content::Specific {
    v1::entity_content::Specific::Campaign(v1::CampaignContent {
        state: state.map(|s| s as i32),
        cleared: Vec::new(),
    })
}

fn item(c: v1::ItemContent) -> v1::entity_content::Specific {
    v1::entity_content::Specific::Item(c)
}

fn id_bytes(id: EntityId) -> Vec<u8> {
    id.as_uuid().as_bytes().to_vec()
}

#[tokio::test]
async fn an_edit_to_shared_content_records_exactly_what_changed() {
    let world = World::new().await;
    let id = world.note(&world.one, "before").await;
    let watch = Watch::open(&world, id, EntityType::Note).await;

    world
        .submit(
            &world.one,
            edit_entity(
                id,
                watch.counter_before as u64,
                v1::EntityContent {
                    title: Some("after".into()),
                    bulk: Some(true),
                    ..Default::default()
                },
            ),
        )
        .await;

    let moved = watch.close(&world, "title and bulk").await;
    assert!(moved.contains(&Part::Content(ContentPart::Title)));
    assert!(moved.contains(&Part::Content(ContentPart::Bulk)));
}

/// **A part addressed with the value it already holds did not move.** The write
/// still applies and the counter still advances; what must not happen is the
/// part being claimed.
#[tokio::test]
async fn a_write_addressing_a_part_without_changing_it_records_nothing() {
    let world = World::new().await;
    let id = world.note(&world.one, "unchanged").await;
    let base = counter_of(&world.db.pool, id).await;
    let watch = Watch::open(&world, id, EntityType::Note).await;

    world
        .submit(
            &world.one,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    title: Some("unchanged".into()),
                    ..Default::default()
                },
            ),
        )
        .await;

    watch
        .close_expecting_no_change(&world, "a title rewritten to itself")
        .await;
    assert_eq!(
        counter_of(&world.db.pool, id).await,
        base + 1,
        "the counter must still advance, or no client learns there was a write"
    );
}

/// Clearing is a change, and an empty part is a value like any other.
#[tokio::test]
async fn clearing_a_part_records_it_as_moved() {
    let world = World::new().await;
    let id = world.note(&world.one, "a title").await;
    let watch = Watch::open(&world, id, EntityType::Note).await;

    world
        .submit(
            &world.one,
            edit_entity(
                id,
                watch.counter_before as u64,
                v1::EntityContent {
                    cleared: vec![ContentPart::Title.field_number()],
                    ..Default::default()
                },
            ),
        )
        .await;

    let moved = watch.close(&world, "a cleared title").await;
    assert_eq!(moved, HashSet::from([Part::Content(ContentPart::Title)]));
}

// ---------------------------------------------------------------------------
// Type-specific parts, across three lanes
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_guidance_state_records_exactly_what_changed() {
    let world = World::new().await;
    let id = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let watch = Watch::open(&world, id, EntityType::Campaign).await;

    world
        .submit(
            &world.one,
            edit_entity(
                id,
                watch.counter_before as u64,
                v1::EntityContent {
                    specific: Some(campaign(Some(v1::GuidanceState::Working))),
                    ..Default::default()
                },
            ),
        )
        .await;

    let moved = watch.close(&world, "a campaign state").await;
    assert_eq!(moved, HashSet::from([Part::Specific(1)]));
}

/// **The companion case.** An item's amount and the moment it was asserted move
/// in one statement, and both are parts. Neither may go unrecorded.
#[tokio::test]
async fn an_item_amount_records_itself_and_its_companion() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            item(v1::ItemContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    let watch = Watch::open(&world, id, EntityType::Item).await;

    world
        .submit(
            &world.one,
            edit_entity(
                id,
                watch.counter_before as u64,
                v1::EntityContent {
                    specific: Some(item(v1::ItemContent {
                        asserted_amount: Some(v1::Decimal { units: 3, nanos: 0 }),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            ),
        )
        .await;

    let moved = watch.close(&world, "an item amount").await;
    assert!(moved.contains(&Part::Specific(1)), "the amount");
    assert!(moved.contains(&Part::Specific(3)), "its assertion time");
}

/// A category's own nesting is type content like any other.
#[tokio::test]
async fn a_category_parent_records_exactly_what_changed() {
    let world = World::new().await;
    let outer = world
        .create(
            &world.one,
            v1::entity_content::Specific::Category(v1::CategoryContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    let inner = world
        .create(
            &world.one,
            v1::entity_content::Specific::Category(v1::CategoryContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    let watch = Watch::open(&world, inner, EntityType::Category).await;

    world
        .submit(
            &world.one,
            edit_entity(
                inner,
                watch.counter_before as u64,
                v1::EntityContent {
                    specific: Some(v1::entity_content::Specific::Category(
                        v1::CategoryContent {
                            parent_category_id: Some(id_bytes(outer)),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                },
            ),
        )
        .await;

    let moved = watch.close(&world, "a category parent").await;
    assert_eq!(moved, HashSet::from([Part::Specific(1)]));
}

// ---------------------------------------------------------------------------
// Blocks
// ---------------------------------------------------------------------------

/// **A body is many parts, one per block**, and the invariant is the same one.
/// Editing one paragraph of three must record that block and not its neighbours.
#[tokio::test]
async fn editing_one_block_of_a_body_records_that_block_alone() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            v1::entity_content::Specific::Note(v1::NoteContent {
                body: Some("one paragraph.\n\ntwo paragraph.\n\nthree paragraph.\n".into()),
                cleared: Vec::new(),
            }),
            v1::EntityContent::default(),
        )
        .await;
    let watch = Watch::open(&world, id, EntityType::Note).await;

    world
        .submit(
            &world.one,
            edit_entity(
                id,
                watch.counter_before as u64,
                v1::EntityContent {
                    specific: Some(v1::entity_content::Specific::Note(v1::NoteContent {
                        body: Some(
                            "one paragraph.\n\ntwo paragraph, edited.\n\nthree paragraph.\n".into(),
                        ),
                        cleared: Vec::new(),
                    })),
                    ..Default::default()
                },
            ),
        )
        .await;

    let moved = watch.close(&world, "one block of three").await;
    assert_eq!(
        moved.len(),
        1,
        "only the edited block moved: {:?}",
        listed(&moved)
    );
    assert!(
        matches!(moved.iter().next(), Some(Part::Block(_))),
        "the part that moved is a block, not {:?}",
        listed(&moved)
    );
}

// ---------------------------------------------------------------------------
// The consequential writes — where "moved and unrecorded" actually lived
// ---------------------------------------------------------------------------

/// Entering a container places the entity at the end, and the position the
/// instance assigns is a part the client never named.
#[tokio::test]
async fn a_position_the_instance_assigns_is_recorded_with_the_category() {
    let world = World::new().await;
    let category = world
        .create(
            &world.one,
            v1::entity_content::Specific::Category(v1::CategoryContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    let id = world.note(&world.one, "filed").await;
    let watch = Watch::open(&world, id, EntityType::Note).await;

    world
        .submit(
            &world.one,
            edit_entity(
                id,
                watch.counter_before as u64,
                v1::EntityContent {
                    category_id: Some(id_bytes(category)),
                    ..Default::default()
                },
            ),
        )
        .await;

    let moved = watch.close(&world, "entering a category").await;
    assert!(moved.contains(&Part::Content(ContentPart::Category)));
    assert!(
        moved.contains(&Part::Content(ContentPart::CategoryPosition)),
        "the instance assigned a position and it is a part like any other"
    );
}

/// **The membership release.** Erasing a category moves two parts on every
/// entity filed in it, and the write is to *those* entities rather than to the
/// one the intent named.
#[tokio::test]
async fn erasing_a_category_records_what_it_moved_on_the_entities_it_released() {
    let world = World::new().await;
    let category = world
        .create(
            &world.one,
            v1::entity_content::Specific::Category(v1::CategoryContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    let filed = world
        .create(
            &world.one,
            note(),
            v1::EntityContent {
                title: Some("filed".into()),
                category_id: Some(id_bytes(category)),
                ..Default::default()
            },
        )
        .await;
    let watch = Watch::open(&world, filed, EntityType::Note).await;

    world.submit(&world.one, erase(&[category])).await;

    let moved = watch.close(&world, "erasing the category above it").await;
    assert!(moved.contains(&Part::Content(ContentPart::Category)));
    assert!(moved.contains(&Part::Content(ContentPart::CategoryPosition)));
}

/// **The type-specific release**, on both the things a location holds.
#[tokio::test]
async fn erasing_a_location_records_what_it_moved_on_what_it_held() {
    let world = World::new().await;
    let cupboard = world
        .create(
            &world.one,
            v1::entity_content::Specific::Location(v1::LocationContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    let shelf = world
        .create(
            &world.one,
            v1::entity_content::Specific::Location(v1::LocationContent {
                parent_location_id: Some(id_bytes(cupboard)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
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

    let on_shelf = Watch::open(&world, shelf, EntityType::Location).await;
    let on_tin = Watch::open(&world, tin, EntityType::Item).await;

    world.submit(&world.one, erase(&[cupboard])).await;

    let moved = on_shelf.close(&world, "a child location released").await;
    assert_eq!(moved, HashSet::from([Part::Specific(1)]));
    let moved = on_tin.close(&world, "an item released").await;
    assert_eq!(moved, HashSet::from([Part::Specific(4)]));
}

/// **The upward climb.** Starting a quest moves the state of every container
/// above it, and those are writes to entities the intent never named.
#[tokio::test]
async fn the_upward_state_climb_records_what_it_moved_on_each_container() {
    let world = World::new().await;
    let campaign_id = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let quest = world
        .create(
            &world.one,
            v1::entity_content::Specific::Quest(v1::QuestContent {
                parent: Some(v1::quest_content::Parent::CampaignId(id_bytes(campaign_id))),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;

    let on_campaign = Watch::open(&world, campaign_id, EntityType::Campaign).await;
    let base = counter_of(&world.db.pool, quest).await;

    world
        .submit(
            &world.one,
            edit_entity(
                quest,
                base as u64,
                v1::EntityContent {
                    specific: Some(v1::entity_content::Specific::Quest(v1::QuestContent {
                        state: Some(v1::GuidanceState::Working as i32),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            ),
        )
        .await;

    let moved = on_campaign
        .close(&world, "a campaign moved by the climb")
        .await;
    assert_eq!(
        moved,
        HashSet::from([Part::Specific(1)]),
        "the container's state moved and only its state"
    );
}

// ---------------------------------------------------------------------------
// The member, which is the third face of the same table
// ---------------------------------------------------------------------------

/// **A no-op write must not take ownership.** `member_id` is overwritten by
/// whoever records last, and it crosses the wire in `conflict.theirs_member_id`
/// for a client to render a name.
#[tokio::test]
async fn a_no_op_write_leaves_the_part_owned_by_whoever_changed_it() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            note(),
            v1::EntityContent {
                title: Some("start".into()),
                audience_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;
    let base = counter_of(&world.db.pool, id).await;

    // Two really changes it.
    world
        .submit(
            &world.two,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    title: Some("agreed".into()),
                    ..Default::default()
                },
            ),
        )
        .await;
    // One, stale, submits the value that is already there.
    world
        .submit(
            &world.one,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    title: Some("agreed".into()),
                    ..Default::default()
                },
            ),
        )
        .await;

    assert_eq!(
        owner_of(&world.db.pool, id, &Part::Content(ContentPart::Title)).await,
        Some(world.two.membership_id()),
        "the member who wrote nothing took ownership of the part"
    );
}

/// And the part each write does change is attributed to whoever wrote it, when
/// two members move different parts of one entity.
#[tokio::test]
async fn each_part_is_owned_by_the_member_who_moved_that_part() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            note(),
            v1::EntityContent {
                title: Some("start".into()),
                audience_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;

    let base = counter_of(&world.db.pool, id).await;
    world
        .submit(
            &world.one,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    title: Some("one wrote this".into()),
                    ..Default::default()
                },
            ),
        )
        .await;
    let base = counter_of(&world.db.pool, id).await;
    world
        .submit(
            &world.two,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    bulk: Some(true),
                    ..Default::default()
                },
            ),
        )
        .await;

    assert_eq!(
        owner_of(&world.db.pool, id, &Part::Content(ContentPart::Title)).await,
        Some(world.one.membership_id())
    );
    assert_eq!(
        owner_of(&world.db.pool, id, &Part::Content(ContentPart::Bulk)).await,
        Some(world.two.membership_id())
    );
}

/// A part moved as a consequence is attributed to the member who caused it —
/// the one who erased the container, not the one who filed the entity.
#[tokio::test]
async fn a_released_part_is_owned_by_the_member_who_erased_the_container() {
    let world = World::new().await;
    let category = world
        .create(
            &world.one,
            v1::entity_content::Specific::Category(v1::CategoryContent::default()),
            v1::EntityContent {
                audience_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;
    let filed = world
        .create(
            &world.two,
            note(),
            v1::EntityContent {
                title: Some("theirs".into()),
                category_id: Some(id_bytes(category)),
                audience_member_ids: vec![world.one.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;

    world.submit(&world.one, erase(&[category])).await;

    assert_eq!(
        owner_of(&world.db.pool, filed, &Part::Content(ContentPart::Category)).await,
        Some(world.one.membership_id()),
        "the release is attributed to whoever caused it"
    );
}

/// The remaining shared-model parts, which are all list-shaped and none of which
/// is a plain column.
#[tokio::test]
async fn the_list_shaped_content_parts_record_exactly_what_changed() {
    let world = World::new().await;
    let id = world.note(&world.one, "listy").await;
    let watch = Watch::open(&world, id, EntityType::Note).await;

    world
        .submit(
            &world.one,
            edit_entity(
                id,
                watch.counter_before as u64,
                v1::EntityContent {
                    dates: vec![v1::LabelledDate {
                        label: "due".into(),
                        when: Some(timestamp(chrono::Utc::now())),
                        bring_forward: true,
                    }],
                    assigned_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
                    audience_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
                    ..Default::default()
                },
            ),
        )
        .await;

    let moved = watch.close(&world, "dates, assignments and audience").await;
    assert!(moved.contains(&Part::Content(ContentPart::Dates)));
    assert!(moved.contains(&Part::Content(ContentPart::Assignments)));
    assert!(moved.contains(&Part::Content(ContentPart::Audience)));
}

/// Guidance's ladder: a parent and the position that moves with it.
#[tokio::test]
async fn a_ladder_parent_records_itself_and_the_position_it_moved() {
    let world = World::new().await;
    let campaign_id = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let arc = world
        .create(
            &world.one,
            v1::entity_content::Specific::Arc(v1::ArcContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    let watch = Watch::open(&world, arc, EntityType::Arc).await;

    world
        .submit(
            &world.one,
            edit_entity(
                arc,
                watch.counter_before as u64,
                v1::EntityContent {
                    specific: Some(v1::entity_content::Specific::Arc(v1::ArcContent {
                        campaign_id: Some(id_bytes(campaign_id)),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            ),
        )
        .await;

    let moved = watch.close(&world, "an arc entering a campaign").await;
    assert!(
        moved.contains(&Part::Specific(2)),
        "the campaign it entered"
    );
    assert!(
        moved.contains(&Part::Specific(3)),
        "the ladder position the instance assigned"
    );
}

/// **The first thing this suite found, and it is a real hole rather than a
/// too-strong invariant.**
///
/// A quest moving from an arc to a campaign clears `arc_id` in the same
/// statement that sets `campaign_id` — the row may never hold two. The vacated
/// column is a part like any other and its value changed, but only the addressed
/// parent and the position are handed back as companions, so `arc_id` records
/// nothing. A concurrent edit to the arc then merges silently instead of
/// retaining both values, and *both values retained on a same-part conflict* is
/// a standing constraint.
///
/// Run it and it fails with exactly:
///
/// ```text
///   recorded but unchanged: []
///   changed but unrecorded: ["Specific(2)"]
/// ```
///
/// **Ignored rather than deleted or weakened.** The seam it needs is built — a
/// type may return as many companions as it moved — and filling it in is
/// `specific/guidance.rs`, which this lane does not own. The Guidance note that
/// predicted this also names the trap in fixing it: hand the vacated parent back
/// **only when the sibling column was actually non-null**, or the write records a
/// part that did not move and a later unrelated edit reads it as an overlap.
///
/// That opposite direction is covered by
/// `a_companion_for_an_empty_sibling_is_not_recorded_on_creation`, on the create
/// path — which is the only path where it is observable, because the edit loop
/// suppresses a phantom companion on its own. **An earlier version of this
/// comment claimed the cover came from
/// `a_write_addressing_a_part_without_changing_it_records_nothing`, and that was
/// false**: that test rewrites an unchanged title, a content part, on the doubly
/// guarded path. A cross-model review caught it with a mutation that escaped the
/// whole suite. The claim is recorded here rather than quietly corrected,
/// because a sentence asserting coverage that does not exist is worse than no
/// sentence — it stops the next reader checking.
///
/// **This test found the hole it now guards.** On its first run, against a tree
/// where a quest moving from an arc to a campaign cleared `arc_id` in the same
/// statement and recorded nothing for it, it failed with
/// `changed but unrecorded: ["Specific(2)"]` — naming the part from behaviour.
/// The Guidance lane had predicted the same hole hours earlier by reading the
/// code, without knowing this suite existed. It was `#[ignore]`d rather than
/// weakened while the fix was owed, and nothing about it changed when the fix
/// landed, which is the whole claim of an invariant written before its subject.
#[tokio::test]
async fn a_vacated_ladder_parent_records_that_it_moved() {
    let world = World::new().await;
    let campaign_id = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let arc = world
        .create(
            &world.one,
            v1::entity_content::Specific::Arc(v1::ArcContent {
                campaign_id: Some(id_bytes(campaign_id)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let quest = world
        .create(
            &world.one,
            v1::entity_content::Specific::Quest(v1::QuestContent {
                parent: Some(v1::quest_content::Parent::ArcId(id_bytes(arc))),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    let watch = Watch::open(&world, quest, EntityType::Quest).await;

    world
        .submit(
            &world.one,
            edit_entity(
                quest,
                watch.counter_before as u64,
                v1::EntityContent {
                    specific: Some(v1::entity_content::Specific::Quest(v1::QuestContent {
                        parent: Some(v1::quest_content::Parent::CampaignId(id_bytes(campaign_id))),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            ),
        )
        .await;

    watch
        .close(&world, "a quest moving from an arc to a campaign")
        .await;
}

/// **The other direction, on a companion: a part handed back that did not move.**
///
/// A quest with no parent at all gains an arc. `campaign_id` was already null
/// and stays null, so it did not move and must not be recorded. A release or a
/// placement that handed back the sibling unconditionally — clearing a column
/// that was already clear — would write an `entity_part_counter` row claiming
/// that part moved, and a later unrelated edit to the campaign would read it as
/// an overlap. Nothing about the stored row would look wrong.
///
/// **On this path the invariant is protected twice**, and that is worth knowing
/// rather than discovering. `write::entity`'s edit loop records a part only when
/// its rendered value changed, so a phantom companion is suppressed here even if
/// a type hands one back. This test therefore asserts a true thing that a
/// mispaired companion alone cannot break — the create path is where that bites,
/// and `a_companion_for_an_empty_sibling_is_not_recorded_on_creation` is the one
/// that catches it.
#[tokio::test]
async fn a_companion_for_a_sibling_that_was_already_empty_is_not_recorded() {
    let world = World::new().await;
    let campaign_id = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let arc = world
        .create(
            &world.one,
            v1::entity_content::Specific::Arc(v1::ArcContent {
                campaign_id: Some(id_bytes(campaign_id)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;
    // A quest hanging from nothing: both ladder parents are null.
    let quest = world
        .create(
            &world.one,
            v1::entity_content::Specific::Quest(v1::QuestContent::default()),
            v1::EntityContent::default(),
        )
        .await;
    let watch = Watch::open(&world, quest, EntityType::Quest).await;

    world
        .submit(
            &world.one,
            edit_entity(
                quest,
                watch.counter_before as u64,
                v1::EntityContent {
                    specific: Some(v1::entity_content::Specific::Quest(v1::QuestContent {
                        parent: Some(v1::quest_content::Parent::ArcId(id_bytes(arc))),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            ),
        )
        .await;

    let moved = watch
        .close(&world, "a parentless quest entering an arc")
        .await;
    assert!(moved.contains(&Part::Specific(2)), "the arc it entered");
    assert!(moved.contains(&Part::Specific(5)), "the ladder position");
    // `close` already asserts recorded == moved, so the campaign being absent
    // from `moved` is what makes a phantom record fail. Stated too, because this
    // is the whole point of the test rather than a detail of it.
    assert!(
        !moved.contains(&Part::Specific(3)),
        "the campaign was already empty and did not move: {:?}",
        listed(&moved)
    );
}

/// **The same mistake on creation, which is where it is observable.**
///
/// `write::entity`'s *edit* loop records a part only when its rendered value
/// changed, so a companion handed back for a sibling that did not move is
/// suppressed there. Its *create* loop records every part it applies
/// unconditionally — correctly, since on a create there is no prior value to
/// compare against — so on that path **a type handing back a companion it did
/// not move is the only thing standing between the store and a phantom
/// `entity_part_counter` row**.
///
/// A quest created straight into an arc has never had a campaign. If the ladder
/// handed back `campaign_id` regardless, the create would record `specific.3` as
/// having moved, and a later unrelated edit to the campaign would read that as an
/// overlap and retain two values for a part nobody touched twice.
///
/// # This is the mutation the cross-model review escaped
///
/// Dropping the `Some(_)` from the sibling guard in `guidance::parent_placement`
/// makes the companion unconditional. Verified against this test: with the guard
/// the create records `["Specific(2)", "Specific(5)"]`, and without it
/// `["Specific(2)", "Specific(3)", "Specific(5)"]`.
///
/// It escaped because the only test asserting this direction did it on a
/// **content** part — a title rewritten to itself, on the edit path, where the
/// second guard makes it unfalsifiable for this fault. *The invariant is checked
/// somewhere* is not the same claim as *the invariant is checked here*, and the
/// suite exists to make it one claim.
#[tokio::test]
async fn a_companion_for_an_empty_sibling_is_not_recorded_on_creation() {
    let world = World::new().await;
    let campaign_id = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let arc = world
        .create(
            &world.one,
            v1::entity_content::Specific::Arc(v1::ArcContent {
                campaign_id: Some(id_bytes(campaign_id)),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;

    // Created straight into an arc: `campaign_id` is null and stays null.
    let quest = world
        .create(
            &world.one,
            v1::entity_content::Specific::Quest(v1::QuestContent {
                parent: Some(v1::quest_content::Parent::ArcId(id_bytes(arc))),
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;

    let counter = counter_of(&world.db.pool, quest).await;
    let recorded = recorded_at(&world.db.pool, quest, counter).await;

    assert!(
        !recorded.contains(&Part::Specific(3)),
        "the quest never had a campaign, so nothing moved it: recorded {:?}",
        listed(&recorded)
    );
    // And the parts that did move are still there, so this cannot pass by the
    // create having recorded nothing at all.
    assert!(recorded.contains(&Part::Specific(2)), "the arc it entered");
    assert!(recorded.contains(&Part::Specific(5)), "the ladder position");
}

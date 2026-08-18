//! Knowledge's type content, and categories.
//!
//! The lane's name oversells it and the suite is arranged to say so. A note has
//! no fields of its own — migration one refuses it a table on purpose — and a
//! file has one field this wave can serve, its media type. **The substance is
//! categories**, and inside categories it is the creation-default audience,
//! which is three separate claims that look like one sentence:
//!
//! - it acts **once, at creation**, so a later move into the category applies
//!   nothing;
//! - it is **not a standing rule**, so editing it reaches nothing already
//!   placed;
//! - it is **never inherited**, so a child category has only its own.
//!
//! Each has a test, because each could break without the other two noticing.
//! The whole point of the narrowness is that a category never sets an audience
//! as a standing rule, and a default that quietly became one would be a leak
//! rather than a bug.
//!
//! The second half is cycle prevention in nested categories, which migration
//! one records as its gap (h) and cannot close: a constraint sees one row. The
//! interesting cases are two hops and longer; self-reference passes for free
//! and proves nothing on its own.

mod common;

use altair_proto::v1;
use altaird::store::entity::EntityType;
use altaird::store::ids::EntityId;
use altaird::write::specific::{self, Held};
use common::*;
use sqlx::Row;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Saying a category
// ---------------------------------------------------------------------------

fn category_content(
    parent: Option<EntityId>,
    default: &[Uuid],
    cleared: Vec<u32>,
) -> v1::entity_content::Specific {
    v1::entity_content::Specific::Category(v1::CategoryContent {
        parent_category_id: parent.map(|p| p.as_uuid().as_bytes().to_vec()),
        creation_default_audience_member_ids: default
            .iter()
            .map(|m| m.as_bytes().to_vec())
            .collect(),
        cleared,
    })
}

/// A plain category, visible to its author alone.
async fn category(world: &World, parent: Option<EntityId>, default: &[Uuid]) -> EntityId {
    world
        .create(
            &world.one,
            category_content(parent, default, Vec::new()),
            v1::EntityContent::default(),
        )
        .await
}

/// A category both members can see, which is what makes a concurrent edit to
/// one possible at all.
async fn shared_category(world: &World, default: &[Uuid]) -> EntityId {
    world
        .create(
            &world.one,
            category_content(None, default, Vec::new()),
            v1::EntityContent {
                audience_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await
}

/// Edit a category's own content, from the counter it currently holds.
async fn edit_category(
    world: &World,
    id: EntityId,
    specific: v1::entity_content::Specific,
) -> v1::Acknowledgement {
    let base = world.counter(id).await as u64;
    world
        .submit(
            &world.one,
            edit_entity(
                id,
                base,
                v1::EntityContent {
                    specific: Some(specific),
                    ..Default::default()
                },
            ),
        )
        .await
}

// ---------------------------------------------------------------------------
// Reading the store back
// ---------------------------------------------------------------------------

async fn parent_of(world: &World, id: EntityId) -> Option<Uuid> {
    sqlx::query("SELECT parent_category_id AS p FROM category WHERE entity_id = $1")
        .bind(id.as_uuid())
        .fetch_one(&world.db.pool)
        .await
        .expect("category row")
        .try_get("p")
        .expect("parent")
}

async fn creation_default_of(world: &World, id: EntityId) -> Vec<Uuid> {
    sqlx::query("SELECT creation_default_audience AS a FROM category WHERE entity_id = $1")
        .bind(id.as_uuid())
        .fetch_one(&world.db.pool)
        .await
        .expect("category row")
        .try_get("a")
        .expect("default")
}

/// The audience an entity actually ended up with.
///
/// A test may read this column directly. `tests/one_predicate.rs` exempts test
/// sources from the column check deliberately: an assertion that both paths
/// call one predicate has to be able to see past it, and ground truth that came
/// through the audience-scoped surface would be checking the predicate against
/// itself.
async fn audience_of(world: &World, id: EntityId) -> Vec<Uuid> {
    sqlx::query("SELECT audience_member_ids AS a FROM entity WHERE id = $1")
        .bind(id.as_uuid())
        .fetch_one(&world.db.pool)
        .await
        .expect("entity")
        .try_get("a")
        .expect("audience")
}

/// A note created inside a category, which is the thing a creation default acts
/// on.
async fn note_in(world: &World, category: EntityId) -> EntityId {
    world
        .create(
            &world.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some("placed".into()),
                category_id: Some(category.as_uuid().as_bytes().to_vec()),
                ..Default::default()
            },
        )
        .await
}

// ---------------------------------------------------------------------------
// A category's two fields, through create and edit
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_categorys_nesting_and_creation_default_round_trip_through_a_create() {
    let world = World::new().await;
    let top = category(&world, None, &[]).await;
    let two = world.two.membership_id();

    let nested = category(&world, Some(top), &[two]).await;

    assert_eq!(parent_of(&world, nested).await, Some(top.as_uuid()));
    assert_eq!(creation_default_of(&world, nested).await, vec![two]);
    assert_eq!(
        parent_of(&world, top).await,
        None,
        "the top nests in nothing"
    );
    assert!(creation_default_of(&world, top).await.is_empty());
}

#[tokio::test]
async fn an_edit_moves_both_fields_and_advances_the_counter() {
    let world = World::new().await;
    let first = category(&world, None, &[]).await;
    let second = category(&world, None, &[]).await;
    let moving = category(&world, Some(first), &[]).await;
    let base = world.counter(moving).await;

    let ack = edit_category(
        &world,
        moving,
        category_content(Some(second), &[world.one.membership_id()], Vec::new()),
    )
    .await;

    assert_eq!(applied(&ack).counter, (base + 1) as u64);
    assert_eq!(parent_of(&world, moving).await, Some(second.as_uuid()));
    assert_eq!(
        creation_default_of(&world, moving).await,
        vec![world.one.membership_id()]
    );
}

#[tokio::test]
async fn clearing_a_parent_returns_a_category_to_the_top() {
    let world = World::new().await;
    let top = category(&world, None, &[]).await;
    let nested = category(&world, Some(top), &[]).await;

    edit_category(&world, nested, category_content(None, &[], vec![1])).await;

    assert_eq!(parent_of(&world, nested).await, None);
}

/// **Emptying is how a repeated field is cleared**, and the column is
/// `NOT NULL DEFAULT '{}'`, so a category that never had a default and one
/// whose default was emptied are the same category rather than two states.
#[tokio::test]
async fn emptying_a_creation_default_clears_it() {
    let world = World::new().await;
    let id = category(&world, None, &[world.two.membership_id()]).await;

    edit_category(&world, id, category_content(None, &[], vec![2])).await;

    assert!(creation_default_of(&world, id).await.is_empty());
}

/// `entity_part_counter` is per part and conflict detection asks which parts
/// moved between two counter values. A part that does not record its movement
/// is invisible to that, and the failure is silent.
#[tokio::test]
async fn both_category_parts_record_the_counter_they_moved_at() {
    let world = World::new().await;
    let top = category(&world, None, &[]).await;
    let id = category(&world, Some(top), &[world.two.membership_id()]).await;

    let rows = sqlx::query(
        "SELECT field_name, counter FROM entity_part_counter \
         WHERE entity_id = $1 ORDER BY field_name",
    )
    .bind(id.as_uuid())
    .fetch_all(&world.db.pool)
    .await
    .expect("query");

    let named: Vec<String> = rows
        .iter()
        .map(|r| r.try_get::<String, _>("field_name").unwrap())
        .collect();
    assert!(
        named.contains(&"specific.1".to_owned()) && named.contains(&"specific.2".to_owned()),
        "both parts record their movement, or nothing can conflict on them: {named:?}"
    );
}

/// Both values retained on a same-part conflict, and the acknowledgement names
/// the type message's own field number.
#[tokio::test]
async fn a_same_part_conflict_on_a_creation_default_retains_both_values() {
    let world = World::new().await;
    let id = shared_category(&world, &[]).await;
    let base = world.counter(id).await;

    let one = world.one.membership_id();
    let two = world.two.membership_id();

    for (member, stated) in [(&world.one, one), (&world.two, two)] {
        world
            .submit(
                member,
                edit_entity(
                    id,
                    base as u64,
                    v1::EntityContent {
                        specific: Some(category_content(None, &[stated], Vec::new())),
                        ..Default::default()
                    },
                ),
            )
            .await;
    }

    let conflicts = world.conflicts(id).await;
    assert_eq!(conflicts.len(), 1);
    let c = &conflicts[0];
    assert_eq!(c.field.as_deref(), Some("specific.2"));
    assert_eq!(c.mine.as_deref(), Some(two.to_string().as_str()));
    assert_eq!(c.theirs.as_deref(), Some(one.to_string().as_str()));
    assert_eq!(creation_default_of(&world, id).await, vec![two]);
}

/// **A set has no order and the wire carries it as a list.** Two clients naming
/// the same two members in different orders agree, and forming a conflict out
/// of agreement is the machinery working against its own purpose.
#[tokio::test]
async fn the_same_two_members_in_either_order_are_the_same_default() {
    let world = World::new().await;
    let id = shared_category(&world, &[]).await;
    let base = world.counter(id).await;
    let one = world.one.membership_id();
    let two = world.two.membership_id();

    for (member, stated) in [(&world.one, [one, two]), (&world.two, [two, one])] {
        world
            .submit(
                member,
                edit_entity(
                    id,
                    base as u64,
                    v1::EntityContent {
                        specific: Some(category_content(None, &stated, Vec::new())),
                        ..Default::default()
                    },
                ),
            )
            .await;
    }

    assert!(
        world.conflicts(id).await.is_empty(),
        "writes producing the same value are not divergent"
    );
}

// ---------------------------------------------------------------------------
// The creation default: three claims, three tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn an_entity_created_in_a_category_takes_its_creation_default() {
    let world = World::new().await;
    let two = world.two.membership_id();
    let holder = category(&world, None, &[two]).await;

    let placed = note_in(&world, holder).await;

    assert_eq!(audience_of(&world, placed).await, vec![two]);
}

/// **It acts at creation only.** Moving something into the category afterwards
/// is an edit, and an edit never asks the category what it defaults — otherwise
/// tidying would broaden an audience, silently, on somebody else's entity.
#[tokio::test]
async fn moving_an_existing_entity_into_a_category_applies_nothing() {
    let world = World::new().await;
    let holder = category(&world, None, &[world.two.membership_id()]).await;
    let loose = world.note(&world.one, "captured outside").await;
    assert!(audience_of(&world, loose).await.is_empty());

    let base = world.counter(loose).await;
    let ack = world
        .submit(
            &world.one,
            edit_entity(
                loose,
                base as u64,
                v1::EntityContent {
                    category_id: Some(holder.as_uuid().as_bytes().to_vec()),
                    ..Default::default()
                },
            ),
        )
        .await;
    assert!(matches!(
        ack.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));

    assert!(
        audience_of(&world, loose).await.is_empty(),
        "the default acts once, at creation; a later move is not a creation"
    );
}

/// **It is not a standing rule.** The value is copied at creation and never
/// consulted again, so editing it reaches nothing already placed. A category
/// that broadened its contents retroactively would be a category setting an
/// audience, which the substrate says it never does.
#[tokio::test]
async fn editing_a_creation_default_does_not_reach_entities_already_placed() {
    let world = World::new().await;
    let holder = category(&world, None, &[]).await;
    let placed = note_in(&world, holder).await;
    assert!(audience_of(&world, placed).await.is_empty());

    edit_category(
        &world,
        holder,
        category_content(None, &[world.two.membership_id()], Vec::new()),
    )
    .await;

    assert!(
        audience_of(&world, placed).await.is_empty(),
        "a default is not a standing rule; it does not reach backwards"
    );
}

/// **It is never inherited.** A child with no default of its own has none, and
/// the read walks no ancestry to find one.
#[tokio::test]
async fn a_child_category_does_not_inherit_its_parents_creation_default() {
    let world = World::new().await;
    let parent = category(&world, None, &[world.two.membership_id()]).await;
    let child = category(&world, Some(parent), &[]).await;

    let placed = note_in(&world, child).await;

    assert!(
        audience_of(&world, placed).await.is_empty(),
        "a default is never inherited, at any depth"
    );
}

/// An audience the write states outranks the default. Clearing it is stating
/// it: the wire says empty and not cleared means untouched, and cleared means
/// private to the author.
#[tokio::test]
async fn an_audience_the_write_states_outranks_the_default() {
    let world = World::new().await;
    let holder = category(&world, None, &[world.two.membership_id()]).await;

    let placed = world
        .create(
            &world.one,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                category_id: Some(holder.as_uuid().as_bytes().to_vec()),
                cleared: vec![5],
                ..Default::default()
            },
        )
        .await;

    assert!(audience_of(&world, placed).await.is_empty());
}

/// **A foreign key cannot reach inside an array**, which is why this is a
/// write-path check and is on the schema's owed list. A member id naming no
/// membership is refused rather than stored and discovered later by whatever
/// tries to resolve it.
#[tokio::test]
async fn a_creation_default_naming_no_membership_is_refused() {
    let world = World::new().await;
    let ack = world
        .submit(
            &world.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    specific: Some(category_content(None, &[Uuid::new_v4()], Vec::new())),
                    ..Default::default()
                },
            ),
        )
        .await;

    assert!(is_not_available(&ack), "{:?}", ack.outcome);
}

// ---------------------------------------------------------------------------
// Cycle prevention, where the schema cannot see it
// ---------------------------------------------------------------------------

/// The refusal a closed loop gets, or a panic naming what came back instead.
async fn nesting_refusal(world: &World, child: EntityId, parent: EntityId) -> String {
    let ack = edit_category(
        world,
        child,
        category_content(Some(parent), &[], Vec::new()),
    )
    .await;
    let refused = refused(&ack);
    assert_eq!(
        refused.reason,
        v1::RefusalReason::Malformed as i32,
        "{refused:?}"
    );
    refused.detail.clone()
}

#[tokio::test]
async fn a_category_cannot_be_its_own_parent() {
    let world = World::new().await;
    let a = category(&world, None, &[]).await;

    let detail = nesting_refusal(&world, a, a).await;
    assert!(detail.contains("close a loop"), "{detail}");
    assert_eq!(parent_of(&world, a).await, None);
}

/// Two hops is the first case the schema cannot see. Its only check is
/// `parent_category_id <> entity_id`, which a loop of length two satisfies at
/// every row.
#[tokio::test]
async fn a_two_hop_loop_is_refused() {
    let world = World::new().await;
    let a = category(&world, None, &[]).await;
    let b = category(&world, Some(a), &[]).await;

    let detail = nesting_refusal(&world, a, b).await;
    assert!(detail.contains("close a loop"), "{detail}");
    assert_eq!(parent_of(&world, a).await, None, "and nothing was written");
}

#[tokio::test]
async fn a_longer_loop_is_refused_too() {
    let world = World::new().await;
    let a = category(&world, None, &[]).await;
    let b = category(&world, Some(a), &[]).await;
    let c = category(&world, Some(b), &[]).await;

    nesting_refusal(&world, a, c).await;
    assert_eq!(parent_of(&world, a).await, None);
    // And the chain that was legal is untouched.
    assert_eq!(parent_of(&world, b).await, Some(a.as_uuid()));
    assert_eq!(parent_of(&world, c).await, Some(b.as_uuid()));
}

#[tokio::test]
async fn an_ordinary_nesting_is_accepted() {
    let world = World::new().await;
    let a = category(&world, None, &[]).await;
    let b = category(&world, Some(a), &[]).await;
    let c = category(&world, None, &[]).await;

    edit_category(&world, c, category_content(Some(b), &[], Vec::new())).await;

    assert_eq!(parent_of(&world, c).await, Some(b.as_uuid()));
}

// ---------------------------------------------------------------------------
// A parent that is not there, one that cannot be seen, and one of the wrong
// type are the same nothing
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_parent_that_is_not_there_is_refused_as_unavailable() {
    let world = World::new().await;
    let ack = world
        .submit(
            &world.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    specific: Some(category_content(
                        Some(EntityId::from_uuid(Uuid::new_v4())),
                        &[],
                        Vec::new(),
                    )),
                    ..Default::default()
                },
            ),
        )
        .await;
    assert!(is_not_available(&ack), "{:?}", ack.outcome);
}

/// **Refusal on audience is indistinguishable from refusal on nonexistence.**
/// Member two's private category and a category that was never created answer
/// with the same nothing, or the refusal is an oracle for what the household
/// holds.
#[tokio::test]
async fn a_parent_the_member_cannot_see_answers_exactly_as_one_that_is_not_there() {
    let world = World::new().await;
    // Two's own category, which one cannot see: audience is private to the
    // author by default.
    let hidden = world
        .create(
            &world.two,
            category_content(None, &[], Vec::new()),
            v1::EntityContent::default(),
        )
        .await;

    let mine = category(&world, None, &[]).await;

    let hidden_ack = edit_category(
        &world,
        mine,
        category_content(Some(hidden), &[], Vec::new()),
    )
    .await;
    let absent_ack = edit_category(
        &world,
        mine,
        category_content(Some(EntityId::from_uuid(Uuid::new_v4())), &[], Vec::new()),
    )
    .await;

    // Both refuse as unavailable, and then the stronger claim: the two answers
    // are the same answer. Equality alone would pass if both had drifted to
    // some other single reason, so the reason is pinned first.
    assert!(is_not_available(&hidden_ack), "{:?}", hidden_ack.outcome);
    assert!(is_not_available(&absent_ack), "{:?}", absent_ack.outcome);
    assert_eq!(
        format!("{:?}", hidden_ack.outcome),
        format!("{:?}", absent_ack.outcome),
        "a category the member cannot see and one that is not there are the same nothing, \
         down to the detail; anything else is an oracle for what the household holds"
    );
    assert_eq!(parent_of(&world, mine).await, None);
}

/// A parent that exists and is not a category is the same nothing again. The
/// schema's composite foreign key would refuse it too, but as a constraint
/// violation that takes the transaction down and says which entity is there.
#[tokio::test]
async fn a_parent_that_is_not_a_category_is_the_same_nothing() {
    let world = World::new().await;
    let mine = category(&world, None, &[]).await;
    let note = world.note(&world.one, "not a container").await;

    let ack = edit_category(&world, mine, category_content(Some(note), &[], Vec::new())).await;

    assert!(is_not_available(&ack), "{:?}", ack.outcome);
    assert_eq!(parent_of(&world, mine).await, None);
}

/// 2.1 refuses an explicit position pending whatever first reorders, and
/// nothing in this lane reorders. Nested categories carry no position at all —
/// migration one's gap (g) — so a category gaining a parent gains nothing to
/// order it by.
#[tokio::test]
async fn an_explicit_position_is_still_refused() {
    let world = World::new().await;
    let holder = category(&world, None, &[]).await;
    let ack = world
        .submit(
            &world.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    specific: Some(v1::entity_content::Specific::Note(
                        v1::NoteContent::default(),
                    )),
                    category_id: Some(holder.as_uuid().as_bytes().to_vec()),
                    category_position: Some(0),
                    ..Default::default()
                },
            ),
        )
        .await;
    let refused = refused(&ack);
    assert_eq!(refused.reason, v1::RefusalReason::Malformed as i32);
    assert!(refused.detail.contains("position"), "{}", refused.detail);
}

// ---------------------------------------------------------------------------
// A note has no fields of its own
// ---------------------------------------------------------------------------

/// Migration one: *"There is no note table. A note holds a body beyond the
/// shared set, a body is its blocks in order, and there is no second
/// representation of the same content for them to disagree with."* So the
/// declaration is one field, it is a body, and it is not a column.
#[test]
fn a_notes_only_field_is_its_body_and_it_has_no_table() {
    let fields = specific::fields(EntityType::Note);
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].number, 1);
    assert!(matches!(fields[0].held, Held::Body));
    assert!(
        specific::side_table(EntityType::Note).is_none(),
        "a note's content is its body; a second representation is what would disagree"
    );
}

/// **Nothing in this lane writes a block.** Bodies are their own lane and their
/// own module; a note created without one holds none, and neither a category
/// nor a file goes anywhere near the table.
#[tokio::test]
async fn nothing_in_this_lane_writes_a_block() {
    let world = World::new().await;
    let holder = category(&world, None, &[world.two.membership_id()]).await;
    let placed = note_in(&world, holder).await;

    for id in [holder, placed] {
        assert_eq!(world.rows_for("block", "entity_id", id).await, 0);
    }
}

// ---------------------------------------------------------------------------
// A file, which is one served field and two refusals that expire
// ---------------------------------------------------------------------------

/// A file entity, made by hand.
///
/// **Scaffolding standing in for Wave 2.3.** A file cannot be created through
/// the public path — the create refuses one, because the schema requires a body
/// and `PutBody` is not served — so the only way to have a file to edit is to
/// put one there. The rows are the minimum the schema demands; 2.3 replaces
/// this with an upload.
async fn a_file(world: &World) -> EntityId {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO entity (id, type, author_member_id, created_at, updated_at, capture_method) \
         VALUES ($1, 'file', $2, now(), now(), 'test')",
    )
    .bind(id)
    .bind(world.one.membership_id())
    .execute(&world.db.pool)
    .await
    .expect("entity");
    sqlx::query("INSERT INTO file (entity_id, body_id) VALUES ($1, $2)")
        .bind(id)
        .bind(Uuid::new_v4())
        .execute(&world.db.pool)
        .await
        .expect("file");
    EntityId::from_uuid(id)
}

async fn media_type_of(world: &World, id: EntityId) -> Option<String> {
    sqlx::query("SELECT media_type AS m FROM file WHERE entity_id = $1")
        .bind(id.as_uuid())
        .fetch_one(&world.db.pool)
        .await
        .expect("file row")
        .try_get("m")
        .expect("media type")
}

fn file_content(media_type: Option<&str>, cleared: Vec<u32>) -> v1::entity_content::Specific {
    v1::entity_content::Specific::File(v1::FileContent {
        media_type: media_type.map(ToOwned::to_owned),
        cleared,
        ..Default::default()
    })
}

async fn edit_file(
    world: &World,
    id: EntityId,
    specific: v1::entity_content::Specific,
) -> v1::Acknowledgement {
    let base = world.counter(id).await as u64;
    world
        .submit(
            &world.one,
            edit_entity(
                id,
                base,
                v1::EntityContent {
                    specific: Some(specific),
                    ..Default::default()
                },
            ),
        )
        .await
}

/// The whole of a v0 file's own content. The scope's v0 files are entities with
/// a title and relations; a media type is what display follows.
#[tokio::test]
async fn a_files_media_type_round_trips() {
    let world = World::new().await;
    let id = a_file(&world).await;
    assert_eq!(media_type_of(&world, id).await, None);

    let ack = edit_file(&world, id, file_content(Some("image/png"), Vec::new())).await;
    assert!(matches!(
        ack.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));
    assert_eq!(
        media_type_of(&world, id).await.as_deref(),
        Some("image/png")
    );

    edit_file(&world, id, file_content(None, vec![2])).await;
    assert_eq!(media_type_of(&world, id).await, None, "and it clears");
}

/// A media type is a machine label rather than the person's words, so it stays
/// out of the text a literal search matches. The title still reaches it.
#[tokio::test]
async fn a_media_type_does_not_reach_search_text() {
    let world = World::new().await;
    let id = a_file(&world).await;
    let base = world.counter(id).await as u64;
    world
        .submit(
            &world.one,
            edit_entity(
                id,
                base,
                v1::EntityContent {
                    title: Some("the receipt".into()),
                    specific: Some(file_content(Some("image/png"), Vec::new())),
                    ..Default::default()
                },
            ),
        )
        .await;

    let text: String = sqlx::query("SELECT search_text FROM entity WHERE id = $1")
        .bind(id.as_uuid())
        .fetch_one(&world.db.pool)
        .await
        .expect("entity")
        .try_get("search_text")
        .expect("search_text");
    assert_eq!(text, "the receipt");
}

/// **Bytes before the record.** There is no identity to name until `PutBody`
/// is served, and the refusal says which lane a client is waiting on rather
/// than that its message was wrong.
#[tokio::test]
async fn a_files_body_id_is_refused_because_put_body_is_not_served() {
    let world = World::new().await;
    let id = a_file(&world).await;
    let ack = edit_file(
        &world,
        id,
        v1::entity_content::Specific::File(v1::FileContent {
            body_id: Some(Uuid::new_v4().as_bytes().to_vec()),
            ..Default::default()
        }),
    )
    .await;
    let refused = refused(&ack);
    assert_eq!(refused.reason, v1::RefusalReason::Malformed as i32);
    assert!(refused.detail.contains("PutBody"), "{}", refused.detail);
}

/// `altair-v0-scope.md` defers text extraction from files, and it governs the
/// implementation plan where the two disagree. A person's correction to
/// extracted text has nothing to outrank while extraction does not exist.
#[tokio::test]
async fn a_files_extracted_text_is_refused_because_extraction_is_deferred() {
    let world = World::new().await;
    let id = a_file(&world).await;
    let ack = edit_file(
        &world,
        id,
        v1::entity_content::Specific::File(v1::FileContent {
            extracted_text: Some("what the picture says".into()),
            ..Default::default()
        }),
    )
    .await;
    let refused = refused(&ack);
    assert_eq!(refused.reason, v1::RefusalReason::Malformed as i32);
    assert!(
        refused.detail.contains("deferred in v0"),
        "{}",
        refused.detail
    );
}

/// And the create still refuses, which is 2.1's refusal and 2.3's to expire.
/// Serving `media_type` does not make a file creatable, and quietly making one
/// would put a record in front of its bytes.
#[tokio::test]
async fn a_file_still_cannot_be_created() {
    let world = World::new().await;
    let ack = world
        .submit(
            &world.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    specific: Some(file_content(Some("image/png"), Vec::new())),
                    ..Default::default()
                },
            ),
        )
        .await;
    let refused = refused(&ack);
    assert_eq!(refused.reason, v1::RefusalReason::Malformed as i32);
    assert!(refused.detail.contains("PutBody"), "{}", refused.detail);
}

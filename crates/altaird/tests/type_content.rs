//! The type-content spine: one type carried end to end, and the boundary
//! everything else hangs off.
//!
//! Wave 2.2's shared item builds the seam, not the domains. So this suite has
//! two halves, and the second is the more important one:
//!
//! - **Campaign's `state`, end to end.** One field, no ordering, no parent. It
//!   is written, read back, conflicted on, and merged, which is what proves a
//!   type-specific part reaches every mechanism the shared model already
//!   reaches. **Not** the Guidance state machine: no transition is checked
//!   anywhere, deliberately, and that is the Guidance lane's.
//! - **Everything unfilled refuses distinguishably.** A stub that accepted
//!   silently would tell a client its content had landed when nothing had, and
//!   the lane filling the module in would have no failing test to turn green.

mod common;

use altair_proto::v1;
use altaird::store::begin_write;
use altaird::store::ids::EntityId;
use altaird::write::specific::{self, Decimal, Held};
use common::*;
use sqlx::Row;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Campaign's state, end to end
// ---------------------------------------------------------------------------

fn campaign(state: Option<v1::GuidanceState>) -> v1::entity_content::Specific {
    v1::entity_content::Specific::Campaign(v1::CampaignContent {
        state: state.map(|s| s as i32),
        cleared: Vec::new(),
    })
}

fn campaign_cleared(cleared: Vec<u32>) -> v1::entity_content::Specific {
    v1::entity_content::Specific::Campaign(v1::CampaignContent {
        state: None,
        cleared,
    })
}

async fn state_of(world: &World, id: EntityId) -> String {
    sqlx::query("SELECT state::text AS s FROM campaign WHERE entity_id = $1")
        .bind(id.as_uuid())
        .fetch_one(&world.db.pool)
        .await
        .expect("campaign row")
        .try_get("s")
        .expect("state")
}

/// A campaign member two can also see, which is what makes a concurrent edit
/// possible at all. Audience is private to the author otherwise.
async fn shared_campaign(world: &World, state: Option<v1::GuidanceState>) -> EntityId {
    world
        .create(
            &world.one,
            campaign(state),
            v1::EntityContent {
                audience_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await
}

#[tokio::test]
async fn a_state_stated_at_creation_reaches_the_store() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            campaign(Some(v1::GuidanceState::Working)),
            v1::EntityContent {
                title: Some("the kitchen".into()),
                ..Default::default()
            },
        )
        .await;
    assert_eq!(state_of(&world, id).await, "working");
}

#[tokio::test]
async fn a_state_not_stated_at_creation_is_the_column_default() {
    let world = World::new().await;
    let id = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    assert_eq!(state_of(&world, id).await, "waiting");
}

#[tokio::test]
async fn an_edit_moves_a_state_and_advances_the_counter() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            campaign(Some(v1::GuidanceState::Working)),
            v1::EntityContent::default(),
        )
        .await;
    let base = world.counter(id).await;

    let ack = world
        .submit(
            &world.one,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    specific: Some(campaign(Some(v1::GuidanceState::Worked))),
                    ..Default::default()
                },
            ),
        )
        .await;

    assert_eq!(state_of(&world, id).await, "worked");
    assert_eq!(applied(&ack).counter, (base + 1) as u64);
}

/// **Clearing a state sets it to waiting**, because the column is
/// `NOT NULL DEFAULT 'waiting'` and there is no state a campaign is not in.
/// That is the plumbing's answer rather than the domain's, and it is what makes
/// the next test true.
#[tokio::test]
async fn clearing_a_state_returns_it_to_the_default() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            campaign(Some(v1::GuidanceState::Worked)),
            v1::EntityContent::default(),
        )
        .await;
    let base = world.counter(id).await;

    world
        .submit(
            &world.one,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    specific: Some(campaign_cleared(vec![1])),
                    ..Default::default()
                },
            ),
        )
        .await;

    assert_eq!(state_of(&world, id).await, "waiting");
}

/// The part is recorded as having moved, keyed the way the wire names it.
///
/// `entity_part_counter` is per part and conflict detection asks which parts
/// moved between two counter values. A specific part that does not record its
/// movement is invisible to that, and the failure is silent.
#[tokio::test]
async fn a_state_records_the_counter_it_moved_at() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            campaign(Some(v1::GuidanceState::Working)),
            v1::EntityContent::default(),
        )
        .await;

    let row = sqlx::query(
        "SELECT field_name, counter FROM entity_part_counter \
         WHERE entity_id = $1 AND field_name = 'specific.1'",
    )
    .bind(id.as_uuid())
    .fetch_optional(&world.db.pool)
    .await
    .expect("query");

    let row = row.expect("the state's movement is recorded, or nothing can conflict on it");
    assert_eq!(row.try_get::<i64, _>("counter").unwrap(), 1);
}

/// Same part, both values retained, and the wire says which field.
#[tokio::test]
async fn a_same_part_conflict_on_a_state_retains_both_values() {
    let world = World::new().await;
    let id = shared_campaign(&world, Some(v1::GuidanceState::Waiting)).await;
    let base = world.counter(id).await;

    // One moves it. Two is still on the base it last saw.
    world
        .submit(
            &world.one,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    specific: Some(campaign(Some(v1::GuidanceState::Working))),
                    ..Default::default()
                },
            ),
        )
        .await;
    let ack = world
        .submit(
            &world.two,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    specific: Some(campaign(Some(v1::GuidanceState::Worked))),
                    ..Default::default()
                },
            ),
        )
        .await;

    let applied = applied(&ack);
    let conflict = applied
        .conflict
        .as_ref()
        .expect("overlapping writes on one part conflict");
    assert_eq!(
        conflict.specific_fields,
        vec![1],
        "the acknowledgement names the type message's own field number"
    );
    assert!(conflict.content_fields.is_empty());

    let conflicts = world.conflicts(id).await;
    assert_eq!(conflicts.len(), 1);
    let c = &conflicts[0];
    assert_eq!(c.field.as_deref(), Some("specific.1"));
    // The arriving value is `mine`, the value it displaced is `theirs`, and
    // each carries the member it came from. The write still applied.
    assert_eq!(c.mine.as_deref(), Some("worked"));
    assert_eq!(c.theirs.as_deref(), Some("working"));
    assert_eq!(c.mine_member, Some(world.two.membership_id()));
    assert_eq!(c.theirs_member, Some(world.one.membership_id()));
    assert_eq!(state_of(&world, id).await, "worked");
}

/// **Writes producing the same value are not divergent**, whatever their base
/// counter said. Two people resolving one thing the same way is the likely
/// case, and forming a conflict out of agreement is the machinery working
/// against its own purpose.
#[tokio::test]
async fn two_members_setting_the_same_state_do_not_conflict() {
    let world = World::new().await;
    let id = shared_campaign(&world, None).await;
    let base = world.counter(id).await;

    for member in [&world.one, &world.two] {
        world
            .submit(
                member,
                edit_entity(
                    id,
                    base as u64,
                    v1::EntityContent {
                        specific: Some(campaign(Some(v1::GuidanceState::Working))),
                        ..Default::default()
                    },
                ),
            )
            .await;
    }

    assert!(world.conflicts(id).await.is_empty());
    assert_eq!(state_of(&world, id).await, "working");
}

/// And the same rule the other way: clearing arrives at waiting, so a member
/// clearing while another set waiting is not a disagreement either.
#[tokio::test]
async fn clearing_and_setting_the_default_are_the_same_value() {
    let world = World::new().await;
    let id = shared_campaign(&world, Some(v1::GuidanceState::Working)).await;
    let base = world.counter(id).await;

    world
        .submit(
            &world.one,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    specific: Some(campaign(Some(v1::GuidanceState::Waiting))),
                    ..Default::default()
                },
            ),
        )
        .await;
    world
        .submit(
            &world.two,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    specific: Some(campaign_cleared(vec![1])),
                    ..Default::default()
                },
            ),
        )
        .await;

    assert!(world.conflicts(id).await.is_empty());
    assert_eq!(state_of(&world, id).await, "waiting");
}

/// **A stale base is never a rejection**, and different parts merge with nobody
/// involved. One writes the title, the other writes the state, both from the
/// same base.
#[tokio::test]
async fn a_stale_base_over_different_parts_merges_and_is_never_a_rejection() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            campaign(None),
            v1::EntityContent {
                title: Some("before".into()),
                audience_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
                ..Default::default()
            },
        )
        .await;
    let base = world.counter(id).await;

    world
        .submit(
            &world.one,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    title: Some("after".into()),
                    ..Default::default()
                },
            ),
        )
        .await;
    let ack = world
        .submit(
            &world.two,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    specific: Some(campaign(Some(v1::GuidanceState::Working))),
                    ..Default::default()
                },
            ),
        )
        .await;

    assert!(
        applied(&ack).conflict.is_none(),
        "different parts merge; a stale base alone is not a conflict and never a rejection"
    );
    assert_eq!(world.title(id).await.as_deref(), Some("after"));
    assert_eq!(state_of(&world, id).await, "working");
}

/// Type immutability still holds with type content in the message. The tag is
/// what says which type an edit thinks it is editing.
#[tokio::test]
async fn an_edit_may_not_change_an_entitys_type() {
    let world = World::new().await;
    let id = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let base = world.counter(id).await;

    let ack = world
        .submit(
            &world.one,
            edit_entity(
                id,
                base as u64,
                v1::EntityContent {
                    specific: Some(v1::entity_content::Specific::Quest(
                        v1::QuestContent::default(),
                    )),
                    ..Default::default()
                },
            ),
        )
        .await;

    let refused = refused(&ack);
    assert_eq!(refused.reason, v1::RefusalReason::Malformed as i32);
    assert!(refused.detail.contains("type"), "{}", refused.detail);
    assert_eq!(state_of(&world, id).await, "waiting");
}

/// `search_text` is the title plus whatever the type contributes. Campaign
/// contributes nothing, which is a valid answer — what matters is that the
/// title still lands there once the type's share is asked for.
#[tokio::test]
async fn the_title_still_reaches_search_text_through_the_types_share() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            campaign(Some(v1::GuidanceState::Working)),
            v1::EntityContent {
                title: Some("the kitchen".into()),
                ..Default::default()
            },
        )
        .await;

    let text: String = sqlx::query("SELECT search_text FROM entity WHERE id = $1")
        .bind(id.as_uuid())
        .fetch_one(&world.db.pool)
        .await
        .expect("entity")
        .try_get("search_text")
        .expect("search_text");
    assert_eq!(text, "the kitchen");
}

// ---------------------------------------------------------------------------
// The two refusals, one level down
// ---------------------------------------------------------------------------

async fn refusal_creating(world: &World, specific: v1::entity_content::Specific) -> String {
    let ack = world
        .submit(
            &world.one,
            create_entity(
                Uuid::new_v4(),
                v1::EntityContent {
                    specific: Some(specific),
                    ..Default::default()
                },
            ),
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
async fn a_field_both_present_and_cleared_is_malformed() {
    let world = World::new().await;
    let detail = refusal_creating(
        &world,
        v1::entity_content::Specific::Campaign(v1::CampaignContent {
            state: Some(v1::GuidanceState::Working as i32),
            cleared: vec![1],
        }),
    )
    .await;
    assert!(detail.contains("both present and listed"), "{detail}");
}

#[tokio::test]
async fn a_cleared_number_naming_no_field_of_this_type_is_malformed() {
    let world = World::new().await;
    // 100 is `cleared` itself, 7 was never anything, and 3 is a real field —
    // of an arc and of a quest. A campaign has one field and it is not that.
    for number in [3, 7, 100] {
        let detail = refusal_creating(&world, campaign_cleared(vec![number])).await;
        assert!(
            detail.contains("not a part of this type"),
            "clearing {number} on a campaign: {detail}"
        );
    }
}

#[tokio::test]
async fn a_state_that_is_present_and_names_nothing_is_malformed() {
    let world = World::new().await;
    let detail = refusal_creating(
        &world,
        v1::entity_content::Specific::Campaign(v1::CampaignContent {
            state: Some(v1::GuidanceState::Unspecified as i32),
            cleared: Vec::new(),
        }),
    )
    .await;
    assert!(detail.contains("guidance state"), "{detail}");
}

/// **A recreation is a whole entity of its type, not a bare shell.**
///
/// 2.2's done-when asks that an edit arriving for an erased entity produce a
/// recreation rather than a create. 2.1 proved that for the shared model; what
/// 2.2 adds underneath it is type content, and nothing exercised the two
/// together. `recreate` hands the same `Written` to `create`, so the type's row
/// is made with its defaults and the parts the edit addressed land on top —
/// but that is a property of one call passing a value through, and a value
/// passed through is exactly what a later refactor drops.
///
/// This is the one path where a create runs on content a client submitted as an
/// edit, so it is the one place the two halves of the wave meet.
#[tokio::test]
async fn a_recreation_carries_the_type_content_the_edit_addressed() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            campaign(Some(v1::GuidanceState::Working)),
            v1::EntityContent::default(),
        )
        .await;
    world.submit(&world.one, erase(&[id])).await;

    let ack = world
        .submit(
            &world.one,
            edit_entity(
                id,
                1,
                v1::EntityContent {
                    specific: Some(campaign(Some(v1::GuidanceState::Worked))),
                    ..Default::default()
                },
            ),
        )
        .await;

    let r = recreated(&ack);
    let new = EntityId::from_uuid(Uuid::from_bytes(
        r.new_entity_id.clone().try_into().expect("16 bytes"),
    ));

    // The side-table row exists at all, which is what "not a shell" means: the
    // erased entity's row is gone with its tombstone and this is a new one.
    // `state_of` fetches exactly one row and panics otherwise, so reaching the
    // assertion is half the claim.
    assert_eq!(
        state_of(&world, new).await,
        "worked",
        "the state the edit carried reached the recreated entity's own row"
    );
    assert_ne!(new, id, "the identity is new");
}

// ---------------------------------------------------------------------------
// What the stub assertions used to hold, and who holds it now
// ---------------------------------------------------------------------------
//
// Until Wave 2.2's four lanes landed, this file asserted that every type whose
// content was not yet built refused and said so, rather than accepting a write
// and storing nothing. That assertion was written against a list of unbuilt
// types, and each lane filling in its own type made its own entries false: a
// file reaches 2.1's own refusal about a body, and a category naming a parent
// that does not exist refuses as unavailable rather than as malformed, because
// nonexistence and audience are the same nothing.
//
// **The list is gone rather than half-corrected**, because a list that is true
// of some types and stale for others is worse than no list: it reads as
// coverage. What it was really guarding is that no type silently accepts a
// write and stores nothing, and that invariant is now carried where it can be
// stated for real — each type's own round-trip tests in the lane test files,
// which assert the stored value rather than the wording of a refusal.
//
// The same applies to the note body that used to refuse because division was
// not wired: bodies are served now, so the refusal it asserted is gone.

/// **A bare create of every type still works exactly as Wave 2.1 left it.** A
/// message addressing nothing addresses nothing, whether or not the type's
/// content is built.
#[tokio::test]
async fn a_create_carrying_no_type_content_is_unaffected() {
    use v1::entity_content::Specific;
    let world = World::new().await;
    let bare = [
        Specific::Campaign(v1::CampaignContent::default()),
        Specific::Arc(v1::ArcContent::default()),
        Specific::Quest(v1::QuestContent::default()),
        Specific::Note(v1::NoteContent::default()),
        Specific::Item(v1::ItemContent::default()),
        Specific::Location(v1::LocationContent::default()),
        Specific::ShoppingList(v1::ShoppingListContent::default()),
        Specific::Category(v1::CategoryContent::default()),
    ];
    for specific in bare {
        world
            .create(&world.one, specific, v1::EntityContent::default())
            .await;
    }
}

// ---------------------------------------------------------------------------
// The two refusals that expire for a reason outside this lane
// ---------------------------------------------------------------------------

/// `altair-v0-scope.md` defers shopping lists deliberately rather than
/// incidentally, and it governs the implementation plan where the two disagree.
/// So the entries refuse with the reason, and the refusal names it rather than
/// saying the content is merely unbuilt.
#[tokio::test]
async fn a_shopping_lists_entries_are_refused_with_the_reason_they_are_deferred() {
    let world = World::new().await;
    let detail = refusal_creating(
        &world,
        v1::entity_content::Specific::ShoppingList(v1::ShoppingListContent {
            body: Some("milk\nbread".into()),
            cleared: Vec::new(),
        }),
    )
    .await;
    assert!(
        detail.contains("deferred in v0") && detail.contains("anchor granularity"),
        "{detail}"
    );
}

/// The same for a person's correction to extracted text, which has nothing to
/// correct: text extraction from files is deferred in v0.
///
/// This refusal is reached before the one that says a file must name an
/// uploaded body, because the content is read before the row is made. Both are
/// expiring refusals and either answer is honest; the assertion is that the
/// deferral is the one a client is told about when it is the thing it asked
/// for.
#[tokio::test]
async fn a_files_extracted_text_is_refused_with_the_reason_extraction_is_deferred() {
    let world = World::new().await;
    let detail = refusal_creating(
        &world,
        v1::entity_content::Specific::File(v1::FileContent {
            extracted_text: Some("what the picture says".into()),
            ..Default::default()
        }),
    )
    .await;
    assert!(detail.contains("deferred in v0"), "{detail}");
}

// ---------------------------------------------------------------------------
// The declared field-to-column mapping, checked against the store
// ---------------------------------------------------------------------------

/// Every column a type declares actually exists, on every type at once.
///
/// The mapping is declared now and used by lanes later, so a mistyped column
/// would otherwise surface as a store fault in whichever lane happened to reach
/// it first. `LIMIT 0` asks the planner the question and reads nothing.
#[tokio::test]
async fn every_declared_column_exists_on_the_types_own_table() {
    use altaird::store::entity::EntityType::{
        Arc, Campaign, Category, File, Item, Location, Note, Quest, ShoppingList,
    };
    let world = World::new().await;
    for kind in [
        Campaign,
        Arc,
        Quest,
        Note,
        File,
        Item,
        Location,
        ShoppingList,
        Category,
    ] {
        for field in specific::fields(kind) {
            let Held::Column(column) = field.held else {
                continue;
            };
            let table = specific::side_table(kind)
                .unwrap_or_else(|| panic!("{kind:?} declares a column but has no table"));
            let sql = format!("SELECT {column} FROM {table} LIMIT 0");
            sqlx::query(sqlx::AssertSqlSafe(sql))
                .fetch_all(&world.db.pool)
                .await
                .unwrap_or_else(|e| {
                    panic!(
                        "{kind:?} field {} names {table}.{column}: {e}",
                        field.number
                    )
                });
        }
    }
}

// ---------------------------------------------------------------------------
// Cycle prevention in a nested container
// ---------------------------------------------------------------------------

/// Called from nowhere yet, so it is exercised directly. Nested locations and
/// nested categories both need it, which is why there is one of it.
async fn location(world: &World) -> EntityId {
    world
        .create(
            &world.one,
            v1::entity_content::Specific::Location(v1::LocationContent::default()),
            v1::EntityContent::default(),
        )
        .await
}

async fn set_parent(world: &World, child: EntityId, parent: Option<EntityId>) {
    sqlx::query("UPDATE location SET parent_location_id = $2 WHERE entity_id = $1")
        .bind(child.as_uuid())
        .bind(parent.map(EntityId::as_uuid))
        .execute(&world.db.pool)
        .await
        .expect("parent");
}

async fn cycles(world: &World, child: EntityId, parent: EntityId) -> bool {
    let mut tx = begin_write(&world.db.pool).await.expect("a transaction");
    let answer =
        specific::nesting::would_cycle(&mut tx, "location", "parent_location_id", child, parent)
            .await
            .expect("the store was reachable");
    tx.rollback().await.expect("rollback");
    answer
}

#[tokio::test]
async fn a_container_cannot_be_its_own_parent() {
    let world = World::new().await;
    let a = location(&world).await;
    assert!(cycles(&world, a, a).await);
}

#[tokio::test]
async fn a_longer_loop_is_refused_where_the_schema_cannot_see_it() {
    let world = World::new().await;
    // c is beneath b is beneath a. Making a's parent c closes the loop, and
    // the schema's only check is self-reference, so nothing else catches it.
    let a = location(&world).await;
    let b = location(&world).await;
    let c = location(&world).await;
    set_parent(&world, b, Some(a)).await;
    set_parent(&world, c, Some(b)).await;

    assert!(cycles(&world, a, c).await);
    assert!(cycles(&world, a, b).await, "one step up is still a loop");
}

#[tokio::test]
async fn an_ordinary_nesting_is_not_a_loop() {
    let world = World::new().await;
    let a = location(&world).await;
    let b = location(&world).await;
    let c = location(&world).await;
    set_parent(&world, b, Some(a)).await;

    // c beneath b: the walk goes c's proposed parent b, then a, then stops.
    assert!(!cycles(&world, c, b).await);
    // And a sibling relationship, where the proposed parent has no ancestry.
    assert!(!cycles(&world, a, c).await);
}

/// A loop already in the data terminates rather than walking forever, and
/// answers the way a loop answers. A hand-written row can make one; a recursive
/// query over it would not terminate.
#[tokio::test]
async fn a_loop_already_in_the_data_terminates() {
    let world = World::new().await;
    let a = location(&world).await;
    let b = location(&world).await;
    let c = location(&world).await;
    set_parent(&world, a, Some(b)).await;
    set_parent(&world, b, Some(a)).await;

    assert!(cycles(&world, c, a).await);
}

// ---------------------------------------------------------------------------
// The decimal a household's stock is carried in
// ---------------------------------------------------------------------------

#[test]
fn a_decimal_renders_the_same_way_coming_and_going() {
    for (units, nanos, text) in [
        (0_i64, 0_i32, "0.000000000"),
        (2, 500_000_000, "2.500000000"),
        (-2, -500_000_000, "-2.500000000"),
        (0, -1, "-0.000000001"),
        (12, 0, "12.000000000"),
    ] {
        let d = Decimal { units, nanos };
        assert_eq!(d.text(), text);
        assert_eq!(Decimal::parse(text).expect("parses"), d, "{text}");
    }
}

/// **Not clamped.** A nanos that is not a fraction, or a sign disagreement,
/// names no number; absorbing one would store something the client never sent.
/// This is the schema's owed check *everything the wire shape implies*.
#[test]
fn a_decimal_out_of_range_is_refused_rather_than_clamped() {
    for (units, nanos) in [
        (0_i64, 1_000_000_000_i32),
        (0, -1_000_000_000),
        (1, -1),
        (-1, 1),
    ] {
        assert!(
            Decimal::from_wire(&v1::Decimal { units, nanos }).is_err(),
            "{units} units and {nanos} nanos is not a decimal"
        );
    }
    assert!(
        Decimal::from_wire(&v1::Decimal {
            units: 3,
            nanos: 250_000_000,
        })
        .is_ok()
    );
}

#[test]
fn a_stored_number_the_wire_cannot_carry_is_refused_rather_than_rounded() {
    assert!(Decimal::parse("0.1234567891").is_err());
    assert!(Decimal::parse("not a number").is_err());
}

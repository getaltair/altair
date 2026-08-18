//! Guidance's type content: the three states, the one upward movement, and the
//! ladder.
//!
//! **Every assertion here reads the stored row.** An acknowledgement says a
//! write was applied; it does not say anything landed in a column, and a type
//! module that accepted a message and wrote nothing would satisfy every
//! acknowledgement-shaped assertion in the suite. So state, ladder parent, and
//! ladder position are read back out of `campaign`, `arc`, and `quest`
//! throughout, including the cases where the right answer is null.
//!
//! Three groups, and the middle one is the domain behaviour:
//!
//! - **Round trips.** Every field of all three messages, through a create and
//!   through an edit, set and cleared.
//! - **The state machine and the one upward movement.** Each transition the
//!   PRD's diagram draws, and one test per clause of the propagation rule's
//!   narrowing.
//! - **The ladder as a container.** Arrangement's rules, including the campaign
//!   ordering arcs and quests together in a sequence that spans two tables.

mod common;

use altair_proto::v1;
use altaird::store::ids::EntityId;
use common::*;
use sqlx::Row;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Saying things to the instance
// ---------------------------------------------------------------------------

fn campaign(state: Option<v1::GuidanceState>) -> v1::entity_content::Specific {
    v1::entity_content::Specific::Campaign(v1::CampaignContent {
        state: state.map(|s| s as i32),
        cleared: Vec::new(),
    })
}

fn arc(state: Option<v1::GuidanceState>, campaign: Option<EntityId>) -> v1::ArcContent {
    v1::ArcContent {
        state: state.map(|s| s as i32),
        campaign_id: campaign.map(|c| c.as_uuid().as_bytes().to_vec()),
        ladder_position: None,
        cleared: Vec::new(),
    }
}

fn quest(
    state: Option<v1::GuidanceState>,
    parent: Option<v1::quest_content::Parent>,
) -> v1::QuestContent {
    v1::QuestContent {
        state: state.map(|s| s as i32),
        parent,
        routine_id: None,
        ladder_position: None,
        cleared: Vec::new(),
    }
}

fn beneath_arc(id: EntityId) -> Option<v1::quest_content::Parent> {
    Some(v1::quest_content::Parent::ArcId(
        id.as_uuid().as_bytes().to_vec(),
    ))
}

fn beneath_campaign(id: EntityId) -> Option<v1::quest_content::Parent> {
    Some(v1::quest_content::Parent::CampaignId(
        id.as_uuid().as_bytes().to_vec(),
    ))
}

fn as_arc(c: v1::ArcContent) -> v1::entity_content::Specific {
    v1::entity_content::Specific::Arc(c)
}

fn as_quest(c: v1::QuestContent) -> v1::entity_content::Specific {
    v1::entity_content::Specific::Quest(c)
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
// Reading the stored row
// ---------------------------------------------------------------------------

async fn state_of(world: &World, table: &str, id: EntityId) -> String {
    let sql = format!("SELECT state::text AS s FROM {table} WHERE entity_id = $1");
    sqlx::query(sqlx::AssertSqlSafe(sql))
        .bind(id.as_uuid())
        .fetch_one(&world.db.pool)
        .await
        .expect("the side-table row exists")
        .try_get("s")
        .expect("state")
}

/// An arc's stored placement: its campaign and its position beneath it.
async fn arc_row(world: &World, id: EntityId) -> (Option<Uuid>, Option<i32>) {
    let row = sqlx::query("SELECT campaign_id, ladder_position FROM arc WHERE entity_id = $1")
        .bind(id.as_uuid())
        .fetch_one(&world.db.pool)
        .await
        .expect("the arc row exists");
    (
        row.try_get("campaign_id").unwrap(),
        row.try_get("ladder_position").unwrap(),
    )
}

/// A quest's stored placement: both parent columns, its routine, its position.
struct QuestRow {
    arc: Option<Uuid>,
    campaign: Option<Uuid>,
    routine: Option<Uuid>,
    position: Option<i32>,
}

async fn quest_row(world: &World, id: EntityId) -> QuestRow {
    let row = sqlx::query(
        "SELECT arc_id, campaign_id, routine_id, ladder_position FROM quest WHERE entity_id = $1",
    )
    .bind(id.as_uuid())
    .fetch_one(&world.db.pool)
    .await
    .expect("the quest row exists");
    QuestRow {
        arc: row.try_get("arc_id").unwrap(),
        campaign: row.try_get("campaign_id").unwrap(),
        routine: row.try_get("routine_id").unwrap(),
        position: row.try_get("ladder_position").unwrap(),
    }
}

/// How many change entries name this entity.
async fn changes_naming(world: &World, id: EntityId) -> usize {
    world
        .changes()
        .await
        .iter()
        .filter(|c| c.entity == Some(id.as_uuid()))
        .count()
}

/// The counter a part last moved at, or nothing if it never recorded one.
async fn part_counter(world: &World, id: EntityId, field: &str) -> Option<i64> {
    sqlx::query("SELECT counter FROM entity_part_counter WHERE entity_id = $1 AND field_name = $2")
        .bind(id.as_uuid())
        .bind(field)
        .fetch_optional(&world.db.pool)
        .await
        .expect("query")
        .map(|r| r.try_get("counter").expect("counter"))
}

// ---------------------------------------------------------------------------
// Building a ladder
// ---------------------------------------------------------------------------

/// A campaign, an arc beneath it, and a quest beneath the arc, all waiting.
///
/// Shared with member two throughout, because the propagation tests need a
/// container the writing member can see and one of them needs to take that
/// away.
struct Chain {
    campaign: EntityId,
    arc: EntityId,
    quest: EntityId,
}

fn shared_with(world: &World) -> v1::EntityContent {
    v1::EntityContent {
        audience_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
        ..Default::default()
    }
}

async fn chain(world: &World) -> Chain {
    let campaign_id = world
        .create(&world.one, campaign(None), shared_with(world))
        .await;
    let arc_id = world
        .create(
            &world.one,
            as_arc(arc(None, Some(campaign_id))),
            shared_with(world),
        )
        .await;
    let quest_id = world
        .create(
            &world.one,
            as_quest(quest(None, beneath_arc(arc_id))),
            shared_with(world),
        )
        .await;
    Chain {
        campaign: campaign_id,
        arc: arc_id,
        quest: quest_id,
    }
}

/// Move one entity's state, and take the acknowledgement.
async fn set_state(
    world: &World,
    member: &altaird::auth::Member,
    id: EntityId,
    specific: impl Fn(Option<v1::GuidanceState>) -> v1::entity_content::Specific,
    state: v1::GuidanceState,
) -> v1::Acknowledgement {
    let base = world.counter(id).await;
    world
        .submit(member, edit_specific(id, base, specific(Some(state))))
        .await
}

fn quest_state(state: Option<v1::GuidanceState>) -> v1::entity_content::Specific {
    as_quest(quest(state, None))
}

fn arc_state(state: Option<v1::GuidanceState>) -> v1::entity_content::Specific {
    as_arc(arc(state, None))
}

// ---------------------------------------------------------------------------
// Round trips: every field of every message, stored and read back
// ---------------------------------------------------------------------------

/// A bare create writes a row with the schema's defaults, and the defaults are
/// what the store says they are rather than what the write path assumed.
#[tokio::test]
async fn a_bare_create_of_each_type_leaves_the_defaults_in_the_row() {
    let world = World::new().await;

    let c = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    assert_eq!(state_of(&world, "campaign", c).await, "waiting");

    let a = world
        .create(
            &world.one,
            as_arc(arc(None, None)),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(state_of(&world, "arc", a).await, "waiting");
    assert_eq!(arc_row(&world, a).await, (None, None));

    let q = world
        .create(
            &world.one,
            as_quest(quest(None, None)),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(state_of(&world, "quest", q).await, "waiting");
    let row = quest_row(&world, q).await;
    assert_eq!(
        (row.arc, row.campaign, row.routine, row.position),
        (None, None, None, None)
    );
}

#[tokio::test]
async fn an_arc_round_trips_every_field_through_a_create_and_an_edit() {
    let world = World::new().await;
    let first = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let second = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;

    // Create: both fields stated, and the position the instance assigned.
    let a = world
        .create(
            &world.one,
            as_arc(arc(Some(v1::GuidanceState::Worked), Some(first))),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(state_of(&world, "arc", a).await, "worked");
    assert_eq!(arc_row(&world, a).await, (Some(first.as_uuid()), Some(0)));

    // Edit: both fields moved.
    let base = world.counter(a).await;
    world
        .submit(
            &world.one,
            edit_specific(
                a,
                base,
                as_arc(arc(Some(v1::GuidanceState::Working), Some(second))),
            ),
        )
        .await;
    assert_eq!(state_of(&world, "arc", a).await, "working");
    assert_eq!(arc_row(&world, a).await, (Some(second.as_uuid()), Some(0)));

    // Cleared: the campaign goes, and the position goes with it.
    let base = world.counter(a).await;
    world
        .submit(
            &world.one,
            edit_specific(
                a,
                base,
                as_arc(v1::ArcContent {
                    cleared: vec![2],
                    ..arc(None, None)
                }),
            ),
        )
        .await;
    assert_eq!(arc_row(&world, a).await, (None, None));
    assert_eq!(
        state_of(&world, "arc", a).await,
        "working",
        "leaving a container is not a state change"
    );
}

#[tokio::test]
async fn a_quest_round_trips_every_field_through_a_create_and_an_edit() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let a = world
        .create(
            &world.one,
            as_arc(arc(None, Some(c))),
            v1::EntityContent::default(),
        )
        .await;
    let routine = Uuid::new_v4();

    let q = world
        .create(
            &world.one,
            as_quest(v1::QuestContent {
                routine_id: Some(routine.as_bytes().to_vec()),
                ..quest(Some(v1::GuidanceState::Worked), beneath_arc(a))
            }),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(state_of(&world, "quest", q).await, "worked");
    let row = quest_row(&world, q).await;
    assert_eq!(row.arc, Some(a.as_uuid()));
    assert_eq!(row.campaign, None);
    assert_eq!(row.routine, Some(routine));
    assert_eq!(row.position, Some(0));

    // The routine is a column with no table behind it: stored, uninterpreted,
    // and cleared like anything else. Recurrence is deferred by the scope.
    let base = world.counter(q).await;
    world
        .submit(
            &world.one,
            edit_specific(
                q,
                base,
                as_quest(v1::QuestContent {
                    cleared: vec![4],
                    ..quest(Some(v1::GuidanceState::Working), None)
                }),
            ),
        )
        .await;
    let row = quest_row(&world, q).await;
    assert_eq!(row.routine, None);
    assert_eq!(
        row.arc,
        Some(a.as_uuid()),
        "clearing a routine is not a move"
    );
    assert_eq!(state_of(&world, "quest", q).await, "working");
}

/// The state is a part, and a part that records no counter movement is
/// invisible to conflict detection. So is a ladder parent, and so is the
/// position that moved with it.
#[tokio::test]
async fn every_guidance_part_records_the_counter_it_moved_at() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let a = world
        .create(
            &world.one,
            as_arc(arc(Some(v1::GuidanceState::Working), Some(c))),
            v1::EntityContent::default(),
        )
        .await;

    assert_eq!(
        part_counter(&world, a, "specific.1").await,
        Some(1),
        "state"
    );
    assert_eq!(
        part_counter(&world, a, "specific.2").await,
        Some(1),
        "the campaign"
    );
    assert_eq!(
        part_counter(&world, a, "specific.3").await,
        Some(1),
        "the position the instance assigned alongside it"
    );
}

// ---------------------------------------------------------------------------
// The state machine
// ---------------------------------------------------------------------------

/// The five movements the PRD's diagram draws, on all three types.
///
/// `[*] → Waiting` is the sixth and is the column default, which
/// `a_bare_create_of_each_type_leaves_the_defaults_in_the_row` covers.
#[tokio::test]
async fn every_transition_the_diagram_draws_holds() {
    use v1::GuidanceState::{Waiting, Worked, Working};
    let world = World::new().await;

    for (from, to) in [
        (Waiting, Working),
        (Working, Worked),
        (Working, Waiting),
        (Worked, Working),
        (Waiting, Worked),
    ] {
        for (table, of) in [
            ("campaign", campaign as fn(Option<v1::GuidanceState>) -> _),
            ("arc", arc_state),
            ("quest", quest_state),
        ] {
            let id = world
                .create(&world.one, of(Some(from)), v1::EntityContent::default())
                .await;
            assert_eq!(state_of(&world, table, id).await, store_name(from));

            let ack = set_state(&world, &world.one, id, of, to).await;
            assert!(
                matches!(ack.outcome, Some(v1::acknowledgement::Outcome::Applied(_))),
                "{table}: {from:?} to {to:?} was not applied: {:?}",
                ack.outcome
            );
            assert_eq!(
                state_of(&world, table, id).await,
                store_name(to),
                "{table}: {from:?} to {to:?}"
            );
        }
    }
}

fn store_name(state: v1::GuidanceState) -> &'static str {
    match state {
        v1::GuidanceState::Waiting => "waiting",
        v1::GuidanceState::Working => "working",
        v1::GuidanceState::Worked => "worked",
        v1::GuidanceState::Unspecified => unreachable!(),
    }
}

/// **The movement the diagram does not draw is still not refused**, and this
/// test is where that reading is visible rather than inferred from an absence
/// of code.
///
/// Worked → waiting is the one ordered pair of distinct states the PRD's
/// diagram leaves undrawn. Nothing in the prose forbids it, the substrate does
/// not reject a well-formed write, and the spine's rule that clearing a state
/// returns it to the column default performs exactly this movement — so a
/// prohibition invented here would contradict behaviour the spine already
/// tests.
#[tokio::test]
async fn the_movement_the_diagram_leaves_undrawn_is_not_refused() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            campaign(Some(v1::GuidanceState::Worked)),
            v1::EntityContent::default(),
        )
        .await;

    let ack = set_state(&world, &world.one, id, campaign, v1::GuidanceState::Waiting).await;
    assert!(
        matches!(ack.outcome, Some(v1::acknowledgement::Outcome::Applied(_))),
        "{:?}",
        ack.outcome
    );
    assert_eq!(state_of(&world, "campaign", id).await, "waiting");
}

/// Restating the state something is already in is not a movement and not a
/// refusal. Two members doing it are not divergent either, which the spine
/// tests; this is the single-member half.
#[tokio::test]
async fn restating_the_state_something_is_already_in_is_accepted() {
    let world = World::new().await;
    let id = world
        .create(
            &world.one,
            campaign(Some(v1::GuidanceState::Working)),
            v1::EntityContent::default(),
        )
        .await;
    set_state(&world, &world.one, id, campaign, v1::GuidanceState::Working).await;
    assert_eq!(state_of(&world, "campaign", id).await, "working");
}

// ---------------------------------------------------------------------------
// The one upward movement, one test per clause of the narrowing
// ---------------------------------------------------------------------------

/// The rule itself: starting a quest moves the containers above it, silently,
/// and it climbs the whole chain.
#[tokio::test]
async fn starting_a_quest_starts_the_whole_chain_above_it() {
    let world = World::new().await;
    let l = chain(&world).await;

    set_state(
        &world,
        &world.one,
        l.quest,
        quest_state,
        v1::GuidanceState::Working,
    )
    .await;

    assert_eq!(state_of(&world, "quest", l.quest).await, "working");
    assert_eq!(state_of(&world, "arc", l.arc).await, "working");
    assert_eq!(state_of(&world, "campaign", l.campaign).await, "working");
}

/// **Attachment need not be adjacent.** A quest hanging directly off a campaign
/// moves that campaign, with no arc in between.
#[tokio::test]
async fn a_quest_attached_directly_to_a_campaign_starts_it() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let q = world
        .create(
            &world.one,
            as_quest(quest(None, beneath_campaign(c))),
            v1::EntityContent::default(),
        )
        .await;

    set_state(
        &world,
        &world.one,
        q,
        quest_state,
        v1::GuidanceState::Working,
    )
    .await;
    assert_eq!(state_of(&world, "campaign", c).await, "working");
}

/// **Waiting to working only, and never in reverse.** A quest reaching the
/// terminal state moves nothing, and neither does one going back to waiting.
#[tokio::test]
async fn the_movement_never_runs_in_reverse() {
    let world = World::new().await;
    let l = chain(&world).await;

    // The containers are working; the quest finishes.
    set_state(
        &world,
        &world.one,
        l.quest,
        quest_state,
        v1::GuidanceState::Working,
    )
    .await;
    let arc_counter = world.counter(l.arc).await;
    let campaign_counter = world.counter(l.campaign).await;

    set_state(
        &world,
        &world.one,
        l.quest,
        quest_state,
        v1::GuidanceState::Worked,
    )
    .await;
    assert_eq!(state_of(&world, "arc", l.arc).await, "working");
    assert_eq!(state_of(&world, "campaign", l.campaign).await, "working");

    // And going back to waiting does not drag them back either.
    set_state(
        &world,
        &world.one,
        l.quest,
        quest_state,
        v1::GuidanceState::Waiting,
    )
    .await;
    assert_eq!(state_of(&world, "arc", l.arc).await, "working");
    assert_eq!(state_of(&world, "campaign", l.campaign).await, "working");

    // Nothing was written to them at all: no counter moved.
    assert_eq!(world.counter(l.arc).await, arc_counter);
    assert_eq!(world.counter(l.campaign).await, campaign_counter);
}

/// **Upward only. Nothing a container does moves a child.**
#[tokio::test]
async fn nothing_a_container_does_moves_a_child() {
    let world = World::new().await;
    let l = chain(&world).await;

    set_state(
        &world,
        &world.one,
        l.campaign,
        campaign,
        v1::GuidanceState::Working,
    )
    .await;
    assert_eq!(state_of(&world, "arc", l.arc).await, "waiting");
    assert_eq!(state_of(&world, "quest", l.quest).await, "waiting");

    set_state(
        &world,
        &world.one,
        l.arc,
        arc_state,
        v1::GuidanceState::Worked,
    )
    .await;
    assert_eq!(state_of(&world, "quest", l.quest).await, "waiting");
}

/// **It never touches a parent already in the terminal state**, because a stray
/// child would silently reopen something the person deliberately closed.
///
/// The clauses are stated per parent, so the climb continues past the worked
/// arc and the campaign above it still moves. Nothing in the PRD makes a
/// campaign's movement depend on the state of the arc below it, and stopping
/// there would be this suite deciding something the PRD did not.
#[tokio::test]
async fn it_never_reopens_a_container_already_worked_and_climbs_past_it() {
    let world = World::new().await;
    let l = chain(&world).await;
    set_state(
        &world,
        &world.one,
        l.arc,
        arc_state,
        v1::GuidanceState::Worked,
    )
    .await;
    let arc_counter = world.counter(l.arc).await;

    set_state(
        &world,
        &world.one,
        l.quest,
        quest_state,
        v1::GuidanceState::Working,
    )
    .await;

    assert_eq!(
        state_of(&world, "arc", l.arc).await,
        "worked",
        "a stray child may not reopen what the person closed"
    );
    assert_eq!(
        world.counter(l.arc).await,
        arc_counter,
        "and nothing was written to it"
    );
    assert_eq!(state_of(&world, "campaign", l.campaign).await, "working");
}

/// **It skips any container already working**, which means leaving it entirely
/// alone rather than writing the value it already holds.
#[tokio::test]
async fn the_climb_skips_a_container_already_working() {
    let world = World::new().await;
    let l = chain(&world).await;
    set_state(
        &world,
        &world.one,
        l.arc,
        arc_state,
        v1::GuidanceState::Working,
    )
    .await;
    let arc_counter = world.counter(l.arc).await;
    let arc_changes = changes_naming(&world, l.arc).await;

    set_state(
        &world,
        &world.one,
        l.quest,
        quest_state,
        v1::GuidanceState::Working,
    )
    .await;

    assert_eq!(state_of(&world, "arc", l.arc).await, "working");
    assert_eq!(world.counter(l.arc).await, arc_counter);
    assert_eq!(
        changes_naming(&world, l.arc).await,
        arc_changes,
        "a container that was already working produced no change entry"
    );
    assert_eq!(
        state_of(&world, "campaign", l.campaign).await,
        "working",
        "and the climb carried on past it"
    );
}

/// A quest created already started moves its containers.
///
/// **This is why a parent is read off the wire before the state.** A create
/// applies parts in the order the type's reader produced them; reading the
/// state first would leave the quest parentless at the moment its state landed,
/// and the movement would find nothing above it.
#[tokio::test]
async fn a_quest_created_already_started_starts_its_containers() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let a = world
        .create(
            &world.one,
            as_arc(arc(None, Some(c))),
            v1::EntityContent::default(),
        )
        .await;

    world
        .create(
            &world.one,
            as_quest(quest(Some(v1::GuidanceState::Working), beneath_arc(a))),
            v1::EntityContent::default(),
        )
        .await;

    assert_eq!(state_of(&world, "arc", a).await, "working");
    assert_eq!(state_of(&world, "campaign", c).await, "working");
}

/// A container this moves takes a counter and a change entry of its own.
///
/// Both are load-bearing: a part that does not record the counter it moved at
/// is invisible to conflict detection, and a movement nothing records is one no
/// client ever learns about. The audience did not change, so both sides of the
/// change entry's pair are what it already was.
#[tokio::test]
async fn a_container_the_movement_starts_records_a_counter_and_a_change() {
    let world = World::new().await;
    let l = chain(&world).await;
    let before = world.counter(l.campaign).await;
    let entries = changes_naming(&world, l.campaign).await;

    set_state(
        &world,
        &world.one,
        l.quest,
        quest_state,
        v1::GuidanceState::Working,
    )
    .await;

    assert_eq!(world.counter(l.campaign).await, before + 1);
    assert_eq!(
        part_counter(&world, l.campaign, "specific.1").await,
        Some(before + 1),
        "the state part records the counter the instance moved it at"
    );
    assert_eq!(changes_naming(&world, l.campaign).await, entries + 1);

    let last = world
        .changes()
        .await
        .into_iter()
        .rfind(|c| c.entity == Some(l.campaign.as_uuid()))
        .expect("an entry for the campaign");
    assert_eq!(last.kind, "entity_written");
    assert_eq!(last.audience_before, last.audience_after);
}

/// **The climb stops at a container this member cannot see.** Moving a state on
/// an entity the writer is not entitled to see would attribute a write, in
/// provenance and in the change sequence, to a member who cannot see what they
/// moved. The PRD already accepts that a container can drift out of step with
/// the work beneath it; this is one more way, and it is the safe way.
#[tokio::test]
async fn the_climb_stops_at_a_container_the_writer_cannot_see() {
    let world = World::new().await;
    let l = chain(&world).await;

    // One narrows the campaign to itself. Two can still see the arc and quest.
    let base = world.counter(l.campaign).await;
    world
        .submit(
            &world.one,
            edit_entity(
                l.campaign,
                base as u64,
                v1::EntityContent {
                    cleared: vec![5],
                    specific: Some(campaign(None)),
                    ..Default::default()
                },
            ),
        )
        .await;

    set_state(
        &world,
        &world.two,
        l.quest,
        quest_state,
        v1::GuidanceState::Working,
    )
    .await;

    assert_eq!(state_of(&world, "arc", l.arc).await, "working");
    assert_eq!(
        state_of(&world, "campaign", l.campaign).await,
        "waiting",
        "the climb does not reach past a container the writer cannot see"
    );
}

/// A quest with no ladder parent starts nothing, and says so by not failing.
///
/// **A recurrence is not a container for this**, and in v0 that clause is moot
/// rather than implemented: `routine_id` is a column with no `routine` table
/// behind it, nothing walks it, and so nothing can propagate through one. The
/// quest here holds a routine and no ladder parent, which is the shape the
/// clause is about.
#[tokio::test]
async fn a_quest_with_no_ladder_parent_starts_nothing() {
    let world = World::new().await;
    let q = world
        .create(
            &world.one,
            as_quest(v1::QuestContent {
                routine_id: Some(Uuid::new_v4().as_bytes().to_vec()),
                ..quest(None, None)
            }),
            v1::EntityContent::default(),
        )
        .await;

    set_state(
        &world,
        &world.one,
        q,
        quest_state,
        v1::GuidanceState::Working,
    )
    .await;
    assert_eq!(state_of(&world, "quest", q).await, "working");
    assert_eq!(
        world.changes().await.len(),
        2,
        "the create and the edit, and nothing the movement added"
    );
}

// ---------------------------------------------------------------------------
// The ladder as a container
// ---------------------------------------------------------------------------

/// **A campaign is one container over mixed kinds.** Arcs and quests beneath it
/// are one sequence, so the order spans two tables and cannot be a constraint.
#[tokio::test]
async fn a_campaign_orders_arcs_and_quests_together_in_one_sequence() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;

    let a1 = world
        .create(
            &world.one,
            as_arc(arc(None, Some(c))),
            v1::EntityContent::default(),
        )
        .await;
    let q1 = world
        .create(
            &world.one,
            as_quest(quest(None, beneath_campaign(c))),
            v1::EntityContent::default(),
        )
        .await;
    let a2 = world
        .create(
            &world.one,
            as_arc(arc(None, Some(c))),
            v1::EntityContent::default(),
        )
        .await;
    let q2 = world
        .create(
            &world.one,
            as_quest(quest(None, beneath_campaign(c))),
            v1::EntityContent::default(),
        )
        .await;

    assert_eq!(arc_row(&world, a1).await.1, Some(0));
    assert_eq!(quest_row(&world, q1).await.position, Some(1));
    assert_eq!(arc_row(&world, a2).await.1, Some(2));
    assert_eq!(
        quest_row(&world, q2).await.position,
        Some(3),
        "the order is total across both kinds: a quest never reuses an arc's position"
    );
}

/// **Leaving a container forgets the position**, and the gap it leaves is not
/// reused. Nothing shifts to close it: the substrate's order is the one the
/// person gave, and repairing it would move things they did not move.
#[tokio::test]
async fn leaving_forgets_the_position_and_nothing_is_repaired() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let a = world
        .create(
            &world.one,
            as_arc(arc(None, Some(c))),
            v1::EntityContent::default(),
        )
        .await;
    let q = world
        .create(
            &world.one,
            as_quest(quest(None, beneath_campaign(c))),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(arc_row(&world, a).await.1, Some(0));
    assert_eq!(quest_row(&world, q).await.position, Some(1));

    // The arc leaves.
    let base = world.counter(a).await;
    world
        .submit(
            &world.one,
            edit_specific(
                a,
                base,
                as_arc(v1::ArcContent {
                    cleared: vec![2],
                    ..arc(None, None)
                }),
            ),
        )
        .await;
    assert_eq!(arc_row(&world, a).await, (None, None));
    assert_eq!(
        quest_row(&world, q).await.position,
        Some(1),
        "nothing else moved to close the gap"
    );

    // And the next entrant appends past the gap.
    let next = world
        .create(
            &world.one,
            as_quest(quest(None, beneath_campaign(c))),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(quest_row(&world, next).await.position, Some(2));
}

/// Restating the container something is already in is not an entry, so it does
/// not append. The position the person can predict is the one it keeps.
#[tokio::test]
async fn restating_the_same_container_does_not_move_the_position() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let first = world
        .create(
            &world.one,
            as_arc(arc(None, Some(c))),
            v1::EntityContent::default(),
        )
        .await;
    let second = world
        .create(
            &world.one,
            as_arc(arc(None, Some(c))),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(arc_row(&world, first).await.1, Some(0));

    let base = world.counter(first).await;
    world
        .submit(
            &world.one,
            edit_specific(first, base, as_arc(arc(None, Some(c)))),
        )
        .await;
    assert_eq!(
        arc_row(&world, first).await,
        (Some(c.as_uuid()), Some(0)),
        "restating the container is not entering it again"
    );
    assert_eq!(arc_row(&world, second).await.1, Some(1));
}

/// **Entering a different container appends at the end of that one**, and the
/// order does not survive the move. That is the substrate's accepted cost.
#[tokio::test]
async fn entering_a_different_container_appends_at_the_end_of_it() {
    let world = World::new().await;
    let from = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let to = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    // Something already sits in the destination.
    world
        .create(
            &world.one,
            as_quest(quest(None, beneath_campaign(to))),
            v1::EntityContent::default(),
        )
        .await;

    let a = world
        .create(
            &world.one,
            as_arc(arc(None, Some(from))),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(arc_row(&world, a).await.1, Some(0));

    let base = world.counter(a).await;
    world
        .submit(
            &world.one,
            edit_specific(a, base, as_arc(arc(None, Some(to)))),
        )
        .await;
    assert_eq!(arc_row(&world, a).await, (Some(to.as_uuid()), Some(1)));
}

/// **A parent and its position never move in separate statements**, and this is
/// the test that would trip the table check if they did.
///
/// A quest moving from an arc to a campaign passes through no intermediate row.
/// Either order in a sequence of two statements is refused by the schema:
/// setting `campaign_id` first leaves both parents non-null, which
/// `CHECK (num_nonnulls(arc_id, campaign_id) <= 1)` refuses; clearing `arc_id`
/// first leaves a position with no parent, which
/// `CHECK ((num_nonnulls(arc_id, campaign_id) = 0) = (ladder_position IS NULL))`
/// refuses. A store fault rather than an acknowledgement is what this failing
/// looks like.
#[tokio::test]
async fn a_quest_moving_between_an_arc_and_a_campaign_moves_in_one_statement() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let a = world
        .create(
            &world.one,
            as_arc(arc(None, Some(c))),
            v1::EntityContent::default(),
        )
        .await;
    let q = world
        .create(
            &world.one,
            as_quest(quest(None, beneath_arc(a))),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(quest_row(&world, q).await.arc, Some(a.as_uuid()));

    // Arc to campaign. The arc is cleared in the same statement.
    let base = world.counter(q).await;
    let ack = world
        .submit(
            &world.one,
            edit_specific(q, base, as_quest(quest(None, beneath_campaign(c)))),
        )
        .await;
    assert!(matches!(
        ack.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));
    let row = quest_row(&world, q).await;
    assert_eq!(row.arc, None);
    assert_eq!(row.campaign, Some(c.as_uuid()));
    assert_eq!(
        row.position,
        Some(1),
        "the arc beneath the campaign already holds 0"
    );

    // And back again.
    let base = world.counter(q).await;
    world
        .submit(
            &world.one,
            edit_specific(q, base, as_quest(quest(None, beneath_arc(a)))),
        )
        .await;
    let row = quest_row(&world, q).await;
    assert_eq!(row.arc, Some(a.as_uuid()));
    assert_eq!(row.campaign, None);
    assert_eq!(row.position, Some(0), "the arc was empty");
}

/// Leaving with nothing left to hold the position clears both, in one
/// statement. Clearing the position alone would trip the same check.
#[tokio::test]
async fn a_quest_leaving_its_campaign_forgets_the_position_in_one_statement() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let q = world
        .create(
            &world.one,
            as_quest(quest(None, beneath_campaign(c))),
            v1::EntityContent::default(),
        )
        .await;
    assert_eq!(quest_row(&world, q).await.position, Some(0));

    let base = world.counter(q).await;
    world
        .submit(
            &world.one,
            edit_specific(
                q,
                base,
                as_quest(v1::QuestContent {
                    cleared: vec![3],
                    ..quest(None, None)
                }),
            ),
        )
        .await;
    let row = quest_row(&world, q).await;
    assert_eq!((row.arc, row.campaign, row.position), (None, None, None));
}

/// Clearing the parent a quest does not have leaves the one it does have alone.
///
/// The two parents are separate parts because the wire's `oneof` can only set
/// one of them, and a clear of the empty one is not a request to detach from
/// the other. Getting this wrong would strip a position the campaign gave.
#[tokio::test]
async fn clearing_the_parent_a_quest_does_not_have_leaves_the_other_alone() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;
    let q = world
        .create(
            &world.one,
            as_quest(quest(None, beneath_campaign(c))),
            v1::EntityContent::default(),
        )
        .await;

    // Clear field 2, the arc, which this quest never had.
    let base = world.counter(q).await;
    world
        .submit(
            &world.one,
            edit_specific(
                q,
                base,
                as_quest(v1::QuestContent {
                    cleared: vec![2],
                    ..quest(None, None)
                }),
            ),
        )
        .await;
    let row = quest_row(&world, q).await;
    assert_eq!(row.campaign, Some(c.as_uuid()));
    assert_eq!(
        row.position,
        Some(0),
        "the campaign still holds the position"
    );
}

// ---------------------------------------------------------------------------
// What a ladder parent has to be
// ---------------------------------------------------------------------------

async fn refusal_creating(world: &World, specific: v1::entity_content::Specific) -> v1::Refused {
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
    refused(&ack).clone()
}

/// **Refusal on audience is indistinguishable from refusal on nonexistence**,
/// and from a parent of the wrong kind. All three are the same nothing.
#[tokio::test]
async fn a_ladder_parent_that_is_not_available_is_always_the_same_nothing() {
    let world = World::new().await;

    // Member two's campaign, which one cannot see.
    let hidden = world
        .create(&world.two, campaign(None), v1::EntityContent::default())
        .await;
    // A note is not a campaign.
    let note = world.note(&world.one, "not a campaign").await;
    let nothing = EntityId::from_uuid(Uuid::new_v4());

    let mut answers = Vec::new();
    for parent in [hidden, note, nothing] {
        let refused = refusal_creating(&world, as_arc(arc(None, Some(parent)))).await;
        answers.push((refused.reason, refused.detail));
    }
    assert_eq!(
        answers[0].0,
        v1::RefusalReason::NotAvailable as i32,
        "{answers:?}"
    );
    assert!(
        answers.windows(2).all(|w| w[0] == w[1]),
        "the three answers differ, which is an oracle for what exists: {answers:?}"
    );

    // And the same for a quest's two parent fields.
    for parent in [beneath_arc(hidden), beneath_campaign(note)] {
        let refused = refusal_creating(&world, as_quest(quest(None, parent))).await;
        assert_eq!(refused.reason, v1::RefusalReason::NotAvailable as i32);
        assert_eq!(refused.detail, answers[0].1);
    }
}

/// **An explicit ladder position is refused**, in the register of the three
/// refusals Wave 2.1 makes that expire when a named lane lands. Appending is
/// what this build implements; reordering is a deliberate act on a surface with
/// the container in front of the person, and nothing reorders yet.
#[tokio::test]
async fn an_explicit_ladder_position_is_refused_with_the_reason_it_expires() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), v1::EntityContent::default())
        .await;

    let stated = [
        as_arc(v1::ArcContent {
            ladder_position: Some(3),
            ..arc(None, Some(c))
        }),
        as_quest(v1::QuestContent {
            ladder_position: Some(3),
            ..quest(None, beneath_campaign(c))
        }),
    ];
    for specific in stated {
        let refused = refusal_creating(&world, specific).await;
        assert_eq!(refused.reason, v1::RefusalReason::Malformed as i32);
        assert!(
            refused.detail.contains("appends on entry to a container")
                && refused.detail.contains("not served yet"),
            "{}",
            refused.detail
        );
    }

    // On an edit too, and nothing was written on the way to the refusal.
    let a = world
        .create(
            &world.one,
            as_arc(arc(None, Some(c))),
            v1::EntityContent::default(),
        )
        .await;
    let base = world.counter(a).await;
    let ack = world
        .submit(
            &world.one,
            edit_specific(
                a,
                base,
                as_arc(v1::ArcContent {
                    ladder_position: Some(7),
                    state: Some(v1::GuidanceState::Working as i32),
                    ..arc(None, None)
                }),
            ),
        )
        .await;
    assert_eq!(
        refused(&ack).reason,
        v1::RefusalReason::Malformed as i32,
        "{:?}",
        ack.outcome
    );
    assert_eq!(
        state_of(&world, "arc", a).await,
        "waiting",
        "the refusal rolled back the state that travelled with it"
    );
    assert_eq!(arc_row(&world, a).await, (Some(c.as_uuid()), Some(0)));
}

// ---------------------------------------------------------------------------
// Conflict, on a part that is Guidance's own
// ---------------------------------------------------------------------------

/// A same-part conflict on a ladder parent retains both values, and the write
/// still applies. **A stale base is never a rejection.**
#[tokio::test]
async fn a_same_part_conflict_on_a_ladder_parent_retains_both_values() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), shared_with(&world))
        .await;
    let first = world
        .create(&world.one, as_arc(arc(None, Some(c))), shared_with(&world))
        .await;
    let second = world
        .create(&world.one, as_arc(arc(None, Some(c))), shared_with(&world))
        .await;
    let q = world
        .create(&world.one, as_quest(quest(None, None)), shared_with(&world))
        .await;
    let base = world.counter(q).await;

    world
        .submit(
            &world.one,
            edit_specific(q, base, as_quest(quest(None, beneath_arc(first)))),
        )
        .await;
    let ack = world
        .submit(
            &world.two,
            edit_specific(q, base, as_quest(quest(None, beneath_arc(second)))),
        )
        .await;

    let applied = applied(&ack);
    let conflict = applied
        .conflict
        .as_ref()
        .expect("two members moving one quest's arc is one part twice");
    assert!(conflict.specific_fields.contains(&2));

    let conflicts = world.conflicts(q).await;
    let arc_conflict = conflicts
        .iter()
        .find(|c| c.field.as_deref() == Some("specific.2"))
        .expect("the arc is the conflicted part");
    assert_eq!(
        arc_conflict.mine.as_deref(),
        Some(second.as_uuid().to_string()).as_deref()
    );
    assert_eq!(
        arc_conflict.theirs.as_deref(),
        Some(first.as_uuid().to_string()).as_deref()
    );
    assert_eq!(
        quest_row(&world, q).await.arc,
        Some(second.as_uuid()),
        "the write still applied"
    );
}

/// Different parts of one type merge with nobody involved: one member moves the
/// state, the other the parent, both from the same base.
#[tokio::test]
async fn two_members_moving_different_guidance_parts_merge() {
    let world = World::new().await;
    let c = world
        .create(&world.one, campaign(None), shared_with(&world))
        .await;
    let q = world
        .create(&world.one, as_quest(quest(None, None)), shared_with(&world))
        .await;
    let base = world.counter(q).await;

    world
        .submit(
            &world.one,
            edit_specific(
                q,
                base,
                as_quest(quest(Some(v1::GuidanceState::Worked), None)),
            ),
        )
        .await;
    let ack = world
        .submit(
            &world.two,
            edit_specific(q, base, as_quest(quest(None, beneath_campaign(c)))),
        )
        .await;

    assert!(
        applied(&ack).conflict.is_none(),
        "different parts merge, and a stale base alone is never a rejection"
    );
    assert_eq!(state_of(&world, "quest", q).await, "worked");
    assert_eq!(quest_row(&world, q).await.campaign, Some(c.as_uuid()));
}

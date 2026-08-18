//! Guidance's type content: campaign, arc, quest.
//!
//! **LANE: Guidance owns this file.** Three types, one state enum, and one
//! container — the ladder — that the shared model does not cover.
//!
//! # The states, and why nothing here refuses a transition
//!
//! The shipped set is waiting, working, worked, and the PRD is explicit that a
//! person may rename them but may not add, remove, reorder, or define their
//! own. That is settled and the store's enum is the whole of it.
//!
//! **No transition is refused, and that is a decision rather than an omission.**
//! The PRD's state diagram draws five movements between the three states and
//! leaves worked → waiting undrawn. Read as a prohibition it would make one
//! write a rejection, and three things say it is not:
//!
//! - **No prose forbids it.** The diagram's neighbouring prose constrains the
//!   *set* of states and the one upward movement below; it says nothing about
//!   which movements a person may make. A rule this file invented from an
//!   undrawn arrow would be a rule nothing normative states.
//! - **The substrate does not reject well-formed writes.** A device that was
//!   away replays what the person did, and the only ways a write does not land
//!   are that the content cannot be read or that the entity is not available.
//!   "The person moved this the wrong way round" is neither.
//! - **The spine already relies on it.** Clearing the state field sets it back
//!   to waiting, because the column is `NOT NULL DEFAULT 'waiting'`; a worked
//!   campaign whose state is cleared therefore performs exactly the undrawn
//!   movement, and `tests/type_content.rs` has asserted since 2.2's spine that
//!   it arrives at waiting.
//!
//! `tests/type_content_guidance.rs` walks every movement the diagram draws and
//! names the undrawn one, so the reading is visible rather than inferred from
//! an absence of code.
//!
//! # The one upward movement
//!
//! Starting a quest moves the containers above it from waiting to working, and
//! it happens silently. [`propagate_start_upward`] is the whole of it, and each
//! clause of the PRD's narrowing has its own test.
//!
//! # The ladder is a container, and Arrangement is where its rules come from
//!
//! **Two documents disagree about where the ladder's ordering rules live, and
//! this file does not resolve it.** The substrate spec's *Arrangement* section
//! says the ladder defines its own order "in the Guidance PRD"; the Guidance
//! PRD says a campaign's mixed-kind order "needs nothing of its own" and then
//! defines no rules. Something has to be implemented, so this file implements
//! **Arrangement's**: entering a container appends at the end, leaving forgets
//! the position, no container is unordered, and the order is total and stable.
//! That is the reading that leaves nothing undefined, and it is what the PRD's
//! "nothing of its own" most plausibly means — the ladder needs no ordering
//! rules *because the substrate's already cover it*.
//!
//! **A campaign is one container over mixed kinds.** Arcs and quests beneath
//! one campaign are ordered together, in one sequence spanning two tables,
//! which is why the order cannot be a unique constraint and why
//! [`next_ladder_position`] takes a maximum across both.
//!
//! # What is deliberately not here
//!
//! - **Explicit reordering.** Appending is what a container entry does; a
//!   client stating a `ladder_position` is refused as malformed, in the
//!   register of the three refusals Wave 2.1 makes that expire when a named
//!   lane lands. See [`explicit_position_refusal`].
//! - **Recurrence.** `quest.routine_id` is a column with no `routine` table
//!   behind it, because the scope defers routines. The field is read and stored
//!   and **nothing interprets it**; whatever builds recurrence owes the
//!   reference check that no foreign key can make today. The PRD's clause that
//!   *a recurrence is not a container for this* is therefore moot rather than
//!   implemented: nothing here walks a routine, so nothing can propagate
//!   through one.
//! - **The prompts on a parent reaching the terminal state, and on deleting
//!   one.** The PRD asks about the children and the person answers. That is a
//!   client behaviour and there is no client until Wave 4, so this file neither
//!   prompts nor cascades — a silent cascade would be the exact thing the PRD
//!   rules out, and building half of it now would foreclose the question.
//! - **Two open questions the PRD holds, which nothing here closes:** the exact
//!   inflections of the state names, and the absence of any mechanism for
//!   ordering one quest against another outside a container.
//!
//! # What is settled by the plumbing rather than by the domain
//!
//! **Clearing a state sets it to waiting**, because the column is
//! `NOT NULL DEFAULT 'waiting'` and there is no state a campaign is not in. Two
//! members arriving at waiting — one by setting it, one by clearing it —
//! therefore compare equal rather than conflict, which is the substrate's rule
//! that writes producing the same value are not divergent.

use sqlx::Row;
use uuid::Uuid;

use altair_proto::v1;

use crate::store::WriteScope;
use crate::store::entity::{EntityRow, EntityType, available_for_write};
use crate::store::ids::EntityId;

use super::super::changes::{self, EntityChange};
use super::super::content::{Malformed, Written, identifier, malformed};
use super::super::entity::{Applied, Ctx, Refusal, type_name};
use super::super::parts::Part;
use super::super::provenance;
use super::{
    Detachment, Field, GuidanceState, Held, Reader, Released, SpecificPart, SpecificValue,
    part_held_in, read_column, write_column,
};

/// The state field. **The same number in all three messages**, which is what
/// lets one propagation record a container's movement without asking which kind
/// of container it reached.
pub const STATE: u32 = 1;

/// Campaign's field 1, its state. The only part a campaign carries.
pub const CAMPAIGN_STATE: u32 = STATE;

/// An arc's campaign, and its position beneath it.
const ARC_CAMPAIGN: u32 = 2;
const ARC_POSITION: u32 = 3;

/// A quest's two possible ladder parents, its routine, and its position.
const QUEST_ARC: u32 = 2;
const QUEST_CAMPAIGN: u32 = 3;
const QUEST_ROUTINE: u32 = 4;
const QUEST_POSITION: u32 = 5;

/// The columns this file names in a statement as well as in a declaration.
///
/// **Named once because two things must not drift.** [`detach_contained`] and
/// [`apply`] write these columns by name, and the `FIELDS` tables below declare
/// the same columns as where a wire field is held; `tests/type_content.rs`
/// checks the declarations against the store but cannot see a statement that
/// spells one differently. Following the convention Tracking arrived at for
/// exactly this reason.
const STATE_COLUMN: &str = "state";
const CAMPAIGN_COLUMN: &str = "campaign_id";
const ARC_COLUMN: &str = "arc_id";
const LADDER_POSITION_COLUMN: &str = "ladder_position";

pub const CAMPAIGN_FIELDS: &[Field] = &[Field {
    number: CAMPAIGN_STATE,
    held: Held::Column(STATE_COLUMN),
}];

pub const ARC_FIELDS: &[Field] = &[
    Field {
        number: STATE,
        held: Held::Column(STATE_COLUMN),
    },
    Field {
        number: ARC_CAMPAIGN,
        held: Held::Column(CAMPAIGN_COLUMN),
    },
    Field {
        number: ARC_POSITION,
        held: Held::Column(LADDER_POSITION_COLUMN),
    },
];

pub const QUEST_FIELDS: &[Field] = &[
    Field {
        number: STATE,
        held: Held::Column(STATE_COLUMN),
    },
    Field {
        number: QUEST_ARC,
        held: Held::Column(ARC_COLUMN),
    },
    Field {
        number: QUEST_CAMPAIGN,
        held: Held::Column(CAMPAIGN_COLUMN),
    },
    Field {
        number: QUEST_ROUTINE,
        held: Held::Column("routine_id"),
    },
    Field {
        number: QUEST_POSITION,
        held: Held::Column(LADDER_POSITION_COLUMN),
    },
];

/// The side table a type keeps its content in.
///
/// Taken from [`super::side_table`] rather than written out, so the statements
/// here cannot disagree with the one place that decides which table a type has.
fn table_of(kind: EntityType) -> Applied<&'static str> {
    super::side_table(kind).ok_or_else(|| {
        Refusal::Malformed(format!("a {} has no table of its own", type_name(kind))).into()
    })
}

// ---------------------------------------------------------------------------
// Reading the three messages
// ---------------------------------------------------------------------------

/// The state a wire field names, resolving a clear to the column's default.
fn state_value(v: Option<i32>) -> Result<SpecificValue, Malformed> {
    Ok(SpecificValue::State(match v {
        Some(number) => GuidanceState::from_wire(number)?,
        None => GuidanceState::DEFAULT,
    }))
}

/// An identifier a wire field names, or nothing where it was cleared.
fn id_value(v: Option<&Vec<u8>>) -> Result<SpecificValue, Malformed> {
    Ok(SpecificValue::Id(
        v.map(|bytes| identifier(bytes)).transpose()?,
    ))
}

/// A position a client stated, which is refused later and read here so that the
/// refusal is about reordering rather than about the number's width.
fn position_value(v: Option<u32>) -> Result<SpecificValue, Malformed> {
    Ok(SpecificValue::Count(
        v.map(|p| i32::try_from(p).map_err(|_| malformed("a position that large is not one")))
            .transpose()?,
    ))
}

/// A campaign's content, off the wire.
pub fn campaign(c: &v1::CampaignContent) -> Result<Written, Malformed> {
    let read = Reader::new(EntityType::Campaign, &c.cleared)?;
    let mut parts = Vec::new();
    read.singular(&mut parts, CAMPAIGN_STATE, c.state, state_value)?;
    Ok(Written::from_specific(parts))
}

/// An arc's content, off the wire.
///
/// **The parent is read before the state, and the order is load-bearing.** A
/// write applies parts in the order this list gives them, and the upward
/// propagation a state can trigger asks the store which container the entity is
/// beneath. Reading the state first would mean a quest created already started
/// beneath a campaign moved nothing, because at the moment its state landed it
/// would have had no parent yet. The same order is used here so that all three
/// messages read the same way.
pub fn arc(c: &v1::ArcContent) -> Result<Written, Malformed> {
    let read = Reader::new(EntityType::Arc, &c.cleared)?;
    let mut parts = Vec::new();
    read.singular(&mut parts, ARC_CAMPAIGN, c.campaign_id.as_ref(), id_value)?;
    read.singular(&mut parts, ARC_POSITION, c.ladder_position, position_value)?;
    read.singular(&mut parts, STATE, c.state, state_value)?;
    Ok(Written::from_specific(parts))
}

/// A quest's content, off the wire.
///
/// **At most one ladder parent in total**, an arc or a campaign, never both:
/// the wire says so with a `oneof` and the table says so with
/// `CHECK (num_nonnulls(arc_id, campaign_id) <= 1)`. The `oneof` means one
/// message can only ever *set* one of the two, so they are separate parts here
/// and setting either clears the other where it is applied.
pub fn quest(c: &v1::QuestContent) -> Result<Written, Malformed> {
    use v1::quest_content::Parent;
    let read = Reader::new(EntityType::Quest, &c.cleared)?;
    let mut parts = Vec::new();
    let arc_id = match &c.parent {
        Some(Parent::ArcId(id)) => Some(id),
        _ => None,
    };
    let campaign_id = match &c.parent {
        Some(Parent::CampaignId(id)) => Some(id),
        _ => None,
    };
    read.singular(&mut parts, QUEST_ARC, arc_id, id_value)?;
    read.singular(&mut parts, QUEST_CAMPAIGN, campaign_id, id_value)?;
    read.singular(&mut parts, QUEST_ROUTINE, c.routine_id.as_ref(), id_value)?;
    read.singular(
        &mut parts,
        QUEST_POSITION,
        c.ladder_position,
        position_value,
    )?;
    read.singular(&mut parts, STATE, c.state, state_value)?;
    Ok(Written::from_specific(parts))
}

// ---------------------------------------------------------------------------
// The ladder
// ---------------------------------------------------------------------------

/// Where an entity sits on the ladder, as its own side table holds it.
#[derive(Debug, Clone, Copy, Default)]
struct Ladder {
    arc: Option<Uuid>,
    campaign: Option<Uuid>,
    position: Option<i32>,
}

impl Ladder {
    /// The parent to climb from, arc first because a quest beneath an arc is
    /// beneath that arc's campaign only through the arc.
    fn parent(self) -> Option<EntityId> {
        self.arc.or(self.campaign).map(EntityId::from_uuid)
    }
}

/// Read one entity's ladder placement.
///
/// A campaign has none — it is a container and never a child — so it answers
/// with the empty placement rather than being refused.
async fn ladder_of(ctx: &mut Ctx<'_>, entity: EntityId, kind: EntityType) -> Applied<Ladder> {
    let sql = match kind {
        EntityType::Arc => {
            "SELECT NULL::uuid AS arc_id, campaign_id, ladder_position FROM arc WHERE entity_id = $1"
        }
        EntityType::Quest => {
            "SELECT arc_id, campaign_id, ladder_position FROM quest WHERE entity_id = $1"
        }
        _ => return Ok(Ladder::default()),
    };
    let row = sqlx::query(sqlx::AssertSqlSafe(sql))
        .bind(entity.as_uuid())
        .fetch_optional(ctx.tx.conn())
        .await?;
    let Some(row) = row else {
        return Ok(Ladder::default());
    };
    Ok(Ladder {
        arc: row.try_get("arc_id")?,
        campaign: row.try_get("campaign_id")?,
        position: row.try_get("ladder_position")?,
    })
}

/// The next position beneath a container, which is one past the last.
///
/// **A campaign's order spans two tables.** Arcs and quests beneath one
/// campaign are one sequence, so the maximum is taken across both and the order
/// cannot be a unique constraint. An arc holds only quests.
///
/// **Deliberately unscoped by audience**, for exactly the reason
/// `store::entity::next_category_position` records: this produces an integer
/// and no content, and a maximum taken over only the entities the writing
/// member can see would hand out a position already held by one they cannot, so
/// two entities would share a position and the order would stop being total.
/// That is a correctness failure produced by applying an access rule to
/// something that is not an access question.
async fn next_ladder_position(
    ctx: &mut Ctx<'_>,
    container: EntityId,
    kind: EntityType,
) -> Applied<i32> {
    let sql = match kind {
        EntityType::Campaign => {
            "SELECT coalesce(max(p), -1) + 1 AS next FROM ( \
               SELECT ladder_position AS p FROM arc WHERE campaign_id = $1 \
               UNION ALL \
               SELECT ladder_position AS p FROM quest WHERE campaign_id = $1 \
             ) AS beneath"
        }
        EntityType::Arc => {
            "SELECT coalesce(max(ladder_position), -1) + 1 AS next FROM quest WHERE arc_id = $1"
        }
        // Nothing else is a ladder container, and the typed foreign keys refuse
        // one. Reached only if this file grows a caller it should not have.
        _ => return Err(Refusal::NotAvailable.into()),
    };
    let row = sqlx::query(sqlx::AssertSqlSafe(sql))
        .bind(container.as_uuid())
        .fetch_one(ctx.tx.conn())
        .await?;
    Ok(row.try_get("next")?)
}

/// The position field of the type whose parent is being written.
fn position_field(kind: EntityType) -> u32 {
    match kind {
        EntityType::Arc => ARC_POSITION,
        _ => QUEST_POSITION,
    }
}

/// **An expiring refusal, in the register of the three Wave 2.1 makes.**
///
/// Wave 2.1 refuses an explicit `category_position` in the same words and for
/// the same reason: appending is the only placement an entry performs, and
/// reordering is a deliberate act on a surface with the container in front of
/// the person. Neither exists until something reorders.
fn explicit_position_refusal() -> Refusal {
    Refusal::Malformed(
        "a position on the ladder is assigned by the instance, which appends on entry to a \
         container; explicit reordering is not served yet"
            .into(),
    )
}

/// The companions a parent part carries with it: the ladder position always,
/// and the parent it vacated where it vacated one.
///
/// **The parent and the position are one movement.** Both of the schema's
/// checks tie them together — an arc's
/// `CHECK ((campaign_id IS NULL) = (ladder_position IS NULL))` and a quest's
/// `CHECK ((num_nonnulls(arc_id, campaign_id) = 0) = (ladder_position IS NULL))`
/// — so writing them in sequence puts the row through a state the schema
/// refuses, whichever order the sequence is in. This is the same shape
/// `write::entity` solves for a category and its position.
///
/// **The vacated parent is a companion too, and it is provenance rather than
/// placement.** A quest has two parent columns and may hold only one, so
/// setting an arc clears the campaign — in the same statement, by the `CASE`
/// in [`apply`], because the row may never hold two. That vacated column is a
/// part like any other: a part that does not record the counter it moved at is
/// invisible to conflict detection, so a concurrent edit to it would merge
/// silently instead of retaining both values. Returning it here is what puts it
/// in provenance. It is already written by the statement that clears it, so it
/// never needs to reach [`apply`].
///
/// **A companion is owed only when the part genuinely moved.** Handing back a
/// vacated parent that was already null would not manufacture a conflict —
/// `provenance::detect` compares rendered text, and arriving equals stored —
/// but it *would* write an `entity_part_counter` row claiming that part moved,
/// and a later unrelated edit would read that as an overlap.
///
/// **The position comes first, and [`write_parent`] checks that rather than
/// trusting it.** The dispatch hands a narrow lane only the first companion, so
/// the one `apply` needs has to lead; the check turns an ordering assumption
/// into a refusal somebody sees.
async fn parent_placement(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
    container_kind: EntityType,
) -> Applied<Vec<SpecificPart>> {
    let SpecificValue::Id(target) = part.value else {
        return Err(Refusal::Malformed("a ladder parent is an identifier".into()).into());
    };
    let ladder = ladder_of(ctx, entity, kind).await?;
    let position = |p: Option<i32>| SpecificPart {
        field: position_field(kind),
        value: SpecificValue::Count(p),
    };

    // The other parent column, which setting this one clears in the same
    // statement, paired with what it holds now. Only a quest has one; an arc's
    // single parent has nothing to displace.
    let sibling = match (kind, container_kind) {
        (EntityType::Quest, EntityType::Arc) => Some((QUEST_CAMPAIGN, ladder.campaign)),
        (EntityType::Quest, EntityType::Campaign) => Some((QUEST_ARC, ladder.arc)),
        _ => None,
    };

    let Some(target) = target else {
        // Leaving a container forgets the position — unless the *other* parent
        // column still holds one. Clearing an arc a quest never had must not
        // strip the position its campaign gave it. Nothing is vacated here:
        // clearing leaves the sibling exactly as it was.
        let held = sibling.and_then(|(_, holds)| holds);
        return Ok(vec![position(held.and(ladder.position))]);
    };

    // The container has to be one this member can see, and it has to be the
    // right kind of container. Both refusals are the same nothing, which is
    // what makes refusal on audience indistinguishable from refusal on
    // nonexistence.
    let target = EntityId::from_uuid(target);
    let found = available_for_write(ctx.tx, ctx.member, target, WriteScope::Active)
        .await?
        .ok_or(Refusal::NotAvailable)?;
    if found.entity_type != container_kind {
        return Err(Refusal::NotAvailable.into());
    }
    // A cycle is not reachable here and needs no check: a campaign has no
    // parent, an arc's only parent is a campaign, and a quest is never a
    // container. The typed foreign keys refuse anything else.

    let already = match container_kind {
        EntityType::Arc => ladder.arc,
        _ => ladder.campaign,
    };
    if already == Some(target.as_uuid()) {
        // Not an entry. The position stands, and is handed back anyway so the
        // parent and the position still move in one statement. Nothing is
        // vacated: the two parents are exclusive, so the sibling is already
        // null.
        return Ok(vec![position(ladder.position)]);
    }

    let next = next_ladder_position(ctx, target, container_kind).await?;
    let mut companions = vec![position(Some(next))];
    if let Some((field, Some(_))) = sibling {
        companions.push(SpecificPart {
            field,
            value: SpecificValue::Id(None),
        });
    }
    Ok(companions)
}

// ---------------------------------------------------------------------------
// The one upward movement
// ---------------------------------------------------------------------------

/// Starting a quest moves the containers above it from waiting to working.
///
/// Every clause of the PRD's narrowing is a line of this function, and
/// `tests/type_content_guidance.rs` has a test per clause:
///
/// - **Waiting to working only, and never in reverse.** [`start_container`]
///   writes nothing unless it finds waiting.
/// - **Upward only.** Nothing here reads a container's children.
/// - **It never touches a parent already in the terminal state**, because a
///   stray child would silently reopen something the person deliberately
///   closed.
/// - **It climbs the whole chain, skipping any container already working**, and
///   a quest attached directly to a campaign moves that campaign because
///   attachment need not be adjacent. The clauses are stated per parent, so a
///   worked arc is left alone *and the climb continues past it*: nothing says a
///   campaign's movement depends on the state of the arc below it.
/// - **A recurrence is not a container for this.** Nothing here walks
///   `routine_id`, so nothing can.
///
/// **Only a quest's own state triggers this.** The PRD names the trigger
/// exactly once — *starting a quest moves the containers above it* — and every
/// clause of the narrowing is written about a quest. An arc set to working by
/// hand therefore moves nothing, and a working quest *moved into* a waiting
/// container moves nothing either: neither is somebody starting work. Both are
/// the drift the PRD accepts as this rule's cost, and closing them would be
/// this file deciding something the PRD did not.
///
/// **The climb stops at a container this member cannot see.** The chain is
/// followed through each container's own row, and moving a state on an entity
/// the writer is not entitled to see would attribute a write — in provenance
/// and in the change sequence — to a member who cannot see what they moved.
///
/// **A container this moves gets a counter and a change entry of its own.** The
/// state is a part like any other: a part that does not record the counter it
/// moved at is invisible to conflict detection, and a movement nothing records
/// is one no client ever learns about.
async fn propagate_start_upward(ctx: &mut Ctx<'_>, quest: EntityId) -> Applied<()> {
    let mut next = ladder_of(ctx, quest, EntityType::Quest).await?.parent();
    while let Some(id) = next {
        let Some(row) = available_for_write(ctx.tx, ctx.member, id, WriteScope::Active).await?
        else {
            break;
        };
        next = match row.entity_type {
            EntityType::Arc => {
                start_container(ctx, &row).await?;
                ladder_of(ctx, id, EntityType::Arc).await?.parent()
            }
            EntityType::Campaign => {
                start_container(ctx, &row).await?;
                None
            }
            // A ladder parent of any other type is not representable: both
            // foreign keys carry the container's type.
            _ => None,
        };
    }
    Ok(())
}

/// Move one container from waiting to working, or leave it exactly as it is.
async fn start_container(ctx: &mut Ctx<'_>, row: &EntityRow) -> Applied<()> {
    let asking = SpecificPart {
        field: STATE,
        value: SpecificValue::State(GuidanceState::DEFAULT),
    };
    let current = read_column(ctx, row.id, row.entity_type, &asking).await?;
    if current.value != SpecificValue::State(GuidanceState::Waiting) {
        // Already working, so there is nothing to do; or already worked, which
        // this movement never touches.
        return Ok(());
    }

    let working = SpecificPart {
        field: STATE,
        value: SpecificValue::State(GuidanceState::Working),
    };
    write_column(ctx, row.id, row.entity_type, &working).await?;

    // The counter advances because this is an accepted write to that entity,
    // and a client holding the old one has to learn its state moved.
    let counter = row.counter + 1;
    sqlx::query("UPDATE entity SET counter = $2, updated_at = $3 WHERE id = $1")
        .bind(row.id.as_uuid())
        .bind(counter)
        .bind(ctx.at)
        .execute(ctx.tx.conn())
        .await?;
    provenance::record(
        ctx.tx,
        row.id,
        &[Part::Specific(STATE)],
        counter,
        ctx.member,
    )
    .await?;

    // Audience did not move, so both sides of the pair are what it already was.
    // The entry still has to exist: to every member who can see this container,
    // its state changed.
    changes::entity_written(
        ctx.tx,
        ctx.at,
        &EntityChange {
            entity: row.id,
            audience_before: Some(row.audience.clone()),
            audience_after: Some(row.audience.clone()),
            author_before: row.author,
            author_after: row.author,
            changed_blocks: Vec::new(),
        },
    )
    .await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// The seams the dispatch calls
// ---------------------------------------------------------------------------

pub async fn validate_and_place(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<Vec<SpecificPart>> {
    match (kind, part.field) {
        // A state owes nothing before it is written. No transition is refused;
        // see the head of this module for why the diagram's undrawn arrow is
        // not a prohibition.
        (EntityType::Campaign | EntityType::Arc | EntityType::Quest, STATE) => Ok(Vec::new()),
        (EntityType::Arc, ARC_CAMPAIGN) => {
            parent_placement(ctx, entity, kind, part, EntityType::Campaign).await
        }
        (EntityType::Quest, QUEST_ARC) => {
            parent_placement(ctx, entity, kind, part, EntityType::Arc).await
        }
        (EntityType::Quest, QUEST_CAMPAIGN) => {
            parent_placement(ctx, entity, kind, part, EntityType::Campaign).await
        }
        // Stored and never interpreted, because the routine it would name has
        // no table to be checked against. See the head of this module.
        (EntityType::Quest, QUEST_ROUTINE) => Ok(Vec::new()),
        (EntityType::Arc, ARC_POSITION) | (EntityType::Quest, QUEST_POSITION) => {
            Err(explicit_position_refusal().into())
        }
        _ => Err(unknown_field(kind, part).into()),
    }
}

pub async fn current(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<SpecificPart> {
    // Every Guidance field is one column of the type's own side table, so the
    // generic reader serves all of them — including the position, which a
    // client may not address but which travels as a companion and needs its
    // stored value read for the same conflict comparison as anything else.
    match (kind, part.field) {
        (EntityType::Campaign, CAMPAIGN_STATE)
        | (EntityType::Arc, STATE | ARC_CAMPAIGN | ARC_POSITION)
        | (
            EntityType::Quest,
            STATE | QUEST_ARC | QUEST_CAMPAIGN | QUEST_ROUTINE | QUEST_POSITION,
        ) => read_column(ctx, entity, kind, part).await,
        _ => Err(unknown_field(kind, part).into()),
    }
}

pub async fn apply(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
    placement: Option<&SpecificPart>,
) -> Applied<()> {
    match (kind, part.field) {
        (EntityType::Campaign | EntityType::Arc, STATE) => {
            // Neither is a child of anything that starts, so neither has a
            // companion and neither propagates.
            debug_assert!(placement.is_none());
            write_column(ctx, entity, kind, part).await
        }
        (EntityType::Quest, STATE) => {
            debug_assert!(placement.is_none());
            write_column(ctx, entity, kind, part).await?;
            if part.value == SpecificValue::State(GuidanceState::Working) {
                propagate_start_upward(ctx, entity).await?;
            }
            Ok(())
        }
        // An arc has one parent and nothing to displace.
        (EntityType::Arc, ARC_CAMPAIGN) => {
            write_parent(ctx, entity, kind, CAMPAIGN_COLUMN, None, part, placement).await
        }
        // Setting one ladder parent clears the other, in the same statement and
        // for the same reason the position moves in it: the row may never hold
        // two. Clearing this one leaves the other alone, because clearing an arc
        // a quest never had is not a request to detach it from its campaign.
        (EntityType::Quest, QUEST_ARC) => {
            write_parent(
                ctx,
                entity,
                kind,
                ARC_COLUMN,
                Some(CAMPAIGN_COLUMN),
                part,
                placement,
            )
            .await
        }
        (EntityType::Quest, QUEST_CAMPAIGN) => {
            write_parent(
                ctx,
                entity,
                kind,
                CAMPAIGN_COLUMN,
                Some(ARC_COLUMN),
                part,
                placement,
            )
            .await
        }
        (EntityType::Quest, QUEST_ROUTINE) => write_column(ctx, entity, kind, part).await,
        // Unreachable: `validate_and_place` refuses a client-stated position
        // before anything is applied, and the instance's own position arrives
        // as the companion above rather than as a part of its own. Refused
        // rather than written, so a future caller finds the rule instead of a
        // silent second path into the column.
        (EntityType::Arc, ARC_POSITION) | (EntityType::Quest, QUEST_POSITION) => {
            Err(explicit_position_refusal().into())
        }
        _ => Err(unknown_field(kind, part).into()),
    }
}

/// Write a ladder parent and the position that belongs with it, in one
/// statement.
///
/// `sibling` is the other parent column where the type has one, cleared in this
/// same statement because the row may never hold two. It is left alone when the
/// addressed parent is being cleared: clearing a parent a quest never had is not
/// a request to detach it from the one it does have.
async fn write_parent(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    column: &'static str,
    sibling: Option<&'static str>,
    part: &SpecificPart,
    placement: Option<&SpecificPart>,
) -> Applied<()> {
    let SpecificValue::Id(parent) = part.value else {
        return Err(Refusal::Malformed("a ladder parent is an identifier".into()).into());
    };
    // **The companion is identified by field number, not by having arrived.**
    // `parent_placement` can return two — the position and a vacated parent —
    // and the dispatch hands a narrow lane only the first, so this checks that
    // what arrived is the position rather than assuming it. An absent or
    // mismatched companion means the ordering there changed, or that this file
    // has grown a second path into the column; either is a refusal somebody
    // sees rather than a position silently written from the wrong part.
    let expected = position_field(kind);
    let position = match placement {
        Some(p) if p.field == expected => match p.value {
            SpecificValue::Count(v) => v,
            _ => {
                return Err(Refusal::Malformed(
                    "a ladder position is a whole number of places".into(),
                )
                .into());
            }
        },
        _ => {
            return Err(Refusal::Malformed(
                "a ladder parent moves with the position that belongs to it".into(),
            )
            .into());
        }
    };
    let table = table_of(kind)?;
    let clear_sibling = match sibling {
        Some(other) => format!("{other} = CASE WHEN $2::uuid IS NULL THEN {other} ELSE NULL END, "),
        None => String::new(),
    };
    let sql = format!(
        "UPDATE {table} SET {column} = $2, {clear_sibling}{LADDER_POSITION_COLUMN} = $3 \
         WHERE entity_id = $1"
    );
    sqlx::query(sqlx::AssertSqlSafe(sql))
        .bind(entity.as_uuid())
        .bind(parent)
        .bind(position)
        .execute(ctx.tx.conn())
        .await?;
    Ok(())
}

/// A number that names no field of this type.
///
/// [`Reader`] refuses a *cleared* number that names nothing before a part is
/// ever built, so reaching this means a part was constructed inside the
/// instance with a number its type does not have.
fn unknown_field(kind: EntityType, part: &SpecificPart) -> Refusal {
    Refusal::Malformed(format!(
        "field {} is not a part of a {}",
        part.field,
        type_name(kind)
    ))
}

// ---------------------------------------------------------------------------
// What an erased ladder container lets go of
// ---------------------------------------------------------------------------

/// Release everything that hung beneath an erased campaign or arc.
///
/// # Erasure only, never removal
///
/// **Removal is recoverable and grouped**, and 2.1 retains the grouping so that
/// restoring one member of an act brings back the rest. A removed campaign is
/// coming back; a child detached on removal would come back parentless with its
/// position forgotten, which quietly destroys the thing the deletion group
/// exists to preserve. Erasure is the case with no prompt, no group and no
/// return — the container is gone for everybody and permanently — which is what
/// makes detaching correct there and only there. 2.1 placed `uncategorise_all`
/// the same way, on the erase path and not on remove.
///
/// The PRD's *"the children survive as standalone quests and arcs"* is about the
/// **declines** branch of a deletion prompt, where the person has chosen that
/// they stand alone. It is not a licence to detach whenever a parent leaves.
///
/// # A detached child is parentless, not promoted
///
/// A quest whose arc is erased does **not** inherit that arc's campaign, even
/// though the campaign usually still exists. Promotion would be the system
/// moving something into a container the person never put it in, and appending
/// it at the end of an order they did not choose — which is the one thing the
/// substrate's Arrangement rules refuse to do anywhere else. The PRD agrees:
/// *"nothing required them to have one in the first place, and they remain
/// reachable rather than stranded inside something that is gone."*
///
/// # Deliberately unscoped
///
/// The reason [`crate::store::entity::uncategorise_all`] gives, and it is no
/// smaller here: **the container is gone for everybody.** Leaving behind the
/// children the erasing member cannot see would keep a dangling reference alive
/// precisely where nobody can find it. Nothing leaves this function that a
/// caller can show anyone — the identities become change entries, and those are
/// assembled per member by the read path, which applies the predicate then.
/// Do not add a predicate here.
///
/// # Why the position moves with the parent
///
/// Same reason as everywhere else in this file: an arc's
/// `CHECK ((campaign_id IS NULL) = (ladder_position IS NULL))` and a quest's
/// `CHECK ((num_nonnulls(arc_id, campaign_id) = 0) = (ladder_position IS NULL))`
/// tie the two together, so a release that cleared only the parent would trip
/// the check on the way. A quest beneath an arc holds no campaign and a quest
/// beneath a campaign holds no arc, so nulling the addressed parent always takes
/// the row to no parent at all, and the position must go with it.
///
/// The counter and the change entry are
/// [`crate::store::entity::detached_from_container`]'s, because a change entry
/// needs the audience and this file may not name that column.
pub async fn detach_contained(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
) -> Applied<Detachment> {
    // Which child tables carry a column naming this kind of container. A
    // campaign is one container over mixed kinds, so it releases across both.
    let children: &[(EntityType, &str)] = match kind {
        EntityType::Campaign => &[
            (EntityType::Arc, CAMPAIGN_COLUMN),
            (EntityType::Quest, CAMPAIGN_COLUMN),
        ],
        EntityType::Arc => &[(EntityType::Quest, ARC_COLUMN)],
        // A quest is a leaf of the ladder: nothing names one as a parent, and
        // nothing ever will.
        _ => return Ok(Detachment::NoContainer),
    };

    let mut released = Vec::new();
    for (child, column) in children {
        let table = table_of(*child)?;
        // **Which parts moved are read out of the declaration, not written
        // here.** A release advances the child's counter, so it owes
        // provenance, and provenance needs the parts. The `FIELDS` lists above
        // already say which field each column holds and `tests/type_content.rs`
        // checks them against the store; naming the numbers again beside the
        // statement would put them in two places, and the copy that drifts is
        // the one nothing checks. The same reasoning that gave `would_cycle`
        // one implementation.
        //
        // **Two columns are cleared, so two parts moved.** The position is not
        // incidental to the parent going away — it is a part of its own, and a
        // part that moves a counter without saying so is invisible to conflict
        // detection. A member holding a base from before the erasure who then
        // moves the child under a new parent writes the position as a
        // companion; with only the parent recorded, the two writes overlap on
        // the position and merge in silence. The membership half of the erase
        // path records exactly this pair for the same reason — *the category
        // and the position within it, and both are recorded, because the
        // counter advanced and something has to be able to say what changed*.
        let parts = [
            part_held_in(*child, column)?,
            part_held_in(*child, LADDER_POSITION_COLUMN)?,
        ];
        let sql = format!(
            "UPDATE {table} SET {column} = NULL, {LADDER_POSITION_COLUMN} = NULL \
             WHERE {column} = $1 RETURNING entity_id"
        );
        let rows = sqlx::query(sqlx::AssertSqlSafe(sql))
            .bind(entity.as_uuid())
            .fetch_all(ctx.tx.conn())
            .await?;
        for r in &rows {
            let child_id = EntityId::from_uuid(r.try_get("entity_id")?);
            // **Both parts genuinely moved, and the schema is what guarantees
            // it.** Elsewhere in this file a companion is owed only when the
            // column it names was actually holding something, because recording
            // a part that did not move is its own silent fault. No runtime check
            // is needed here: an arc's `CHECK ((campaign_id IS NULL) =
            // (ladder_position IS NULL))` and a quest's
            // `CHECK ((num_nonnulls(arc_id, campaign_id) = 0) = (ladder_position
            // IS NULL))` mean a row this statement matched had a parent, and so
            // had a position.
            for part in &parts {
                released.push(Released {
                    entity: child_id,
                    part: part.clone(),
                });
            }
        }
    }
    Ok(Detachment::Released(released))
}

/// Guidance contributes nothing to searchable text.
///
/// A state is a value rather than words, and the ladder's shape is reachable
/// through relations. **LANE: Guidance** — if that turns out to be wrong, this
/// is the seam, and nothing in `entity.rs` needs reopening to use it.
pub async fn search_text(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
) -> Applied<Option<String>> {
    let _ = (ctx, entity, kind);
    Ok(None)
}

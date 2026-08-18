//! Guidance's type content: campaign, arc, quest.
//!
//! **LANE: Guidance owns this file.** Campaign's `state` is filled in as the
//! spine's proof that a type-specific part reaches the store, conflicts, and
//! comes back; arc and quest are declared and refused.
//!
//! # What is deliberately not here
//!
//! **The Guidance state machine.** Which transitions are legal, the silent
//! upward `Waiting → Working` propagation when something beneath starts, and
//! what a state means for the ladder are all the Guidance PRD's and the
//! Guidance lane's. Nothing below checks a transition, and that is not an
//! omission: the spine is proving that a value crosses the boundary intact, and
//! a rule bolted on here would be a rule written by whoever was proving the
//! plumbing.
//!
//! What is settled, and is the plumbing's own answer rather than the domain's:
//! **clearing a state sets it to waiting**, because the column is
//! `NOT NULL DEFAULT 'waiting'` and there is no state a campaign is not in. Two
//! members arriving at waiting — one by setting it, one by clearing it —
//! therefore compare equal rather than conflict, which is the substrate's rule
//! that writes producing the same value are not divergent.

use altair_proto::v1;

use crate::store::entity::EntityType;
use crate::store::ids::EntityId;

use super::super::content::{Malformed, Written};
use super::super::entity::{Applied, Ctx, Refusal};
use super::{
    Field, GuidanceState, Held, Reader, SpecificPart, SpecificValue, not_yet_built, read_column,
    unbuilt, write_column,
};

/// Campaign's field 1, its state. The only part a campaign carries.
pub const CAMPAIGN_STATE: u32 = 1;

pub const CAMPAIGN_FIELDS: &[Field] = &[Field {
    number: CAMPAIGN_STATE,
    held: Held::Column("state"),
}];

pub const ARC_FIELDS: &[Field] = &[
    Field {
        number: 1,
        held: Held::Column("state"),
    },
    Field {
        number: 2,
        held: Held::Column("campaign_id"),
    },
    Field {
        number: 3,
        held: Held::Column("ladder_position"),
    },
];

pub const QUEST_FIELDS: &[Field] = &[
    Field {
        number: 1,
        held: Held::Column("state"),
    },
    Field {
        number: 2,
        held: Held::Column("arc_id"),
    },
    Field {
        number: 3,
        held: Held::Column("campaign_id"),
    },
    Field {
        number: 4,
        held: Held::Column("routine_id"),
    },
    Field {
        number: 5,
        held: Held::Column("ladder_position"),
    },
];

/// A campaign's content, off the wire.
pub fn campaign(c: &v1::CampaignContent) -> Result<Written, Malformed> {
    let read = Reader::new(EntityType::Campaign, &c.cleared)?;
    let mut parts = Vec::new();
    read.singular(&mut parts, CAMPAIGN_STATE, c.state, |v| {
        Ok(SpecificValue::State(match v {
            Some(number) => GuidanceState::from_wire(number)?,
            None => GuidanceState::DEFAULT,
        }))
    })?;
    Ok(Written::from_specific(parts))
}

/// An arc's content, off the wire.
///
/// **LANE: Guidance.** The campaign it hangs beneath and its position on the
/// ladder are here, and the ladder is a container the shared model does not
/// cover: entering one appends, exactly as entering a category does, which is
/// why `campaign_id` and `ladder_position` must move in one statement — the
/// table's `CHECK ((campaign_id IS NULL) = (ladder_position IS NULL))` refuses
/// the state in between. [`validate_and_place`] is where that companion is
/// produced.
pub fn arc(c: &v1::ArcContent) -> Result<Written, Malformed> {
    unbuilt(
        EntityType::Arc,
        &c.cleared,
        &[
            (1, c.state.is_some()),
            (2, c.campaign_id.is_some()),
            (3, c.ladder_position.is_some()),
        ],
    )
}

/// A quest's content, off the wire.
///
/// **LANE: Guidance.** At most one ladder parent in total, an arc or a
/// campaign, never both — the wire says so with a `oneof` and the table says so
/// with `CHECK (num_nonnulls(arc_id, campaign_id) <= 1)`. Moving between them
/// clears the other, and both move with `ladder_position`.
pub fn quest(c: &v1::QuestContent) -> Result<Written, Malformed> {
    unbuilt(
        EntityType::Quest,
        &c.cleared,
        &[
            (1, c.state.is_some()),
            (
                2,
                matches!(c.parent, Some(v1::quest_content::Parent::ArcId(_))),
            ),
            (
                3,
                matches!(c.parent, Some(v1::quest_content::Parent::CampaignId(_))),
            ),
            (4, c.routine_id.is_some()),
            (5, c.ladder_position.is_some()),
        ],
    )
}

pub async fn validate_and_place(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<Option<SpecificPart>> {
    let _ = (ctx, entity);
    match (kind, part.field) {
        // A state owes nothing before it is written. Whether the transition is
        // legal is the Guidance lane's question and is deliberately not asked
        // here; see the note at the head of this module.
        (EntityType::Campaign, CAMPAIGN_STATE) => Ok(None),
        _ => Err(Refusal::Malformed(not_yet_built(kind).0).into()),
    }
}

pub async fn current(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<SpecificPart> {
    match (kind, part.field) {
        (EntityType::Campaign, CAMPAIGN_STATE) => read_column(ctx, entity, kind, part).await,
        _ => Err(Refusal::Malformed(not_yet_built(kind).0).into()),
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
        (EntityType::Campaign, CAMPAIGN_STATE) => {
            // A campaign has no companion part, so there is nothing to write
            // alongside. An arc's or a quest's ladder position will be one.
            debug_assert!(placement.is_none());
            write_column(ctx, entity, kind, part).await
        }
        _ => Err(Refusal::Malformed(not_yet_built(kind).0).into()),
    }
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

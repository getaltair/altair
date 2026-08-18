//! A category's type content: where it nests, and the audience it defaults.
//!
//! **LANE: Knowledge + categories owns this file.**
//!
//! # This file must not issue SQL, and the reason is a guard rather than taste
//!
//! The wire calls a category's creation default
//! `creation_default_audience_member_ids`, which contains the store's audience
//! column as a substring. `tests/one_predicate.rs` refuses any source outside
//! `store/audience.rs` that both names that column and issues SQL, and it is
//! right to: a paraphrased audience predicate is a second implementation
//! whatever it is spelled like, and that is how a leak arrives in month four.
//!
//! This file legitimately names the wire field, because reading the wire is
//! what it is for, and it has nothing to say about who can see anything — a
//! creation default acts once, at the creation of an entity placed in this
//! category, and is never a standing rule and never inherited. So the split is:
//! **this file transcribes, and [`super::write_column`] and
//! [`super::read_column`] run the query**, naming the column only through the
//! `&'static str` in [`CATEGORY_FIELDS`], which is the store's own spelling and
//! not the wire's.
//!
//! Add a bespoke query here and the guard fails on the next run. That is the
//! guard working. Read its module documentation before deciding it is in the
//! way.
//!
//! # What is not decided here
//!
//! **Cycle prevention in nested categories.** Migration one refuses
//! self-reference and records a longer loop as its gap (h).
//! [`super::nesting::would_cycle`] is the check, and it is written once because
//! nested locations need exactly the same one.

use altair_proto::v1;

use crate::store::entity::EntityType;
use crate::store::ids::EntityId;

use super::super::content::{Malformed, Written};
use super::super::entity::{Applied, Ctx, Refusal};
use super::{Field, Held, SpecificPart, not_yet_built, unbuilt};

pub const CATEGORY_FIELDS: &[Field] = &[
    Field {
        number: 1,
        held: Held::Column("parent_category_id"),
    },
    Field {
        number: 2,
        held: Held::Column("creation_default_audience"),
    },
];

/// A category's content, off the wire.
///
/// **LANE: Knowledge + categories.** Wave 2.1 already *reads* the creation
/// default when it places a newly created entity, so filling this in is what
/// makes that path reachable by anything other than a hand-written row.
pub fn category(c: &v1::CategoryContent) -> Result<Written, Malformed> {
    unbuilt(
        EntityType::Category,
        &c.cleared,
        &[
            (1, c.parent_category_id.is_some()),
            (2, !c.creation_default_audience_member_ids.is_empty()),
        ],
    )
}

pub async fn validate_and_place(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    part: &SpecificPart,
) -> Applied<Option<SpecificPart>> {
    let _ = (ctx, entity, part);
    Err(Refusal::Malformed(not_yet_built(EntityType::Category).0).into())
}

pub async fn current(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    part: &SpecificPart,
) -> Applied<SpecificPart> {
    let _ = (ctx, entity, part);
    Err(Refusal::Malformed(not_yet_built(EntityType::Category).0).into())
}

pub async fn apply(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    part: &SpecificPart,
    placement: Option<&SpecificPart>,
) -> Applied<()> {
    let _ = (ctx, entity, part, placement);
    Err(Refusal::Malformed(not_yet_built(EntityType::Category).0).into())
}

/// A category contributes nothing to searchable text.
///
/// Its own title is the shared model's and is already searchable; what it holds
/// is other entities, and each of those carries its own words.
pub async fn search_text(ctx: &mut Ctx<'_>, entity: EntityId) -> Applied<Option<String>> {
    let _ = (ctx, entity);
    Ok(None)
}

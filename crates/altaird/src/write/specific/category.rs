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
//! way. The same rule is why the two checks below reach the store through
//! [`available_for_write`], [`memberships_exist`] and
//! [`nesting::would_cycle`] rather than through a statement of their own.
//!
//! # What a creation default is, and the three things it is not
//!
//! `altair-substrate-spec.md` states it in one sentence and the schema repeats
//! it: **it acts once, at the creation of an entity placed in this category.
//! Not a standing rule, and never inherited.** Each clause is a separate claim,
//! and none of them is enforced here — they are all consequences of where the
//! value is *read*, which is [`super::super::entity`]'s create and nowhere
//! else:
//!
//! - **Once, at creation.** Only the create path asks a category for its
//!   default, so an edit moving an existing entity into this category cannot
//!   apply one.
//! - **Not a standing rule.** The value is copied onto the entity at creation
//!   and never consulted again, so editing it here reaches nothing already
//!   placed.
//! - **Never inherited.** The read is of one row and walks no ancestry, so a
//!   child category with no default of its own has none.
//!
//! Writing it is therefore an ordinary column write, and the care goes into not
//! accidentally making it anything more than that. A category **never sets an
//! audience** as a standing rule, which is exactly why the default is this
//! narrow.
//!
//! # Cycle prevention in nested categories
//!
//! Migration one refuses self-reference and records a longer loop as its gap
//! (h). [`nesting::would_cycle`] is the check, and it is written once because
//! nested locations need exactly the same one.

use altair_proto::v1;

use crate::store::WriteScope;
use crate::store::entity::{EntityType, available_for_write};
use crate::store::ids::EntityId;

use super::super::content::{Malformed, Written, identifier};
use super::super::entity::{Applied, Ctx, Refusal, memberships_exist};
use super::super::parts::Part;
use super::{
    Detachment, Field, Held, Reader, Released, SpecificPart, SpecificValue, nesting, read_column,
    write_column,
};

/// A category's field 1, the category it nests inside.
pub const CATEGORY_PARENT: u32 = 1;

/// A category's field 2, the audience a newly created entity placed here takes
/// when the write states none of its own.
pub const CATEGORY_CREATION_DEFAULT: u32 = 2;

/// The store's spelling of the parent column, named once.
///
/// [`nesting::would_cycle`] is generic over the column so that one walk serves
/// nested categories and nested locations, and it takes a `&'static str` so
/// that nothing a caller sent can ever reach it. This is that string, and it is
/// the same one [`CATEGORY_FIELDS`] declares.
const PARENT_COLUMN: &str = "parent_category_id";

pub const CATEGORY_FIELDS: &[Field] = &[
    Field {
        number: CATEGORY_PARENT,
        held: Held::Column(PARENT_COLUMN),
    },
    Field {
        number: CATEGORY_CREATION_DEFAULT,
        held: Held::Column("creation_default_audience"),
    },
];

/// A category's content, off the wire.
///
/// **LANE: Knowledge + categories.** Wave 2.1 already *reads* the creation
/// default when it places a newly created entity, so filling this in is what
/// makes that path reachable by anything other than a hand-written row.
///
/// Emptying the member list is how the default is cleared, which is the
/// repeated field's rule and not a special case: a category with no default and
/// one whose default was emptied are the same category, and the column is
/// `NOT NULL DEFAULT '{}'` so there is no third state for them to differ in.
pub fn category(c: &v1::CategoryContent) -> Result<Written, Malformed> {
    let read = Reader::new(EntityType::Category, &c.cleared)?;
    let mut parts = Vec::new();
    read.singular(
        &mut parts,
        CATEGORY_PARENT,
        c.parent_category_id.as_deref(),
        |v| Ok(SpecificValue::Id(v.map(identifier).transpose()?)),
    )?;
    read.repeated(
        &mut parts,
        CATEGORY_CREATION_DEFAULT,
        &c.creation_default_audience_member_ids,
        |v| {
            let mut ids = Vec::with_capacity(v.len());
            for member in v {
                ids.push(identifier(member)?);
            }
            Ok(SpecificValue::Members(ids))
        },
    )?;
    Ok(Written::from_specific(parts))
}

/// What a category's parts owe before they are applied.
///
/// **No companion part.** Entering a category places an entity at the end of
/// it, and that is the shared model's `category_id` and `category_position`
/// moving together. Nesting a category inside another is not that: migration
/// one's gap (g) records that nested categories carry no position, because
/// nothing yet states that their contents are hand ordered. So this returns
/// `None` always, and the day something states otherwise is the day this grows
/// a companion, exactly as an arc's ladder position is one.
pub async fn validate_and_place(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    part: &SpecificPart,
) -> Applied<Option<SpecificPart>> {
    match (part.field, &part.value) {
        // Leaving the nesting owes nothing. There is no position to forget, and
        // a category at the top is an ordinary category.
        (CATEGORY_PARENT, SpecificValue::Id(None)) => Ok(None),
        (CATEGORY_PARENT, SpecificValue::Id(Some(parent))) => {
            let parent = EntityId::from_uuid(*parent);
            // The parent has to be one this member can see, and it has to be a
            // category. **Both refusals are the same nothing**, which is the
            // same answer the shared model gives for the category an entity is
            // placed in: a category the member cannot see and one that does not
            // exist are indistinguishable from outside, or the refusal is an
            // oracle for what else the household holds.
            let found = available_for_write(ctx.tx, ctx.member, parent, WriteScope::Active)
                .await?
                .ok_or(Refusal::NotAvailable)?;
            if found.entity_type != EntityType::Category {
                return Err(Refusal::NotAvailable.into());
            }
            // And the nesting has to terminate. The schema refuses the one-step
            // case and can see no further; this is its gap (h), closed where
            // the write is decided because a constraint cannot walk a chain.
            //
            // **Malformed rather than not-available**, and that is not a leak:
            // the member has already been told the parent is there — the check
            // above passed — so saying the nesting would close a loop discloses
            // nothing further, and telling them the category is merely
            // unavailable would send them looking for a permission problem that
            // is not there.
            let table = super::side_table(EntityType::Category)
                .ok_or_else(|| Refusal::Malformed("a category has no table".into()))?;
            if nesting::would_cycle(ctx.tx, table, PARENT_COLUMN, entity, parent).await? {
                return Err(Refusal::Malformed(
                    "nesting this category there would close a loop, and a category cannot \
                     contain itself at any distance"
                        .into(),
                )
                .into());
            }
            Ok(None)
        }
        // **Every member named must answer to a real membership.** A foreign
        // key cannot reach inside an array, which is why the schema lists this
        // among the checks it cannot make. The refusal is the same nothing
        // again: a membership in another household and one that was never
        // created are both simply not there.
        (CATEGORY_CREATION_DEFAULT, SpecificValue::Members(ids)) => {
            if !memberships_exist(ctx.tx, ids).await? {
                return Err(Refusal::NotAvailable.into());
            }
            Ok(None)
        }
        _ => Err(
            Refusal::Malformed(format!("field {} is not a part of a category", part.field)).into(),
        ),
    }
}

pub async fn current(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    part: &SpecificPart,
) -> Applied<SpecificPart> {
    read_column(ctx, entity, EntityType::Category, part).await
}

pub async fn apply(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    part: &SpecificPart,
    placement: Option<&SpecificPart>,
) -> Applied<()> {
    // A category nests without a position, so nothing moves alongside either of
    // its parts. See [`validate_and_place`].
    debug_assert!(placement.is_none());
    write_column(ctx, entity, EntityType::Category, part).await
}

/// A category contributes nothing to searchable text.
///
/// Its own title is the shared model's and is already searchable; what it holds
/// is other entities, and each of those carries its own words. A category is an
/// entity rather than a label — describable, relatable, and returned by
/// retrieval in its own right — and the title is what makes that true.
pub async fn search_text(ctx: &mut Ctx<'_>, entity: EntityId) -> Applied<Option<String>> {
    let _ = (ctx, entity);
    Ok(None)
}

/// Let go of the categories nested inside this one, because it is gone.
///
/// # A category is a container in two senses and only one of them is here
///
/// **Membership** is an entity filed *in* a category, held on the `entity` row
/// as `category_id`, and released by `uncategorise_all` — the shared model's,
/// built in Wave 2.1, and called in the same erase path a few lines further on.
/// **Nesting** is a category sitting *under* another, held in this type's own
/// side table as `parent_category_id`, and released by nobody until now. The
/// erase path's comment names "the ladder and nested locations" and silently
/// omits this third case, which is how it stayed missing.
///
/// The two sets are different and the two releases do not double-count. A
/// category nested under the erased one is reached here, through
/// `parent_category_id`. An entity filed in it is reached there, through
/// `category_id`. A category that is *both* — nested under it and filed in it —
/// is released once by each, of two different columns, and ends up with neither
/// a parent nor a filing, which is exactly right. Nothing is written twice
/// because nothing writes the same column twice.
///
/// # Parentless, not promoted
///
/// A child does not inherit its erased parent's parent. Guidance settled this
/// for the ladder and the reasoning carries without change: promotion would be
/// the system moving something into a container the person never put it in.
/// Being parentless is a complete state — the substrate's rule that *an entity
/// whose category is deleted becomes uncategorised, which is a valid state
/// requiring no repair* is the same rule one level up.
///
/// # Erasure only, never removal
///
/// Removal is recoverable and grouped, and restoring the group is meant to put
/// back what was there. A child detached on removal would come back parentless,
/// so the release would quietly destroy what the deletion group exists to
/// preserve. This is reached from the erase path alone.
///
/// # Unscoped
///
/// [`nesting::detach_children`] applies no audience predicate, and that is
/// deliberate rather than overlooked — see its documentation for the reasoning,
/// which is `uncategorise_all`'s and is no smaller here.
pub async fn detach_contained(ctx: &mut Ctx<'_>, entity: EntityId) -> Applied<Detachment> {
    let table = super::side_table(EntityType::Category)
        .ok_or_else(|| Refusal::Malformed("a category has no table".into()))?;
    let released = nesting::detach_children(ctx.tx, table, PARENT_COLUMN, entity).await?;
    Ok(Detachment::Released(
        released
            .into_iter()
            // The part that moved is this type's field 1. A release advances the
            // released entity's counter, and a write that advances a counter
            // owes provenance; which part moved is the type's knowledge, and a
            // child category lost its parent rather than its shelf.
            .map(|entity| Released {
                entity,
                part: Part::Specific(CATEGORY_PARENT),
            })
            .collect(),
    ))
}

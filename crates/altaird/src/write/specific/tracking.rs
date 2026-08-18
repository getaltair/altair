//! Tracking's type content: item, location, shopping list.
//!
//! **LANE: Tracking owns this file.**
//!
//! Three things about this domain are settled by the schema and the scope
//! rather than by whoever fills the module in, and each is expensive to
//! rediscover:
//!
//! - **An item's amount and the time it was asserted move together.** The
//!   table's `CHECK ((asserted_amount IS NULL) = (asserted_at IS NULL))`
//!   refuses the state in between, so writing them in sequence fails whichever
//!   order the sequence is in. This is the same shape as a category and the
//!   position within it, and [`super::validate_and_place`] is where the
//!   companion part is produced.
//! - **A nested location can close a loop and the schema will not notice.**
//!   Self-reference is refused by a constraint; a longer loop is migration
//!   one's gap (h), and a recursive query would not terminate on one.
//!   [`super::nesting::would_cycle`] is the check, written once because nested
//!   categories need the same one.
//! - **A shopping list's content is deferred in v0**, deliberately rather than
//!   incidentally. `altair-v0-scope.md` says so and governs the implementation
//!   plan where the two disagree, so the field carries an expiring refusal
//!   naming the reason rather than being quietly absent.

use altair_proto::v1;

use crate::store::entity::EntityType;
use crate::store::ids::EntityId;

use super::super::content::{Malformed, Written};
use super::super::entity::{Applied, Ctx, Refusal};
use super::{Field, Held, SpecificPart, not_yet_built, unbuilt};

pub const ITEM_FIELDS: &[Field] = &[
    Field {
        number: 1,
        held: Held::Column("asserted_amount"),
    },
    Field {
        number: 2,
        held: Held::Column("unit"),
    },
    Field {
        number: 3,
        held: Held::Column("asserted_at"),
    },
    Field {
        number: 4,
        held: Held::Column("location_id"),
    },
    Field {
        number: 5,
        held: Held::Column("template_id"),
    },
    Field {
        number: 6,
        held: Held::Rows("entity_property_value"),
    },
];

pub const LOCATION_FIELDS: &[Field] = &[
    Field {
        number: 1,
        held: Held::Column("parent_location_id"),
    },
    Field {
        number: 2,
        held: Held::Column("template_id"),
    },
    Field {
        number: 3,
        held: Held::Rows("entity_property_value"),
    },
];

pub const SHOPPING_LIST_FIELDS: &[Field] = &[Field {
    number: 1,
    held: Held::NotServed(
        "shopping lists are deferred in v0: their entry model leans on an anchor granularity \
         the substrate does not yet define, and a first release should not be the thing that \
         forces that question",
    ),
}];

/// An item's content, off the wire.
///
/// **LANE: Tracking.** The amount is a [`super::Decimal`] and crosses as text
/// on purpose — binary floating point is the wrong place to put a household's
/// stock, and the reading and the range check are already written in
/// [`super::Decimal::from_wire`].
pub fn item(c: &v1::ItemContent) -> Result<Written, Malformed> {
    unbuilt(
        EntityType::Item,
        &c.cleared,
        &[
            (1, c.asserted_amount.is_some()),
            (2, c.unit.is_some()),
            (3, c.asserted_at.is_some()),
            (4, c.location_id.is_some()),
            (5, c.template_id.is_some()),
            (6, !c.property_values.is_empty()),
        ],
    )
}

/// A location's content, off the wire.
///
/// **LANE: Tracking.** `parent_location_id` is the one that owes
/// [`super::nesting::would_cycle`], from [`super::validate_and_place`], before
/// the column is written.
pub fn location(c: &v1::LocationContent) -> Result<Written, Malformed> {
    unbuilt(
        EntityType::Location,
        &c.cleared,
        &[
            (1, c.parent_location_id.is_some()),
            (2, c.template_id.is_some()),
            (3, !c.property_values.is_empty()),
        ],
    )
}

/// A shopping list's content, off the wire.
///
/// Refused with the reason, which expires when the substrate defines the anchor
/// granularity its entries need. The list itself is still a thing that can be
/// created; it is the entries that have nowhere to go.
pub fn shopping_list(c: &v1::ShoppingListContent) -> Result<Written, Malformed> {
    unbuilt(
        EntityType::ShoppingList,
        &c.cleared,
        &[(1, c.body.is_some())],
    )
}

pub async fn validate_and_place(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<Option<SpecificPart>> {
    let _ = (ctx, entity, part);
    Err(Refusal::Malformed(not_yet_built(kind).0).into())
}

pub async fn current(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<SpecificPart> {
    let _ = (ctx, entity, part);
    Err(Refusal::Malformed(not_yet_built(kind).0).into())
}

pub async fn apply(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
    placement: Option<&SpecificPart>,
) -> Applied<()> {
    let _ = (ctx, entity, part, placement);
    Err(Refusal::Malformed(not_yet_built(kind).0).into())
}

/// What Tracking contributes to searchable text.
///
/// **LANE: Tracking.** The person's word for a unit is words rather than a
/// value, so it is the plausible one. Contributing nothing is a valid answer
/// and is what this returns until the lane decides otherwise.
pub async fn search_text(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
) -> Applied<Option<String>> {
    let _ = (ctx, entity, kind);
    Ok(None)
}

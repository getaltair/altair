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
//!   position within it, and [`validate_and_place`] is where the companion part
//!   is produced.
//! - **A nested location can close a loop and the schema will not notice.**
//!   Self-reference is refused by a constraint; a longer loop is migration
//!   one's gap (h), and a recursive query would not terminate on one.
//!   [`super::nesting::would_cycle`] is the check, written once because nested
//!   categories need the same one.
//! - **A shopping list's content is deferred in v0**, deliberately rather than
//!   incidentally. `altair-v0-scope.md` says so and governs the implementation
//!   plan where the two disagree, so the field carries an expiring refusal
//!   naming the reason rather than being quietly absent.
//!
//! # Templates are in force, and that is a reading rather than an assumption
//!
//! Migration one records the ambiguity as its gap (i), at lines 935–938: the
//! scope names Tracking's core as *items, nested locations and quantity*, which
//! reads as an enumeration, while also saying anything not named as deferred is
//! in force — and templates are not named as deferred. **The schema resolved
//! this on the second reading and created the tables**, so this module follows
//! the schema: `template_id` on both types is served, and property values are
//! written. The reasoning is recorded here so the next reader finds it rather
//! than re-arguing it.
//!
//! What a template contributes is names, never content
//! (`proto/altair/v1/altair.proto:465`). So nothing here reads
//! `template_property`: a value is stored under the name it was given, keyed by
//! that name and not by a reference to the template's property, because
//! detaching a template keeps every value with the names it had then
//! (`0001_initial.sql:518-519`, `docs/altair-tracking-prd.md:100`). Detaching is
//! therefore clearing one column and touching nothing else, which is what makes
//! it *never destructive* without any code here saying so.
//!
//! **A property kind is not a constraint.** No lengths, no ranges, nothing
//! required. Every value crosses as text and nothing here interprets it.
//!
//! # What is deliberately not here
//!
//! **Availability.** It is the asserted amount minus what live quantified
//! relations commit, it is derived, and it is never stored
//! (`0001_initial.sql:336-337`, `docs/altair-tracking-prd.md:172`). There is no
//! column for it and this module computes none.
//!
//! **An item's name.** It is the shared model's `title` under this domain's
//! word for it, and not a property of its own
//! (`docs/altair-tracking-prd.md:78`). Duplicating it here would give two places
//! to look and two things to keep in step.
//!
//! **Anything that would make a field required.** No field is required on
//! either path: *an item with a name and nothing else is a complete item*. Every
//! column below is nullable and has no default, and nothing here supplies one.

use sqlx::Row;
use uuid::Uuid;

use altair_proto::v1;

use crate::store::WriteScope;
use crate::store::entity::{EntityType, available_for_write};
use crate::store::ids::EntityId;

use super::super::content::{Malformed, Written, identifier, instant, malformed};
use super::super::entity::{Applied, Ctx, Failed, Refusal, type_name};
use super::{
    Decimal, Field, Held, Property, Reader, SpecificPart, SpecificValue, nesting, read_column,
    read_properties, unbuilt, write_column, write_properties,
};

// ---------------------------------------------------------------------------
// The fields, and the columns spelled once
// ---------------------------------------------------------------------------

/// The columns a statement in this file names directly.
///
/// Every other column reaches the store through [`super::write_column`], which
/// takes the name from the [`Field`] declaration and issues the SQL itself.
/// These cannot: the amount and its timestamp move in one statement, and the
/// unit is read back for searchable text. Naming them through a constant used
/// by both the declaration and the statement is what stops the two drifting,
/// because `tests/type_content.rs` only checks the declaration.
const ASSERTED_AMOUNT: &str = "asserted_amount";
const ASSERTED_AT: &str = "asserted_at";
const UNIT: &str = "unit";

/// The store's spelling of a location's parent column, named once.
///
/// [`nesting::would_cycle`] is generic over the column so that one walk serves
/// nested locations and nested categories, and it takes a `&'static str` so that
/// nothing a caller sent can ever reach it. This is that string, and it is the
/// same one [`LOCATION_FIELDS`] declares. The categories lane names its own the
/// same way, deliberately: two callers of one walk feeding it two different
/// ways is how the walk ends up running against a column one of them renamed.
const PARENT_LOCATION: &str = "parent_location_id";

/// The side table a type keeps its content in.
///
/// Taken from [`super::side_table`] rather than written out, so the three
/// statements in this file that name a table cannot disagree with the one place
/// that decides which table a type has. The `None` arm is unreachable for the
/// types that reach it — both have a table — and is an answer rather than a
/// panic for the same reason everything else here refuses rather than asserts.
fn table_of(kind: EntityType) -> Applied<&'static str> {
    super::side_table(kind).ok_or_else(|| {
        Refusal::Malformed(format!("a {} has no table of its own", type_name(kind))).into()
    })
}

pub const ITEM_AMOUNT: u32 = 1;
pub const ITEM_UNIT: u32 = 2;
pub const ITEM_ASSERTED_AT: u32 = 3;
pub const ITEM_LOCATION: u32 = 4;
pub const ITEM_TEMPLATE: u32 = 5;
pub const ITEM_PROPERTIES: u32 = 6;

pub const ITEM_FIELDS: &[Field] = &[
    Field {
        number: ITEM_AMOUNT,
        held: Held::Column(ASSERTED_AMOUNT),
    },
    Field {
        number: ITEM_UNIT,
        held: Held::Column(UNIT),
    },
    Field {
        number: ITEM_ASSERTED_AT,
        held: Held::Column(ASSERTED_AT),
    },
    Field {
        number: ITEM_LOCATION,
        held: Held::Column("location_id"),
    },
    Field {
        number: ITEM_TEMPLATE,
        held: Held::Column("template_id"),
    },
    Field {
        number: ITEM_PROPERTIES,
        held: Held::Rows("entity_property_value"),
    },
];

pub const LOCATION_PARENT: u32 = 1;
pub const LOCATION_TEMPLATE: u32 = 2;
pub const LOCATION_PROPERTIES: u32 = 3;

pub const LOCATION_FIELDS: &[Field] = &[
    Field {
        number: LOCATION_PARENT,
        held: Held::Column(PARENT_LOCATION),
    },
    Field {
        number: LOCATION_TEMPLATE,
        held: Held::Column("template_id"),
    },
    Field {
        number: LOCATION_PROPERTIES,
        held: Held::Rows("entity_property_value"),
    },
];

/// Why a shopping list's entries are refused, said once.
const SHOPPING_LIST_DEFERRED: &str = "shopping lists are deferred in v0: their entry model leans on an anchor granularity \
     the substrate does not yet define, and a first release should not be the thing that \
     forces that question";

pub const SHOPPING_LIST_FIELDS: &[Field] = &[Field {
    number: 1,
    held: Held::NotServed(SHOPPING_LIST_DEFERRED),
}];

/// Why an item's assertion time is not a field a client states.
///
/// **An expiring refusal, in the register of the three Wave 2.1 makes.** The
/// field exists on the wire and will exist forever — field numbers are
/// permanent — so this says what a client is waiting on rather than that its
/// message was wrong.
///
/// The instance assigns it, for the same reason it assigns a category position:
/// the column is constrained against another column, so the two move in one
/// statement, and [`validate_and_place`] can see only one part at a time. A
/// client free to state the time independently could put the row through the
/// state `CHECK ((asserted_amount IS NULL) = (asserted_at IS NULL))` refuses —
/// by clearing the amount and stating a time in one message, or by stating a
/// time on an item that has no amount — and the answer would be a constraint
/// violation that takes the transaction down rather than a refusal anybody can
/// act on.
///
/// **It expires when a device's own assertion time can cross.** A count taken
/// offline at nine and replayed at six should read nine, not six. Today no
/// client exists to have taken one, and the outbox that will carry it is Wave
/// 4.1's; when it lands this becomes a companion carrying the stated instant
/// rather than a refusal, and the pairing below is already where it goes.
const ASSERTED_AT_IS_THE_INSTANCES: &str = "the time an amount was asserted is the amount's own and moves with it, so the instance \
     sets it when the amount is written; stating it separately is not served yet";

// ---------------------------------------------------------------------------
// Reading the wire
// ---------------------------------------------------------------------------

/// An item's content, off the wire.
///
/// **LANE: Tracking.** The amount is a [`Decimal`] and crosses as text on
/// purpose — binary floating point is the wrong place to put a household's
/// stock, and the reading and the range check are already written in
/// [`Decimal::from_wire`], which refuses rather than clamps.
pub fn item(c: &v1::ItemContent) -> Result<Written, Malformed> {
    let read = Reader::new(EntityType::Item, &c.cleared)?;
    let mut parts = Vec::new();

    read.singular(&mut parts, ITEM_AMOUNT, c.asserted_amount.as_ref(), |v| {
        Ok(SpecificValue::Number(
            v.map(Decimal::from_wire).transpose()?,
        ))
    })?;
    read.singular(&mut parts, ITEM_UNIT, c.unit.as_deref(), |v| {
        Ok(SpecificValue::Text(v.map(str::to_owned)))
    })?;
    read.singular(&mut parts, ITEM_ASSERTED_AT, c.asserted_at.as_ref(), |v| {
        Ok(SpecificValue::Instant(v.map(instant).transpose()?))
    })?;
    read.singular(&mut parts, ITEM_LOCATION, c.location_id.as_deref(), |v| {
        Ok(SpecificValue::Id(v.map(identifier).transpose()?))
    })?;
    read.singular(&mut parts, ITEM_TEMPLATE, c.template_id.as_deref(), |v| {
        Ok(SpecificValue::Id(v.map(identifier).transpose()?))
    })?;
    read.repeated(&mut parts, ITEM_PROPERTIES, &c.property_values, |v| {
        Ok(SpecificValue::Properties(properties(v)?))
    })?;

    Ok(Written::from_specific(parts))
}

/// A location's content, off the wire.
///
/// **LANE: Tracking.** `parent_location_id` is the one that owes
/// [`super::nesting::would_cycle`], from [`validate_and_place`], before the
/// column is written. Nesting exists and is never required; nothing here bounds
/// the depth, and a flat set of locations is complete rather than degraded.
pub fn location(c: &v1::LocationContent) -> Result<Written, Malformed> {
    let read = Reader::new(EntityType::Location, &c.cleared)?;
    let mut parts = Vec::new();

    read.singular(
        &mut parts,
        LOCATION_PARENT,
        c.parent_location_id.as_deref(),
        |v| Ok(SpecificValue::Id(v.map(identifier).transpose()?)),
    )?;
    read.singular(
        &mut parts,
        LOCATION_TEMPLATE,
        c.template_id.as_deref(),
        |v| Ok(SpecificValue::Id(v.map(identifier).transpose()?)),
    )?;
    read.repeated(&mut parts, LOCATION_PROPERTIES, &c.property_values, |v| {
        Ok(SpecificValue::Properties(properties(v)?))
    })?;

    Ok(Written::from_specific(parts))
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

/// The property values one message carries.
///
/// **A kind is not a constraint**, so nothing here consults what a template says
/// a property holds, and no value is interpreted, measured, or refused for its
/// content. The two refusals below are about the message rather than about the
/// person's answer:
///
/// - **A name twice.** The store keys a value by name, so two values under one
///   name are two instructions with no reading that honours both — the same
///   class as a field both present and cleared. The store's own upsert would
///   silently keep whichever came last.
/// - **A name that is nothing.** The name is the key. A value filed under no
///   name could never be found again, and the primary key would happily hold it.
fn properties(values: &[v1::PropertyValue]) -> Result<Vec<Property>, Malformed> {
    let mut out: Vec<Property> = Vec::with_capacity(values.len());
    for v in values {
        if v.name.is_empty() {
            return Err(malformed(
                "a property value is keyed by its name, and this one carries none",
            ));
        }
        if out.iter().any(|p| p.name == v.name) {
            return Err(malformed(format!(
                "the property `{}` is given a value twice, and a value is keyed by its name",
                v.name
            )));
        }
        out.push(Property {
            name: v.name.clone(),
            value: v.value.clone(),
        });
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// What a part owes before it is applied
// ---------------------------------------------------------------------------

pub async fn validate_and_place(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<Option<SpecificPart>> {
    match (kind, part.field) {
        // **The companion.** An amount and the time it was asserted are
        // constrained against each other, so they move in one statement and the
        // instance supplies the time. Asserting an amount stamps now; clearing
        // one clears the stamp with it, because a time an amount was asserted at
        // is not something an item with no amount has.
        //
        // Re-stating the same amount is a real act — *I looked again, still
        // three* — and it moves the stamp while leaving the amount where it was.
        // That is what makes freshness recordable at all, given that a count is
        // only as good as the last time somebody looked.
        //
        // Which is why two people counting the same shelf at once, and
        // agreeing, must not surface a conflict on the stamp: their two
        // `ctx.at` readings genuinely differ and would never compare equal.
        // Nothing here has to arrange that — a placement is not a part the
        // write addressed, so it does not reach conflict detection at all. The
        // rule and the reasoning are in `write::entity::edit`.
        (EntityType::Item, ITEM_AMOUNT) => {
            let asserted = matches!(part.value, SpecificValue::Number(Some(_)));
            Ok(Some(SpecificPart {
                field: ITEM_ASSERTED_AT,
                value: SpecificValue::Instant(asserted.then_some(ctx.at)),
            }))
        }

        // **Not a general modified time.** Renaming an item or correcting its
        // notes does not confirm what is on the shelf, so nothing but the amount
        // moves this, and a client may not move it by hand.
        (EntityType::Item, ITEM_ASSERTED_AT) => {
            Err(Refusal::Malformed(ASSERTED_AT_IS_THE_INSTANCES.into()).into())
        }

        // A location has to be one this member can see, and it has to be a
        // location — the column is a typed foreign key, so pointing at something
        // else is a refusal rather than a cast.
        (EntityType::Item, ITEM_LOCATION) => {
            if let SpecificValue::Id(Some(id)) = part.value {
                must_be(ctx, EntityId::from_uuid(id), EntityType::Location).await?;
            }
            Ok(None)
        }

        (EntityType::Location, LOCATION_PARENT) => {
            if let SpecificValue::Id(Some(id)) = part.value {
                let parent = EntityId::from_uuid(id);
                // Availability answers first, and it answers before anything
                // about ancestry is read. A parent this member cannot see and a
                // parent that does not exist are the same nothing; the cycle
                // refusal below says more than nothing, so it must not be
                // reachable for a parent the member was never entitled to know
                // about.
                must_be(ctx, parent, EntityType::Location).await?;
                let table = table_of(kind)?;
                if nesting::would_cycle(ctx.tx, table, PARENT_LOCATION, entity, parent).await? {
                    return Err(Refusal::Malformed(
                        "a location cannot be nested inside itself, directly or through what \
                         it already contains"
                            .into(),
                    )
                    .into());
                }
            }
            Ok(None)
        }

        // A template is not an entity and carries no audience: one shared set
        // reaches items and locations, and following one is the person's own
        // act. What it owes is existence, checked here so a missing one is a
        // refusal rather than a foreign-key violation that takes the whole
        // transaction down.
        (EntityType::Item, ITEM_TEMPLATE) | (EntityType::Location, LOCATION_TEMPLATE) => {
            if let SpecificValue::Id(Some(id)) = part.value {
                template_exists(ctx, id).await?;
            }
            Ok(None)
        }

        // Words the person wrote. Nothing to check that reading them did not
        // already check.
        (EntityType::Item, ITEM_UNIT | ITEM_PROPERTIES)
        | (EntityType::Location, LOCATION_PROPERTIES) => Ok(None),

        (EntityType::ShoppingList, _) => {
            Err(Refusal::Malformed(SHOPPING_LIST_DEFERRED.into()).into())
        }
        _ => Err(unaddressable(kind, part.field)),
    }
}

/// An entity this member may act on, and of the type the column requires.
///
/// **The three refusals are one.** A location the member cannot see, a location
/// that does not exist, and an identifier naming something that is not a
/// location all answer the same nothing — otherwise the refusal would report the
/// type of a thing the member was never entitled to know existed.
async fn must_be(ctx: &mut Ctx<'_>, id: EntityId, kind: EntityType) -> Applied<()> {
    let found = available_for_write(ctx.tx, ctx.member, id, WriteScope::Active)
        .await?
        .ok_or(Refusal::NotAvailable)?;
    if found.entity_type != kind {
        return Err(Refusal::NotAvailable.into());
    }
    Ok(())
}

/// A template the household has declared.
///
/// Not audience-scoped, because a template is not an entity and carries no
/// audience: it is one shared set reaching items and locations, and remembering
/// which set a template lives in is exactly the recall this product refuses to
/// ask for.
async fn template_exists(ctx: &mut Ctx<'_>, id: Uuid) -> Applied<()> {
    let found = sqlx::query("SELECT id FROM template WHERE id = $1")
        .bind(id)
        .fetch_optional(ctx.tx.conn())
        .await?;
    found.ok_or(Refusal::NotAvailable)?;
    Ok(())
}

/// A field number the type does not have.
///
/// Unreachable through the wire — [`Reader`] validates a number against the type
/// before a part exists — so this is honesty about a part built some other way
/// rather than a case that is expected.
fn unaddressable(kind: EntityType, number: u32) -> Failed {
    Refusal::Malformed(format!(
        "field {number} is not a part of a {}",
        type_name(kind)
    ))
    .into()
}

// ---------------------------------------------------------------------------
// Reading back, and writing
// ---------------------------------------------------------------------------

pub async fn current(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<SpecificPart> {
    match (kind, part.field) {
        // Rows rather than a column, so the generic column reader cannot serve
        // them. The order is the one they compare in, which `read_properties`
        // already puts them in.
        (EntityType::Item, ITEM_PROPERTIES) | (EntityType::Location, LOCATION_PROPERTIES) => {
            Ok(SpecificPart {
                field: part.field,
                value: SpecificValue::Properties(read_properties(ctx, entity).await?),
            })
        }
        // Everything else is one column, read back through the same rendering
        // the arriving value goes through. `ITEM_ASSERTED_AT` is here because
        // the companion is compared like any other part even though no client
        // may state it.
        (
            EntityType::Item,
            ITEM_AMOUNT | ITEM_UNIT | ITEM_ASSERTED_AT | ITEM_LOCATION | ITEM_TEMPLATE,
        )
        | (EntityType::Location, LOCATION_PARENT | LOCATION_TEMPLATE) => {
            read_column(ctx, entity, kind, part).await
        }
        (EntityType::ShoppingList, _) => {
            Err(Refusal::Malformed(SHOPPING_LIST_DEFERRED.into()).into())
        }
        _ => Err(unaddressable(kind, part.field)),
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
        // **One statement, both columns.** The table refuses the state in
        // between, so writing them in sequence fails whichever order the
        // sequence is in.
        (EntityType::Item, ITEM_AMOUNT) => {
            let Some(SpecificPart {
                field: ITEM_ASSERTED_AT,
                value: SpecificValue::Instant(at),
            }) = placement
            else {
                // Unreachable: `validate_and_place` always produces the
                // companion for this part. Refused rather than written alone,
                // because the alternative is a statement the table rejects with
                // a message nobody can act on.
                return Err(Refusal::Malformed(
                    "an amount is written with the time it was asserted, and this write \
                     carries no time"
                        .into(),
                )
                .into());
            };
            let SpecificValue::Number(amount) = &part.value else {
                return Err(unaddressable(kind, part.field));
            };
            // Bound as text and cast, so nothing rounds on the way through —
            // the same route `write_column` takes for a number.
            let table = table_of(kind)?;
            let sql = format!(
                "UPDATE {table} SET {ASSERTED_AMOUNT} = $2::numeric, {ASSERTED_AT} = $3 \
                 WHERE entity_id = $1"
            );
            sqlx::query(sqlx::AssertSqlSafe(sql))
                .bind(entity.as_uuid())
                .bind(amount.as_ref().map(Decimal::text))
                .bind(*at)
                .execute(ctx.tx.conn())
                .await?;
            Ok(())
        }

        (EntityType::Item, ITEM_PROPERTIES) | (EntityType::Location, LOCATION_PROPERTIES) => {
            let SpecificValue::Properties(values) = &part.value else {
                return Err(unaddressable(kind, part.field));
            };
            debug_assert!(placement.is_none());
            write_properties(ctx, entity, values).await
        }

        (EntityType::Item, ITEM_UNIT | ITEM_LOCATION | ITEM_TEMPLATE)
        | (EntityType::Location, LOCATION_PARENT | LOCATION_TEMPLATE) => {
            debug_assert!(placement.is_none());
            write_column(ctx, entity, kind, part).await
        }

        // Refused before it could reach here; see `ASSERTED_AT_IS_THE_INSTANCES`.
        (EntityType::Item, ITEM_ASSERTED_AT) => {
            Err(Refusal::Malformed(ASSERTED_AT_IS_THE_INSTANCES.into()).into())
        }
        (EntityType::ShoppingList, _) => {
            Err(Refusal::Malformed(SHOPPING_LIST_DEFERRED.into()).into())
        }
        _ => Err(unaddressable(kind, part.field)),
    }
}

// ---------------------------------------------------------------------------
// Searchable text
// ---------------------------------------------------------------------------

/// What Tracking contributes to searchable text.
///
/// **LANE: Tracking.** The person's own words, and only those. A unit is what
/// they call it — *tins*, *rolls*, *bottle* — and a property is a name they
/// chose with a value they typed. Those are the words somebody reaches for when
/// looking for the thing, and the literal arm is a permanent arm rather than a
/// fallback, which is what makes a just-captured item findable by them before
/// any derivation runs.
///
/// **Not here:** the amount, the assertion time, and the identifiers of a
/// location or a template. A number and an instant are values rather than words,
/// nothing ever reads an identifier (DR-007), and a location's own title is
/// already searchable on the location.
///
/// Availability is not here either, and not because it is awkward: it is
/// derived and never stored, and putting a derived figure into a person's search
/// text would make the index disagree with itself the moment a relation moved.
pub async fn search_text(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
) -> Applied<Option<String>> {
    let mut words: Vec<String> = Vec::new();

    if kind == EntityType::Item {
        let table = table_of(kind)?;
        let sql = format!("SELECT {UNIT} AS u FROM {table} WHERE entity_id = $1");
        let row = sqlx::query(sqlx::AssertSqlSafe(sql))
            .bind(entity.as_uuid())
            .fetch_optional(ctx.tx.conn())
            .await?;
        if let Some(row) = row
            && let Some(unit) = row.try_get::<Option<String>, _>("u")?
        {
            words.push(unit);
        }
    }

    // A shopping list holds no property values and has no side-table row to read
    // them from. The query would answer empty, but asking it would say this
    // module thought it might not.
    if matches!(kind, EntityType::Item | EntityType::Location) {
        for p in read_properties(ctx, entity).await? {
            words.push(p.name);
            words.push(p.value);
        }
    }

    words.retain(|w| !w.trim().is_empty());
    Ok((!words.is_empty()).then(|| words.join(" ")))
}

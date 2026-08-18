//! Type-specific content: what each type is made of beyond the shared model.
//!
//! # What this module is
//!
//! **The dispatch, and the vocabulary the dispatch is written in.** Every
//! entity type has an arm here, and every arm points at a module that owns one
//! domain's types. A lane building a domain fills in its own module and touches
//! nothing else — not this file, not [`super::content`], not [`super::entity`].
//! That is the whole reason this file exists: the alternative is four lanes
//! editing the same three files, and the merge is where a type quietly loses
//! its arm.
//!
//! # Why a field number and a value, rather than a message
//!
//! A write is scoped to the part that changed, and the substrate decides a
//! conflict at that grain. A type's message is not a part; each of its fields
//! is. So the wire's message is read once, into a list of
//! [`SpecificPart`]s, and everything downstream — provenance, conflict
//! detection, the acknowledgement — deals only in a number and a value. See
//! [`super::parts`] for why the number rather than the name.
//!
//! # Every value crosses as text
//!
//! Both retained sides of a conflict are text whatever the part's kind is (the
//! proto's gap (g), the schema's gap (c)). So every value kind here renders
//! through [`SpecificValue::text`], one arm per kind, and the *same* function
//! renders an arriving value and a stored one. That is what makes *writes
//! producing the same value are not divergent* a comparison rather than a hope.
//!
//! # What a lane implements
//!
//! Six items, all of them in the lane's own module:
//!
//! - `*_FIELDS` — the field numbers the type's message addresses, each saying
//!   where the store holds it. Declared here already, for every type, because
//!   the mapping is the boundary rather than the behaviour and
//!   `tests/write_parts.rs` checks it now.
//! - one reader per message — the wire's set/cleared/untouched rule, applied
//!   through [`Reader`] so the two refusals it owes are not re-derived.
//! - `validate_and_place` — what the part owes before it is applied, and any
//!   companion part the instance moves as a consequence.
//! - `current` — the value the store holds, rendered by the same code.
//! - `apply` — the write.
//! - `search_text` — what the type contributes to the entity's searchable text
//!   beyond its title. Contributing nothing is a valid answer.

pub mod category;
pub mod guidance;
pub mod knowledge;
pub mod nesting;
pub mod tracking;

use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

use altair_proto::v1;

use crate::store::entity::EntityType;
use crate::store::ids::EntityId;

use super::content::{Malformed, Written, malformed};
use super::entity::{Applied, Ctx, Refusal};

// ---------------------------------------------------------------------------
// What a type-specific field is
// ---------------------------------------------------------------------------

/// One field of a type's message, as this instance knows it.
///
/// The list of these per type is what makes *a cleared number naming no field
/// of this type* refusable, and it is what `tests/write_parts.rs` walks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field {
    /// The field number in the type's own message. Permanent, per DR-004.
    pub number: u32,
    /// Where the store keeps it.
    pub held: Held,
}

/// Where the store keeps a type-specific field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Held {
    /// A column of the type's side table. [`write_column`] and [`read_column`]
    /// serve these generically, so a module naming one need issue no SQL.
    Column(&'static str),
    /// The blocks of a body. Not a column and not one part: a body divides,
    /// and each block is a part of its own. See [`super::body`].
    Body,
    /// Rows of a table keyed by the entity. Property values are the only shape
    /// like this; [`read_properties`] and [`write_properties`] serve them.
    Rows(&'static str),
    /// Named by the wire and deliberately not served, with the reason.
    ///
    /// **An expiring refusal, in the register of the three Wave 2.1 already
    /// makes.** The field exists on the wire and will exist forever — field
    /// numbers are permanent — so pretending it is not a field of this type
    /// would tell a client the wrong thing. Addressing it is refused as
    /// malformed with this text as the detail.
    NotServed(&'static str),
}

/// The field a number names within one type's message.
#[must_use]
pub fn field(kind: EntityType, number: u32) -> Option<Field> {
    fields(kind).iter().copied().find(|f| f.number == number)
}

/// Every field of one type's message.
///
/// **A field number valid for one type is not valid for another.** Campaign's
/// field 1 is a state and a location's field 1 is a parent; validating a
/// cleared number against the wrong type would accept a clear the type has no
/// field for.
#[must_use]
pub fn fields(kind: EntityType) -> &'static [Field] {
    match kind {
        EntityType::Campaign => guidance::CAMPAIGN_FIELDS,
        EntityType::Arc => guidance::ARC_FIELDS,
        EntityType::Quest => guidance::QUEST_FIELDS,
        EntityType::Note => knowledge::NOTE_FIELDS,
        EntityType::File => knowledge::FILE_FIELDS,
        EntityType::Item => tracking::ITEM_FIELDS,
        EntityType::Location => tracking::LOCATION_FIELDS,
        EntityType::ShoppingList => tracking::SHOPPING_LIST_FIELDS,
        EntityType::Category => category::CATEGORY_FIELDS,
        // The schema deliberately creates no table for these and the scope
        // defers each until its domain is designed. Their messages reserve
        // 1..=20 and address nothing.
        EntityType::Routine | EntityType::FocusSession | EntityType::CheckIn => &[],
    }
}

/// The table a type keeps its content in, beyond the `entity` row.
///
/// **`None` is a result rather than an omission** for a note and a shopping
/// list: a body is its blocks in order and there is nothing else for a row to
/// hold. The three deferred types have no table because the schema creates
/// none.
///
/// Named here rather than beside each of its three callers — the create that
/// makes the row, the erase that removes it, and the column helpers below —
/// so that a type gaining a table is one line.
#[must_use]
pub fn side_table(kind: EntityType) -> Option<&'static str> {
    match kind {
        EntityType::Campaign => Some("campaign"),
        EntityType::Arc => Some("arc"),
        EntityType::Quest => Some("quest"),
        EntityType::File => Some("file"),
        EntityType::Item => Some("item"),
        EntityType::Location => Some("location"),
        EntityType::Category => Some("category"),
        EntityType::Note
        | EntityType::ShoppingList
        | EntityType::Routine
        | EntityType::FocusSession
        | EntityType::CheckIn => None,
    }
}

// ---------------------------------------------------------------------------
// Values
// ---------------------------------------------------------------------------

/// One type-specific part: which field, and the value a write is setting it to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecificPart {
    /// The field number in the type's message.
    pub field: u32,
    pub value: SpecificValue,
}

impl SpecificPart {
    /// The text this value is retained as when it is one side of a conflict.
    #[must_use]
    pub fn text(&self) -> Option<String> {
        self.value.text()
    }
}

/// The kinds of value a type-specific field can hold.
///
/// One enum rather than one per type, because everything downstream of the
/// read — provenance, conflict detection, the acknowledgement — is written once
/// and must not grow a branch per type. A kind is added here when a type needs
/// one the store cannot already hold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecificValue {
    Text(Option<String>),
    Id(Option<Uuid>),
    Count(Option<i32>),
    Number(Option<Decimal>),
    Instant(Option<DateTime<Utc>>),
    /// **Not optional, because the column is not.** A guidance state is
    /// `NOT NULL DEFAULT 'waiting'`, so clearing the field is setting it back
    /// to waiting rather than to nothing. [`Reader`] resolves a clear to that
    /// default at read time, which is what makes two members arriving at
    /// waiting — one by setting it, one by clearing it — compare equal rather
    /// than conflict.
    State(GuidanceState),
    Members(Vec<Uuid>),
    Properties(Vec<Property>),
}

/// A property value as the person stated it. A kind is not a constraint, so
/// the value crosses as text and nothing interprets it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Property {
    pub name: String,
    pub value: String,
}

/// The state a piece of guidance is in, as the store's enum names them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuidanceState {
    Waiting,
    Working,
    Worked,
}

impl GuidanceState {
    /// What the column holds when nothing has said otherwise.
    pub const DEFAULT: Self = Self::Waiting;

    /// The spelling of the store's `guidance_state` enum.
    #[must_use]
    pub fn store_name(self) -> &'static str {
        match self {
            Self::Waiting => "waiting",
            Self::Working => "working",
            Self::Worked => "worked",
        }
    }

    /// The inverse, for reading a column back.
    #[must_use]
    pub fn from_store_name(name: &str) -> Option<Self> {
        match name {
            "waiting" => Some(Self::Waiting),
            "working" => Some(Self::Working),
            "worked" => Some(Self::Worked),
            _ => None,
        }
    }

    /// The state the wire named.
    ///
    /// `UNSPECIFIED` on a field that is present is a message that cannot be
    /// read: the field says a state was stated and then names none. Absent is
    /// how "untouched" is said, and it never reaches here.
    pub fn from_wire(number: i32) -> Result<Self, Malformed> {
        match v1::GuidanceState::try_from(number) {
            Ok(v1::GuidanceState::Waiting) => Ok(Self::Waiting),
            Ok(v1::GuidanceState::Working) => Ok(Self::Working),
            Ok(v1::GuidanceState::Worked) => Ok(Self::Worked),
            _ => Err(malformed(
                "a guidance state that is present names one of waiting, working, or worked",
            )),
        }
    }
}

/// An exact decimal, as the wire carries one and as the store's `numeric`
/// holds it.
///
/// Binary floating point is the wrong place to put a household's stock, which
/// the proto says and gives the reason for. Kept as the wire's own two numbers
/// so nothing rounds on the way through, and rendered to text for both the
/// store and the conflict row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decimal {
    pub units: i64,
    /// A billionth. Carries the same sign as `units`, as a duration does.
    pub nanos: i32,
}

/// A nanos outside this is not a fraction of one.
const NANOS_LIMIT: i32 = 1_000_000_000;

impl Decimal {
    /// The decimal the wire carried.
    ///
    /// **Refused rather than absorbed**, which is the schema's owed check
    /// *everything the wire shape implies: identifier length, decimal range*.
    /// A nanos of two billion is not a large fraction and units and nanos
    /// disagreeing about sign name no number at all; either is a message that
    /// cannot be read, and clamping one would store something the client never
    /// sent.
    pub fn from_wire(d: &v1::Decimal) -> Result<Self, Malformed> {
        if d.nanos <= -NANOS_LIMIT || d.nanos >= NANOS_LIMIT {
            return Err(malformed("a decimal's nanos are a fraction of one unit"));
        }
        if (d.units > 0 && d.nanos < 0) || (d.units < 0 && d.nanos > 0) {
            return Err(malformed(
                "a decimal's units and nanos carry the same sign, as a duration does",
            ));
        }
        Ok(Self {
            units: d.units,
            nanos: d.nanos,
        })
    }

    /// The decimal a `numeric` column reads back as.
    ///
    /// Refuses a value with more precision than the wire can carry. Nothing can
    /// put one there through this path, so this is honesty about a value that
    /// arrived some other way rather than a case that is expected.
    pub fn parse(text: &str) -> Result<Self, Malformed> {
        let bad = || malformed(format!("`{text}` is not a decimal this instance can carry"));
        let (negative, rest) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text.strip_prefix('+').unwrap_or(text)),
        };
        let (whole, fraction) = rest.split_once('.').unwrap_or((rest, ""));
        if fraction.len() > 9 {
            return Err(bad());
        }
        let units: i64 = whole.parse().map_err(|_| bad())?;
        let mut padded = fraction.to_owned();
        while padded.len() < 9 {
            padded.push('0');
        }
        let nanos: i32 = if padded.is_empty() {
            0
        } else {
            padded.parse().map_err(|_| bad())?
        };
        Ok(Self {
            units: if negative { -units } else { units },
            nanos: if negative { -nanos } else { nanos },
        })
    }

    /// The one rendering, used for the store and for a conflict row alike.
    #[must_use]
    pub fn text(&self) -> String {
        let sign = if self.units < 0 || self.nanos < 0 {
            "-"
        } else {
            ""
        };
        format!(
            "{sign}{}.{:09}",
            self.units.unsigned_abs(),
            self.nanos.unsigned_abs()
        )
    }
}

/// The separators a composite value is rendered with.
///
/// The same two [`super::content`] uses for a labelled date, because a
/// composite value is a composite value and two spellings of the same idea is
/// how a comparison starts returning the wrong answer.
const WITHIN: char = '\u{1f}';
const BETWEEN: char = '\u{1e}';

impl SpecificValue {
    /// The text this value is retained as when it is one side of a conflict.
    ///
    /// `None` means the part holds nothing — cleared, or never set — which is
    /// distinct from holding the empty string.
    #[must_use]
    pub fn text(&self) -> Option<String> {
        match self {
            Self::Text(v) => v.clone(),
            Self::Id(v) => v.map(|id| id.to_string()),
            Self::Count(v) => v.map(|n| n.to_string()),
            Self::Number(v) => v.as_ref().map(Decimal::text),
            Self::Instant(v) => v.map(|t| t.to_rfc3339()),
            Self::State(v) => Some(v.store_name().to_owned()),
            // Sorted, so that two clients listing the same members in different
            // orders are not read as disagreeing. A set has no order and the
            // wire carries it as a list.
            Self::Members(v) => (!v.is_empty()).then(|| {
                let mut ids: Vec<String> = v.iter().map(ToString::to_string).collect();
                ids.sort_unstable();
                ids.join(",")
            }),
            // Sorted by name for the same reason. The name is the key the store
            // holds them under, so the order the wire listed them in carries
            // nothing.
            Self::Properties(v) => (!v.is_empty()).then(|| {
                let mut sorted = v.clone();
                sorted.sort();
                sorted
                    .iter()
                    .map(|p| format!("{}{WITHIN}{}", p.name, p.value))
                    .collect::<Vec<_>>()
                    .join(&BETWEEN.to_string())
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Reading one type's message
// ---------------------------------------------------------------------------

/// Whether and how a message addresses one of its fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Addressed {
    Set,
    Cleared,
    Untouched,
}

/// The set / cleared / untouched rule, one level down.
///
/// The wire's rule is the same in a type's message as it is in
/// `EntityContent`, and so are the two refusals it owes — **a field both
/// present and cleared**, and **a cleared number naming no field** — so they
/// are written once here rather than nine times in the modules below.
///
/// The second refusal is validated **against the type actually present**, not
/// against some union of every type's numbers. Campaign's field 1 is a state
/// and a location's is a parent; a shared list would accept a clear for a
/// field the type does not have.
pub struct Reader {
    kind: EntityType,
    cleared: Vec<u32>,
}

impl Reader {
    /// Reads a type's message, refusing a cleared number that names no field
    /// of it.
    pub fn new(kind: EntityType, cleared: &[u32]) -> Result<Self, Malformed> {
        for number in cleared {
            if field(kind, *number).is_none() {
                return Err(malformed(format!(
                    "cleared names field {number}, which is not a part of this type"
                )));
            }
        }
        Ok(Self {
            kind,
            cleared: cleared.to_vec(),
        })
    }

    /// Whether the message addresses a field, and how.
    ///
    /// `present` is what the wire says: `Some` for a singular field, non-empty
    /// for a repeated one. Both refusals happen here.
    pub fn addressed(&self, number: u32, present: bool) -> Result<Addressed, Malformed> {
        let listed = self.cleared.contains(&number);
        if present && listed {
            return Err(malformed(format!(
                "field {number} is both present and listed as cleared"
            )));
        }
        let addressed = match (present, listed) {
            (true, _) => Addressed::Set,
            (false, true) => Addressed::Cleared,
            (false, false) => Addressed::Untouched,
        };
        // A field the wire names and this instance does not serve. The refusal
        // carries the reason the declaration gives, so a client is told what it
        // is waiting on rather than that its message was wrong.
        if addressed != Addressed::Untouched
            && let Some(Field {
                held: Held::NotServed(why),
                ..
            }) = field(self.kind, number)
        {
            return Err(malformed(why));
        }
        Ok(addressed)
    }

    /// A singular field, read into the value the store will hold.
    ///
    /// `value` is handed the wire's value where the field is set and `None`
    /// where it is cleared, so one closure covers both and a clear cannot
    /// quietly mean something different from an absent value.
    pub fn singular<T>(
        &self,
        into: &mut Vec<SpecificPart>,
        number: u32,
        present: Option<T>,
        value: impl FnOnce(Option<T>) -> Result<SpecificValue, Malformed>,
    ) -> Result<(), Malformed> {
        let part = match self.addressed(number, present.is_some())? {
            Addressed::Set => value(present)?,
            Addressed::Cleared => value(None)?,
            Addressed::Untouched => return Ok(()),
        };
        into.push(SpecificPart {
            field: number,
            value: part,
        });
        Ok(())
    }

    /// A repeated field, where emptied is how it is cleared.
    pub fn repeated<T>(
        &self,
        into: &mut Vec<SpecificPart>,
        number: u32,
        present: &[T],
        value: impl FnOnce(&[T]) -> Result<SpecificValue, Malformed>,
    ) -> Result<(), Malformed> {
        let part = match self.addressed(number, !present.is_empty())? {
            Addressed::Set => value(present)?,
            Addressed::Cleared => value(&[])?,
            Addressed::Untouched => return Ok(()),
        };
        into.push(SpecificPart {
            field: number,
            value: part,
        });
        Ok(())
    }
}

/// A type whose content this build does not write yet.
///
/// `present` is every field of the type's message paired with whether the wire
/// carries it. The cleared list is still validated and a field the type does
/// not serve still refuses with its own reason, because both of those are true
/// whether or not the type is built — a deferral has a reason and *not built*
/// is not it.
///
/// A message addressing nothing is accepted and writes nothing, which is what
/// keeps a bare create of every type working exactly as Wave 2.1 left it.
pub fn unbuilt(
    kind: EntityType,
    cleared: &[u32],
    present: &[(u32, bool)],
) -> Result<Written, Malformed> {
    let read = Reader::new(kind, cleared)?;
    let mut addresses = false;
    for (number, is_present) in present {
        if read.addressed(*number, *is_present)? != Addressed::Untouched {
            addresses = true;
        }
    }
    if addresses {
        return Err(not_yet_built(kind));
    }
    Ok(Written::default())
}

/// The refusal a type whose content is not built yet makes.
///
/// **Distinguishable on purpose.** A stub that silently accepted a write would
/// tell a client its content had landed when nothing had, and the lane filling
/// the module would have no failing test to turn green.
/// `tests/type_content.rs` asserts this is what an unfilled type does.
pub fn not_yet_built(kind: EntityType) -> Malformed {
    malformed(format!(
        "the content of a {} is not built yet, so this instance will not pretend to have \
         written it",
        crate::write::entity::type_name(kind)
    ))
}

// ---------------------------------------------------------------------------
// The dispatch
// ---------------------------------------------------------------------------

/// Every part `content.specific` addresses, and the body it carries.
///
/// **Every type has an arm.** A lane fills in the function its arm points at
/// and never edits this match, which is what keeps four parallel lanes off one
/// file.
pub fn parts_written(specific: &v1::entity_content::Specific) -> Result<Written, Malformed> {
    use v1::entity_content::Specific;
    match specific {
        Specific::Campaign(c) => guidance::campaign(c),
        Specific::Arc(c) => guidance::arc(c),
        Specific::Quest(c) => guidance::quest(c),
        Specific::Note(c) => knowledge::note(c),
        Specific::File(c) => knowledge::file(c),
        Specific::Item(c) => tracking::item(c),
        Specific::Location(c) => tracking::location(c),
        Specific::ShoppingList(c) => tracking::shopping_list(c),
        Specific::Category(c) => category::category(c),
        // The three the schema has no table for. Their messages reserve every
        // number they will ever use and address nothing, so there is nothing to
        // read; a create of one is refused where the row would be made.
        Specific::Routine(_) | Specific::FocusSession(_) | Specific::CheckIn(_) => {
            Ok(Written::default())
        }
    }
}

/// What a type-specific part owes before it is applied, and the companion part
/// the instance moves as a consequence.
///
/// The companion exists for the same reason the shared model's does: a
/// container and a position within it are constrained together, so writing them
/// in sequence puts the row through a state the schema refuses.
pub async fn validate_and_place(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<Option<SpecificPart>> {
    match kind {
        EntityType::Campaign | EntityType::Arc | EntityType::Quest => {
            guidance::validate_and_place(ctx, entity, kind, part).await
        }
        EntityType::Note | EntityType::File => {
            knowledge::validate_and_place(ctx, entity, kind, part).await
        }
        EntityType::Item | EntityType::Location | EntityType::ShoppingList => {
            tracking::validate_and_place(ctx, entity, kind, part).await
        }
        EntityType::Category => category::validate_and_place(ctx, entity, part).await,
        EntityType::Routine | EntityType::FocusSession | EntityType::CheckIn => {
            Err(Refusal::Malformed(not_yet_built(kind).0).into())
        }
    }
}

/// The value the store holds for a part, rendered by the same code the arriving
/// value goes through.
pub async fn current(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<SpecificPart> {
    match kind {
        EntityType::Campaign | EntityType::Arc | EntityType::Quest => {
            guidance::current(ctx, entity, kind, part).await
        }
        EntityType::Note | EntityType::File => knowledge::current(ctx, entity, kind, part).await,
        EntityType::Item | EntityType::Location | EntityType::ShoppingList => {
            tracking::current(ctx, entity, kind, part).await
        }
        EntityType::Category => category::current(ctx, entity, part).await,
        EntityType::Routine | EntityType::FocusSession | EntityType::CheckIn => {
            Err(Refusal::Malformed(not_yet_built(kind).0).into())
        }
    }
}

/// Apply one type-specific part.
///
/// `placement` is whatever [`validate_and_place`] returned, passed through so a
/// module can write a field and its companion in one statement.
pub async fn apply(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
    placement: Option<&SpecificPart>,
) -> Applied<()> {
    match kind {
        EntityType::Campaign | EntityType::Arc | EntityType::Quest => {
            guidance::apply(ctx, entity, kind, part, placement).await
        }
        EntityType::Note | EntityType::File => {
            knowledge::apply(ctx, entity, kind, part, placement).await
        }
        EntityType::Item | EntityType::Location | EntityType::ShoppingList => {
            tracking::apply(ctx, entity, kind, part, placement).await
        }
        EntityType::Category => category::apply(ctx, entity, part, placement).await,
        EntityType::Routine | EntityType::FocusSession | EntityType::CheckIn => {
            Err(Refusal::Malformed(not_yet_built(kind).0).into())
        }
    }
}

/// What a type contributes to the entity's searchable text, beyond its title.
///
/// **Contributing nothing is a valid answer and the common one**, so this is
/// not a stub that refuses. `search_text` is maintained by the write path
/// because a type's content lives in a side table and a generated column
/// cannot reach it; this is the seam a lane fills without reopening
/// [`super::entity`].
pub async fn search_text(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
) -> Applied<Option<String>> {
    match kind {
        EntityType::Campaign | EntityType::Arc | EntityType::Quest => {
            guidance::search_text(ctx, entity, kind).await
        }
        EntityType::Note | EntityType::File => knowledge::search_text(ctx, entity, kind).await,
        EntityType::Item | EntityType::Location | EntityType::ShoppingList => {
            tracking::search_text(ctx, entity, kind).await
        }
        EntityType::Category => category::search_text(ctx, entity).await,
        EntityType::Routine | EntityType::FocusSession | EntityType::CheckIn => Ok(None),
    }
}

// ---------------------------------------------------------------------------
// Reading and writing a column, generically
// ---------------------------------------------------------------------------

/// The column a field is held in, or a refusal saying why it is not one.
fn column_of(kind: EntityType, part: &SpecificPart) -> Applied<(&'static str, &'static str)> {
    let table = side_table(kind).ok_or_else(|| Refusal::Malformed(not_yet_built(kind).0))?;
    match field(kind, part.field) {
        Some(Field {
            held: Held::Column(column),
            ..
        }) => Ok((table, column)),
        Some(Field {
            held: Held::NotServed(why),
            ..
        }) => Err(Refusal::Malformed(why.to_owned()).into()),
        _ => Err(Refusal::Malformed(format!(
            "field {} is not held in a column of this type's table",
            part.field
        ))
        .into()),
    }
}

/// The cast a value's kind needs on its way into a column.
///
/// Two kinds do not bind as themselves. A guidance state is an enum the driver
/// has no type for from a string, and a decimal is carried as text so nothing
/// rounds on the way through.
fn cast(value: &SpecificValue) -> &'static str {
    match value {
        SpecificValue::State(_) => "::guidance_state",
        SpecificValue::Number(_) => "::numeric",
        _ => "",
    }
}

/// Write one value into one column of a type's side table.
///
/// **A module that names an audience-shaped wire field must use this rather
/// than issuing its own SQL** — see the note at the head of [`category`].
pub async fn write_column(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<()> {
    let (table, column) = column_of(kind, part)?;
    let sql = format!(
        "UPDATE {table} SET {column} = $2{} WHERE entity_id = $1",
        cast(&part.value)
    );
    let q = sqlx::query(sqlx::AssertSqlSafe(sql)).bind(entity.as_uuid());
    let q = match &part.value {
        SpecificValue::Text(v) => q.bind(v.clone()),
        SpecificValue::Id(v) => q.bind(*v),
        SpecificValue::Count(v) => q.bind(*v),
        SpecificValue::Instant(v) => q.bind(*v),
        SpecificValue::Members(v) => q.bind(v.clone()),
        SpecificValue::State(v) => q.bind(v.store_name()),
        SpecificValue::Number(v) => q.bind(v.as_ref().map(Decimal::text)),
        SpecificValue::Properties(_) => {
            return Err(
                Refusal::Malformed("property values are rows rather than a column".into()).into(),
            );
        }
    };
    q.execute(ctx.tx.conn()).await?;
    Ok(())
}

/// Read one column back, in the shape of the arriving value.
///
/// The shape comes from `part` for the same reason the shared model's `current`
/// takes the arriving part: the store column alone does not say which of two
/// kinds a value is meant to be compared as.
pub async fn read_column(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<SpecificPart> {
    let (table, column) = column_of(kind, part)?;
    // Both text-carried kinds are read back through the same cast they were
    // written through, so the rendering round-trips rather than depending on
    // the driver's own formatting.
    let projection = match part.value {
        SpecificValue::State(_) | SpecificValue::Number(_) => format!("{column}::text AS v"),
        _ => format!("{column} AS v"),
    };
    let sql = format!("SELECT {projection} FROM {table} WHERE entity_id = $1");
    let row = sqlx::query(sqlx::AssertSqlSafe(sql))
        .bind(entity.as_uuid())
        .fetch_optional(ctx.tx.conn())
        .await?;
    let Some(row) = row else {
        // No side-table row is no value. A create makes the row before any part
        // is applied, so this is a type whose row was never made.
        return Ok(SpecificPart {
            field: part.field,
            value: empty_like(&part.value),
        });
    };
    let value = match &part.value {
        SpecificValue::Text(_) => SpecificValue::Text(row.try_get("v")?),
        SpecificValue::Id(_) => SpecificValue::Id(row.try_get("v")?),
        SpecificValue::Count(_) => SpecificValue::Count(row.try_get("v")?),
        SpecificValue::Instant(_) => SpecificValue::Instant(row.try_get("v")?),
        SpecificValue::Members(_) => SpecificValue::Members(
            row.try_get::<Option<Vec<Uuid>>, _>("v")?
                .unwrap_or_default(),
        ),
        SpecificValue::State(_) => SpecificValue::State(
            row.try_get::<Option<String>, _>("v")?
                .and_then(|s| GuidanceState::from_store_name(&s))
                .unwrap_or(GuidanceState::DEFAULT),
        ),
        SpecificValue::Number(_) => SpecificValue::Number(
            row.try_get::<Option<String>, _>("v")?
                .map(|s| Decimal::parse(&s))
                .transpose()?,
        ),
        SpecificValue::Properties(_) => {
            return Err(
                Refusal::Malformed("property values are rows rather than a column".into()).into(),
            );
        }
    };
    Ok(SpecificPart {
        field: part.field,
        value,
    })
}

/// The same kind of value, holding nothing.
fn empty_like(value: &SpecificValue) -> SpecificValue {
    match value {
        SpecificValue::Text(_) => SpecificValue::Text(None),
        SpecificValue::Id(_) => SpecificValue::Id(None),
        SpecificValue::Count(_) => SpecificValue::Count(None),
        SpecificValue::Number(_) => SpecificValue::Number(None),
        SpecificValue::Instant(_) => SpecificValue::Instant(None),
        SpecificValue::State(_) => SpecificValue::State(GuidanceState::DEFAULT),
        SpecificValue::Members(_) => SpecificValue::Members(Vec::new()),
        SpecificValue::Properties(_) => SpecificValue::Properties(Vec::new()),
    }
}

/// Every property value an entity holds, in the one order they compare in.
pub async fn read_properties(ctx: &mut Ctx<'_>, entity: EntityId) -> Applied<Vec<Property>> {
    let rows = sqlx::query("SELECT name, value FROM entity_property_value WHERE entity_id = $1")
        .bind(entity.as_uuid())
        .fetch_all(ctx.tx.conn())
        .await?;
    let mut out = Vec::with_capacity(rows.len());
    for r in &rows {
        out.push(Property {
            name: r.try_get("name")?,
            value: r.try_get::<Option<String>, _>("value")?.unwrap_or_default(),
        });
    }
    out.sort();
    Ok(out)
}

/// Replace an entity's property values whole.
///
/// Replaced rather than merged, because the wire carries the list and a list is
/// the value. Anything nested is replaced whole; the proto says so of a
/// routine's occurrence description and the reasoning is the same here.
pub async fn write_properties(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    values: &[Property],
) -> Applied<()> {
    sqlx::query("DELETE FROM entity_property_value WHERE entity_id = $1")
        .bind(entity.as_uuid())
        .execute(ctx.tx.conn())
        .await?;
    for p in values {
        sqlx::query(
            "INSERT INTO entity_property_value (entity_id, name, value) VALUES ($1, $2, $3) \
             ON CONFLICT (entity_id, name) DO UPDATE SET value = excluded.value",
        )
        .bind(entity.as_uuid())
        .bind(&p.name)
        .bind(&p.value)
        .execute(ctx.tx.conn())
        .await?;
    }
    Ok(())
}

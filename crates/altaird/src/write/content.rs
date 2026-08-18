//! Reading `EntityContent` off the wire, and saying what a write touched.
//!
//! # Set, cleared, untouched
//!
//! The wire's rule is the same in every message that holds parts:
//!
//! ```text
//! present            set to this value
//! listed in cleared  cleared, or emptied where the field is repeated
//! neither            untouched
//! ```
//!
//! Two of the schema's owed checks live here and nowhere else, because this is
//! the only place that can see them: **a field both present and cleared**, and
//! **a cleared number naming no field**. Both are malformed — the message
//! cannot be read as a well-formed intent — and neither is something a later
//! layer can recover, because by then the ambiguity has been resolved one way
//! or the other by whoever wrote the code.
//!
//! # Why a part carries its own text
//!
//! Both retained sides of a conflict cross as text whatever the part's kind is
//! (the proto's gap (g), the schema's gap (c)). So conflict detection compares
//! text, which means the rendering has to be stable and has to be produced the
//! same way for an arriving value and for a stored one. Producing it here, from
//! one function per part, is what makes *writes producing the same value are
//! not divergent* a comparison rather than a hope.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use altair_proto::v1;

use crate::store::entity::EntityType;
use crate::store::ids::EntityId;

use super::parts::{ContentPart, Part};

/// A labelled date, as the store holds one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Date {
    pub label: String,
    pub occurs_at: DateTime<Utc>,
    pub bring_forward: bool,
}

/// One part of `EntityContent`, with the value a write is setting it to.
///
/// Clearing is `None` at the inner level for a singular field and an empty
/// vector for a repeated one, which is the wire's own rule rather than a
/// convention invented here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartWrite {
    Title(Option<String>),
    Dates(Vec<Date>),
    Category(Option<EntityId>),
    CategoryPosition(Option<i32>),
    Assignments(Vec<Uuid>),
    Audience(Vec<Uuid>),
    Bulk(Option<bool>),
}

impl PartWrite {
    #[must_use]
    pub fn part(&self) -> Part {
        Part::Content(match self {
            Self::Title(_) => ContentPart::Title,
            Self::Dates(_) => ContentPart::Dates,
            Self::Category(_) => ContentPart::Category,
            Self::CategoryPosition(_) => ContentPart::CategoryPosition,
            Self::Assignments(_) => ContentPart::Assignments,
            Self::Audience(_) => ContentPart::Audience,
            Self::Bulk(_) => ContentPart::Bulk,
        })
    }

    /// The text this value is retained as when it is one side of a conflict.
    ///
    /// `None` means the part holds nothing — cleared, or never set — which is
    /// distinct from holding the empty string.
    #[must_use]
    pub fn text(&self) -> Option<String> {
        match self {
            Self::Title(v) => v.clone(),
            Self::Dates(v) => (!v.is_empty()).then(|| {
                v.iter()
                    .map(|d| {
                        format!(
                            "{}\u{1f}{}\u{1f}{}",
                            d.label,
                            d.occurs_at.to_rfc3339(),
                            d.bring_forward
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\u{1e}")
            }),
            Self::Category(v) => v.map(|c| c.as_uuid().to_string()),
            Self::CategoryPosition(v) => v.map(|p| p.to_string()),
            // Sorted, so that two clients listing the same members in different
            // orders are not read as disagreeing. A set has no order and the
            // wire carries it as a list.
            Self::Assignments(v) | Self::Audience(v) => (!v.is_empty()).then(|| {
                let mut ids: Vec<String> = v.iter().map(ToString::to_string).collect();
                ids.sort_unstable();
                ids.join(",")
            }),
            Self::Bulk(v) => v.map(|b| b.to_string()),
        }
    }
}

/// Why a piece of content could not be read.
#[derive(Debug, Clone)]
pub struct Malformed(pub String);

fn malformed(what: impl Into<String>) -> Malformed {
    Malformed(what.into())
}

/// A sixteen-byte identifier, or nothing.
///
/// Length is one of the checks the schema names as *everything the wire shape
/// implies*. An identifier of the wrong length is not a short identifier; it is
/// a message that cannot be read.
pub fn identifier(bytes: &[u8]) -> Result<Uuid, Malformed> {
    let sized: [u8; 16] = bytes.try_into().map_err(|_| {
        malformed(format!(
            "an identifier is 16 bytes, this one is {}",
            bytes.len()
        ))
    })?;
    Ok(Uuid::from_bytes(sized))
}

/// The type an entity is, which is the tag of `content.specific`.
///
/// There is no parallel type field for it to disagree with, so an absent tag is
/// an entity of no type, which nothing can store.
pub fn entity_type(content: &v1::EntityContent) -> Result<EntityType, Malformed> {
    use v1::entity_content::Specific;
    match content.specific.as_ref() {
        Some(Specific::Campaign(_)) => Ok(EntityType::Campaign),
        Some(Specific::Arc(_)) => Ok(EntityType::Arc),
        Some(Specific::Quest(_)) => Ok(EntityType::Quest),
        Some(Specific::Routine(_)) => Ok(EntityType::Routine),
        Some(Specific::FocusSession(_)) => Ok(EntityType::FocusSession),
        Some(Specific::CheckIn(_)) => Ok(EntityType::CheckIn),
        Some(Specific::Note(_)) => Ok(EntityType::Note),
        Some(Specific::File(_)) => Ok(EntityType::File),
        Some(Specific::Item(_)) => Ok(EntityType::Item),
        Some(Specific::Location(_)) => Ok(EntityType::Location),
        Some(Specific::ShoppingList(_)) => Ok(EntityType::ShoppingList),
        Some(Specific::Category(_)) => Ok(EntityType::Category),
        None => Err(malformed(
            "the type is the tag of content.specific, and this content carries none",
        )),
    }
}

/// Every part this content addresses, in the order the message declares them.
///
/// # What it refuses
///
/// - A field both present and cleared. The two instructions contradict each
///   other and there is no reading that honours both.
/// - A cleared number naming no addressable part. A reserved number, an unknown
///   one, or `cleared` itself. Silently ignoring it would let a client believe
///   it had cleared something.
///
/// # What it does not do
///
/// It does not reach `content.specific`. Type content is Wave 2.2's, and 2.1
/// creates each type's row with defaults so nothing is ever half-formed. A
/// create carrying type content therefore lands as an entity of that type whose
/// type-specific fields hold their defaults, and the fields themselves are not
/// applied. That is the plan's division rather than an omission here, and it is
/// invisible in v0 because no client exists before Wave 4.
pub fn parts_written(content: &v1::EntityContent) -> Result<Vec<PartWrite>, Malformed> {
    let mut cleared = Vec::new();
    for number in &content.cleared {
        let part = ContentPart::from_field_number(*number).ok_or_else(|| {
            malformed(format!("cleared names field {number}, which is not a part"))
        })?;
        cleared.push(part);
    }
    let is_cleared = |p: ContentPart| cleared.contains(&p);

    let both = |p: ContentPart| {
        malformed(format!(
            "field {} is both present and listed as cleared",
            p.field_number()
        ))
    };

    let mut written = Vec::new();

    if let Some(title) = &content.title {
        if is_cleared(ContentPart::Title) {
            return Err(both(ContentPart::Title));
        }
        written.push(PartWrite::Title(Some(title.clone())));
    } else if is_cleared(ContentPart::Title) {
        written.push(PartWrite::Title(None));
    }

    if !content.dates.is_empty() {
        if is_cleared(ContentPart::Dates) {
            return Err(both(ContentPart::Dates));
        }
        let mut dates = Vec::with_capacity(content.dates.len());
        for d in &content.dates {
            let when = d
                .when
                .as_ref()
                .and_then(|t| {
                    DateTime::from_timestamp(t.seconds, u32::try_from(t.nanos).unwrap_or(0))
                })
                .ok_or_else(|| malformed("a labelled date carries no usable instant"))?;
            dates.push(Date {
                label: d.label.clone(),
                occurs_at: when,
                bring_forward: d.bring_forward,
            });
        }
        written.push(PartWrite::Dates(dates));
    } else if is_cleared(ContentPart::Dates) {
        written.push(PartWrite::Dates(Vec::new()));
    }

    if let Some(id) = &content.category_id {
        if is_cleared(ContentPart::Category) {
            return Err(both(ContentPart::Category));
        }
        written.push(PartWrite::Category(Some(EntityId::from_uuid(identifier(
            id,
        )?))));
    } else if is_cleared(ContentPart::Category) {
        written.push(PartWrite::Category(None));
    }

    if let Some(p) = content.category_position {
        if is_cleared(ContentPart::CategoryPosition) {
            return Err(both(ContentPart::CategoryPosition));
        }
        written.push(PartWrite::CategoryPosition(Some(
            i32::try_from(p).map_err(|_| malformed("a position that large is not one"))?,
        )));
    } else if is_cleared(ContentPart::CategoryPosition) {
        written.push(PartWrite::CategoryPosition(None));
    }

    if !content.assigned_member_ids.is_empty() {
        if is_cleared(ContentPart::Assignments) {
            return Err(both(ContentPart::Assignments));
        }
        let mut ids = Vec::new();
        for m in &content.assigned_member_ids {
            ids.push(identifier(m)?);
        }
        written.push(PartWrite::Assignments(ids));
    } else if is_cleared(ContentPart::Assignments) {
        written.push(PartWrite::Assignments(Vec::new()));
    }

    if !content.audience_member_ids.is_empty() {
        if is_cleared(ContentPart::Audience) {
            return Err(both(ContentPart::Audience));
        }
        let mut ids = Vec::new();
        for m in &content.audience_member_ids {
            ids.push(identifier(m)?);
        }
        written.push(PartWrite::Audience(ids));
    } else if is_cleared(ContentPart::Audience) {
        written.push(PartWrite::Audience(Vec::new()));
    }

    if let Some(b) = content.bulk {
        if is_cleared(ContentPart::Bulk) {
            return Err(both(ContentPart::Bulk));
        }
        written.push(PartWrite::Bulk(Some(b)));
    } else if is_cleared(ContentPart::Bulk) {
        written.push(PartWrite::Bulk(None));
    }

    Ok(written)
}

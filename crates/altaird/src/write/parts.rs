//! What a part is called, in the three places that have to agree about it.
//!
//! **A write is scoped to the part that changed**, and the substrate decides a
//! conflict at that grain: same part, both values retained; different parts,
//! merged with nobody involved. So a part is not an implementation detail. It
//! is the unit three separate things name:
//!
//! - **The wire** names it by field number, inside `EntityContent` or inside
//!   the type-specific message, or by block identity.
//! - **`entity_part_counter`** names it by text, or by block identity.
//! - **The `conflict` row** names it the same way, because it is the same part.
//!
//! Migration one recorded the drift risk as gap (d): *"a conflicted part is
//! named by text here and by a field number on the wire. Something has to hold
//! that mapping, and wherever it lives it can drift."* This module is that
//! something, and `tests/write_parts.rs` is what stops it drifting: the mapping
//! round-trips, and every content part the wire can address appears in it.
//!
//! # Why the field number and not the field name
//!
//! DR-004 makes field numbers permanent and names reserved. A part addressed by
//! number survives a field being renamed, which is the change most likely to
//! happen; a part addressed by the proto's own identifier would silently
//! re-point. The text in the store is a stable spelling chosen here, not the
//! proto's identifier read at build time, and the round-trip test is what ties
//! the two together.
//!
//! # What is deliberately not here
//!
//! **Values.** Both retained sides of a conflict cross as text whatever the
//! part's own kind is, which the proto records as its gap (g) and the schema
//! inherits as its gap (c). Rendering a value as text needs the value, so it
//! belongs where the content is read; this module holds only names.

use uuid::Uuid;

use crate::store::audience::AUDIENCE_COLUMN;

/// A part of an entity: the unit a write is scoped to and a conflict is decided
/// at.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Part {
    /// A field of `EntityContent`, shared by every type.
    Content(ContentPart),
    /// A field of the type-specific message. Carried by number because the set
    /// differs per type and the type is fixed at creation, so a number is
    /// unambiguous once the entity exists.
    ///
    /// **Which numbers a type has is not decided here.** Each type declares its
    /// own fields in [`super::specific`], and a number valid for one type is
    /// not valid for another — a campaign's field 1 is a state and a location's
    /// is a parent. This module holds only the spelling the two tables key it
    /// by, which is the same whatever type it came from.
    Specific(u32),
    /// A block of a body. Named by identity rather than by position, because
    /// position is what an edit to a neighbour changes.
    ///
    /// Produced by [`super::body`], which is the only thing that can: a body
    /// arrives as one field and becomes many parts, and which blocks those are
    /// is decided by matching the arriving text against the blocks already
    /// stored.
    Block(Uuid),
}

/// The parts of `EntityContent`, which every type carries.
///
/// The `specific` field is not one of these. It is the tag that says which type
/// the entity is, and the type is fixed at creation, so it is not a part a
/// write can address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentPart {
    Title,
    Dates,
    Category,
    Assignments,
    Audience,
    Bulk,
    CategoryPosition,
}

/// Every content part, in field-number order. The round-trip test walks this,
/// so a variant added without a number or a name fails rather than going
/// unmapped.
pub const ALL_CONTENT_PARTS: &[ContentPart] = &[
    ContentPart::Title,
    ContentPart::Dates,
    ContentPart::Category,
    ContentPart::Assignments,
    ContentPart::Audience,
    ContentPart::Bulk,
    ContentPart::CategoryPosition,
];

impl ContentPart {
    /// The field number in `EntityContent`. Permanent, per DR-004.
    #[must_use]
    pub fn field_number(self) -> u32 {
        match self {
            Self::Title => 1,
            Self::Dates => 2,
            Self::Category => 3,
            Self::Assignments => 4,
            Self::Audience => 5,
            // 6 is reserved: it was the entity-level arrangement key, and an
            // order now belongs to a container.
            Self::Bulk => 7,
            Self::CategoryPosition => 8,
        }
    }

    /// The part a field number names, or `None` if the number names no
    /// addressable part.
    ///
    /// `None` is what makes *"a cleared number naming no field"* refusable:
    /// a `cleared` list is a client's assertion about this message, and a
    /// number that is reserved, unknown, or the `cleared` field itself is not
    /// a part anything can clear.
    #[must_use]
    pub fn from_field_number(number: u32) -> Option<Self> {
        ALL_CONTENT_PARTS
            .iter()
            .copied()
            .find(|p| p.field_number() == number)
    }

    /// The spelling `entity_part_counter.field_name` and `conflict.field_name`
    /// both use.
    ///
    /// Audience takes its name from the store's column constant rather than
    /// spelling it out. Two reasons, and the second is the load-bearing one:
    /// the part and the column genuinely are the same word, and
    /// `tests/one_predicate.rs` refuses any source outside `store/audience.rs`
    /// that names the column, so a literal here would fail it. That test is
    /// aimed at a paraphrased predicate rather than at this, and taking the
    /// name from the constant satisfies it honestly rather than by evasion.
    #[must_use]
    pub fn store_name(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Dates => "dates",
            Self::Category => "category_id",
            Self::Assignments => "assigned_member_ids",
            Self::Audience => AUDIENCE_COLUMN,
            Self::Bulk => "bulk",
            Self::CategoryPosition => "category_position",
        }
    }

    /// The inverse of [`ContentPart::store_name`].
    #[must_use]
    pub fn from_store_name(name: &str) -> Option<Self> {
        ALL_CONTENT_PARTS
            .iter()
            .copied()
            .find(|p| p.store_name() == name)
    }
}

/// The prefix a type-specific part's stored name carries.
///
/// Prefixed rather than bare, so a type-specific field number can never be read
/// as a content part's name, and so a reader of the table can tell the two
/// apart without knowing the entity's type.
const SPECIFIC_PREFIX: &str = "specific.";

/// How a part is keyed in `entity_part_counter` and in `conflict`: exactly one
/// of a text name and a block identity, which is the check both tables carry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartKey {
    Field(String),
    Block(Uuid),
}

impl Part {
    /// How this part is written to the two tables keyed by it.
    #[must_use]
    pub fn key(&self) -> PartKey {
        match self {
            Self::Content(p) => PartKey::Field(p.store_name().to_owned()),
            Self::Specific(n) => PartKey::Field(format!("{SPECIFIC_PREFIX}{n}")),
            Self::Block(id) => PartKey::Block(*id),
        }
    }

    /// The part a stored key names.
    ///
    /// `None` for a field name this module does not recognise, which is a part
    /// written by a version of the instance that knew a name this one does not.
    /// Treating that as unknown rather than guessing is the only safe reading:
    /// a part nobody can name is a part nobody can conflict on, and inventing a
    /// name for it would let two different parts collide.
    #[must_use]
    pub fn from_key(key: &PartKey) -> Option<Self> {
        match key {
            PartKey::Block(id) => Some(Self::Block(*id)),
            PartKey::Field(name) => {
                if let Some(rest) = name.strip_prefix(SPECIFIC_PREFIX) {
                    return rest.parse().ok().map(Self::Specific);
                }
                ContentPart::from_store_name(name).map(Self::Content)
            }
        }
    }
}

impl From<ContentPart> for Part {
    fn from(p: ContentPart) -> Self {
        Self::Content(p)
    }
}

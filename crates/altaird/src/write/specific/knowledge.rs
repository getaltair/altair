//! Knowledge's type content: note, file.
//!
//! **LANE: Knowledge owns this file.**
//!
//! A note is the one type whose whole content is a body, and a body is not a
//! part: it divides into blocks and each block is a part of its own. So the
//! wire reading below produces a [`BodyWrite`] rather than a
//! [`SpecificPart`], and everything downstream of it lives in
//! [`super::super::body`] — division, matching against the blocks already held,
//! and writing only what changed. That split is deliberate: reading the wire
//! needs no store and the matching needs nothing else.
//!
//! A file's content waits on Wave 2.3 in full, because Wave 2.1 refuses a file
//! create for a reason that has not expired: the schema requires a file to name
//! a body and there is no way to have uploaded one.

use altair_proto::v1;

use crate::store::entity::EntityType;
use crate::store::ids::EntityId;

use super::super::content::{BodyWrite, Malformed, Written};
use super::super::entity::{Applied, Ctx, Refusal};
use super::{Addressed, Field, Held, Reader, SpecificPart, not_yet_built, unbuilt};

/// A note's field 1, its body.
pub const NOTE_BODY: u32 = 1;

pub const NOTE_FIELDS: &[Field] = &[Field {
    number: NOTE_BODY,
    held: Held::Body,
}];

pub const FILE_FIELDS: &[Field] = &[
    Field {
        number: 1,
        held: Held::Column("body_id"),
    },
    Field {
        number: 2,
        held: Held::Column("media_type"),
    },
    // A person's correction to extracted text, which has nothing to correct.
    // `altair-v0-scope.md` defers text extraction from files, and that document
    // governs the implementation plan where the two disagree.
    Field {
        number: 3,
        held: Held::NotServed(
            "text extraction from files is deferred in v0, so there is no extracted text for a \
             correction to outrank",
        ),
    },
];

/// A note's content, off the wire.
///
/// The whole body arrives as one field, plain markdown with no relation markers
/// in it, and the instance divides it — DR-004, so the division rule has one
/// implementation and devices cannot disagree about the units reconciliation is
/// decided in. Nothing here divides anything; it says only that a body arrived
/// and what it says.
pub fn note(c: &v1::NoteContent) -> Result<Written, Malformed> {
    let read = Reader::new(EntityType::Note, &c.cleared)?;
    let body = match read.addressed(NOTE_BODY, c.body.is_some())? {
        Addressed::Set => Some(BodyWrite(c.body.clone())),
        // Clearing a body is a body of no text, which divides into no blocks.
        // Distinct from an untouched body, which says nothing about the blocks
        // already held.
        Addressed::Cleared => Some(BodyWrite(None)),
        Addressed::Untouched => None,
    };
    Ok(Written {
        parts: Vec::new(),
        body,
    })
}

/// A file's content, off the wire.
///
/// **LANE: 2.3, file bodies.** `body_id` names bytes already uploaded through
/// `PutBody`; until that call is served there is no identity to name, which is
/// why 2.1 refuses a file create outright. The column is `NOT NULL`, so
/// clearing it is not a thing an edit can do and this module must say so rather
/// than letting the store raise a fault.
pub fn file(c: &v1::FileContent) -> Result<Written, Malformed> {
    unbuilt(
        EntityType::File,
        &c.cleared,
        &[
            (1, c.body_id.is_some()),
            (2, c.media_type.is_some()),
            (3, c.extracted_text.is_some()),
        ],
    )
}

pub async fn validate_and_place(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<Option<SpecificPart>> {
    let _ = (ctx, entity, part);
    // A note addresses no part at all — its content is its body — so anything
    // reaching here is a file's, and a file's content is 2.3's.
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

/// What Knowledge contributes to searchable text.
///
/// **LANE: Knowledge.** A note's words are the obvious candidate and they are
/// deliberately not taken here: a body's blocks are written by
/// [`super::super::body`], which this lane also fills, and taking half the
/// answer now would settle the shape of the other half. Contributing nothing is
/// a valid answer, and the literal arm still finds a note by its title.
pub async fn search_text(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
) -> Applied<Option<String>> {
    let _ = (ctx, entity, kind);
    Ok(None)
}

//! Knowledge's type content: note, file.
//!
//! **LANE: Knowledge owns this file.**
//!
//! # A note has no fields of its own, and that is the answer rather than a gap
//!
//! Migration one says it plainly: *"There is no note table. A note holds a body
//! beyond the shared set, a body is its blocks in order, and there is no second
//! representation of the same content for them to disagree with."* So a note's
//! whole content is a body, and a body is not a part: it divides into blocks and
//! each block is a part of its own. The wire reading below therefore produces a
//! [`BodyWrite`] rather than a [`SpecificPart`], and everything downstream of it
//! lives in [`super::super::body`] — division, matching against the blocks
//! already held, and writing only what changed. That split is deliberate:
//! reading the wire needs no store and the matching needs nothing else.
//!
//! Nothing here writes a block. [`validate_and_place`] and its two companions
//! can be reached for a note only by a part that cannot be constructed, and they
//! say so rather than pretending a note's content is unbuilt.
//!
//! # A file is mostly two other lanes'
//!
//! Of the three fields the wire gives a file, one is served here:
//!
//! - `body_id` is **Wave 2.3's**. The bytes come first — the standing constraint
//!   is *bytes before the record on creation* — and there is no way to have
//!   uploaded any, because `PutBody` is not served. So the field refuses with
//!   that reason, and [`super::super::entity`]'s create refuses a file outright
//!   for the same one.
//! - `extracted_text` is **deferred by `altair-v0-scope.md`**, which governs the
//!   implementation plan where the two disagree. A person's correction to
//!   extracted text has nothing to correct while extraction does not exist.
//! - `media_type` is served, and is the whole of a v0 file's own content. The
//!   scope's v0 files are *entities with a title and relations*, and a media
//!   type is what display follows.
//!
//! `file.byte_size` exists in the store and **the wire has no field for it**, on
//! purpose: it is what the object store measured rather than what a client
//! claimed. Nothing in this module can set it, and 2.3 writes it beside the
//! bytes.

use altair_proto::v1;

use crate::store::entity::EntityType;
use crate::store::ids::EntityId;

use super::super::content::{BodyWrite, Malformed, Written};
use super::super::entity::{Applied, Ctx, Refusal};
use super::{
    Addressed, Field, Held, Reader, SpecificPart, SpecificValue, read_column, write_column,
};

/// A note's field 1, its body.
pub const NOTE_BODY: u32 = 1;

pub const NOTE_FIELDS: &[Field] = &[Field {
    number: NOTE_BODY,
    held: Held::Body,
}];

/// A file's field 2, the media type as captured.
pub const FILE_MEDIA_TYPE: u32 = 2;

pub const FILE_FIELDS: &[Field] = &[
    // **LANE: 2.3, file bodies.** `body_id` names bytes already uploaded
    // through `PutBody`; until that call is served there is no identity to
    // name, which is why the create path refuses a file outright. The column is
    // `NOT NULL`, so clearing it is not a thing an edit can do either, and
    // saying so here is better than letting the store raise a fault.
    //
    // 2.3 turns this back into `Held::Column("body_id")`; the column already
    // exists and `tests/type_content.rs` will check it again the moment it does.
    Field {
        number: 1,
        held: Held::NotServed(
            "a file names a body that must be uploaded first, and PutBody is not served yet",
        ),
    },
    Field {
        number: FILE_MEDIA_TYPE,
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
/// Only the media type is read. The other two refuse with their own reasons,
/// through the declaration above rather than through a branch here, so a client
/// is told what it is waiting on rather than that its message was wrong.
pub fn file(c: &v1::FileContent) -> Result<Written, Malformed> {
    let read = Reader::new(EntityType::File, &c.cleared)?;
    let mut parts = Vec::new();
    // The two unserved fields are addressed so their refusals are reached.
    // Asking for them and using nothing but the answer is the point: leaving
    // them out would accept a write that named them and silently drop it.
    read.addressed(1, c.body_id.is_some())?;
    read.addressed(3, c.extracted_text.is_some())?;
    read.singular(&mut parts, FILE_MEDIA_TYPE, c.media_type.clone(), |v| {
        Ok(SpecificValue::Text(v))
    })?;
    Ok(Written::from_specific(parts))
}

/// What a Knowledge part owes before it is applied.
///
/// A media type owes nothing. It is the person's word for what the bytes are,
/// interpreted by nothing here — the same treatment a unit gets in Tracking —
/// and the entity stores no display preference beside it.
pub async fn validate_and_place(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<Option<SpecificPart>> {
    let _ = (ctx, entity);
    match (kind, part.field) {
        (EntityType::File, FILE_MEDIA_TYPE) => Ok(None),
        _ => Err(unreachable_part(kind, part).into()),
    }
}

pub async fn current(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &SpecificPart,
) -> Applied<SpecificPart> {
    match (kind, part.field) {
        (EntityType::File, FILE_MEDIA_TYPE) => read_column(ctx, entity, kind, part).await,
        _ => Err(unreachable_part(kind, part).into()),
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
        (EntityType::File, FILE_MEDIA_TYPE) => {
            // Neither Knowledge type nests or is ordered, so nothing moves
            // alongside.
            debug_assert!(placement.is_none());
            write_column(ctx, entity, kind, part).await
        }
        _ => Err(unreachable_part(kind, part).into()),
    }
}

/// A part no Knowledge message can produce.
///
/// **Not the *not built yet* refusal**, which would be a lie in both directions.
/// A note's one field is its body and a body never becomes a [`SpecificPart`];
/// a file's other two fields refuse while the message is being read and never
/// reach here. So anything arriving is a part that was constructed rather than
/// transcribed, and saying the content is merely unbuilt would send whoever
/// reads the log to the wrong module.
fn unreachable_part(kind: EntityType, part: &SpecificPart) -> Refusal {
    Refusal::Malformed(format!(
        "field {} is not a part a {} carries",
        part.field,
        super::super::entity::type_name(kind)
    ))
}

/// What Knowledge contributes to searchable text.
///
/// **LANE: Knowledge.** A note's words are the obvious candidate and they are
/// deliberately not taken here: a body's blocks are written by
/// [`super::super::body`], which this lane also fills, and taking half the
/// answer now would settle the shape of the other half. Contributing nothing is
/// a valid answer, and the literal arm still finds a note by its title.
///
/// A media type is deliberately not contributed either. It is a machine label
/// rather than the person's words, and putting `image/png` into the text a
/// literal search matches would make every photograph answer a search for
/// *image*.
pub async fn search_text(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
) -> Applied<Option<String>> {
    let _ = (ctx, entity, kind);
    Ok(None)
}

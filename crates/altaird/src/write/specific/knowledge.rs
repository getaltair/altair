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
//! - `body_id` is **Wave 2.3's, and it never reaches this module as a part**.
//!   The bytes come first — the standing constraint is *bytes before the
//!   record on creation* — so [`super::super::content::file_reference`] reads
//!   it straight out of `content.specific`, at creation, and checks it against
//!   the object store before [`super::super::entity::create_file_row`] inserts
//!   the row. [`file`] below still addresses field 1 through the ordinary
//!   [`Reader`], so the wire's own refusals apply to it — present and cleared
//!   at once, or a clear naming nothing — but the value itself is read
//!   elsewhere and an edit naming a new one is silently not applied, exactly
//!   as every other type-specific field was before this lane began.
//! - `extracted_text` is **deferred by `altair-v0-scope.md`**, which governs the
//!   implementation plan where the two disagree. A person's correction to
//!   extracted text has nothing to correct while extraction does not exist.
//! - `media_type` is served, and is the whole of a v0 file's own content. The
//!   scope's v0 files are *entities with a title and relations*, and a media
//!   type is what display follows.
//!
//! `file.byte_size` exists in the store and **the wire has no field for it**, on
//! purpose: it is what the object store measured rather than what a client
//! claimed. Nothing in this module can set it; `create_file_row` writes it
//! beside the bytes, at creation, alongside `body_id`.

use altair_proto::v1;
use sqlx::Row;

use crate::store::entity::EntityType;
use crate::store::ids::EntityId;

use super::super::content::{BodyWrite, Malformed, Written, malformed};
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
    // **LANE: 2.3, file bodies.** Declared as a column so `Reader::new` accepts
    // a `cleared` list naming it and [`file`] can enforce the wire's own rules
    // over it, but nothing here ever writes through this declaration:
    // `body_id` is read by `content::file_reference` and written by
    // `entity::create_file_row`, both outside this module, because setting it
    // means checking the object store first — not a plain column write.
    Field {
        number: 1,
        held: Held::Column("body_id"),
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
/// Only the media type becomes a part here. `extracted_text` refuses with its
/// own reason, through the declaration above rather than a branch here, so a
/// client is told what it is waiting on rather than that its message was
/// wrong. `body_id` is addressed but never turned into a part — see the doc at
/// the head of this module for where it actually goes.
pub fn file(c: &v1::FileContent) -> Result<Written, Malformed> {
    let read = Reader::new(EntityType::File, &c.cleared)?;
    let mut parts = Vec::new();
    // Addressed for the wire's own two refusals — present and cleared at
    // once, or a clear naming nothing — and then discarded. `Set` is
    // consumed elsewhere, at creation, by `content::file_reference`.
    // `Cleared` names a `NOT NULL` column: nothing a write can clear, so it
    // refuses rather than silently doing nothing with an explicit clear.
    if read.addressed(1, c.body_id.is_some())? == Addressed::Cleared {
        return Err(malformed(
            "a file's body cannot be cleared; every file names one",
        ));
    }
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

/// The separator the blocks of a body are joined with.
///
/// Both indexes treat any run of non-alphanumeric text as a separator, so this
/// changes nothing about what matches. It is a newline rather than a space
/// because the column holds plain text that a person may read and an export may
/// carry, and paragraphs that ran together would be worse to look at for no
/// gain.
const BETWEEN_BLOCKS: &str = "\n";

/// What Knowledge contributes to searchable text.
///
/// # A note's words, because otherwise nothing has them
///
/// **This is the whole of a note's literal searchability.** Both literal
/// indexes are on the `entity` row — `entity_search_vector_idx` over the
/// generated `tsvector` and `entity_search_trgm_idx` over the plain text — and
/// `block` carries no search index at all. So a body that does not reach
/// `search_text` reaches no index by any route, and a note captured with three
/// paragraphs about descaling a kettle would be findable only by its title.
/// That would make the standing claim false for exactly the type whose whole
/// content is words: *literal matching is a permanent arm, not a fallback,
/// which is why a just-captured entity is findable by its words before
/// derivation runs.*
///
/// Migration one's own comment on the column describes this case in as many
/// words — maintained by the write path "from the title and whatever the type
/// contributes, because those live in side tables and a generated column cannot
/// reach them". A body is a side table and this is the seam for it.
///
/// # Why the text is copied rather than the blocks indexed
///
/// A second index on `block.text` would put the literal arm on two tables with
/// two candidate sets to fuse, which is the read path's decision at 3.1 and not
/// one this lane should force. It would also put a candidate set on a table
/// that carries no audience: the predicate must sit **inside** the candidate
/// query, and `entity` is where the audience column is. One index on one table
/// keeps that true without a join.
///
/// The cost is that a long body is stored twice. It is real and it is the price
/// of the two properties above; see the note on refresh cost in the lane's
/// report.
///
/// # What is deliberately still not contributed
///
/// **A media type.** It is a machine label rather than the person's words, and
/// putting `image/png` into the text a literal search matches would make every
/// photograph answer a search for *image*. A file therefore contributes
/// nothing, which is still a valid answer.
pub async fn search_text(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
) -> Applied<Option<String>> {
    if kind != EntityType::Note {
        return Ok(None);
    }
    // In the order the body reads. A set of paragraphs is not a body, and the
    // column holds text a person may read.
    let rows = sqlx::query("SELECT text FROM block WHERE entity_id = $1 ORDER BY position")
        .bind(entity.as_uuid())
        .fetch_all(ctx.tx.conn())
        .await?;
    if rows.is_empty() {
        // No body is no contribution, not an empty one. `concat_ws` drops a
        // null, so a note without a body keeps a `search_text` of exactly its
        // title — which is what it was before this lane filled the seam in.
        return Ok(None);
    }
    let mut words: Vec<String> = Vec::with_capacity(rows.len());
    for r in &rows {
        words.push(r.try_get("text")?);
    }
    Ok(Some(words.join(BETWEEN_BLOCKS)))
}

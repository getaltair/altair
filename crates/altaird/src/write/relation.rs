//! The five verbs, on a relation.
//!
//! # Types are declarations, not branches
//!
//! **Nothing here names a relation type.** Whether a relation is read from one
//! end or from both, and whether it may carry a quantity, are read out of
//! `relation_type` as two booleans and interpreted. There is no `match` on
//! *blocks*, *uses*, or *references* anywhere, and there must not be one.
//!
//! This is a build constraint from the relation types specification rather than
//! a style preference: *the constraint worth honouring meanwhile is to build
//! these as declarations the system interprets rather than as hardcoded
//! branches, so that exposing the extension point later is exposing what
//! already exists.* It is free now and a rewrite later. `tests/write_relations.rs`
//! proves it by declaring a type the shipped set has never contained and
//! showing it gets the whole of both behaviours.
//!
//! # Canonical ordering
//!
//! One record joins two entities and is traversable from either end; there is
//! no reverse row. Where the type is symmetric or absent, the endpoints have no
//! privileged order, so the same connection submitted as A→B and as B→A would
//! be two records of one fact. The write path puts them in a canonical order —
//! by identifier, which carries no meaning and is therefore the only stable
//! thing to sort by (DR-007) — so it cannot be recorded twice.
//!
//! A constraint cannot do this, because symmetry is a property of the type
//! declaration and a check constraint cannot reach another table.
//!
//! # Anchors, and why an anchor never refuses a connection away
//!
//! An anchor records **where a relation was formed** — a phrase inside a block,
//! or the block itself. It belongs to the relation and not to the body, so the
//! body stays plain text and removing the relation rewrites nothing. It is not
//! a type-declared property and needs no type.
//!
//! The rule that shapes every decision below is one sentence of the substrate:
//! *"Removing a person's connection as a side effect of them editing a sentence
//! is not something the system does."* An anchor is the weakest thing a
//! relation carries, so **the anchor gives way and the relation never does**.
//! That is why a phrase this instance cannot find is not a refusal — see
//! [`anchor`].

use sqlx::Row;
use uuid::Uuid;

use altair_proto::v1;

use crate::store::entity::available_for_write;
use crate::store::ids::EntityId;
use crate::store::{WriteScope, WriteTx};

use super::changes;
use super::content::identifier;
use super::entity::{Applied, Ctx, Refusal};

/// What a relation type declares. Read, never branched on.
struct Declaration {
    is_asymmetric: bool,
    is_quantified: bool,
}

async fn declaration(tx: &mut WriteTx, id: Uuid) -> Applied<Declaration> {
    let row = sqlx::query("SELECT is_asymmetric, is_quantified FROM relation_type WHERE id = $1")
        .bind(id)
        .fetch_optional(tx.conn())
        .await?
        .ok_or(Refusal::NotAvailable)?;
    Ok(Declaration {
        is_asymmetric: row.try_get("is_asymmetric")?,
        is_quantified: row.try_get("is_quantified")?,
    })
}

/// A relation as this write path holds one, after validation.
struct Resolved {
    from: EntityId,
    to: EntityId,
    relation_type: Option<Uuid>,
    quantity: Option<String>,
    resolution: Option<&'static str>,
    anchor: Anchored,
}

/// An anchor as the store holds one: three columns, and only the combinations
/// the schema admits.
///
/// `0001_initial.sql` states them as three checks — the anchor entity is one of
/// the two endpoints, a block requires an entity, a phrase requires a block —
/// so the reachable shapes are: nothing at all; an entity alone; an entity and
/// a block; all three.
///
/// **An entity alone is a residue, never a submission.** It is what a removed
/// block leaves behind, because `anchor_block_id ... ON DELETE SET NULL` takes
/// the block and leaves where the relation was formed standing. [`anchor`]
/// refuses it on the way in and produces it on the way out, which is not an
/// inconsistency: one is a client inventing a third kind of anchor, the other
/// is this instance saying an anchor used to be here.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Anchored {
    entity: Option<Uuid>,
    block: Option<Uuid>,
    phrase: Option<String>,
}

/// An exact decimal, as text, for a `numeric` column.
///
/// **Range is checked here**, which is one of the schema's owed checks —
/// *everything the wire shape implies: identifier length, decimal range, a
/// field both present and cleared, a cleared number naming no field*. Two
/// shapes are refusable rather than renderable, and both were silently
/// mis-rendered until a cross-model review found them:
///
/// - **A fraction of a billion or more.** `{ units: 1, nanos: 1_500_000_000 }`
///   is not 2.5, and formatting it into a fixed nine-place field produces
///   `1.15` — a different number, stated with total confidence. A household's
///   stock is the last place to guess.
/// - **Opposite signs.** The wire says units and nanos carry the same sign, as
///   a duration does. `{ units: 1, nanos: -250_000_000 }` is either 0.75 or
///   -1.25 depending on which half you believe, so it is neither.
///
/// A negative amount itself is ordinary and is rendered: availability is the
/// asserted amount minus what live relations commit, and negative availability
/// is meaningful information about a cupboard.
fn decimal(d: &v1::Decimal) -> Result<String, Refusal> {
    const BILLION: i64 = 1_000_000_000;
    let nanos = i64::from(d.nanos);

    if nanos.abs() >= BILLION {
        return Err(Refusal::Malformed(
            "a decimal's nanos are a fraction of one unit, and this one is a whole unit or more"
                .into(),
        ));
    }
    if (d.units > 0 && nanos < 0) || (d.units < 0 && nanos > 0) {
        return Err(Refusal::Malformed(
            "a decimal's units and nanos carry the same sign, as a duration does".into(),
        ));
    }

    let negative = d.units < 0 || nanos < 0;
    let whole = d.units.unsigned_abs();
    let fraction = nanos.unsigned_abs();
    Ok(format!(
        "{}{whole}.{fraction:09}",
        if negative { "-" } else { "" }
    ))
}

/// Endpoints, type, and type-declared properties, checked against everything
/// that is not the store's to check.
async fn resolve(ctx: &mut Ctx<'_>, content: &v1::RelationContent) -> Applied<Resolved> {
    let (Some(from), Some(to)) = (&content.from_entity_id, &content.to_entity_id) else {
        return Err(Refusal::Malformed(
            "a relation joins two entities and this names fewer".into(),
        )
        .into());
    };
    let from = EntityId::from_uuid(identifier(from)?);
    let to = EntityId::from_uuid(identifier(to)?);
    if from == to {
        return Err(
            Refusal::Malformed("a relation joins two entities, not one to itself".into()).into(),
        );
    }

    // Both ends have to be visible. A relation whose far end a member cannot
    // see would show them that something is there, which is the disclosure the
    // single refusal reason exists to prevent.
    for end in [from, to] {
        if available_for_write(ctx.tx, ctx.member, end, WriteScope::Extant)
            .await?
            .is_none()
        {
            return Err(Refusal::NotAvailable.into());
        }
    }

    let relation_type = content
        .relation_type_id
        .as_ref()
        .map(|b| identifier(b))
        .transpose()?;

    let declared = match relation_type {
        Some(id) => Some(declaration(ctx.tx, id).await?),
        None => None,
    };

    // Where the relation was formed, if it says. Resolved after the endpoints,
    // because an anchor is only meaningful against them and because the block
    // lookup below may only run once both ends are known visible.
    let anchored = match &content.anchor {
        Some(a) => anchor(ctx, a, from, to).await?,
        None => Anchored::default(),
    };

    // Properties the type declares, and no others.
    let (quantity, resolution) = match (&content.uses, &declared) {
        (Some(uses), Some(d)) if d.is_quantified => {
            let q = uses.quantity.as_ref().ok_or_else(|| {
                Refusal::Malformed("a quantified relation carries a quantity".into())
            })?;
            let resolution = match v1::UsesResolution::try_from(uses.resolution) {
                Ok(v1::UsesResolution::Returned) => "returned",
                Ok(v1::UsesResolution::Consumed) => "consumed",
                _ => "unresolved",
            };
            (Some(decimal(q)?), Some(resolution))
        }
        (Some(_), _) => {
            return Err(Refusal::Malformed(
                "this relation's type declares no properties, and properties follow from the type"
                    .into(),
            )
            .into());
        }
        _ => (None, None),
    };

    // Where the type is symmetric or absent, neither end is privileged, so the
    // pair is put in a canonical order and the same connection cannot be
    // recorded twice.
    let symmetric = declared.as_ref().is_none_or(|d| !d.is_asymmetric);
    let (from, to) = if symmetric && to.as_uuid() < from.as_uuid() {
        (to, from)
    } else {
        (from, to)
    };

    Ok(Resolved {
        from,
        to,
        relation_type,
        quantity,
        resolution,
        anchor: anchored,
    })
}

/// Read an anchor off the wire and check it against the store.
///
/// # What is refused, and what is quietly given up instead
///
/// Two malformed shapes, both statements about the message rather than about
/// what is in the store, so both may say what is wrong. The client already
/// holds both endpoint identifiers, so neither discloses anything.
///
/// - **An entity that is not one of the two endpoints.** *"An anchor locates
///   into one of the two things this joins, and nothing else."*
/// - **No block.** The substrate names two kinds of anchor, *"at a phrase
///   within a block or at the block itself"*, and an entity alone is neither.
///
/// One refusal that says nothing: **a block identifier this entity does not
/// hold**, whether because nothing holds it or because something else does.
/// The two are one answer. A block belongs to an entity, so confirming that a
/// block identifier exists would confirm that an entity exists, which is the
/// disclosure the single refusal reason exists to prevent — arriving by way of
/// a field nobody thinks of as naming an entity.
///
/// # A phrase that is not there is not a refusal
///
/// **This is the decision worth reading twice.** A client submits an anchor
/// against the body it last saw. If somebody else has since reworded that
/// sentence, the phrase is gone by the time the intent arrives.
///
/// Refusing would destroy the whole connection, permanently: a refusal is an
/// answer, the outbox holds no retry for it, and the person's connection would
/// be gone because somebody else edited a sentence. That is a worse version of
/// the exact thing the substrate prohibits.
///
/// So the anchor gives way and the relation is formed without one, which is
/// the rule already written for the same situation later in the entity's life:
/// *"an anchor whose text survives holds, and one whose text does not leaves
/// the relation intact without an anchor."*
///
/// **The block goes with the phrase rather than the anchor falling back to
/// it.** Both places the substrate states this say *without an anchor*, not
/// "with a coarser one". Somebody pointed at particular words; re-pointing the
/// connection at the whole paragraph would be this file making up a place the
/// person never chose, which is the error `body::identity` refuses to make
/// with over-eager matching. Where it was formed — the entity — is still true
/// and stays.
async fn anchor(
    ctx: &mut Ctx<'_>,
    a: &v1::Anchor,
    from: EntityId,
    to: EntityId,
) -> Applied<Anchored> {
    if a.entity_id.is_empty() {
        return Err(Refusal::Malformed(
            "an anchor names the endpoint whose body it locates into".into(),
        )
        .into());
    }
    let entity = EntityId::from_uuid(identifier(&a.entity_id)?);
    if entity != from && entity != to {
        return Err(Refusal::Malformed(
            "an anchor locates into one of the two entities the relation joins, and this \
             names neither"
                .into(),
        )
        .into());
    }
    if a.block_id.is_empty() {
        return Err(Refusal::Malformed(
            "an anchor attaches at a block, or at a phrase within one, and this names no \
             block"
                .into(),
        )
        .into());
    }
    let block = identifier(&a.block_id)?;

    // One read answers both questions: whether this entity holds the block, and
    // what the block says. Scoped by entity, so a block of something else is
    // the same nothing as a block of nothing.
    let text: String = sqlx::query("SELECT text FROM block WHERE id = $1 AND entity_id = $2")
        .bind(block)
        .bind(entity.as_uuid())
        .fetch_optional(ctx.tx.conn())
        .await?
        .ok_or(Refusal::NotAvailable)?
        .try_get("text")?;

    // Empty is the block itself, which the wire states outright. Stored as
    // null rather than as an empty string: a phrase nobody wrote is not a
    // phrase, and the schema's checks read presence.
    if a.phrase.is_empty() {
        return Ok(Anchored {
            entity: Some(entity.as_uuid()),
            block: Some(block),
            phrase: None,
        });
    }

    // Matched against the block's content rather than its verbatim slice, for
    // the reason `write::body` sets out at length: the verbatim text carries
    // the blank line after the block, which belongs to the body's shape rather
    // than to anything a person wrote.
    if !crate::body::content_of(&text).contains(&a.phrase) {
        return Ok(Anchored {
            entity: Some(entity.as_uuid()),
            block: None,
            phrase: None,
        });
    }

    Ok(Anchored {
        entity: Some(entity.as_uuid()),
        block: Some(block),
        phrase: Some(a.phrase.clone()),
    })
}

/// Refuse a duplicate untyped unanchored relation.
///
/// **Only that shape.** Migration one records why this is not a unique index: a
/// relation that loses its anchor when its block is removed becomes unanchored,
/// would collide with an existing relation between the same pair, and the block
/// removal would fail. A body edit must never fail because a relation lost an
/// anchor, and that outranks refusing a duplicate structurally.
///
/// Only active relations count. One sitting in holding is not a connection that
/// exists, and forming it again is the person's answer to having removed it.
///
/// # Both sides have to be unanchored, and only one of them was
///
/// The query below asks whether an *existing* row is untyped and unanchored.
/// Until anchors could be formed that was the whole question, because nothing
/// arriving could be anchored either. It is now half of it: an anchored
/// relation arriving where an unanchored one already exists is a second
/// connection, recorded at a place in the text, and refusing it would tell
/// somebody their connection was already there when what is there says nothing
/// about where they made theirs.
///
/// So the arriving side is checked here and the stored side in the query, and
/// **an anchor of any kind is enough** — including the residue of one, an
/// entity with no block left. That is deliberate rather than an oversight of
/// the residue case: it is still a record of where a connection was formed, and
/// it is what keeps a body edit that removes a block from turning a relation
/// into somebody else's duplicate.
async fn is_duplicate(ctx: &mut Ctx<'_>, r: &Resolved, itself: Option<Uuid>) -> Applied<bool> {
    if r.relation_type.is_some() || r.anchor.entity.is_some() {
        return Ok(false);
    }
    let row = sqlx::query(
        "SELECT 1 AS found FROM relation \
         WHERE from_entity_id = $1 AND to_entity_id = $2 \
           AND relation_type_id IS NULL AND anchor_entity_id IS NULL \
           AND lifecycle = 'active' AND ($3::uuid IS NULL OR id <> $3) LIMIT 1",
    )
    .bind(r.from.as_uuid())
    .bind(r.to.as_uuid())
    .bind(itself)
    .fetch_optional(ctx.tx.conn())
    .await?;
    Ok(row.is_some())
}

/// The refusal a duplicate produces, in one place because two verbs raise it.
fn already_recorded() -> Refusal {
    Refusal::Malformed(
        "this connection is already recorded, and an untyped unanchored relation is \
         the same connection however it is submitted"
            .into(),
    )
}

/// Create a relation.
pub async fn create(ctx: &mut Ctx<'_>, id: Uuid, content: &v1::RelationContent) -> Applied<Uuid> {
    // Already here: one relation, not two. The same convergence a create of an
    // entity gets, and for the same reason.
    if let Some(existing) = load(ctx, id).await? {
        return Ok(existing.0);
    }

    let r = resolve(ctx, content).await?;
    if is_duplicate(ctx, &r, None).await? {
        return Err(already_recorded().into());
    }

    // `ON CONFLICT DO NOTHING`, with the row count read rather than ignored.
    // `load` above answered `None` both for an identifier nothing holds and for
    // one held by a relation this member cannot see, and only the first of
    // those may be created. A bare insert turns the second into a primary-key
    // violation, which leaves this path as a store fault where an unused
    // identifier would have succeeded — and a fault that appears only when
    // something is there is an oracle for whether something is there.
    let inserted = sqlx::query(
        "INSERT INTO relation \
         (id, from_entity_id, to_entity_id, relation_type_id, quantity, resolution, \
          anchor_entity_id, anchor_block_id, anchor_phrase, author_member_id, created_at) \
         VALUES ($1, $2, $3, $4, $5::numeric, $6::uses_resolution, $7, $8, $9, $10, $11) \
         ON CONFLICT (id) DO NOTHING",
    )
    .bind(id)
    .bind(r.from.as_uuid())
    .bind(r.to.as_uuid())
    .bind(r.relation_type)
    .bind(r.quantity.as_deref())
    .bind(r.resolution)
    .bind(r.anchor.entity)
    .bind(r.anchor.block)
    .bind(r.anchor.phrase.as_deref())
    .bind(ctx.member.as_uuid())
    .bind(ctx.at)
    .execute(ctx.tx.conn())
    .await?;
    if inserted.rows_affected() == 0 {
        return Err(Refusal::NotAvailable.into());
    }

    changes::relation_written(ctx.tx, ctx.at, id).await?;
    Ok(id)
}

/// The relation, if it is there and both its ends are visible to this member.
async fn load(ctx: &mut Ctx<'_>, id: Uuid) -> Applied<Option<(Uuid, String)>> {
    let row = sqlx::query(
        "SELECT id, from_entity_id, to_entity_id, lifecycle::text AS lifecycle \
         FROM relation WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(ctx.tx.conn())
    .await?;
    let Some(row) = row else {
        return Ok(None);
    };
    for column in ["from_entity_id", "to_entity_id"] {
        let end = EntityId::from_uuid(row.try_get(column)?);
        if available_for_write(ctx.tx, ctx.member, end, WriteScope::AnyIncludingErased)
            .await?
            .is_none()
        {
            return Ok(None);
        }
    }
    Ok(Some((row.try_get("id")?, row.try_get("lifecycle")?)))
}

/// Edit a relation.
///
/// **Unconditional, and last write wins.** Relations carry no write counter
/// anywhere in the documents, which the proto records as its gap (a): moot in
/// v0 with one device, and not moot afterwards. Nothing here pretends
/// otherwise, and nothing detects a concurrent relation edit.
pub async fn edit(ctx: &mut Ctx<'_>, id: Uuid, content: &v1::RelationContent) -> Applied<Uuid> {
    if load(ctx, id).await?.is_none() {
        return Err(Refusal::NotAvailable.into());
    }
    let r = resolve(ctx, content).await?;

    // An edit can make a relation into a duplicate as easily as a create can:
    // clearing the type off one of two relations between the same pair leaves
    // two untyped unanchored records of one connection. The relation being
    // edited is excluded, or an edit that changes nothing would refuse itself.
    if is_duplicate(ctx, &r, Some(id)).await? {
        return Err(already_recorded().into());
    }

    // Every column, from the message. **A relation's content has no untouched
    // state**: there is no base counter to scope a write against and no part
    // machinery over a relation, so an edit is the whole record and a field the
    // message does not carry is a field the message cleared. That was already
    // true of the type and of a quantity; the anchor joins them rather than
    // acquiring a rule of its own. A `cleared` list therefore has nothing to
    // add here, and naming a field in one changes nothing.
    sqlx::query(
        "UPDATE relation SET from_entity_id = $2, to_entity_id = $3, relation_type_id = $4, \
         quantity = $5::numeric, resolution = $6::uses_resolution, \
         anchor_entity_id = $7, anchor_block_id = $8, anchor_phrase = $9 WHERE id = $1",
    )
    .bind(id)
    .bind(r.from.as_uuid())
    .bind(r.to.as_uuid())
    .bind(r.relation_type)
    .bind(r.quantity.as_deref())
    .bind(r.resolution)
    .bind(r.anchor.entity)
    .bind(r.anchor.block)
    .bind(r.anchor.phrase.as_deref())
    .execute(ctx.tx.conn())
    .await?;

    changes::relation_written(ctx.tx, ctx.at, id).await?;
    Ok(id)
}

/// Put relations into the holding state, as part of one act.
///
/// Converges exactly as an entity removal does.
pub async fn remove(ctx: &mut Ctx<'_>, ids: &[Uuid], group: Uuid) -> Applied<Vec<Uuid>> {
    let mut removed = Vec::new();
    for id in ids {
        match load(ctx, *id).await? {
            Some((_, lifecycle)) if lifecycle == "active" => {}
            _ => continue,
        }
        sqlx::query(
            "UPDATE relation SET lifecycle = 'deleted', deleted_at = $2, \
             deletion_group_id = $3 WHERE id = $1",
        )
        .bind(id)
        .bind(ctx.at)
        .bind(group)
        .execute(ctx.tx.conn())
        .await?;
        // A removed relation is gone from the stream, not a relation carrying a
        // lifecycle: the wire's `Relation` has no lifecycle field to put one in.
        changes::relation_gone(ctx.tx, ctx.at, *id).await?;
        removed.push(*id);
    }
    Ok(removed)
}

/// Bring back the relations one act removed.
///
/// **A relation comes back with the act that removed it, and there is no
/// restoring one on its own.** The asymmetry with entities is deliberate:
/// nothing lists removed connections for a person to notice one missing from,
/// and forming a connection again costs a gesture, which is not true of an
/// entity whose content nobody can retype from memory.
pub async fn restore_group(ctx: &mut Ctx<'_>, group: Uuid) -> Applied<Vec<Uuid>> {
    let rows = sqlx::query("SELECT id FROM relation WHERE deletion_group_id = $1")
        .bind(group)
        .fetch_all(ctx.tx.conn())
        .await?;

    let mut restored = Vec::new();
    for row in &rows {
        let id: Uuid = row.try_get("id")?;
        if load(ctx, id).await?.is_none() {
            continue;
        }
        sqlx::query(
            "UPDATE relation SET lifecycle = 'active', deleted_at = NULL, \
             deletion_group_id = NULL WHERE id = $1",
        )
        .bind(id)
        .execute(ctx.tx.conn())
        .await?;
        changes::relation_written(ctx.tx, ctx.at, id).await?;
        restored.push(id);
    }
    Ok(restored)
}

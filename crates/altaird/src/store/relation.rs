//! The shared relation query surface: read-only, and the counterpart of
//! [`super::entity`] for the one other thing the change stream has to speak
//! about.
//!
//! Nothing here writes. `crates/altaird/src/write/relation.rs` is the wire's
//! way in; this is the read path's way of turning a stored relation back into
//! the wire shape, and of answering the one question the write path never had
//! to: whether a *particular member* may be shown this relation at all.

use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

use altair_proto::v1;

use super::ReadScope;
use super::entity::available_for_read;
use super::ids::{EntityId, MemberId};
use super::tx::ReadTx;

/// A relation, as the store holds one — every column, unconverted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationRow {
    pub id: Uuid,
    pub from_entity_id: Uuid,
    pub to_entity_id: Uuid,
    pub relation_type_id: Option<Uuid>,
    pub anchor_entity_id: Option<Uuid>,
    pub anchor_block_id: Option<Uuid>,
    pub anchor_phrase: Option<String>,
    /// `numeric`, read as text. Never parsed here except on the way to the
    /// wire — see [`quantity_to_wire`] — because nothing in this layer does
    /// arithmetic on it.
    pub quantity: Option<String>,
    pub resolution: Option<String>,
    pub author_member_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// A relation by its identity, regardless of whether any member may see it.
///
/// **Deliberately unscoped.** A relation carries no audience of its own —
/// [`endpoints_visible`] is the only place that question is answered — so
/// there is no predicate this query could carry. Nothing here is shown to
/// anyone until a caller has checked that separately.
pub async fn load(tx: &mut ReadTx, id: Uuid) -> sqlx::Result<Option<RelationRow>> {
    let row = sqlx::query(
        "SELECT id, from_entity_id, to_entity_id, relation_type_id, \
         anchor_entity_id, anchor_block_id, anchor_phrase, \
         quantity::text AS quantity, resolution::text AS resolution, \
         author_member_id, created_at \
         FROM relation WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(tx.conn())
    .await?;
    row.as_ref()
        .map(|r| {
            Ok(RelationRow {
                id: r.try_get("id")?,
                from_entity_id: r.try_get("from_entity_id")?,
                to_entity_id: r.try_get("to_entity_id")?,
                relation_type_id: r.try_get("relation_type_id")?,
                anchor_entity_id: r.try_get("anchor_entity_id")?,
                anchor_block_id: r.try_get("anchor_block_id")?,
                anchor_phrase: r.try_get("anchor_phrase")?,
                quantity: r.try_get("quantity")?,
                resolution: r.try_get("resolution")?,
                author_member_id: r.try_get("author_member_id")?,
                created_at: r.try_get("created_at")?,
            })
        })
        .transpose()
}

/// Whether a member may currently be shown this relation.
///
/// **Decision, not derived from any document:** a relation reaches a member
/// only if they can see *both* of its endpoints. The substrate never states
/// this — a relation's own row carries no audience, by design (see
/// `write::changes::write_relation_row`) — so the conservative reading is
/// taken: a relation naming an entity a member cannot see would otherwise
/// disclose that entity's existence through a field nobody thinks of as
/// naming one, which is exactly the shape of leak the single refusal reason
/// exists to prevent everywhere else on this path.
///
/// [`ReadScope::Extant`], not every lifecycle: an erased endpoint clears its
/// own audience on the way to becoming a tombstone (see `write::entity`'s
/// erase path), so a member who could see it before erasure no longer can —
/// which is correct, because the entity's own disappearance already reached
/// them as its own change, and a relation naming a tombstone is not a
/// connection this instance can still show anyone.
pub async fn endpoints_visible(
    tx: &mut ReadTx,
    member: MemberId,
    from: EntityId,
    to: EntityId,
) -> sqlx::Result<bool> {
    for end in [from, to] {
        if available_for_read(tx, member, end, ReadScope::Extant)
            .await?
            .is_none()
        {
            return Ok(false);
        }
    }
    Ok(true)
}

/// An exact decimal, back from the text a `numeric` column holds, for the
/// wire's `Decimal`.
///
/// The inverse of `write::relation::decimal`. That function's two refusals —
/// a fraction of a billion or more, and units and nanos of opposite sign —
/// are both write-path questions about what a client may submit; nothing
/// stored here can violate either, because nothing reaches this column except
/// through that function.
fn quantity_to_wire(text: &str) -> v1::Decimal {
    let negative = text.starts_with('-');
    let unsigned = text.trim_start_matches('-');
    let (whole, frac) = unsigned.split_once('.').unwrap_or((unsigned, "0"));
    let units: i64 = whole.parse().unwrap_or(0);
    let mut frac_digits = frac.chars().chain(std::iter::repeat('0'));
    let nanos: i32 = frac_digits
        .by_ref()
        .take(9)
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    let sign = if negative { -1i64 } else { 1i64 };
    v1::Decimal {
        units: sign * units,
        nanos: (sign as i32) * nanos,
    }
}

fn resolution_to_wire(text: &str) -> v1::UsesResolution {
    match text {
        "returned" => v1::UsesResolution::Returned,
        "consumed" => v1::UsesResolution::Consumed,
        _ => v1::UsesResolution::Unresolved,
    }
}

impl RelationRow {
    /// The shared model, in the shape the wire states it.
    #[must_use]
    pub fn into_wire(self) -> v1::Relation {
        let anchor = self.anchor_entity_id.map(|entity_id| v1::Anchor {
            entity_id: entity_id.as_bytes().to_vec(),
            block_id: self
                .anchor_block_id
                .map(|b| b.as_bytes().to_vec())
                .unwrap_or_default(),
            phrase: self.anchor_phrase.unwrap_or_default(),
        });
        let uses = self.quantity.as_deref().map(|q| v1::UsesProperties {
            quantity: Some(quantity_to_wire(q)),
            resolution: self
                .resolution
                .as_deref()
                .map(resolution_to_wire)
                .unwrap_or(v1::UsesResolution::Unresolved) as i32,
        });
        v1::Relation {
            relation_id: self.id.as_bytes().to_vec(),
            content: Some(v1::RelationContent {
                from_entity_id: Some(self.from_entity_id.as_bytes().to_vec()),
                to_entity_id: Some(self.to_entity_id.as_bytes().to_vec()),
                relation_type_id: self.relation_type_id.map(|id| id.as_bytes().to_vec()),
                anchor,
                uses,
                cleared: Vec::new(),
            }),
            author_member_id: self
                .author_member_id
                .map(|id| id.as_bytes().to_vec())
                .unwrap_or_default(),
            created_at: Some(altair_proto::prost_types::Timestamp {
                seconds: self.created_at.timestamp(),
                nanos: self.created_at.timestamp_subsec_nanos() as i32,
            }),
        }
    }
}

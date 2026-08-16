//! The shared entity query surface, in the two shapes the paths need.
//!
//! Both go through [`CandidateQuery`], so both carry the same audience
//! predicate, and there is no third shape that does not.

use sqlx::Row;
use sqlx::postgres::PgRow;

use super::audience::{AUDIENCE_COLUMN, Bind, CandidateQuery, ReadScope, WriteScope};
use super::ids::{EntityId, MemberId, MemberRef};
use super::tx::{ReadTx, WriteTx};

/// The shared model, as much of it as lives on the `entity` row itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityRow {
    pub id: EntityId,
    pub entity_type: EntityType,
    pub title: Option<String>,
    /// Absent on an entity captured before its device was bound. Such a row is
    /// visible to nobody until binding gives it one, which the predicate
    /// enforces itself.
    ///
    /// A [`MemberRef`] rather than a [`MemberId`]: an author may since have
    /// departed, and the value carries no claim that they still participate.
    pub author: Option<MemberRef>,
    /// Who else can see it. Empty is private to the author. Entries survive
    /// departure, so these too are references and not requesters.
    pub audience: Vec<MemberRef>,
    pub lifecycle: LifecycleState,
    /// Advances on every accepted write. Never shown to anyone.
    pub counter: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "entity_type", rename_all = "snake_case")]
pub enum EntityType {
    Campaign,
    Arc,
    Quest,
    Routine,
    FocusSession,
    CheckIn,
    Note,
    File,
    Item,
    Location,
    ShoppingList,
    Category,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "lifecycle_state", rename_all = "snake_case")]
pub enum LifecycleState {
    Active,
    Deleted,
    Erased,
}

/// The projection every read here uses. Built through [`AUDIENCE_COLUMN`] so
/// that the audience column is named in exactly one file.
fn columns() -> String {
    format!(
        "e.id, e.type AS entity_type, e.title, e.author_member_id, \
         e.{AUDIENCE_COLUMN} AS audience, e.lifecycle, e.counter"
    )
}

fn row(r: &PgRow) -> sqlx::Result<EntityRow> {
    Ok(EntityRow {
        id: r.try_get("id")?,
        entity_type: r.try_get("entity_type")?,
        title: r.try_get("title")?,
        author: r
            .try_get::<Option<uuid::Uuid>, _>("author_member_id")?
            .map(MemberRef::from_uuid),
        audience: r
            .try_get::<Vec<uuid::Uuid>, _>("audience")?
            .into_iter()
            .map(MemberRef::from_uuid)
            .collect(),
        lifecycle: r.try_get("lifecycle")?,
        counter: r.try_get("counter")?,
    })
}

/// **The write path's shape.** The entity this member is entitled to act on.
///
/// `None` is the only refusal, and it covers both "no such entity" and "not
/// visible to this member". The caller cannot tell them apart, which is
/// deliberate and is why they are not two variants: the component model
/// requires that "a write naming an entity the submitting member cannot see is
/// refused exactly as a write naming an entity that does not exist is refused",
/// and the schema gives both one `refusal_reason`, `not_available`. Splitting
/// them here would put the distinction one `match` away from the wire.
///
/// `lifecycle` is stated by the caller because the write path means different
/// things at different moments: an edit is [`WriteScope::Active`], a restore is
/// [`WriteScope::Holding`], and deciding whether an arriving edit is a
/// recreation needs [`WriteScope::AnyIncludingErased`]. That last variant does
/// not exist on [`ReadScope`], so no read surface can ask for a tombstone.
pub async fn available_for_write(
    tx: &mut WriteTx,
    member: MemberId,
    id: EntityId,
    lifecycle: WriteScope,
) -> sqlx::Result<Option<EntityRow>> {
    let q = CandidateQuery::new(member, lifecycle.into(), &columns())
        .and_where("e.id = $?", [Bind::Uuid(id.as_uuid())]);
    let found = q.build().fetch_optional(tx.conn()).await?;
    found.as_ref().map(row).transpose()
}

/// **The read path's shape.** Candidates, with the predicate inside the query
/// that produced them.
///
/// Wave 3 adds the literal arm's matching to this; the ordering is already
/// deterministic with a stable tiebreak because "the same query over the same
/// data produces byte-identical ordering" is that lane's done-when and an
/// order that varies between members is on the read path's `Never` list.
pub async fn candidates(
    tx: &mut ReadTx,
    member: MemberId,
    lifecycle: ReadScope,
    limit: i64,
) -> sqlx::Result<Vec<EntityRow>> {
    let q = CandidateQuery::new(member, lifecycle.into(), &columns()).tail(
        "ORDER BY e.created_at DESC, e.id LIMIT $?",
        [Bind::Int(limit)],
    );
    let rows = q.build().fetch_all(tx.conn()).await?;
    rows.iter().map(row).collect()
}

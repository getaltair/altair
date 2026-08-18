//! The shared entity query surface, in the two shapes the paths need.
//!
//! Both go through [`CandidateQuery`], so both carry the same audience
//! predicate, and there is no third shape that does not.

use chrono::{DateTime, Utc};
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
    /// When the entity entered the holding state. Null unless deleted, which
    /// the store checks.
    pub deleted_at: Option<DateTime<Utc>>,
    /// The act a removal of several entities was, retained so that restoring
    /// any of them can bring back the rest.
    pub deletion_group_id: Option<uuid::Uuid>,
    /// Advances on every accepted write. Never shown to anyone.
    pub counter: i64,
    /// Set once by the creating client. Settles the order of nothing.
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// An open set: clients add ways to create things.
    pub capture_method: String,
    /// Whether the entity is still filterable as bulk captured.
    pub bulk: bool,
    /// The category it is in, if any. Containment, not a relation.
    pub category_id: Option<EntityId>,
    /// Its position within that category. Assigned by the write path, which
    /// appends on entry.
    pub category_position: Option<i32>,
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
         e.{AUDIENCE_COLUMN} AS audience, e.lifecycle, e.deleted_at, \
         e.deletion_group_id, e.counter, e.created_at, e.updated_at, \
         e.capture_method, e.bulk, e.category_id, e.category_position"
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
        deleted_at: r.try_get("deleted_at")?,
        deletion_group_id: r.try_get("deletion_group_id")?,
        counter: r.try_get("counter")?,
        created_at: r.try_get("created_at")?,
        updated_at: r.try_get("updated_at")?,
        capture_method: r.try_get("capture_method")?,
        bulk: r.try_get("bulk")?,
        category_id: r
            .try_get::<Option<uuid::Uuid>, _>("category_id")?
            .map(EntityId::from_uuid),
        category_position: r.try_get("category_position")?,
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

/// The next position in a category's order, which is one past the last.
///
/// **Deliberately unscoped, and this is the one read here that is.** Every
/// other query in this module carries the audience predicate because it
/// produces candidates — rows a member is entitled to see. This produces an
/// integer and no content at all.
///
/// Scoping it would be actively wrong. Position is unique within a container,
/// and the substrate requires the order to be total and stable. A maximum taken
/// over only the entities the writing member can see would hand out a position
/// already held by one they cannot, so two entities would share a position and
/// the order would stop being total — a correctness failure produced by
/// applying an access rule to something that is not an access question.
///
/// Nothing about the answer discloses anything: the count of things in a
/// container is not reachable from an integer that is one larger than a number
/// the caller never learns the provenance of, and the caller already knows the
/// container exists because it had to be visible to name it.
///
/// The substrate's rule is *entering a container places the entity at the end*,
/// and it holds whether the entity is newly created, moved from another
/// container, or restored.
pub async fn next_category_position(tx: &mut WriteTx, category: EntityId) -> sqlx::Result<i32> {
    let row = sqlx::query(
        "SELECT coalesce(max(category_position), -1) + 1 AS next \
         FROM entity WHERE category_id = $1",
    )
    .bind(category.as_uuid())
    .fetch_one(tx.conn())
    .await?;
    row.try_get("next")
}

/// One entity that has just left a container, as the change sequence needs it.
pub struct Uncategorised {
    pub id: EntityId,
    pub audience: Vec<MemberRef>,
    pub author: Option<MemberRef>,
    pub counter: i64,
}

/// Take every entity out of a category, because the category is gone.
///
/// The substrate's rule is that *an entity whose category is deleted becomes
/// uncategorised, which is a valid state requiring no repair*, and erasure is
/// the stronger form of the same thing. Leaving the reference would point every
/// member of the category at a tombstone.
///
/// **Deliberately unscoped**, for the same reason as
/// [`next_category_position`] and a sharper one: the container is gone for
/// everybody, so leaving the entities a writing member cannot see still in it
/// would keep a dangling reference alive precisely where nobody can find it.
/// Nothing leaves this function that a caller can show anyone — the identities
/// go into change entries, and those are assembled per member by the read path,
/// which applies the predicate then.
///
/// The counter advances on each, because it is an accepted write to those
/// entities and a client that holds one has to learn its category went away.
pub async fn uncategorise_all(
    tx: &mut WriteTx,
    category: EntityId,
    at: DateTime<Utc>,
) -> sqlx::Result<Vec<Uncategorised>> {
    let sql = format!(
        "UPDATE entity SET category_id = NULL, category_position = NULL, \
         counter = counter + 1, updated_at = $2 \
         WHERE category_id = $1 \
         RETURNING id, {AUDIENCE_COLUMN} AS audience, author_member_id, counter"
    );
    let rows = sqlx::query(sqlx::AssertSqlSafe(sql))
        .bind(category.as_uuid())
        .bind(at)
        .fetch_all(tx.conn())
        .await?;

    rows.iter()
        .map(|r| {
            Ok(Uncategorised {
                id: r.try_get("id")?,
                audience: r
                    .try_get::<Vec<uuid::Uuid>, _>("audience")?
                    .into_iter()
                    .map(MemberRef::from_uuid)
                    .collect(),
                author: r
                    .try_get::<Option<uuid::Uuid>, _>("author_member_id")?
                    .map(MemberRef::from_uuid),
                counter: r.try_get("counter")?,
            })
        })
        .collect()
}

/// The entities a single removal act put into holding, as this member can see
/// them.
///
/// Scoped, and through [`CandidateQuery`] like every other candidate set: a
/// group is a set of entities, so producing one is producing candidates. That
/// it cannot leak anything in practice — a member could not have removed what
/// they could not see — is not a reason to build the one query that would if
/// the assumption ever stopped holding.
pub async fn holding_group(
    tx: &mut WriteTx,
    group: uuid::Uuid,
    member: MemberId,
) -> sqlx::Result<Vec<EntityRow>> {
    let q = CandidateQuery::new(member, WriteScope::Holding.into(), &columns())
        .and_where("e.deletion_group_id = $?", [Bind::Uuid(group)]);
    let rows = q.build().fetch_all(tx.conn()).await?;
    rows.iter().map(row).collect()
}

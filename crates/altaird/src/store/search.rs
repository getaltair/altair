//! The literal retrieval arm: full text search plus trigram, over the same
//! maintained columns migration one describes — "search_text maintained by
//! the write path, with the tsvector generated from it so the two cannot
//! drift, and trigram matching over the same text." Literal search derives
//! nothing, so it answers from whatever has been captured, whether or not
//! derivation has run over it.
//!
//! Built entirely through [`CandidateQuery`], so it carries the audience
//! predicate the same way every other candidate set on this instance does.

use sqlx::Row;
use sqlx::postgres::PgRow;

use super::audience::{Bind, CandidateQuery, LifecycleScope};
use super::entity::{self, EntityRow, EntityType};
use super::ids::{EntityId, MemberId};
use super::tx::ReadTx;

/// One of the three domains an entity belongs to, for scoping a query to it.
///
/// The substrate names "everything, one domain, one campaign and what hangs
/// off it" as ordinary queries. This is the "one domain" case; [`Scope`]
/// carries the others.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Domain {
    Guidance,
    Knowledge,
    Tracking,
}

impl Domain {
    /// The entity types this domain buckets. Category is not one of them —
    /// it belongs to no single domain, the same way it belongs to no single
    /// document in `docs/`.
    fn entity_types(self) -> &'static [EntityType] {
        use EntityType::{
            Arc, Campaign, CheckIn, File, FocusSession, Item, Location, Note, Quest, Routine,
            ShoppingList,
        };
        match self {
            Domain::Guidance => &[Campaign, Arc, Quest, Routine, FocusSession, CheckIn],
            Domain::Knowledge => &[Note, File],
            Domain::Tracking => &[Item, Location, ShoppingList],
        }
    }
}

/// What a caller may narrow a literal query to. Every field is optional, and
/// leaving all of them unset is "everything" — breadth is available, never
/// obligatory, and a narrow query is not a degraded one.
#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub domain: Option<Domain>,
    pub entity_type: Option<EntityType>,
    /// An entity results must be filed under: itself, or a member of its
    /// category. Containment lives on the shared `entity` row, so this reaches
    /// across every domain the same way; it is not the campaign/arc/quest
    /// ladder specifically, which is a narrower, type-specific relationship.
    pub container: Option<EntityId>,
}

/// One candidate the literal arm found, with its combined match strength.
pub struct LiteralResult {
    pub entity: EntityRow,
    pub score: f64,
    /// Whether this entity has never had an embedding computed over it. See
    /// [`derivation_outstanding`].
    pub lacks_derivation: bool,
}

/// The DB label a type-scoped condition compares against. Spelled out rather
/// than derived from the `sqlx::Type` mapping, so a caller reading this file
/// sees the exact string sent to Postgres.
fn label(t: EntityType) -> &'static str {
    match t {
        EntityType::Campaign => "campaign",
        EntityType::Arc => "arc",
        EntityType::Quest => "quest",
        EntityType::Routine => "routine",
        EntityType::FocusSession => "focus_session",
        EntityType::CheckIn => "check_in",
        EntityType::Note => "note",
        EntityType::File => "file",
        EntityType::Item => "item",
        EntityType::Location => "location",
        EntityType::ShoppingList => "shopping_list",
        EntityType::Category => "category",
    }
}

/// Candidates matching `text`, scoped and ranked.
///
/// Ranking combines full text rank (`ts_rank` against `search_vector`) and
/// trigram similarity (`similarity` against `search_text`) into one score,
/// with a fully deterministic tiebreak — entity id — so the same query over
/// the same data produces byte-identical ordering. Both signals are a
/// property of the entity's match to the query text, never of the requester,
/// so two members running the same query against the same data get the same
/// order.
///
/// `LifecycleScope::Active` always: an ordinary search surface over live
/// entities, the same choice [`entity::candidates`] makes.
pub async fn literal(
    tx: &mut ReadTx,
    member: MemberId,
    text: &str,
    scope: &Scope,
    limit: i64,
) -> sqlx::Result<Vec<LiteralResult>> {
    let mut q = CandidateQuery::new(member, LifecycleScope::Active, &entity::columns())
        .select_also(
            "(ts_rank(e.search_vector, plainto_tsquery('english', $?)) \
             + similarity(e.search_text, $?))::float8 AS altair_score",
            [Bind::Text(text.to_owned()), Bind::Text(text.to_owned())],
        )
        .select_also(
            "NOT EXISTS (SELECT 1 FROM embedding em WHERE em.entity_id = e.id) \
             AS altair_lacks_derivation",
            [],
        )
        .and_where(
            "(e.search_vector @@ plainto_tsquery('english', $?) \
             OR similarity(e.search_text, $?) > 0.1)",
            [Bind::Text(text.to_owned()), Bind::Text(text.to_owned())],
        );

    if let Some(domain) = scope.domain {
        let types = domain.entity_types();
        let condition = types
            .iter()
            .map(|_| "e.type = $?::entity_type")
            .collect::<Vec<_>>()
            .join(" OR ");
        let binds = types
            .iter()
            .map(|t| Bind::Text(label(*t).to_owned()))
            .collect::<Vec<_>>();
        q = q.and_where(&format!("({condition})"), binds);
    }

    if let Some(entity_type) = scope.entity_type {
        q = q.and_where(
            "e.type = $?::entity_type",
            [Bind::Text(label(entity_type).to_owned())],
        );
    }

    if let Some(container) = scope.container {
        q = q.and_where(
            "(e.id = $? OR e.category_id = $?)",
            [
                Bind::Uuid(container.as_uuid()),
                Bind::Uuid(container.as_uuid()),
            ],
        );
    }

    let rows = q
        .tail(
            "ORDER BY altair_score DESC, e.id ASC LIMIT $?",
            [Bind::Int(limit)],
        )
        .build()
        .fetch_all(tx.conn())
        .await?;

    rows.iter().map(row).collect()
}

fn row(r: &PgRow) -> sqlx::Result<LiteralResult> {
    Ok(LiteralResult {
        entity: entity::row(r)?,
        score: r.try_get("altair_score")?,
        lacks_derivation: r.try_get("altair_lacks_derivation")?,
    })
}

/// Whether any of these results has content derivation has not yet covered.
/// Honest rather than hardcoded: nothing writes an `embedding` row before
/// Wave 5's derivation worker exists, so this is presently "was the result
/// set nonempty" — a true statement about the instance's current state of
/// coverage, not a placeholder standing in for one.
pub fn derivation_outstanding(results: &[LiteralResult]) -> bool {
    results.iter().any(|r| r.lacks_derivation)
}

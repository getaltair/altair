//! `Changes`: one member's own stream, assembled from the sequence every
//! write appends to.
//!
//! See `write::changes` for what an entry says and why positions cannot have
//! gaps. This is the half that module's docs hand to Wave 3.2: turning the
//! sequence, which is written once for the whole household, into the stream
//! one member is shown.
//!
//! # Filtered at the source, not after
//!
//! The architecture is explicit that a shared stream filtered late — fetch
//! broadly, filter per consumer — is "the most likely place in the system for
//! a leak." An entity change carries its own audience before and after the
//! write (see [`crate::write::changes::EntityChange`]), so the SQL below
//! tests membership in those two arrays directly, in the `WHERE` clause, the
//! same way [`crate::store::audience`] puts its predicate inside the query
//! that produces candidates rather than around it.
//!
//! A relation change carries no audience at all — a relation's own row has
//! none to carry — so it cannot be filtered the same way. Every
//! `relation_written` and `relation_gone` row in range is fetched and then
//! checked against [`crate::store::relation::endpoints_visible`], which is
//! itself audience-scoped through [`crate::store::entity::available_for_read`].
//! Nothing unfiltered ever leaves this function; the difference is only
//! whether the filter runs in the query or immediately after it, inside the
//! same read transaction, before anything is returned to a caller.

use sqlx::Row;
use sqlx::postgres::{PgPool, PgRow};
use uuid::Uuid;

use altair_proto::v1;

use crate::store::entity::available_for_read;
use crate::store::ids::{EntityId, MemberId};
use crate::store::{ReadScope, begin_read, relation};

/// How many changes one response carries at most.
///
/// A household-scale instance, not a public one: this is a page size chosen
/// to keep one response small, not a limit load-tested against anything.
const PAGE_SIZE: i64 = 200;

/// What assembling a member's stream produced.
pub enum Outcome {
    Answered(v1::ChangeSet),
    /// The stated position is before what the sequence still holds. An
    /// outcome, not a fault: the caller's correct response is to rebuild.
    Unanswerable,
}

/// Assemble one member's change set since a position they hold.
///
/// `since` absent means position zero — a client with nothing yet — and is
/// answered exactly as any other position: unanswerable if the sequence no
/// longer reaches back that far, otherwise everything from the start that
/// this member may see.
///
/// # Errors
///
/// Only a store fault. No property of the request produces one.
pub async fn assemble(
    pool: &PgPool,
    member: MemberId,
    since: Option<v1::Position>,
) -> sqlx::Result<Outcome> {
    // The wire's `Position` is a bare `uint64`; nothing on the wire bounds it
    // to what this store could ever have issued. Our own sequence is `bigint`
    // (`i64`), so a value that does not fit is not a position this instance
    // ever produced — saturating it to `i64::MAX` rather than truncating with
    // `as` keeps it in the "nothing new, already ahead of everything real"
    // shape instead of silently wrapping to a small or negative number and
    // answering a completely different, wrong question.
    let since_value = since.map_or(0i64, |p| i64::try_from(p.value).unwrap_or(i64::MAX));

    let mut tx = begin_read(pool).await?;

    // The horizon check and the page read must see one consistent snapshot
    // of `change`, or a concurrent write or reclamation landing between two
    // separate statements could make either wrong: a row deleted after the
    // check but before the read makes an answerable request silently miss
    // history instead of correctly answering `PositionUnanswerable`, and a
    // row written after `latest` was captured but before the page read would
    // make the reported "no more" cursor stale relative to what the response
    // actually contains. One statement, not two: `bounds` and `page` are
    // evaluated together, so both come from the same view of the table
    // rather than from two queries a concurrent commit could land between.
    //
    // `LEFT JOIN` rather than a plain join because `bounds` is exactly one
    // row and must survive even when `page` matches nothing — an empty
    // result cannot be told apart from "nothing to report" otherwise.
    let rows = sqlx::query(
        "WITH bounds AS ( \
             SELECT (SELECT MIN(position) FROM change) AS earliest, \
                    (SELECT next_position - 1 FROM change_position) AS latest \
         ), \
         page AS ( \
             SELECT position, kind::text AS kind, entity_id, relation_id, \
                    changed_block_ids \
             FROM change \
             WHERE position > $1 \
               AND ( \
                     (kind IN ('entity_written', 'entity_gone') \
                      AND ( (author_before = $2 OR $2 = ANY(audience_before)) \
                         OR (author_after  = $2 OR $2 = ANY(audience_after)) )) \
                  OR kind IN ('relation_written', 'relation_gone') \
               ) \
             ORDER BY position ASC \
             LIMIT $3 \
         ) \
         SELECT bounds.earliest, bounds.latest, page.position, page.kind, \
                page.entity_id, page.relation_id, page.changed_block_ids \
         FROM bounds LEFT JOIN page ON true \
         ORDER BY page.position ASC NULLS FIRST",
    )
    .bind(since_value)
    .bind(member.as_uuid())
    .bind(PAGE_SIZE + 1)
    .fetch_all(tx.conn())
    .await?;

    // `bounds` is exactly one row, so this always has at least one element.
    let earliest: Option<i64> = rows[0].try_get("earliest")?;
    let latest: i64 = rows[0].try_get("latest")?;

    // `since_value` is already clamped to `i64::MAX` above, so the one
    // remaining overflow is `+ 1` when it lands exactly on that clamp.
    // Nothing real is ever positioned there, so there is nothing this
    // client could be missing — `checked_add` failing here means "already
    // past anything the horizon check could name," which is answerable, not
    // unanswerable, so `is_some_and` rather than `is_none_or`.
    if let Some(earliest) = earliest
        && since_value
            .checked_add(1)
            .is_some_and(|next| next < earliest)
    {
        return Ok(Outcome::Unanswerable);
    }

    // `page.position` is null on exactly one row: the placeholder `bounds`
    // produces when nothing in `page` matched. Every other row is a real
    // page row, and `ORDER BY ... NULLS FIRST` puts that placeholder first
    // so it is never mistaken for the last of a full page below.
    let mut first_page_row = rows.len();
    for (i, row) in rows.iter().enumerate() {
        if row.try_get::<Option<i64>, _>("position")?.is_some() {
            first_page_row = i;
            break;
        }
    }
    let page: &[PgRow] = &rows[first_page_row..];

    let more = page.len() as i64 > PAGE_SIZE;
    let page: &[PgRow] = if more {
        &page[..PAGE_SIZE as usize]
    } else {
        page
    };

    let mut changes = Vec::with_capacity(page.len());
    // The boundary of what this page actually scanned, updated whether or
    // not a given row survived its visibility check. A row filtered out
    // below (an invisible relation's endpoints, say) was still examined; not
    // advancing past it would make the next poll re-examine it for nothing,
    // which is waste, not a correctness problem, but there is no reason to
    // pay for it either.
    let mut scanned_to = since_value;

    for row in page {
        let position: i64 = row.try_get("position")?;
        scanned_to = position;
        let kind: String = row.try_get("kind")?;
        let changed_block_ids: Vec<Vec<u8>> = row
            .try_get::<Option<Vec<Uuid>>, _>("changed_block_ids")?
            .unwrap_or_default()
            .into_iter()
            .map(|b| b.as_bytes().to_vec())
            .collect();

        let what = match kind.as_str() {
            "entity_gone" => {
                let id: Uuid = row.try_get("entity_id")?;
                Some(v1::change::What::GoneEntityId(id.as_bytes().to_vec()))
            }
            "entity_written" => {
                let id: Uuid = row.try_get("entity_id")?;
                match available_for_read(
                    &mut tx,
                    member,
                    EntityId::from_uuid(id),
                    ReadScope::Extant,
                )
                .await?
                {
                    Some(entity_row) => Some(v1::change::What::Entity(entity_row.into_wire())),
                    // The row's own before/after snapshot said this member
                    // belongs in the stream for this position, but the
                    // entity is not currently visible to them — narrowed
                    // again since, or otherwise gone. Nothing distinguishes
                    // that from gone anywhere else on this path, and nothing
                    // does here either.
                    None => Some(v1::change::What::GoneEntityId(id.as_bytes().to_vec())),
                }
            }
            "relation_written" | "relation_gone" => {
                let id: Uuid = row.try_get("relation_id")?;
                match relation::load(&mut tx, id).await? {
                    Some(r)
                        if relation::endpoints_visible(
                            &mut tx,
                            member,
                            EntityId::from_uuid(r.from_entity_id),
                            EntityId::from_uuid(r.to_entity_id),
                        )
                        .await? =>
                    {
                        if kind == "relation_gone" {
                            Some(v1::change::What::GoneRelationId(id.as_bytes().to_vec()))
                        } else {
                            Some(v1::change::What::Relation(r.into_wire()))
                        }
                    }
                    // Either the relation is gone from the store entirely
                    // (it never is — see `relation::load`'s own docs — kept
                    // as a safe fallback rather than an assumption) or this
                    // member may not see both endpoints, which is this
                    // relation's whole visibility rule.
                    _ => None,
                }
            }
            _ => None,
        };

        if let Some(what) = what {
            changes.push(v1::Change {
                position: Some(v1::Position {
                    value: position as u64,
                }),
                changed_block_ids,
                what: Some(what),
            });
        }
    }

    // Reaching the end of everything that matched — `more` false — means
    // nothing after `latest` exists yet, visible or not, so the client's next
    // position is the sequence's own latest rather than merely the last row
    // this page happened to touch. Not reaching the end means exactly the
    // opposite: positions past this page are unexamined, so the cursor can
    // only be where scanning stopped.
    let cursor = if more { scanned_to } else { latest };

    Ok(Outcome::Answered(v1::ChangeSet {
        changes,
        position: Some(v1::Position {
            value: cursor as u64,
        }),
        more,
    }))
}

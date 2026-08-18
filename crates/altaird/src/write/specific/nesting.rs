//! Cycle prevention in a nested container, written once for the two that nest.
//!
//! # Why this is a write-path check and not a constraint
//!
//! Migration one refuses the one-step case with
//! `CHECK (parent_location_id IS NULL OR parent_location_id <> entity_id)` and
//! the same on a category, and then records the rest as its gap (h): *"Nothing
//! prevents a cycle in nested locations or categories. Self reference is
//! refused; a longer loop is not, and a recursive query would not terminate on
//! one."* A constraint cannot see past one row, so the check has to run where
//! the write is decided. Wave 0 named this and correctly did not take it; it
//! belongs to type content because both containers are type content.
//!
//! # One helper, not two
//!
//! Nested locations and nested categories are the same shape — a side table
//! with a self-referencing parent column — and are owned by two different
//! lanes. Two implementations of a graph walk is two chances to get the
//! termination wrong, and the one that is wrong is the one nobody runs until a
//! person builds a loop by accident.
//!
//! **Called from nowhere yet.** The Tracking lane calls it for
//! `parent_location_id` and the categories lane for `parent_category_id`, from
//! their own `validate_and_place`. `tests/type_content.rs` exercises it
//! directly rather than through either.
//!
//! # Audience is deliberately not applied
//!
//! Like `next_category_position` in the store, this answers a structural
//! question rather than producing candidates. Walking only the containers the
//! writing member can see would let them build a loop through one they cannot,
//! and the loop would then be real for everybody. Nothing about the answer
//! discloses anything either: it is one boolean about a parent the caller has
//! already named.

use std::collections::HashSet;

use uuid::Uuid;

use crate::store::WriteTx;
use crate::store::ids::EntityId;

use sqlx::Row;

/// Whether making `parent` the parent of `child` would close a loop.
///
/// Generic over the table and its parent column, which is what lets one
/// implementation serve `location.parent_location_id` and
/// `category.parent_category_id`. Both are `&'static str` and come from a
/// [`super::Field`] declaration, so neither is ever built from anything a
/// caller sent.
///
/// # Termination
///
/// The walk keeps the identities it has seen and stops on a repeat. That covers
/// the ordinary case — the walk reaches the top and the parent column is null —
/// and the case this check exists to make impossible but which a hand-written
/// row could still have created: **a loop already in the data**. Reaching one
/// returns `true`, because a parent whose ancestry does not terminate is not a
/// parent anything should be given.
///
/// # Errors
///
/// Only where the store could not be read.
pub async fn would_cycle(
    tx: &mut WriteTx,
    table: &'static str,
    parent_column: &'static str,
    child: EntityId,
    parent: EntityId,
) -> sqlx::Result<bool> {
    // The one-step case, which the schema also refuses. Checked here so a
    // caller gets a refusal it can explain rather than a constraint violation
    // that takes the transaction down.
    if child == parent {
        return Ok(true);
    }

    let sql = format!("SELECT {parent_column} AS parent FROM {table} WHERE entity_id = $1");
    let mut seen: HashSet<Uuid> = HashSet::new();
    seen.insert(child.as_uuid());

    let mut at = parent.as_uuid();
    loop {
        if !seen.insert(at) {
            // Either we have reached the child — the loop this call exists to
            // refuse — or the data already held one. Both answer the same way.
            return Ok(true);
        }
        let row = sqlx::query(sqlx::AssertSqlSafe(sql.clone()))
            .bind(at)
            .fetch_optional(tx.conn())
            .await?;
        let next: Option<Uuid> = match row {
            Some(row) => row.try_get("parent")?,
            // No row for the proposed ancestor. It has no parent to follow, so
            // the chain ends here and nothing loops.
            None => return Ok(false),
        };
        match next {
            Some(next) => at = next,
            None => return Ok(false),
        }
    }
}

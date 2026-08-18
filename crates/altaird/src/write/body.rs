//! Writing a body: dividing it, matching it against the blocks already held,
//! and writing only what changed.
//!
//! **LANE: Bodies owns this file, and owns only this file.** The dispatch on
//! either side of it is written: [`super::content`] reads a body off the wire
//! into a [`BodyWrite`], [`super::entity`] hands one here for every verb that
//! can carry one, and everything this returns is already wired into
//! provenance, conflict detection, and the change sequence. What is missing is
//! the middle.
//!
//! # Why a body is not a part
//!
//! Everything else a write addresses is one part with one value. A body is one
//! field on the wire and many parts in the store: the substrate makes the
//! *block* the smallest independently addressable unit of a body, because
//! "treating the field as the unit would make two people working on different
//! paragraphs of a shared plan look like a disagreement". Which blocks a body
//! write touches is not knowable from the text alone — it depends on the blocks
//! already stored — so this is the one place where reading the wire cannot say
//! what a write touched, and the answer comes back from here instead.
//!
//! # What is already built, and must be used rather than rewritten
//!
//! Wave 1.2 landed both halves of the rule as pure functions, and DR-004 puts
//! them at the instance so devices cannot disagree about the units
//! reconciliation is decided in:
//!
//! - [`crate::body::divide`] — text to boundaries. Atomic constructs never
//!   split; list items do; a long unbroken stretch of prose being one block is
//!   correct rather than a bug.
//! - [`crate::body::reconcile`] — matching the new boundaries against the
//!   stored blocks so identity survives an edit to a block and to its
//!   neighbours.
//!
//! # What this owes when it is filled in
//!
//! - Only two types carry a body and the store refuses a block on any other,
//!   which is `block.body_type CHECK (body_type IN ('note', 'shopping_list'))`.
//! - `block_position_unique` is `DEFERRABLE INITIALLY DEFERRED` on purpose:
//!   rewriting a body renumbers what survives, and the intermediate states of
//!   that renumbering are not violations.
//! - **Every block whose text moved must appear in [`BodyTouch::written`].**
//!   `entity_part_counter` is per part, conflict detection asks which parts
//!   moved between two counter values, and a block that does not record its
//!   movement is invisible to it. The failure is silent.
//! - The stored text goes in [`BodyTouch::touching`] beside the arriving text,
//!   because that pair is what decides a conflict, and *writes producing the
//!   same value are not divergent* is a comparison over exactly those two
//!   strings.

use uuid::Uuid;

use crate::store::entity::EntityType;
use crate::store::ids::EntityId;

use super::content::BodyWrite;
use super::entity::{Applied, Ctx, Refusal};
use super::parts::Part;

/// What writing a body touched, at the grain a conflict is decided at.
#[derive(Debug, Clone, Default)]
pub struct BodyTouch {
    /// Per block: the part, the text arriving, and the text it displaced. This
    /// is the shape conflict detection takes, and it is a list because a body
    /// write is many parts at once.
    pub touching: Vec<(Part, Option<String>, Option<String>)>,
    /// Every block part this write moved, for provenance. A superset of the
    /// parts that conflicted and a subset of the blocks the body has.
    pub written: Vec<Part>,
    /// The blocks the change sequence should name, so a poller learns which
    /// parts of a body it needs rather than the whole thing.
    pub changed: Vec<Uuid>,
}

/// Write a body, returning what it touched.
///
/// **Not built yet.** Refusing is the honest answer: accepting silently would
/// tell a client its writing had landed when no block existed, and the outbox
/// would never send it again.
///
/// # Errors
///
/// Refuses until the Bodies lane fills this in; after that, refuses a body on a
/// type that carries none, and faults only where the store could not be
/// written.
pub async fn apply(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    body: &BodyWrite,
) -> Applied<BodyTouch> {
    let _ = (ctx, entity, body);
    Err(Refusal::Malformed(format!(
        "a body is divided into blocks by the instance and that is not built yet, so this \
         instance will not pretend to have written the body of a {}",
        super::entity::type_name(kind)
    ))
    .into())
}

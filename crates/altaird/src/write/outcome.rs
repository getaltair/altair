//! What an intent produced, in the three shapes the wire has.
//!
//! Separate from the wire types so that nothing in the write path has to hold a
//! generated message, and so that the store's enum, the wire's oneof, and this
//! cannot quietly grow a fourth member each.

use uuid::Uuid;

use crate::store::ids::EntityId;

use super::parts::Part;

/// Parts whose two values are both retained, in the shape the wire names them.
///
/// Empty is not the same as absent: an acknowledgement carries a conflict only
/// where one was detected, so the caller checks [`ConflictParts::is_empty`]
/// rather than sending an empty one.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConflictParts {
    pub content_fields: Vec<u32>,
    pub specific_fields: Vec<u32>,
    pub block_ids: Vec<Uuid>,
}

impl ConflictParts {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.content_fields.is_empty()
            && self.specific_fields.is_empty()
            && self.block_ids.is_empty()
    }

    pub fn push(&mut self, part: &Part) {
        match part {
            Part::Content(p) => self.content_fields.push(p.field_number()),
            Part::Specific(n) => self.specific_fields.push(*n),
            Part::Block(id) => self.block_ids.push(*id),
        }
    }
}

/// Why an intent was refused.
///
/// **The two are indistinguishable from outside and that is the point.**
/// `NotAvailable` covers an entity that does not exist and one the submitting
/// member cannot see, and nothing anywhere — not the reason, not the detail,
/// not the gRPC status — tells them apart, so a refusal never reveals that
/// something is there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefusalReason {
    NotAvailable,
    Malformed,
}

/// What the instance answered, and will answer again on replay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Applied {
        entities: Vec<EntityId>,
        relations: Vec<Uuid>,
        /// The counter after the write, where a single entity was written.
        counter: Option<i64>,
        conflict: Option<ConflictParts>,
    },
    Refused {
        reason: RefusalReason,
        /// Free text for a person reading a log, never parsed by a client.
        ///
        /// **Empty for [`RefusalReason::NotAvailable`], always.** A detail
        /// distinguishing "no such entity" from "not yours to see" would undo
        /// the whole point of there being one reason, and it would do it in the
        /// field nobody thinks of as carrying meaning.
        detail: String,
    },
    Recreated {
        original: EntityId,
        new: EntityId,
        counter: i64,
    },
}

impl Outcome {
    /// A refusal that says nothing, which is the only kind
    /// [`RefusalReason::NotAvailable`] has.
    #[must_use]
    pub fn not_available() -> Self {
        Self::Refused {
            reason: RefusalReason::NotAvailable,
            detail: String::new(),
        }
    }

    /// A refusal for an intent that could not be read as a well-formed one.
    ///
    /// The detail is safe to be specific: malformed is a statement about the
    /// message the caller sent, which the caller already holds.
    #[must_use]
    pub fn malformed(detail: impl Into<String>) -> Self {
        Self::Refused {
            reason: RefusalReason::Malformed,
            detail: detail.into(),
        }
    }

    /// Applied, affecting one entity, with no conflict.
    #[must_use]
    pub fn applied_entity(entity: EntityId, counter: i64) -> Self {
        Self::Applied {
            entities: vec![entity],
            relations: Vec::new(),
            counter: Some(counter),
            conflict: None,
        }
    }
}

//! Identifiers as they cross this layer.
//!
//! DR-007: every identifier is a random UUIDv4 and nothing ever reads one.
//! Nothing here parses, orders, or derives meaning from an identifier; the
//! newtypes exist only so that a member cannot be passed where an entity is
//! expected. That mistake would be silent, and in the audience predicate it
//! would be a leak.

use uuid::Uuid;

/// A household member, as the instance has already established them.
///
/// **Constructing one is an assertion.** It says the caller has resolved this
/// member from a validated token and that the membership *currently
/// participates* in the household — that is, it has not departed. The substrate
/// puts it plainly: an audience entry "confers nothing by itself. Access
/// follows current participation."
///
/// Participation is therefore checked once, where the member is resolved, and
/// not inside the audience predicate. It is a fact about the requester, not
/// about the row, so it is constant across every candidate in a query; joining
/// `membership` into every candidate query to re-answer it would pay per row
/// for a check that cannot vary, and would be a second place the rule could be
/// forgotten.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemberId(Uuid);

impl MemberId {
    /// The caller asserts this membership exists and currently participates.
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

/// An entity, by the identifier whoever created it generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct EntityId(Uuid);

impl EntityId {
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

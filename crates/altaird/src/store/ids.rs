//! Identifiers as they cross this layer.
//!
//! DR-007: every identifier is a random UUIDv4 and nothing ever reads one.
//! Nothing here parses, orders, or derives meaning from an identifier; the
//! newtypes exist only so that a member cannot be passed where an entity is
//! expected. That mistake would be silent, and in the audience predicate it
//! would be a leak.

use uuid::Uuid;

/// A household member, as the instance has already resolved them.
///
/// # What this type does and does not enforce
///
/// The audience predicate deliberately does not check `membership.departed_at`,
/// because participation is a fact about the *requester* — constant across
/// every candidate in a query — rather than about the row. Re-answering it per
/// candidate would pay for a value that cannot vary, and would be a second
/// place the rule can be forgotten. The substrate is what requires it at all:
/// an audience entry "confers nothing by itself. Access follows current
/// participation."
///
/// So something has to establish participation before a `MemberId` exists, and
/// this type is the marker that something did. **What the compiler enforces is
/// that the marker cannot be minted outside this crate:**
/// [`MemberId::assert_participating`] is crate-visible, so the set of places
/// that can produce one is the set of modules in `altaird`, which is small and
/// greppable. The public API offers no way to construct one at all.
///
/// **What the compiler does not enforce is that any given call is correct.**
/// The name says `assert` because that is what a call site is doing. Within the
/// crate, exactly one caller should be right: the path that resolves a
/// validated token's subject to a membership row and filters
/// `departed_at IS NULL`. Anything else minting one is a bug, and this doc is
/// the only thing that says so.
///
/// An earlier version of this comment claimed the type carried the assertion.
/// It did not — the constructor was public and took a bare `Uuid` — and the
/// composed system was correct only because token validation independently
/// filtered departed members. That is worth recording rather than quietly
/// fixing, because the failure was a doc comment describing a guarantee nobody
/// had built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemberId(Uuid);

impl MemberId {
    /// The caller asserts this membership exists and currently participates in
    /// the household.
    ///
    /// Crate-visible on purpose. Callers outside `altaird` have no route to a
    /// `MemberId`, so they cannot address the audience predicate at all.
    // Unused until the token-validation lane lands and calls it. Kept rather
    // than deferred, because it is what makes the constructor crate-private,
    // and a public constructor "for now" is how the invariant was lost the
    // first time.
    #[allow(dead_code)]
    pub(crate) fn assert_participating(id: Uuid) -> Self {
        Self(id)
    }

    /// For tests, which stand in for the resolution path.
    ///
    /// Behind the `testing` feature so it cannot reach a release build. It is
    /// separate from [`MemberId::assert_participating`] rather than a widening
    /// of it, so that broadening the real constructor's visibility stays a
    /// deliberate act.
    #[cfg(any(test, feature = "testing"))]
    pub fn for_test(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

/// A membership as *stored data* refers to it: an author, an audience entry,
/// an assignment.
///
/// Deliberately not a [`MemberId`], and there is no conversion. The substrate
/// says references survive departure — "authorship never changes, so someone
/// who leaves remains the author of what they wrote. An audience entry naming
/// them stays where it is" — so a value read out of a row carries no claim
/// about participation, which is exactly the claim `MemberId` exists to carry.
///
/// Making them one type would let a row's own author be handed back to the
/// audience predicate as a requester, which is the shape of the bug that
/// separating them prevents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemberRef(Uuid);

impl MemberRef {
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl MemberId {
    /// Whether this requester is the member a stored reference names.
    ///
    /// The comparison the two types would otherwise tempt someone to make by
    /// unifying them.
    pub fn is(self, other: MemberRef) -> bool {
        self.as_uuid() == other.as_uuid()
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

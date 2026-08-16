//! The audience predicate. **This file is the only place it exists.**
//!
//! `tests/one_predicate.rs` walks the crates and fails if a second copy
//! appears, including a paraphrased one, because it also refuses any other
//! Rust or SQL source that names the audience column at all. Read that test
//! before adding a query over `entity` anywhere.
//!
//! # What the predicate says
//!
//! An entity is visible to a member if that member authored it or is named in
//! its audience. The SQL is `AUDIENCE_PREDICATE` below and is not reproduced
//! here, because a copy in a doc comment is a copy that can drift.
//!
//! Derived from the substrate specification's *Audience* section — "audience
//! attaches to the entity and answers exactly one question: who in the
//! household can see this" — and from the schema, where `audience_member_ids`
//! is an enumerated `uuid[]` whose empty default means private to the author.
//!
//! Four things it deliberately does not do:
//!
//! - **It does not treat a null author as visible.** An entity captured before
//!   its device was bound has no author yet, and the substrate says an
//!   unattributed entity "has none and cannot be shared". `NULL = $1` is NULL
//!   rather than true, so such a row is visible to nobody until binding gives
//!   it an author. That is the intended answer and it is also the closed one,
//!   so the failure mode of this expression is silence rather than a leak.
//!   Nothing here may be softened into `COALESCE` or `IS NOT DISTINCT FROM`.
//! - **It does not consult lifecycle.** See [`LifecycleScope`].
//! - **It does not consult `membership.administrator`.** The architecture is
//!   explicit that the flag "does not touch audience, and an administrator does
//!   not see another member's private entities".
//! - **It does not re-check participation.** See [`MemberId`].
//!
//! # Why it is a fragment and not a filter
//!
//! The standing constraint is that the predicate sits *inside* the query that
//! produces candidates, on every path including similarity, and is never
//! applied to rows after they come back. Post-filtering both leaks and gets
//! limits wrong: trimming a fixed-size candidate set can empty a result that
//! had matches the person was entitled to (DR-002).
//!
//! [`CandidateQuery`] is therefore the only way this layer will build a query
//! over `entity`, and its constructor emits the predicate. There is no
//! constructor that omits it and no method that removes it.

use sqlx::AssertSqlSafe;
use sqlx::postgres::Postgres;
use sqlx::query::Query;
use uuid::Uuid;

use super::ids::MemberId;

/// The audience predicate.
///
/// `$1` is the requesting member, always. [`CandidateQuery`] guarantees that
/// by binding it first and starting every other bind at `$2`.
///
/// `e` is the fixed alias for `entity`.
const AUDIENCE_PREDICATE: &str = "(e.author_member_id = $1 OR $1 = ANY (e.audience_member_ids))";

/// The column's name, and the only spelling of it in the tree.
///
/// Selecting and writing the column are both legitimate — the write path owes
/// `audience_before` and `audience_after` to the change sequence — but from
/// outside a file they are indistinguishable from a second copy of the
/// predicate, and the test that keeps this module honest cannot tell them
/// apart either. So anything that must name the column takes the name from
/// here, and the test can then insist that no other source spells it out.
///
/// Cheap, and it makes "the predicate is written once" a fact about the whole
/// repository rather than about the files somebody remembered to check.
pub const AUDIENCE_COLUMN: &str = "audience_member_ids";

/// The predicate's SQL, for the test that asserts it exists exactly once.
///
/// Nothing else should call this. Building a query from the string by hand is
/// the thing this module exists to prevent; use [`CandidateQuery`].
pub fn predicate_sql() -> &'static str {
    AUDIENCE_PREDICATE
}

/// Which lifecycle states a query is asking about.
///
/// **Lifecycle is not part of the audience predicate, and this type is the
/// evidence that the separation is deliberate.** Three things in the substrate
/// force it:
///
/// - A deleted entity goes to a holding state where it "remain[s] listed
///   somewhere the user can go and look, and restoring is a single action". A
///   surface that lists it has to be writable without escaping the audience
///   predicate — which it could not be if the predicate had already excluded
///   deleted rows.
/// - An edit arriving for an erased entity is a *recreation*, not a create.
///   The write path can only tell those apart by finding the tombstone, so the
///   tombstone has to be reachable.
/// - DR-002 names lifecycle as its own thing: "a relation record surviving the
///   deletion of an endpoint is a predicate on that endpoint's lifecycle
///   state".
///
/// So audience answers *who may see this at all*, permanently; lifecycle
/// answers *which surface shows it now*. Folding the second into the first
/// would produce a predicate that has to be bypassed to build the holding list
/// and the recreation check, and a predicate with two legitimate bypasses stops
/// being enforced.
///
/// There is no default. Every caller states which it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleScope {
    /// Ordinary surfaces: lists, search, traversal.
    Active,
    /// The holding state a deleted entity sits in until the window expires.
    Holding,
    /// Active or deleted. Everything that still has content.
    Extant,
    /// Including tombstones. The write path needs this to tell a recreation
    /// from a create; a read surface almost certainly does not.
    AnyIncludingErased,
}

impl LifecycleScope {
    fn sql(self) -> &'static str {
        match self {
            Self::Active => "e.lifecycle = 'active'",
            Self::Holding => "e.lifecycle = 'deleted'",
            Self::Extant => "e.lifecycle <> 'erased'",
            Self::AnyIncludingErased => "TRUE",
        }
    }
}

/// A value bound into a candidate query after the member.
#[derive(Debug, Clone)]
pub enum Bind {
    Uuid(Uuid),
    Text(String),
    Int(i64),
    Bool(bool),
}

/// A query over `entity` that carries the audience predicate by construction.
///
/// The constructor emits `FROM entity e WHERE <audience predicate> AND
/// <lifecycle>`. Everything a caller adds is conjoined onto that, so there is
/// no sequence of calls on this type that produces a query over `entity`
/// without the predicate in it.
///
/// The select list and any added fragment are caller-supplied SQL. This type
/// cannot stop someone writing a second `FROM entity` inside a subquery there;
/// the one-place test is what stops that, by refusing any other file that names
/// the audience column.
pub struct CandidateQuery {
    sql: String,
    member: Uuid,
    binds: Vec<Bind>,
}

impl CandidateQuery {
    /// `select_list` is the projection, written against the alias `e`.
    pub fn new(member: MemberId, lifecycle: LifecycleScope, select_list: &str) -> Self {
        let sql = format!(
            "SELECT {select_list} FROM entity e WHERE {AUDIENCE_PREDICATE} AND {}",
            lifecycle.sql()
        );
        Self {
            sql,
            member: member.as_uuid(),
            binds: Vec::new(),
        }
    }

    /// Conjoins another condition. Write `$?` for each bound value; positions
    /// are assigned here, starting at `$2`, because `$1` is the member.
    ///
    /// # Panics
    ///
    /// If the number of `$?` markers does not match the number of binds. That
    /// is a programming error in a string literal, and a mismatch would
    /// otherwise surface as a runtime type error far from its cause.
    pub fn and_where(mut self, condition: &str, binds: impl IntoIterator<Item = Bind>) -> Self {
        let (rendered, binds) = self.render(condition, binds);
        self.sql.push_str(" AND ");
        self.sql.push_str(&rendered);
        self.binds.extend(binds);
        self
    }

    /// Substitutes `$?` for the positions these binds will occupy. `$1` is the
    /// member, so the first substitution here is `$2`.
    fn render(&self, fragment: &str, binds: impl IntoIterator<Item = Bind>) -> (String, Vec<Bind>) {
        let binds: Vec<Bind> = binds.into_iter().collect();
        let mut rendered = String::with_capacity(fragment.len());
        let mut used = 0usize;
        let mut rest = fragment;
        while let Some(at) = rest.find("$?") {
            rendered.push_str(&rest[..at]);
            used += 1;
            rendered.push_str(&format!("${}", self.binds.len() + used + 1));
            rest = &rest[at + 2..];
        }
        rendered.push_str(rest);
        assert_eq!(
            used,
            binds.len(),
            "fragment has {used} placeholders and {} binds",
            binds.len()
        );
        (rendered, binds)
    }

    /// Appends trailing SQL that is not a condition: `ORDER BY`, `LIMIT`,
    /// `FOR UPDATE`. Same `$?` convention.
    ///
    /// # Panics
    ///
    /// As [`CandidateQuery::and_where`].
    pub fn tail(mut self, sql: &str, binds: impl IntoIterator<Item = Bind>) -> Self {
        let (rendered, binds) = self.render(sql, binds);
        self.sql.push(' ');
        self.sql.push_str(&rendered);
        self.binds.extend(binds);
        self
    }

    /// The assembled SQL. For diagnostics and for tests.
    pub fn sql(&self) -> &str {
        &self.sql
    }

    /// The query, with the member bound as `$1` and everything else after it.
    pub fn build(&self) -> Query<'_, Postgres, sqlx::postgres::PgArguments> {
        // Audited, which is what `AssertSqlSafe` asks for: every part of this
        // string is either a constant in this file or a fragment a caller
        // wrote as a literal. No value reaches it — values are bound, and
        // `$?` is substituted for a position, never for content.
        let mut q = sqlx::query(AssertSqlSafe(self.sql.clone())).bind(self.member);
        for b in &self.binds {
            q = match b {
                Bind::Uuid(v) => q.bind(*v),
                Bind::Text(v) => q.bind(v.as_str()),
                Bind::Int(v) => q.bind(*v),
                Bind::Bool(v) => q.bind(*v),
            };
        }
        q
    }
}

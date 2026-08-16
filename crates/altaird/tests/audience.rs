//! Audience, observed through both paths.
//!
//! Neither the write path (Wave 2) nor the read path (Wave 3) exists, so these
//! exercise the two *shapes* the store layer offers them:
//! `entity::available_for_write`, the "may this member act on this" lookup, and
//! `entity::candidates`, the query that produces candidates. Both are built
//! from the same `CandidateQuery`, so both carry the same predicate; the point
//! of testing both is that they *agree*, including on what they refuse.
//!
//! Rows are inserted with raw SQL here on purpose. The write path is Wave 2's,
//! and a fixture that pre-empted it would be a second write implementation.

use altaird::store::audience::AUDIENCE_COLUMN;
use altaird::store::entity::{self, LifecycleState};
use altaird::store::{EntityId, MemberId, ReadScope, WriteScope};
use altaird::store::{begin_read, begin_write};
use altaird::testing::TestDb;
use sqlx::PgPool;
use uuid::Uuid;

async fn household(pool: &PgPool) -> Uuid {
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO household (id, name, created_at) VALUES ($1, 'test', now())")
        .bind(id)
        .execute(pool)
        .await
        .expect("household");
    id
}

async fn member(pool: &PgPool, household_id: Uuid, administrator: bool) -> MemberId {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO membership (id, household_id, subject, administrator, joined_at) \
         VALUES ($1, $2, $3, $4, now())",
    )
    .bind(id)
    .bind(household_id)
    .bind(id.to_string())
    .bind(administrator)
    .execute(pool)
    .await
    .expect("membership");
    MemberId::for_test(id)
}

/// `author` is `None` for an entity captured before its device was bound.
async fn note(
    pool: &PgPool,
    author: Option<MemberId>,
    audience: &[MemberId],
    lifecycle: LifecycleState,
) -> EntityId {
    let id = Uuid::new_v4();
    let audience: Vec<Uuid> = audience.iter().map(|m| m.as_uuid()).collect();
    let state = match lifecycle {
        LifecycleState::Active => "active",
        LifecycleState::Deleted => "deleted",
        LifecycleState::Erased => "erased",
    };
    // The column is named through the store's constant, like everything else
    // that has to name it. See `tests/one_predicate.rs`.
    let sql = format!(
        "INSERT INTO entity \
           (id, type, title, author_member_id, created_at, updated_at, capture_method, \
            {AUDIENCE_COLUMN}, lifecycle, deleted_at) \
         VALUES ($1, 'note', 'a note', $2, now(), now(), 'test', $3, $4::lifecycle_state, \
            CASE WHEN $4 = 'deleted' THEN now() ELSE NULL END)"
    );
    sqlx::query(sqlx::AssertSqlSafe(sql))
        .bind(id)
        .bind(author.map(|m| m.as_uuid()))
        .bind(&audience)
        .bind(state)
        .execute(pool)
        .await
        .expect("entity");
    EntityId::from_uuid(id)
}

/// Both shapes, one answer. `(write path saw it, read path saw it)`.
///
/// The two scopes are separate arguments because the read path can no longer
/// name the erased scope at all; where they mean the same thing the wrappers
/// below say so.
async fn seen(
    pool: &PgPool,
    member: MemberId,
    id: EntityId,
    write_scope: WriteScope,
    read_scope: ReadScope,
) -> (bool, bool) {
    let mut w = begin_write(pool).await.expect("begin write");
    let by_write = entity::available_for_write(&mut w, member, id, write_scope)
        .await
        .expect("write path lookup");
    w.rollback().await.expect("rollback");

    let mut r = begin_read(pool).await.expect("begin read");
    let by_read = entity::candidates(&mut r, member, read_scope, 100)
        .await
        .expect("read path candidates");

    (by_write.is_some(), by_read.iter().any(|e| e.id == id))
}

/// The scopes that mean the same thing on both sides.
async fn seen_active(pool: &PgPool, member: MemberId, id: EntityId) -> (bool, bool) {
    seen(pool, member, id, WriteScope::Active, ReadScope::Active).await
}

/// As far as each path is permitted to reach. The write path reaches
/// tombstones; the read path's widest scope stops short of them, which is the
/// point of the split.
async fn seen_anywhere(pool: &PgPool, member: MemberId, id: EntityId) -> (bool, bool) {
    seen(
        pool,
        member,
        id,
        WriteScope::AnyIncludingErased,
        ReadScope::Extant,
    )
    .await
}

#[tokio::test]
async fn an_author_reads_their_own_entity_back_through_both_paths() {
    let db = TestDb::new().await;
    let h = household(&db.pool).await;
    let author = member(&db.pool, h, false).await;

    let id = note(&db.pool, Some(author), &[], LifecycleState::Active).await;

    let mut w = begin_write(&db.pool).await.expect("begin write");
    let by_write = entity::available_for_write(&mut w, author, id, WriteScope::Active)
        .await
        .expect("write path lookup")
        .expect("author may act on their own entity");
    w.rollback().await.expect("rollback");

    let mut r = begin_read(&db.pool).await.expect("begin read");
    let by_read = entity::candidates(&mut r, author, ReadScope::Active, 100)
        .await
        .expect("read path candidates");

    let found = by_read
        .iter()
        .find(|e| e.id == id)
        .expect("author's own entity is a candidate");

    assert_eq!(
        &by_write, found,
        "the two paths disagree about the same row"
    );
    // `is` rather than `==`: an author is a stored reference and a requester
    // is a participating member, and the types are separate so that one cannot
    // be handed to the predicate as the other.
    assert!(by_write.author.is_some_and(|a| author.is(a)));
    assert_eq!(by_write.counter, 1);
}

#[tokio::test]
async fn a_private_entity_is_invisible_to_everyone_else_through_both_paths() {
    let db = TestDb::new().await;
    let h = household(&db.pool).await;
    let author = member(&db.pool, h, false).await;
    let other = member(&db.pool, h, false).await;

    let id = note(&db.pool, Some(author), &[], LifecycleState::Active).await;

    assert_eq!(
        seen_active(&db.pool, other, id).await,
        (false, false),
        "a private entity reached somebody who is not its author"
    );
    assert_eq!(seen_active(&db.pool, author, id).await, (true, true));
}

#[tokio::test]
async fn refusal_on_audience_is_the_same_answer_as_refusal_on_nonexistence() {
    let db = TestDb::new().await;
    let h = household(&db.pool).await;
    let author = member(&db.pool, h, false).await;
    let other = member(&db.pool, h, false).await;

    let hidden = note(&db.pool, Some(author), &[], LifecycleState::Active).await;
    let absent = EntityId::from_uuid(Uuid::new_v4());

    let mut w = begin_write(&db.pool).await.expect("begin write");
    let for_hidden = entity::available_for_write(&mut w, other, hidden, WriteScope::Active)
        .await
        .expect("lookup");
    let for_absent = entity::available_for_write(&mut w, other, absent, WriteScope::Active)
        .await
        .expect("lookup");
    w.rollback().await.expect("rollback");

    assert_eq!(
        for_hidden, for_absent,
        "the caller can tell an entity it may not see from one that is not there"
    );
    assert!(for_hidden.is_none());
}

#[tokio::test]
async fn an_audience_entry_makes_it_visible_through_both_paths() {
    let db = TestDb::new().await;
    let h = household(&db.pool).await;
    let author = member(&db.pool, h, false).await;
    let shared_with = member(&db.pool, h, false).await;
    let outsider = member(&db.pool, h, false).await;

    let id = note(
        &db.pool,
        Some(author),
        &[shared_with],
        LifecycleState::Active,
    )
    .await;

    assert_eq!(seen_active(&db.pool, shared_with, id).await, (true, true));
    assert_eq!(seen_active(&db.pool, outsider, id).await, (false, false));
}

#[tokio::test]
async fn an_unattributed_entity_is_visible_to_nobody() {
    // The substrate: "Audience is defined relative to a household, so an
    // unattributed entity has none and cannot be shared." A null author must
    // not read as a wildcard, which is what `COALESCE` in the predicate would
    // make it.
    let db = TestDb::new().await;
    let h = household(&db.pool).await;
    let a = member(&db.pool, h, false).await;
    let b = member(&db.pool, h, false).await;

    let id = note(&db.pool, None, &[], LifecycleState::Active).await;

    for m in [a, b] {
        assert_eq!(
            seen_anywhere(&db.pool, m, id).await,
            (false, false),
            "an entity with no author reached a member"
        );
    }
}

#[tokio::test]
async fn an_administrator_does_not_see_another_members_private_entity() {
    // The architecture: the flag "does not touch audience, and an administrator
    // does not see another member's private entities".
    let db = TestDb::new().await;
    let h = household(&db.pool).await;
    let author = member(&db.pool, h, false).await;
    let admin = member(&db.pool, h, true).await;

    let id = note(&db.pool, Some(author), &[], LifecycleState::Active).await;

    assert_eq!(
        seen_anywhere(&db.pool, admin, id).await,
        (false, false),
        "the administrator flag has become a permission over entities"
    );
}

#[tokio::test]
async fn lifecycle_scopes_the_query_and_audience_decides_visibility() {
    // Deleted and erased rows stay visible to those the audience admits. The
    // holding list has to be able to show a deleted entity, and the write path
    // has to be able to find an erased tombstone to tell a recreation from a
    // create. Both go through the same predicate.
    let db = TestDb::new().await;
    let h = household(&db.pool).await;
    let author = member(&db.pool, h, false).await;
    let outsider = member(&db.pool, h, false).await;

    let deleted = note(&db.pool, Some(author), &[], LifecycleState::Deleted).await;
    let erased = note(&db.pool, Some(author), &[], LifecycleState::Erased).await;

    assert_eq!(
        seen(
            &db.pool,
            author,
            deleted,
            WriteScope::Active,
            ReadScope::Active
        )
        .await,
        (false, false),
        "a deleted entity appeared on an active surface"
    );
    assert_eq!(
        seen(
            &db.pool,
            author,
            deleted,
            WriteScope::Holding,
            ReadScope::Holding
        )
        .await,
        (true, true),
        "the holding surface cannot see what it holds"
    );
    assert_eq!(
        seen(
            &db.pool,
            author,
            erased,
            WriteScope::Extant,
            ReadScope::Extant
        )
        .await,
        (false, false),
        "a tombstone appeared among entities that still have content"
    );

    // The write path reaches the tombstone. The read path has no scope that
    // can, which is why this asymmetry is spelled out rather than hidden in a
    // wrapper.
    assert_eq!(
        seen_anywhere(&db.pool, author, erased).await,
        (true, false),
        "the write path cannot reach a tombstone, so an edit to an erased \
         entity would read as a create rather than a recreation"
    );

    // And no scope on either path lets audience through.
    for (w, r) in [
        (WriteScope::Active, ReadScope::Active),
        (WriteScope::Holding, ReadScope::Holding),
        (WriteScope::Extant, ReadScope::Extant),
        (WriteScope::AnyIncludingErased, ReadScope::Extant),
    ] {
        assert_eq!(
            seen(&db.pool, outsider, deleted, w, r).await,
            (false, false)
        );
        assert_eq!(seen(&db.pool, outsider, erased, w, r).await, (false, false));
    }
}

#[tokio::test]
async fn no_read_scope_reaches_a_tombstone() {
    // The erased variant does not exist on `ReadScope`, so this is the whole
    // of what a read surface can ask for. A tombstone retains its title until
    // Wave 2 strips it, so reaching one from the read path would be serving
    // content somebody erased.
    let db = TestDb::new().await;
    let h = household(&db.pool).await;
    let author = member(&db.pool, h, false).await;

    let erased = note(&db.pool, Some(author), &[], LifecycleState::Erased).await;

    for scope in [ReadScope::Active, ReadScope::Holding, ReadScope::Extant] {
        let mut r = begin_read(&db.pool).await.expect("begin read");
        let rows = entity::candidates(&mut r, author, scope, 100)
            .await
            .expect("candidates");
        assert!(
            !rows.iter().any(|e| e.id == erased),
            "a read surface reached a tombstone through {scope:?}"
        );
    }
}

#[tokio::test]
async fn the_read_transaction_refuses_a_write() {
    // "Nothing crosses from the read path to the write path. The read path
    // writes nothing, including no record of what was asked." Enforced by the
    // database rather than by discipline.
    let db = TestDb::new().await;
    let mut r = begin_read(&db.pool).await.expect("begin read");

    let attempted = sqlx::query(
        "INSERT INTO household (id, name, created_at) VALUES (gen_random_uuid(), 'x', now())",
    )
    .execute(r.conn_for_test())
    .await;

    let err = attempted.expect_err("the read path was allowed to write");
    let code = err
        .as_database_error()
        .and_then(|e| e.code())
        .map(|c| c.to_string());
    assert_eq!(
        code.as_deref(),
        Some("25006"),
        "expected read_only_sql_transaction, got {err}"
    );
}

#[tokio::test]
async fn an_unattributed_entity_is_not_shareable_by_its_audience_array() {
    // The state the schema permits and the substrate forbids: no author, but a
    // populated audience. Nothing ties the two columns together, so the write
    // path could produce this and the predicate would have to answer for it.
    // The predicate's leading `author_member_id IS NOT NULL` is what makes the
    // answer closed here rather than two components away.
    let db = TestDb::new().await;
    let h = household(&db.pool).await;
    let named = member(&db.pool, h, false).await;

    let id = note(&db.pool, None, &[named], LifecycleState::Active).await;

    assert_eq!(
        seen_anywhere(&db.pool, named, id).await,
        (false, false),
        "an entity with no author was shared by its audience array. The \
         substrate: an unattributed entity 'has none and cannot be shared'"
    );
}

#[test]
fn a_candidate_query_cannot_be_built_without_the_predicate() {
    // Structural rather than behavioural: whatever a caller adds is conjoined
    // onto what the constructor already emitted.
    use altaird::store::{Bind, CandidateQuery, LifecycleScope};

    let m = MemberId::for_test(Uuid::new_v4());
    let q = CandidateQuery::new(m, LifecycleScope::Active, "e.id")
        .and_where("e.type = $?::entity_type", [Bind::Text("note".into())])
        .tail("LIMIT $?", [Bind::Int(10)]);

    assert!(
        q.sql().contains(altaird::store::audience::predicate_sql()),
        "the predicate is not in the assembled SQL: {}",
        q.sql()
    );
    assert!(q.sql().contains("$2::entity_type"), "{}", q.sql());
    assert!(q.sql().ends_with("LIMIT $3"), "{}", q.sql());
}

#[test]
#[should_panic(expected = "names the entity table again")]
fn a_tail_cannot_bolt_on_an_unscoped_second_arm() {
    // The escape the builder used to concede and not control: a second arm
    // over `entity` carries no predicate, and returns everything.
    use altaird::store::{CandidateQuery, LifecycleScope};

    let m = MemberId::for_test(Uuid::new_v4());
    let _ = CandidateQuery::new(m, LifecycleScope::Active, "e.id")
        // Assembled rather than written: `tests/one_predicate.rs` scans for
        // exactly this shape at rest, and a negative test should not read as
        // the thing it forbids.
        .tail(&format!("UNION ALL SELECT e2.id {} entity e2", "FROM"), []);
}

#[test]
#[should_panic(expected = "names the entity table again")]
fn a_projection_cannot_hide_an_unscoped_subquery() {
    use altaird::store::{CandidateQuery, LifecycleScope};

    let m = MemberId::for_test(Uuid::new_v4());
    let _ = CandidateQuery::new(
        m,
        LifecycleScope::Active,
        &format!("e.id, (SELECT count(*) {} entity) AS total", "FROM"),
    );
}

#[test]
#[should_panic(expected = "raw positional parameter")]
fn a_fragment_cannot_write_a_raw_position_and_silently_get_the_requester() {
    // `$1` is the member. Writing it by hand compiles, runs, and compares
    // against the requester — a wrong answer rather than an error.
    use altaird::store::{CandidateQuery, LifecycleScope};

    let m = MemberId::for_test(Uuid::new_v4());
    let _ = CandidateQuery::new(m, LifecycleScope::Active, "e.id")
        .and_where("e.author_member_id = $1", []);
}

#[test]
#[should_panic(expected = "raw positional parameter")]
fn a_fragment_cannot_write_a_raw_position_that_would_be_unbound() {
    use altaird::store::{CandidateQuery, LifecycleScope};

    let m = MemberId::for_test(Uuid::new_v4());
    let _ = CandidateQuery::new(m, LifecycleScope::Active, "e.id").tail("LIMIT $2", []);
}

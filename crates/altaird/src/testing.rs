//! A fresh, migrated database per test.
//!
//! Migrations are applied once into a template, and each test branches it with
//! `CREATE DATABASE ... TEMPLATE`, which Postgres does as a file copy. Per-test
//! isolation for roughly the cost of a connection, so tests stay fast enough
//! not to erode the verify-often cadence.

use sqlx::AssertSqlSafe;
use sqlx::postgres::PgPool;
use sqlx::{Connection, Executor, PgConnection};
use std::sync::Arc;
use tokio::sync::OnceCell;

static TEMPLATE: OnceCell<String> = OnceCell::const_new();

/// Load `.env` before anything reads the environment.
///
/// **Every reader of the environment calls this first, not just the first one
/// that happened to need it.** `prefix` used to read `ALTAIR_TEST_PREFIX`
/// without loading, and `ensure_template` calls `prefix` before `admin_url` —
/// so the template name was computed from an environment `.env` had not
/// reached yet, and every worktree got the default name however its `.env` was
/// written. Five lanes then shared one template, and each one starting a run
/// dropped it with FORCE out from under the others: a suite that had passed
/// twice would fail with *template database "altair_test_template" does not
/// exist*, blaming the lane that was already running rather than the one that
/// had just started.
///
/// dotenvy walks up from the working directory, which is what lets `cargo
/// test` work from a crate directory as well as from the repo root.
fn load_env() {
    static LOAD: std::sync::Once = std::sync::Once::new();
    LOAD.call_once(|| {
        dotenvy::dotenv().ok();
    });
}

fn admin_url() -> String {
    load_env();
    std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set. Check .env at the repo root, and that compose is up.")
}

/// Worktrees share one Postgres. Without a prefix, five parallel lanes fight
/// over one template name.
fn prefix() -> String {
    load_env();
    std::env::var("ALTAIR_TEST_PREFIX").unwrap_or_else(|_| "altair_test".into())
}

fn with_database(url: &str, name: &str) -> String {
    let (base, _) = url.rsplit_once('/').expect("DATABASE_URL has no database");
    format!("{base}/{name}")
}

/// The advisory-lock key for one prefix's template.
///
/// Any stable mapping from the name to an `i64` does; collisions between two
/// different prefixes would only make them wait for each other, which is
/// harmless and vanishingly unlikely.
fn template_lock_key(name: &str) -> i64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut h);
    h.finish() as i64
}

/// The migrated template, created once and then left alone.
///
/// **It is not dropped and rebuilt every time.** It used to be: every test
/// binary opened with `DROP DATABASE ... WITH (FORCE)` and a fresh `CREATE`,
/// which is thirty-odd forced drops per `cargo test --workspace` — and each one
/// makes Postgres take an immediate checkpoint, while `WITH (FORCE)` terminates
/// every other connection to it. Several sessions doing that at once drove the
/// cluster into crash recovery three times in one afternoon, with the symptom
/// landing on whichever lane happened to be connecting (`57P03`, *the database
/// system is in recovery mode*) rather than on the run that caused it.
///
/// So the template is created only when it is absent. **The advisory lock is
/// what makes that safe**: without it two binaries starting together both see
/// nothing, both create, and one fails — or worse, one branches a template the
/// other has not finished migrating yet.
///
/// **A stale template is not a risk in the ordinary case**, because nothing
/// ever writes to it — tests branch it and write to the branch. It goes stale
/// only when the migrations themselves change, and the fix is one line:
///
/// ```text
/// docker exec altair-postgres-1 psql -U postgres -d altair \
///   -c 'DROP DATABASE IF EXISTS "<prefix>_template" WITH (FORCE)'
/// ```
///
/// Rebuilding it on every run to avoid that was paying a constant cost for a
/// rare event, and the cost turned out to be the cluster.
async fn ensure_template() -> String {
    let name = format!("{}_template", prefix());
    let admin = admin_url();

    let mut conn = PgConnection::connect(&admin).await.expect("connect");

    // Held until this connection closes, which is after the template is
    // migrated and ready to branch.
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(template_lock_key(&name))
        .execute(&mut conn)
        .await
        .expect("take the template lock");

    let present: Option<(i32,)> = sqlx::query_as("SELECT 1 FROM pg_database WHERE datname = $1")
        .bind(&name)
        .fetch_optional(&mut conn)
        .await
        .expect("look for the template");

    if present.is_some() {
        // Somebody else built it. Releasing the lock is closing the connection.
        conn.close().await.ok();
        return name;
    }

    let create_sql = format!(r#"CREATE DATABASE "{name}""#);
    conn.execute(sqlx::raw_sql(AssertSqlSafe(create_sql)))
        .await
        .expect("create template");

    // Migrate over a single connection, then close it. Postgres refuses to
    // branch a template that has any connection open, so a pool here would
    // make every later CREATE DATABASE fail intermittently.
    let mut migrating = PgConnection::connect(&with_database(&admin, &name))
        .await
        .expect("connect to template");
    crate::store::MIGRATOR
        .run(&mut migrating)
        .await
        .expect("migrations apply");
    migrating.close().await.ok();

    // Only now, so nobody branches a half-migrated template.
    conn.close().await.ok();

    name
}

/// Drops its database on the way out, so a failing test leaves nothing behind.
pub struct TestDb {
    pub pool: PgPool,
    name: String,
}

impl TestDb {
    pub async fn new() -> Arc<Self> {
        let template = TEMPLATE.get_or_init(ensure_template).await;
        let name = format!("{}_{}", prefix(), uuid::Uuid::new_v4().simple());
        let admin = admin_url();

        let branch_sql = format!(r#"CREATE DATABASE "{name}" TEMPLATE "{template}""#);

        let mut conn = PgConnection::connect(&admin).await.expect("connect");
        conn.execute(sqlx::raw_sql(AssertSqlSafe(branch_sql)))
            .await
            .expect("branch template");
        conn.close().await.ok();

        let pool = PgPool::connect(&with_database(&admin, &name))
            .await
            .expect("connect to test database");

        Arc::new(Self { pool, name })
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let (admin, name) = (admin_url(), self.name.clone());
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("runtime");
            let sql = format!(r#"DROP DATABASE IF EXISTS "{name}" WITH (FORCE)"#);
            rt.block_on(async {
                if let Ok(mut conn) = PgConnection::connect(&admin).await {
                    let _ = conn.execute(sqlx::raw_sql(AssertSqlSafe(sql))).await;
                }
            });
        })
        .join()
        .ok();
    }
}

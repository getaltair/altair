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

async fn ensure_template() -> String {
    let name = format!("{}_template", prefix());
    let admin = admin_url();

    let drop_sql = format!(r#"DROP DATABASE IF EXISTS "{name}" WITH (FORCE)"#);
    let create_sql = format!(r#"CREATE DATABASE "{name}""#);

    let mut conn = PgConnection::connect(&admin).await.expect("connect");
    conn.execute(sqlx::raw_sql(AssertSqlSafe(drop_sql)))
        .await
        .expect("drop template");
    conn.execute(sqlx::raw_sql(AssertSqlSafe(create_sql)))
        .await
        .expect("create template");
    conn.close().await.ok();

    // Migrate over a single connection, then close it. Postgres refuses to
    // branch a template that has any connection open, so a pool here would
    // make every later CREATE DATABASE fail intermittently.
    let mut conn = PgConnection::connect(&with_database(&admin, &name))
        .await
        .expect("connect to template");
    crate::store::MIGRATOR
        .run(&mut conn)
        .await
        .expect("migrations apply");
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

//! What the store must already be, before the daemon will serve anything.
//!
//! **Refuse to start rather than start degraded.** An instance that comes up
//! without `vector` and finds out at the first similarity query has turned a
//! misconfiguration into a retrieval fault, at a moment when nobody is
//! watching the logs and the person is holding a phone. The same
//! misconfiguration found here is a process that never started, which is
//! loud, immediate, and unmistakable.
//!
//! The extensions are named in migration one, which creates both. That does
//! **not** make this redundant: a database restored from a dump taken without
//! them, a managed Postgres that permits no `CREATE EXTENSION`, or an image
//! swapped for one without pgvector all leave the migration recorded as
//! applied over a schema that is missing them. The migration says what this
//! instance asked for; this says what it got.

use sqlx::{PgPool, Row};

/// Extensions migration one creates and the instance depends on.
///
/// `pg_trgm` backs the literal arm's index and `vector` backs the similarity
/// arm's. Both are named here rather than discovered, because a required
/// extension is a decision and a list read out of the schema at runtime would
/// only ever agree with itself.
pub const REQUIRED_EXTENSIONS: &[&str] = &["pg_trgm", "vector"];

/// Which of [`REQUIRED_EXTENSIONS`] the connected database does not have.
///
/// # Errors
///
/// If the store cannot be read at all, which is the other precondition and is
/// reported by the caller as such.
pub async fn missing_extensions(pool: &PgPool) -> sqlx::Result<Vec<String>> {
    let rows = sqlx::query("SELECT extname::text AS name FROM pg_extension")
        .fetch_all(pool)
        .await?;
    let present: Vec<String> = rows
        .iter()
        .map(|r| r.try_get::<String, _>("name"))
        .collect::<sqlx::Result<_>>()?;

    Ok(REQUIRED_EXTENSIONS
        .iter()
        .filter(|wanted| !present.iter().any(|p| p == *wanted))
        .map(|wanted| (*wanted).to_owned())
        .collect())
}

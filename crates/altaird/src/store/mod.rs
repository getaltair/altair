//! The structured store: connection handling, transactions, and the shared
//! query surface the write and read paths both consume.
//!
//! The load-bearing thing in here is [`audience`]. Read its module docs before
//! adding any query over `entity`, on either path.

pub mod audience;
pub mod entity;
pub mod health;
pub mod ids;
pub mod preflight;
pub mod relation;
pub mod search;
pub mod tx;
mod wire;

pub use audience::{Bind, CandidateQuery, LifecycleScope, ReadScope, WriteScope};
pub use ids::{EntityId, MemberId};
pub use tx::{ReadTx, WriteTx, begin_read, begin_write};

use std::str::FromStr;

use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

/// How many connections the daemon holds open.
///
/// **Not a tuning knob, and raising it will not make writes faster.** Every
/// write allocates its position from a single row inside its own transaction
/// (see [`crate::write::changes`]), so writes serialise on that row by design.
/// A larger pool does not shorten that queue, it lengthens it: more connections
/// arrive at the same lock and wait there, holding a backend each. Contention
/// on that row is the design working rather than a size to tune around.
///
/// Eight is enough that reads, the change stream and a body transfer keep
/// moving while one write holds the sequence, for one household.
const POOL_SIZE: u32 = 8;

/// How long the store gets to answer at all before the daemon gives up on
/// starting.
///
/// **This bounds startup and nothing else.** sqlx's own acquire timeout is
/// thirty seconds and it applies to every acquisition, including ones under
/// load, so shortening *that* to make a misconfigured start fail quickly would
/// be shortening the patience a busy instance has for its own pool. A wrong
/// URL or an absent database is misconfiguration, and half a minute of silence
/// before saying so is half a minute an operator spends wondering whether it
/// worked.
///
/// Long enough that a database still finishing its own start is waited for.
/// A supervisor that starts the two together and a compose file that orders
/// them are both ordinary; this is not a substitute for either, and a daemon
/// that gives up is restarted.
const REACHABLE_WITHIN: std::time::Duration = std::time::Duration::from_secs(10);

/// Connect, and apply every migration this build carries.
///
/// **Statement logging is turned off at the source**, not filtered at the
/// subscriber. sqlx logs every statement it runs — at `DEBUG` ordinarily, and
/// at `WARN` with the whole SQL attached the moment one takes longer than a
/// second, which for a write waiting on the sequence row is an ordinary
/// Tuesday. The read path is required to keep no record of what was asked, and
/// a library default that writes the statement into the log is exactly how
/// that invariant is lost somewhere nobody thinks to audit. A subscriber
/// filter would hold until somebody raised a level to chase an unrelated bug;
/// this cannot be re-enabled by configuration at all. See [`crate::daemon::logging`]
/// for the policy it belongs to, and `tests/logging.rs`, which induces the
/// slow-statement path on purpose and greps for what it would have printed.
///
/// # Errors
///
/// If the URL does not parse, the store cannot be reached, or a migration
/// fails to apply. All three are preconditions rather than conditions a
/// request meets, and the daemon refuses to start on any of them.
pub async fn connect(url: &str) -> anyhow::Result<PgPool> {
    let options = PgConnectOptions::from_str(url)?.disable_statement_logging();
    let pool = PgPoolOptions::new()
        .max_connections(POOL_SIZE)
        .connect_lazy_with(options);

    // Reachability, bounded, before anything that might take a while for a
    // good reason. Migrating is deliberately outside the deadline: a migration
    // that is slow is doing work, and abandoning one part-way to report a
    // timeout would be reporting the wrong thing about a store that answered
    // perfectly well.
    tokio::time::timeout(REACHABLE_WITHIN, pool.acquire())
        .await
        .map_err(|_| {
            anyhow::anyhow!(
                "the structured store did not answer within {} seconds",
                REACHABLE_WITHIN.as_secs()
            )
        })??;

    MIGRATOR.run(&pool).await?;
    Ok(pool)
}

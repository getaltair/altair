//! The structured store: connection handling, transactions, and the shared
//! query surface the write and read paths both consume.
//!
//! The load-bearing thing in here is [`audience`]. Read its module docs before
//! adding any query over `entity`, on either path.

pub mod audience;
pub mod entity;
pub mod ids;
pub mod tx;

pub use audience::{Bind, CandidateQuery, LifecycleScope, ReadScope, WriteScope};
pub use ids::{EntityId, MemberId};
pub use tx::{ReadTx, WriteTx, begin_read, begin_write};

use sqlx::postgres::{PgPool, PgPoolOptions};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

pub async fn connect(url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new().max_connections(8).connect(url).await?;
    MIGRATOR.run(&pool).await?;
    Ok(pool)
}

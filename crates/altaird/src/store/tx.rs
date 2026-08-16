//! Transactions, in the two shapes the instance has.
//!
//! The component model gives the read path a `Never` list whose first entry is
//! "Write anything: not entity content, not derived data, not a record of what
//! was asked or opened", and the plan restates it as a standing constraint:
//! nothing crosses from the read path to the write path.
//!
//! That is normally kept true by discipline, which is to say by nobody having
//! written the offending line yet. Here it is kept true by Postgres:
//! [`begin_read`] issues `SET TRANSACTION READ ONLY`, so a write from the read
//! path is an error from the database rather than a review comment. It costs
//! one statement per read transaction and it cannot be forgotten.
//!
//! [`ReadTx`] has no `commit`. There is nothing to commit, and offering the
//! method would suggest otherwise.

use sqlx::postgres::{PgConnection, PgPool, Postgres};
use sqlx::{Executor, Transaction};

/// A transaction that may write.
pub struct WriteTx(Transaction<'static, Postgres>);

impl WriteTx {
    /// The connection, for the query surface in this layer.
    pub fn conn(&mut self) -> &mut PgConnection {
        &mut self.0
    }

    pub async fn commit(self) -> sqlx::Result<()> {
        self.0.commit().await
    }

    pub async fn rollback(self) -> sqlx::Result<()> {
        self.0.rollback().await
    }
}

/// A transaction the database itself refuses writes on.
///
/// Dropping it rolls back, which is the only ending it has.
pub struct ReadTx(Transaction<'static, Postgres>);

impl ReadTx {
    pub fn conn(&mut self) -> &mut PgConnection {
        &mut self.0
    }
}

pub async fn begin_write(pool: &PgPool) -> sqlx::Result<WriteTx> {
    Ok(WriteTx(pool.begin().await?))
}

pub async fn begin_read(pool: &PgPool) -> sqlx::Result<ReadTx> {
    let mut tx = pool.begin().await?;
    // First statement inside the transaction, which is where Postgres will
    // accept it.
    tx.execute("SET TRANSACTION READ ONLY").await?;
    Ok(ReadTx(tx))
}

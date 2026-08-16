//! Transactions, in the two shapes the instance has.
//!
//! The component model gives the read path a `Never` list whose first entry is
//! "Write anything: not entity content, not derived data, not a record of what
//! was asked or opened", and the plan restates it as a standing constraint:
//! nothing crosses from the read path to the write path.
//!
//! [`begin_read`] issues `SET TRANSACTION READ ONLY`, so the ordinary way to
//! break that is an error from the database rather than a review comment. Two
//! honest limits on how much that buys, because an overstated guarantee is
//! worse than a modest one:
//!
//! **What Postgres actually blocks.** `INSERT`, `UPDATE`, `DELETE`, `MERGE`
//! and `COPY FROM` against permanent tables, every form of DDL, `GRANT`,
//! `REVOKE` and `TRUNCATE`. It does *not* block DML against a temporary table
//! that already existed, and it is not a promise that nothing touches disk.
//! For this layer's purpose — the read path must not record what was asked —
//! the covered set is the relevant one, but it is a set and not a totality.
//!
//! **What the type adds.** [`ReadTx`] has no `commit`, and its connection is
//! crate-visible rather than public, so nothing outside `altaird` can reach the
//! raw connection to `COMMIT` out of the read-only transaction and then write
//! in Postgres's implicit read-write mode. Inside the crate that escape is
//! still reachable by anyone who writes the line; what is claimed is that it
//! cannot be reached by accident and cannot be reached from outside at all.

use sqlx::postgres::{PgConnection, PgPool, Postgres};
use sqlx::{Executor, Transaction};

/// A transaction that may write.
pub struct WriteTx(Transaction<'static, Postgres>);

impl WriteTx {
    /// The connection, for the query surface in this layer and for the write
    /// path. Crate-visible: nothing outside `altaird` touches the store
    /// directly.
    pub(crate) fn conn(&mut self) -> &mut PgConnection {
        &mut self.0
    }

    /// For tests, which need to observe the connection directly.
    #[cfg(any(test, feature = "testing"))]
    pub fn conn_for_test(&mut self) -> &mut PgConnection {
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
    /// Crate-visible, so the `COMMIT`-and-then-write escape is not reachable
    /// from outside `altaird`. See the module docs for what this does and does
    /// not guarantee.
    pub(crate) fn conn(&mut self) -> &mut PgConnection {
        &mut self.0
    }

    /// For tests, including the one that proves the database refuses a write
    /// through this transaction.
    #[cfg(any(test, feature = "testing"))]
    pub fn conn_for_test(&mut self) -> &mut PgConnection {
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

//! The read path: everything that answers a question without writing one.
//!
//! `store::begin_read` enforces the standing constraint at the database level
//! — a genuine `INSERT`/`UPDATE`/`DELETE` from in here is a database error,
//! not a review comment — but the deeper half of the constraint, "no record of
//! what was asked", is this module's to keep. Nothing in here may write, and
//! nothing in here does.

pub mod changes;

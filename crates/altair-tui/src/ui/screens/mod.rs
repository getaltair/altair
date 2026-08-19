//! The screens.
//!
//! Three groups, which is how the wave's last stage divided them and how they
//! were built: [`browse`] is the ways in, [`write`] is making and changing
//! something, and [`sideways`] is what the client shows when something did not
//! go the way it was supposed to.
//!
//! **Every screen is a function of a view and nothing else.** None of them can
//! reach a store, an instance, or a clock, which is what makes each one
//! checkable as a snapshot in both themes — and what keeps a screen from
//! becoming the place a rule quietly lives.

pub mod browse;
pub mod rows;
pub mod sideways;
pub mod write;

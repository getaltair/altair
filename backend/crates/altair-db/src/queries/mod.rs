//! Queries module - Database query functions
//!
//! This module provides query functions for all database operations,
//! organized by domain entity.

pub mod credential;
pub mod session;
pub mod user;

// Re-export query functions for convenience
pub use credential::*;
pub use session::*;
pub use user::*;

//! Altair Core - Shared types, errors, and traits for the Altair ecosystem
//!
//! This crate provides common functionality used across all Altair applications:
//! - Error types and Result aliases
//! - Common domain types (UserId, Timestamp, etc.)
//! - Shared traits for extensibility
//! - Constants and configuration types

pub mod error;
pub mod traits;
pub mod types;

// Re-export commonly used items
pub use error::{Error, Result};
pub use types::{EnergyCost, EntityId, EntityStatus, Timestamp, UserId};

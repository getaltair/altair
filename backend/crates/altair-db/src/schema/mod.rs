//! Schema module - Rust type definitions matching SurrealDB schema
//!
//! This module provides type-safe Rust structs that mirror the SurrealDB schema
//! defined in migrations. All types support serde serialization for database
//! roundtrips.

pub mod capture;
pub mod credential;
pub mod enums;
pub mod gamification;
pub mod item;
pub mod note;
pub mod quest;
pub mod session;
pub mod shared;

// Re-export all types for convenience
pub use capture::*;
pub use credential::*;
pub use enums::*;
pub use gamification::*;
pub use item::*;
pub use note::*;
pub use quest::*;
pub use session::*;
pub use shared::*;

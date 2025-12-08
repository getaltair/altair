//! Tauri command handlers for Mobile app
//!
//! This module contains all command handlers that can be invoked
//! from the frontend via Tauri's IPC mechanism.

pub mod health;

// Re-export commands for easy access
pub use health::health_check;

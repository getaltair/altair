//! Tauri command handlers for Guidance app
//!
//! This module contains all command handlers that can be invoked
//! from the frontend via Tauri's IPC mechanism.

pub mod health;
pub mod storage;

// Re-export commands for easy access
pub use health::health_check;
pub use storage::{
    storage_confirm_upload, storage_delete, storage_get_quota, storage_get_url,
    storage_is_available, storage_request_upload,
};

//! Authentication command handlers for Tauri IPC
//!
//! This module provides the bridge between Svelte frontend and authentication
//! functionality, exposing commands for local auth operations.

use serde::{Deserialize, Serialize};

// Re-export types from altair-auth for use in commands
pub use altair_auth::types::{AuthError, AuthResponse};
pub use altair_auth::User;
pub use altair_db::schema::UserPreferences;

/// Input for user registration command
///
/// Used by `auth_register` Tauri command to create a new user account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RegisterInput {
    /// User email (required, must be unique)
    pub email: String,
    /// Display name (optional)
    pub display_name: Option<String>,
    /// Password (optional for passwordless setup)
    pub password: Option<String>,
}

/// Input for user login command
///
/// Used by `auth_login` Tauri command to authenticate an existing user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LoginInput {
    /// User email
    pub email: String,
    /// Password (optional for passwordless auth)
    pub password: Option<String>,
}

/// Input for updating user preferences
///
/// Used by `auth_update_prefs` Tauri command to update user preferences.
/// All fields are optional to support partial updates (merging).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UpdatePrefsInput {
    /// Theme preference
    pub theme: Option<String>,
    /// Default energy filter
    pub energy_filter_default: Option<Option<String>>,
    /// Gamification enabled
    pub gamification_enabled: Option<bool>,
    /// Weekly harvest day (0-6, Sunday-Saturday)
    pub weekly_harvest_day: Option<i32>,
    /// Weekly harvest time (HH:MM format)
    pub weekly_harvest_time: Option<String>,
    /// Focus session duration in minutes
    pub focus_session_duration: Option<i32>,
    /// Pomodoro break duration in minutes
    pub pomodoro_break_duration: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_input_serialization() {
        let input = RegisterInput {
            email: "test@example.com".to_string(),
            display_name: Some("Test User".to_string()),
            password: Some("secure-password".to_string()),
        };

        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("test@example.com"));

        let deserialized: RegisterInput = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.email, input.email);
        assert_eq!(deserialized.display_name, input.display_name);
        assert_eq!(deserialized.password, input.password);
    }

    #[test]
    fn test_login_input_serialization() {
        let input = LoginInput {
            email: "test@example.com".to_string(),
            password: Some("password123".to_string()),
        };

        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("test@example.com"));

        let deserialized: LoginInput = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.email, input.email);
        assert_eq!(deserialized.password, input.password);
    }

    #[test]
    fn test_login_input_passwordless() {
        let input = LoginInput {
            email: "test@example.com".to_string(),
            password: None,
        };

        let json = serde_json::to_string(&input).unwrap();
        let deserialized: LoginInput = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.email, input.email);
        assert_eq!(deserialized.password, None);
    }

    #[test]
    fn test_update_prefs_input_partial() {
        let input = UpdatePrefsInput {
            theme: Some("dark".to_string()),
            energy_filter_default: None,
            gamification_enabled: Some(false),
            weekly_harvest_day: None,
            weekly_harvest_time: None,
            focus_session_duration: Some(50),
            pomodoro_break_duration: None,
        };

        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("dark"));

        let deserialized: UpdatePrefsInput = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.theme, Some("dark".to_string()));
        assert_eq!(deserialized.gamification_enabled, Some(false));
        assert_eq!(deserialized.focus_session_duration, Some(50));
    }

    #[test]
    fn test_update_prefs_input_all_none() {
        let input = UpdatePrefsInput {
            theme: None,
            energy_filter_default: None,
            gamification_enabled: None,
            weekly_harvest_day: None,
            weekly_harvest_time: None,
            focus_session_duration: None,
            pomodoro_break_duration: None,
        };

        let json = serde_json::to_string(&input).unwrap();
        let deserialized: UpdatePrefsInput = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.theme, None);
        assert_eq!(deserialized.gamification_enabled, None);
    }
}

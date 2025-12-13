//! Authentication command handlers for Tauri IPC
//!
//! This module provides the bridge between Svelte frontend and authentication
//! functionality, exposing commands for local auth operations.

use altair_core::Result;
use altair_db::schema::User as DbUser;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;

// Re-export types from altair-auth for use in commands
pub use altair_auth::User;
pub use altair_auth::types::{AuthError, AuthResponse};
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

// Command implementations
// These will be registered as Tauri commands in the app

/// Check if initial setup has been completed
///
/// Returns true if a user exists in the system, false for first-launch.
/// Used by frontend to determine whether to show setup wizard or login screen.
pub async fn auth_check_setup<C: surrealdb::Connection>(db: &Surreal<C>) -> Result<bool> {
    altair_db::queries::user_exists(db).await
}

/// Register a new user account
///
/// Creates a new user, optional credential record, and initial session.
/// Stores the session token in the OS keychain for persistent authentication.
pub async fn auth_register<C: surrealdb::Connection>(
    db: &Surreal<C>,
    input: RegisterInput,
) -> Result<AuthResponse> {
    // Check if user already exists
    let existing = altair_db::queries::get_user_by_email(db, &input.email).await?;
    if existing.is_some() {
        return Err(altair_core::Error::Auth(
            "User with this email already exists".to_string(),
        ));
    }

    // Create user
    let display_name = input.display_name.unwrap_or_else(|| input.email.clone());
    let device_id = "default-device".to_string(); // TODO: Get from system in later spec

    let user =
        altair_db::queries::create_user(db, input.email.clone(), display_name, device_id.clone())
            .await?;

    // Create credential if password provided
    if let Some(password) = input.password {
        let password_hash = altair_auth::local::password::hash_password(&password).await?;
        let user_id = user
            .id
            .clone()
            .ok_or_else(|| altair_core::Error::Database("User created without ID".to_string()))?;
        altair_db::queries::create_credential(db, user_id, password_hash).await?;
    }

    // Create session
    let user_id_thing = user
        .id
        .clone()
        .ok_or_else(|| altair_core::Error::Database("User created without ID".to_string()))?;

    let session =
        altair_auth::local::Session::new(user_id_thing.to_string(), Some(device_id.clone()));

    // Extract token and expiration (guaranteed to exist when device_id is provided)
    let token = session
        .token
        .clone()
        .ok_or_else(|| altair_core::Error::Auth("Session token not generated".to_string()))?;
    let expires_at = session
        .expires_at
        .ok_or_else(|| altair_core::Error::Auth("Session expiration not set".to_string()))?;

    // Store session in database
    altair_db::queries::create_session(db, token.clone(), user_id_thing, expires_at, device_id)
        .await?;

    // Store token in keychain
    let keychain = altair_auth::local::keychain::KeychainStorage::new();
    keychain.store_token(&token).await?;

    // Convert DB user to auth User
    let auth_user = User {
        id: user.id.unwrap().to_string(),
        email: user.email,
        name: Some(user.display_name),
    };

    Ok(AuthResponse::new(auth_user, token, expires_at))
}

/// Authenticate an existing user
///
/// Validates credentials (if password set), creates a new session,
/// and stores the token in the OS keychain.
pub async fn auth_login<C: surrealdb::Connection>(
    db: &Surreal<C>,
    input: LoginInput,
) -> Result<AuthResponse> {
    // Get user by email
    let user = altair_db::queries::get_user_by_email(db, &input.email)
        .await?
        .ok_or_else(|| altair_core::Error::Auth("Invalid credentials".to_string()))?;

    let user_id = user
        .id
        .clone()
        .ok_or_else(|| altair_core::Error::Database("User missing ID".to_string()))?;

    // Check if password is required
    let credential = altair_db::queries::get_credential_by_user(db, &user_id).await?;

    if let Some(cred) = credential {
        // Password is set, verify it
        let password = input
            .password
            .ok_or_else(|| altair_core::Error::Auth("Password required".to_string()))?;

        if !altair_auth::local::password::verify_password(&password, &cred.password_hash).await? {
            return Err(altair_core::Error::Auth("Invalid credentials".to_string()));
        }
    } else if input.password.is_some() {
        // User provided password but none is set
        return Err(altair_core::Error::Auth("Invalid credentials".to_string()));
    }

    // Create session
    let device_id = "default-device".to_string(); // TODO: Get from system in later spec
    let user_id_str = user_id.to_string();
    let session = altair_auth::local::Session::new(user_id_str, Some(device_id.clone()));

    // Extract token and expiration (guaranteed to exist when device_id is provided)
    let token = session
        .token
        .clone()
        .ok_or_else(|| altair_core::Error::Auth("Session token not generated".to_string()))?;
    let expires_at = session
        .expires_at
        .ok_or_else(|| altair_core::Error::Auth("Session expiration not set".to_string()))?;

    // Store session in database
    altair_db::queries::create_session(db, token.clone(), user_id.clone(), expires_at, device_id)
        .await?;

    // Store token in keychain
    let keychain = altair_auth::local::keychain::KeychainStorage::new();
    keychain.store_token(&token).await?;

    // Convert DB user to auth User
    let auth_user = User {
        id: user_id.to_string(),
        email: user.email,
        name: Some(user.display_name),
    };

    Ok(AuthResponse::new(auth_user, token, expires_at))
}

/// Logout the current user
///
/// Retrieves token from keychain, deletes the session from database,
/// and removes the token from keychain.
pub async fn auth_logout<C: surrealdb::Connection>(db: &Surreal<C>) -> Result<()> {
    // Get token from keychain
    let keychain = altair_auth::local::keychain::KeychainStorage::new();
    let token = keychain
        .get_token()
        .await?
        .ok_or_else(|| altair_core::Error::Auth("No active session".to_string()))?;

    // Delete session from database
    altair_db::queries::delete_session(db, &token).await?;

    // Remove token from keychain
    keychain.delete_token().await?;

    Ok(())
}

/// Validate the current session
///
/// Retrieves token from keychain and validates it against the database.
/// Returns the user profile if valid, error if expired or invalid.
pub async fn auth_validate<C: surrealdb::Connection>(db: &Surreal<C>) -> Result<DbUser> {
    // Get token from keychain
    let keychain = altair_auth::local::keychain::KeychainStorage::new();
    let token = keychain
        .get_token()
        .await?
        .ok_or_else(|| altair_core::Error::Auth("No active session".to_string()))?;

    // Get session from database
    let session = altair_db::queries::get_session_by_token(db, &token)
        .await?
        .ok_or_else(|| altair_core::Error::Auth("Session not found".to_string()))?;

    // Check if expired
    if session.is_expired() {
        return Err(altair_core::Error::Auth("Session expired".to_string()));
    }

    // Get user - session.user is already a Thing (not wrapped in Value)
    altair_db::queries::get_user_by_id(db, session.user).await
}

/// Refresh the current session
///
/// Extends the session expiration by 7 days. Returns the updated session.
pub async fn auth_refresh<C: surrealdb::Connection>(
    db: &Surreal<C>,
) -> Result<altair_db::schema::Session> {
    // Get token from keychain
    let keychain = altair_auth::local::keychain::KeychainStorage::new();
    let token = keychain
        .get_token()
        .await?
        .ok_or_else(|| altair_core::Error::Auth("No active session".to_string()))?;

    // Get session from database
    let session = altair_db::queries::get_session_by_token(db, &token)
        .await?
        .ok_or_else(|| altair_core::Error::Auth("Session not found".to_string()))?;

    // Check if expired
    if session.is_expired() {
        return Err(altair_core::Error::Auth("Session expired".to_string()));
    }

    // Extend expiration by 7 days
    let new_expiry = chrono::Utc::now() + chrono::Duration::days(7);

    altair_db::queries::refresh_session(db, &token, new_expiry).await
}

/// Get the current user profile
///
/// Retrieves token from keychain and returns the full user profile.
pub async fn auth_get_user<C: surrealdb::Connection>(db: &Surreal<C>) -> Result<DbUser> {
    // Reuse validate logic
    auth_validate(db).await
}

/// Update user preferences
///
/// Merges partial preference updates with existing user preferences.
/// Returns the updated user profile.
pub async fn auth_update_prefs<C: surrealdb::Connection>(
    db: &Surreal<C>,
    input: UpdatePrefsInput,
) -> Result<DbUser> {
    // Get current user
    let user = auth_validate(db).await?;
    let user_id = user
        .id
        .ok_or_else(|| altair_core::Error::Database("User missing ID".to_string()))?;

    // Merge preferences
    let mut prefs = user.preferences;

    if let Some(theme) = input.theme {
        prefs.theme = theme;
    }
    if let Some(energy_filter) = input.energy_filter_default {
        prefs.energy_filter_default = energy_filter;
    }
    if let Some(gamification) = input.gamification_enabled {
        prefs.gamification_enabled = gamification;
    }
    if let Some(harvest_day) = input.weekly_harvest_day {
        prefs.weekly_harvest_day = harvest_day;
    }
    if let Some(harvest_time_str) = input.weekly_harvest_time {
        // Parse HH:MM format to NaiveTime
        let time = chrono::NaiveTime::parse_from_str(&harvest_time_str, "%H:%M")
            .map_err(|e| altair_core::Error::Validation(format!("Invalid time format: {}", e)))?;
        prefs.weekly_harvest_time = time;
    }
    if let Some(focus_duration) = input.focus_session_duration {
        prefs.focus_session_duration = focus_duration;
    }
    if let Some(break_duration) = input.pomodoro_break_duration {
        prefs.pomodoro_break_duration = break_duration;
    }

    // Update in database
    altair_db::queries::update_user_preferences(db, user_id, prefs).await
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

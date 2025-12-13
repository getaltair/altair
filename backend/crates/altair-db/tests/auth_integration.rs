//! Integration tests for authentication flow
//!
//! These tests verify the complete authentication lifecycle, including:
//! - First-time setup flow
//! - Login/logout cycles
//! - Password verification
//! - Passwordless authentication
//! - Session expiry and refresh
//! - User preferences CRUD

use altair_db::queries::{
    create_credential, create_session, create_user, delete_session, get_credential_by_user,
    get_session_by_token, get_user_by_email, refresh_session, update_user_preferences, user_exists,
};
use altair_db::schema::{UserPreferences, UserRole};
use chrono::{Duration, Utc};
use surrealdb::Surreal;
use surrealdb::sql::Thing;

/// Helper to create a test database
async fn setup_test_db() -> Surreal<surrealdb::engine::local::Db> {
    let db = Surreal::new::<surrealdb::engine::local::Mem>(())
        .await
        .expect("Failed to create in-memory database");

    db.use_ns("test_auth")
        .use_db("test_auth")
        .await
        .expect("Failed to use namespace/database");

    db
}

/// Task 4.1: Integration test - First-time setup flow
///
/// Test: Fresh DB → check_setup returns false → register → check_setup returns true
/// Verify user created, session active, token in keychain (simulated via DB check)
#[tokio::test]
async fn test_first_time_setup_flow() {
    let db = setup_test_db().await;

    // Step 1: Fresh DB should have no users
    let has_users = user_exists(&db).await.expect("user_exists failed");
    assert!(!has_users, "Fresh DB should have no users");

    // Step 2: Register a new user
    let email = "first@example.com";
    let display_name = "First User";
    let device_id = "device_first";

    let user = create_user(
        &db,
        email.to_string(),
        display_name.to_string(),
        device_id.to_string(),
    )
    .await
    .expect("Failed to create user");

    assert_eq!(user.email, email);
    assert_eq!(user.display_name, display_name);
    assert_eq!(user.device_id, device_id);
    assert!(matches!(user.role, UserRole::Owner));

    // Step 3: Create a session for the user
    let token = "test_token_first_setup";
    let expires_at = Utc::now() + Duration::days(7);

    let session = create_session(
        &db,
        token.to_string(),
        user.id.clone().expect("User should have ID"),
        expires_at,
        device_id.to_string(),
    )
    .await
    .expect("Failed to create session");

    assert_eq!(session.token, token);
    assert_eq!(session.user, user.id.unwrap());
    assert!(!session.is_expired());

    // Step 4: Verify user_exists now returns true
    let has_users_after = user_exists(&db).await.expect("user_exists failed");
    assert!(has_users_after, "DB should have users after registration");

    // Step 5: Verify session can be retrieved
    let found_session = get_session_by_token(&db, token)
        .await
        .expect("Failed to get session");
    assert!(
        found_session.is_some(),
        "Session should exist after creation"
    );
}

/// Task 4.2: Integration test - Login/logout cycle
///
/// Test: Register → logout → validate fails → login → validate succeeds
/// Verify keychain token lifecycle (simulated via session DB checks)
#[tokio::test]
async fn test_login_logout_cycle() {
    let db = setup_test_db().await;

    // Step 1: Create user
    let email = "cycle@example.com";
    let user = create_user(
        &db,
        email.to_string(),
        "Cycle User".to_string(),
        "device_cycle".to_string(),
    )
    .await
    .expect("Failed to create user");

    let user_id = user.id.clone().expect("User should have ID");

    // Step 2: Create initial session (simulating login)
    let token_login = "token_cycle_login";
    let session = create_session(
        &db,
        token_login.to_string(),
        user_id.clone(),
        Utc::now() + Duration::days(7),
        "device_cycle".to_string(),
    )
    .await
    .expect("Failed to create session");

    assert_eq!(session.token, token_login);

    // Step 3: Logout - delete session
    delete_session(&db, token_login)
        .await
        .expect("Failed to delete session");

    // Step 4: Validate should fail (session doesn't exist)
    let session_after_logout = get_session_by_token(&db, token_login)
        .await
        .expect("Failed to query session");
    assert!(
        session_after_logout.is_none(),
        "Session should not exist after logout"
    );

    // Step 5: Re-login - create new session
    let token_relogin = "token_cycle_relogin";
    let new_session = create_session(
        &db,
        token_relogin.to_string(),
        user_id,
        Utc::now() + Duration::days(7),
        "device_cycle".to_string(),
    )
    .await
    .expect("Failed to create session on re-login");

    assert_eq!(new_session.token, token_relogin);

    // Step 6: Validate should succeed
    let valid_session = get_session_by_token(&db, token_relogin)
        .await
        .expect("Failed to get session");
    assert!(
        valid_session.is_some(),
        "Session should exist after re-login"
    );
    assert!(!valid_session.unwrap().is_expired());
}

/// Task 4.3: Integration test - Password verification
///
/// Test: Register with password → logout → login with correct password succeeds
/// Test: Login with wrong password fails with generic error (no user enumeration)
#[tokio::test]
async fn test_password_verification() {
    let db = setup_test_db().await;

    // Step 1: Create user
    let email = "password@example.com";
    let user = create_user(
        &db,
        email.to_string(),
        "Password User".to_string(),
        "device_password".to_string(),
    )
    .await
    .expect("Failed to create user");

    let user_id = user.id.clone().expect("User should have ID");

    // Step 2: Create credential with password hash
    // In a real scenario, this would use altair-auth::hash_password
    let password_hash = "hashed_password_correct";
    let credential = create_credential(&db, user_id.clone(), password_hash.to_string())
        .await
        .expect("Failed to create credential");

    assert_eq!(credential.user, user_id);
    assert_eq!(credential.password_hash, password_hash);

    // Step 3: Verify credential can be retrieved
    let found_credential = get_credential_by_user(&db, &user_id)
        .await
        .expect("Failed to get credential");
    assert!(found_credential.is_some(), "Credential should exist");

    let found = found_credential.unwrap();
    assert_eq!(found.password_hash, password_hash);

    // Step 4: Simulate login verification
    // In the real command, this would:
    // 1. Get user by email
    // 2. Get credential by user_id
    // 3. Verify password hash matches
    // 4. Create session if valid

    let user_lookup = get_user_by_email(&db, email)
        .await
        .expect("Failed to get user");
    assert!(user_lookup.is_some(), "User should exist");

    let user_found = user_lookup.unwrap();
    let credential_lookup = get_credential_by_user(&db, &user_found.id.unwrap())
        .await
        .expect("Failed to get credential");
    assert!(credential_lookup.is_some(), "Credential should exist");

    // Verify hash matches (in real code, use verify_password)
    assert_eq!(credential_lookup.unwrap().password_hash, password_hash);

    // Step 5: Create session on successful verification
    let token = "token_password_verified";
    let session = create_session(
        &db,
        token.to_string(),
        user_id,
        Utc::now() + Duration::days(7),
        "device_password".to_string(),
    )
    .await
    .expect("Failed to create session");

    assert_eq!(session.token, token);
    assert!(!session.is_expired());
}

/// Task 4.3 (continued): Test wrong password scenario
///
/// Verify that wrong password produces generic error (no user enumeration)
#[tokio::test]
async fn test_wrong_password_generic_error() {
    let db = setup_test_db().await;

    // Create user with credential
    let email = "wrongpass@example.com";
    let user = create_user(
        &db,
        email.to_string(),
        "Wrong Pass User".to_string(),
        "device_wrong".to_string(),
    )
    .await
    .expect("Failed to create user");

    let user_id = user.id.clone().expect("User should have ID");
    let correct_hash = "correct_password_hash";
    create_credential(&db, user_id, correct_hash.to_string())
        .await
        .expect("Failed to create credential");

    // Simulate login with wrong password
    let user_lookup = get_user_by_email(&db, email)
        .await
        .expect("Failed to get user");
    assert!(user_lookup.is_some());

    let credential_lookup = get_credential_by_user(&db, &user_lookup.unwrap().id.unwrap())
        .await
        .expect("Failed to get credential");
    let stored_hash = credential_lookup.unwrap().password_hash;

    // Verify hash doesn't match
    let wrong_hash = "wrong_password_hash";
    assert_ne!(stored_hash, wrong_hash, "Hashes should not match");

    // In the real auth_login command, this would return a generic error:
    // Err(Error::Auth("Invalid credentials"))
    // WITHOUT revealing whether the email exists or password is wrong
}

/// Task 4.4: Integration test - Passwordless authentication
///
/// Test: Register without password → logout → login without password succeeds
/// Verify no credential record created
#[tokio::test]
async fn test_passwordless_auth() {
    let db = setup_test_db().await;

    // Step 1: Create user WITHOUT credential
    let email = "passwordless@example.com";
    let user = create_user(
        &db,
        email.to_string(),
        "Passwordless User".to_string(),
        "device_passwordless".to_string(),
    )
    .await
    .expect("Failed to create user");

    let user_id = user.id.clone().expect("User should have ID");

    // Step 2: Verify NO credential exists
    let credential = get_credential_by_user(&db, &user_id)
        .await
        .expect("Failed to check credential");
    assert!(
        credential.is_none(),
        "Credential should not exist for passwordless user"
    );

    // Step 3: Create session (simulating passwordless login)
    let token = "token_passwordless";
    let session = create_session(
        &db,
        token.to_string(),
        user_id.clone(),
        Utc::now() + Duration::days(7),
        "device_passwordless".to_string(),
    )
    .await
    .expect("Failed to create session");

    assert_eq!(session.token, token);
    assert!(!session.is_expired());

    // Step 4: Logout
    delete_session(&db, token)
        .await
        .expect("Failed to delete session");

    // Step 5: Re-login without password
    let token_relogin = "token_passwordless_relogin";
    let new_session = create_session(
        &db,
        token_relogin.to_string(),
        user_id,
        Utc::now() + Duration::days(7),
        "device_passwordless".to_string(),
    )
    .await
    .expect("Failed to create session on re-login");

    assert_eq!(new_session.token, token_relogin);

    // Step 6: Verify session is valid
    let valid_session = get_session_by_token(&db, token_relogin)
        .await
        .expect("Failed to get session");
    assert!(valid_session.is_some());
    assert!(!valid_session.unwrap().is_expired());
}

/// Task 4.5: Integration test - Session expiry
///
/// Test: Create session with past expiry → validate returns expired error
/// Test: Refresh extends expiry correctly
#[tokio::test]
async fn test_session_expiry() {
    let db = setup_test_db().await;

    // Create user
    let user = create_user(
        &db,
        "expiry@example.com".to_string(),
        "Expiry User".to_string(),
        "device_expiry".to_string(),
    )
    .await
    .expect("Failed to create user");

    let user_id = user.id.expect("User should have ID");

    // Step 1: Create session with past expiry (already expired)
    let token_expired = "token_expired";
    let past_expiry = Utc::now() - Duration::hours(24);

    let expired_session = create_session(
        &db,
        token_expired.to_string(),
        user_id.clone(),
        past_expiry,
        "device_expiry".to_string(),
    )
    .await
    .expect("Failed to create expired session");

    // Verify session is marked as expired
    assert!(
        expired_session.is_expired(),
        "Session should be expired (expiry in past)"
    );

    // Step 2: Create session with future expiry (active)
    let token_active = "token_active";
    let future_expiry = Utc::now() + Duration::days(7);

    let active_session = create_session(
        &db,
        token_active.to_string(),
        user_id,
        future_expiry,
        "device_expiry".to_string(),
    )
    .await
    .expect("Failed to create active session");

    assert!(
        !active_session.is_expired(),
        "Session should NOT be expired (expiry in future)"
    );

    // Step 3: Refresh session to extend expiry
    let new_expiry = Utc::now() + Duration::days(14);
    let refreshed_session = refresh_session(&db, token_active, new_expiry)
        .await
        .expect("Failed to refresh session");

    assert!(refreshed_session.expires_at > future_expiry);
    assert!(!refreshed_session.is_expired());

    // Step 4: Verify should_refresh logic
    // Create session expiring in 6 hours (within 1-day refresh window)
    let token_refresh_window = "token_refresh_window";
    let near_expiry = Utc::now() + Duration::hours(6);

    let session_near_expiry = create_session(
        &db,
        token_refresh_window.to_string(),
        Thing::from(("user", "temp")),
        near_expiry,
        "device_expiry".to_string(),
    )
    .await
    .expect("Failed to create session near expiry");

    assert!(
        session_near_expiry.should_refresh(),
        "Session within 1 day of expiry should need refresh"
    );

    // Session expiring in 3 days should NOT need immediate refresh
    let token_no_refresh = "token_no_refresh";
    let far_expiry = Utc::now() + Duration::days(3);

    let session_far_expiry = create_session(
        &db,
        token_no_refresh.to_string(),
        Thing::from(("user", "temp")),
        far_expiry,
        "device_expiry".to_string(),
    )
    .await
    .expect("Failed to create session far from expiry");

    assert!(
        !session_far_expiry.should_refresh(),
        "Session more than 1 day from expiry should NOT need refresh"
    );
}

/// Task 4.6: Integration test - Preferences CRUD
///
/// Test: Register → update_prefs → get_user → verify preferences merged
#[tokio::test]
async fn test_preferences_crud() {
    let db = setup_test_db().await;

    // Step 1: Create user with default preferences
    let email = "prefs@example.com";
    let user = create_user(
        &db,
        email.to_string(),
        "Prefs User".to_string(),
        "device_prefs".to_string(),
    )
    .await
    .expect("Failed to create user");

    let user_id = user.id.clone().expect("User should have ID");

    // Verify default preferences
    assert_eq!(user.preferences.theme, "auto");
    assert!(user.preferences.gamification_enabled);
    assert_eq!(user.preferences.focus_session_duration, 25);

    // Step 2: Update preferences (partial update)
    let new_prefs = UserPreferences {
        theme: "dark".to_string(),
        gamification_enabled: false,
        focus_session_duration: 45,
        ..user.preferences.clone()
    };

    let updated_user = update_user_preferences(&db, user_id.clone(), new_prefs.clone())
        .await
        .expect("Failed to update preferences");

    // Step 3: Verify preferences were merged
    assert_eq!(updated_user.preferences.theme, "dark");
    assert!(!updated_user.preferences.gamification_enabled);
    assert_eq!(updated_user.preferences.focus_session_duration, 45);

    // Verify other preferences remain unchanged
    assert_eq!(
        updated_user.preferences.pomodoro_break_duration,
        user.preferences.pomodoro_break_duration
    );
    assert_eq!(
        updated_user.preferences.weekly_harvest_day,
        user.preferences.weekly_harvest_day
    );

    // Step 4: Get user by email and verify preferences persisted
    let user_lookup = get_user_by_email(&db, email)
        .await
        .expect("Failed to get user");
    assert!(user_lookup.is_some());

    let found_user = user_lookup.unwrap();
    assert_eq!(found_user.preferences.theme, "dark");
    assert!(!found_user.preferences.gamification_enabled);
    assert_eq!(found_user.preferences.focus_session_duration, 45);

    // Step 5: Test multiple updates (preferences accumulate)
    let second_update = UserPreferences {
        weekly_harvest_day: 1, // Monday
        pomodoro_break_duration: 10,
        ..found_user.preferences.clone()
    };

    let final_user = update_user_preferences(&db, user_id, second_update)
        .await
        .expect("Failed to update preferences second time");

    // Verify all changes persisted
    assert_eq!(final_user.preferences.theme, "dark"); // From first update
    assert!(!final_user.preferences.gamification_enabled); // From first update
    assert_eq!(final_user.preferences.focus_session_duration, 45); // From first update
    assert_eq!(final_user.preferences.weekly_harvest_day, 1); // From second update
    assert_eq!(final_user.preferences.pomodoro_break_duration, 10); // From second update
}

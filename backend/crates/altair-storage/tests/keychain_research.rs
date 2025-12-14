//! Research tests for OS keychain integration using the keyring crate
//!
//! This module validates that the keyring crate works correctly for storing
//! and retrieving S3 credentials on the current platform.
//!
//! Note: These tests interact with the actual OS keychain and should be run
//! with caution. They create and delete test entries.

use keyring::{Entry, Error};

const TEST_SERVICE: &str = "altair-storage-test";
const TEST_USERNAME: &str = "test-user";

/// Helper to clean up test credentials
fn cleanup_test_credentials() {
    if let Ok(entry) = Entry::new(TEST_SERVICE, TEST_USERNAME) {
        let _ = entry.delete_credential();
    }
}

#[test]
fn test_store_and_retrieve_credentials() {
    // Test: Validate store/retrieve S3 credentials
    // Acceptance: Test `keyring` crate storing and retrieving access_key and secret_key

    cleanup_test_credentials();

    // Create entry for test
    let entry = Entry::new(TEST_SERVICE, TEST_USERNAME).expect("Failed to create keyring entry");

    // Store test credentials
    let test_password = "test-secret-key-12345";
    entry
        .set_password(test_password)
        .expect("Failed to store password in keychain");

    // Retrieve credentials
    let retrieved = entry
        .get_password()
        .expect("Failed to retrieve password from keychain");

    assert_eq!(
        retrieved, test_password,
        "Retrieved password should match stored password"
    );

    // Cleanup
    cleanup_test_credentials();
}

#[test]
fn test_overwrite_credentials() {
    // Test: Validate that updating credentials works correctly

    cleanup_test_credentials();

    let entry = Entry::new(TEST_SERVICE, TEST_USERNAME).expect("Failed to create keyring entry");

    // Store initial credentials
    entry
        .set_password("initial-password")
        .expect("Failed to store initial password");

    // Overwrite with new credentials
    let new_password = "updated-password-67890";
    entry
        .set_password(new_password)
        .expect("Failed to update password");

    // Retrieve and verify it's the new value
    let retrieved = entry
        .get_password()
        .expect("Failed to retrieve updated password");

    assert_eq!(
        retrieved, new_password,
        "Retrieved password should be the updated value"
    );

    cleanup_test_credentials();
}

#[test]
fn test_delete_credentials() {
    // Test: Validate credential deletion

    cleanup_test_credentials();

    let entry = Entry::new(TEST_SERVICE, TEST_USERNAME).expect("Failed to create keyring entry");

    // Store credentials
    entry
        .set_password("to-be-deleted")
        .expect("Failed to store password");

    // Delete credentials
    entry
        .delete_credential()
        .expect("Failed to delete credentials");

    // Verify they're gone
    let result = entry.get_password();
    assert!(
        result.is_err(),
        "Getting password after deletion should fail"
    );

    match result {
        Err(Error::NoEntry) => (),
        other => panic!("Expected NoEntry error, got: {:?}", other),
    }
}

#[test]
fn test_missing_credentials_error() {
    // Test: Validate error handling for missing credentials

    cleanup_test_credentials();

    let entry =
        Entry::new(TEST_SERVICE, "nonexistent-user").expect("Failed to create keyring entry");

    // Try to retrieve non-existent credentials
    let result = entry.get_password();

    assert!(result.is_err(), "Should fail for non-existent credentials");

    match result {
        Err(Error::NoEntry) => (),
        other => panic!("Expected NoEntry error, got: {:?}", other),
    }
}

#[test]
fn test_multiple_users() {
    // Test: Validate storing credentials for multiple users

    let user1_entry = Entry::new(TEST_SERVICE, "user1").expect("Failed to create entry for user1");
    let user2_entry = Entry::new(TEST_SERVICE, "user2").expect("Failed to create entry for user2");

    // Store different credentials for each user
    user1_entry
        .set_password("user1-secret")
        .expect("Failed to store user1 credentials");
    user2_entry
        .set_password("user2-secret")
        .expect("Failed to store user2 credentials");

    // Retrieve and verify both
    let user1_password = user1_entry
        .get_password()
        .expect("Failed to retrieve user1 credentials");
    let user2_password = user2_entry
        .get_password()
        .expect("Failed to retrieve user2 credentials");

    assert_eq!(user1_password, "user1-secret");
    assert_eq!(user2_password, "user2-secret");

    // Cleanup
    let _ = user1_entry.delete_credential();
    let _ = user2_entry.delete_credential();
}

#[test]
fn test_credential_structure_for_s3() {
    // Test: Demonstrate pattern for storing S3 credentials (access_key + secret_key)
    // Pattern: Store as JSON in a single keychain entry

    cleanup_test_credentials();

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct S3Credentials {
        access_key_id: String,
        secret_access_key: String,
        endpoint: Option<String>,
    }

    let entry = Entry::new(TEST_SERVICE, TEST_USERNAME).expect("Failed to create keyring entry");

    // Create credentials structure
    let credentials = S3Credentials {
        access_key_id: "AKIAIOSFODNN7EXAMPLE".to_string(),
        secret_access_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string(),
        endpoint: Some("http://localhost:9000".to_string()),
    };

    // Serialize and store
    let credentials_json =
        serde_json::to_string(&credentials).expect("Failed to serialize credentials");

    entry
        .set_password(&credentials_json)
        .expect("Failed to store credentials");

    // Retrieve and deserialize
    let retrieved_json = entry
        .get_password()
        .expect("Failed to retrieve credentials");

    let retrieved_credentials: S3Credentials =
        serde_json::from_str(&retrieved_json).expect("Failed to deserialize credentials");

    assert_eq!(
        retrieved_credentials, credentials,
        "Retrieved credentials should match stored credentials"
    );

    cleanup_test_credentials();
}

#[cfg(target_os = "macos")]
#[test]
fn test_macos_keychain_backend() {
    // Test: Verify we're using macOS Keychain backend

    cleanup_test_credentials();

    let entry = Entry::new(TEST_SERVICE, TEST_USERNAME).expect("Failed to create keyring entry");

    entry
        .set_password("test-macos-keychain")
        .expect("Failed to store in macOS Keychain");

    // On macOS, credentials should be accessible via security command-line tool
    // We just verify that storage and retrieval work
    let retrieved = entry
        .get_password()
        .expect("Failed to retrieve from macOS Keychain");

    assert_eq!(retrieved, "test-macos-keychain");

    cleanup_test_credentials();
}

#[cfg(target_os = "windows")]
#[test]
fn test_windows_credential_manager() {
    // Test: Verify we're using Windows Credential Manager

    cleanup_test_credentials();

    let entry = Entry::new(TEST_SERVICE, TEST_USERNAME).expect("Failed to create keyring entry");

    entry
        .set_password("test-windows-credentials")
        .expect("Failed to store in Windows Credential Manager");

    let retrieved = entry
        .get_password()
        .expect("Failed to retrieve from Windows Credential Manager");

    assert_eq!(retrieved, "test-windows-credentials");

    cleanup_test_credentials();
}

#[cfg(target_os = "linux")]
#[test]
fn test_linux_secret_service() {
    // Test: Verify we're using Linux Secret Service (libsecret/gnome-keyring)

    cleanup_test_credentials();

    let entry = Entry::new(TEST_SERVICE, TEST_USERNAME).expect("Failed to create keyring entry");

    // Note: On Linux, this requires a running keyring daemon (gnome-keyring or KWallet)
    // In headless environments, this test may fail
    match entry.set_password("test-linux-secret-service") {
        Ok(_) => {
            let retrieved = entry
                .get_password()
                .expect("Failed to retrieve from Linux Secret Service");

            assert_eq!(retrieved, "test-linux-secret-service");

            cleanup_test_credentials();
        }
        Err(Error::PlatformFailure(msg)) => {
            // On headless Linux (CI), secret service may not be available
            eprintln!(
                "Note: Secret service not available on this Linux system: {}",
                msg
            );
            eprintln!("This is expected in headless environments.");
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

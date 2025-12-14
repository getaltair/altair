# OS Keychain Support for Storage Credentials

**Status**: Phase 0 Research
**Spec**: CORE-011-STORAGE-SERVICE
**Updated**: 2025-12-12

## Overview

Altair uses the OS native keychain to securely store S3 storage credentials (access key ID and secret access key). This document describes platform support, error handling, and the first-run setup flow.

## Platform Support

### Tested Platforms

| Platform         | Keychain Service           | Status     | Notes                             |
| ---------------- | -------------------------- | ---------- | --------------------------------- |
| macOS            | macOS Keychain             | ✅ Full    | Native integration, highly secure |
| Windows          | Windows Credential Manager | ✅ Full    | Native integration                |
| Linux (GNOME)    | Secret Service (libsecret) | ✅ Full    | Requires gnome-keyring daemon     |
| Linux (KDE)      | KWallet                    | ✅ Full    | Requires KWallet daemon           |
| Linux (headless) | File-based fallback        | ⚠️ Limited | Environment variables only        |

### Implementation via `keyring` Crate

We use the [`keyring`](https://docs.rs/keyring/) crate which provides a unified API across platforms:

```rust
use keyring::{Entry, Error};

// Create entry for service and username
let entry = Entry::new("altair-storage", "default-user")?;

// Store credentials
entry.set_password(&credentials_json)?;

// Retrieve credentials
let credentials_json = entry.get_password()?;
```

### Backend Selection

The `keyring` crate automatically selects the appropriate backend:

- **macOS**: Security Framework (Keychain)
- **Windows**: Credential Manager API
- **Linux**: Secret Service API (via D-Bus)
  - GNOME: gnome-keyring
  - KDE: KWallet
  - Other: Compatible secret service implementations

## Credential Structure

### Storage Format

Credentials are stored as JSON in a single keychain entry:

```rust
#[derive(Serialize, Deserialize)]
struct StorageCredentials {
    access_key_id: String,
    secret_access_key: String,
    endpoint: String,
    bucket: String,
}
```

**Serialized Example:**

```json
{
  "access_key_id": "AKIAIOSFODNN7EXAMPLE",
  "secret_access_key": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
  "endpoint": "http://localhost:9000",
  "bucket": "altair-guidance"
}
```

### Keychain Entry Naming

- **Service**: `com.altair.{app}.storage` (e.g., `com.altair.guidance.storage`)
- **Account/Username**: User's SurrealDB user ID (e.g., `user:abc123`)

**Rationale:**

- Service name namespaced by app to avoid conflicts
- Username allows multi-user support on same device
- JSON format allows adding fields without breaking changes

## Error Handling

### Error Types from `keyring` Crate

```rust
pub enum Error {
    NoEntry,              // Credential not found
    PlatformFailure(msg), // OS keychain error
    TooLong,              // Credential data too large
    Invalid,              // Invalid service/account name
    Ambiguous(msg),       // Multiple matching entries
    // ... other variants
}
```

### Application Error Handling Strategy

| Error Condition                          | Handling Strategy                            |
| ---------------------------------------- | -------------------------------------------- |
| `NoEntry` (credentials not found)        | Prompt user for first-run setup              |
| `PlatformFailure` (keychain unavailable) | Fall back to environment variables           |
| `TooLong` (credentials too large)        | Should never happen; fail with clear message |
| `Invalid` (bad service/account name)     | Developer error; log and fail                |

### Fallback Behavior

When OS keychain is unavailable (e.g., headless Linux, CI environments):

1. **Check environment variables:**

   ```bash
   ALTAIR_STORAGE_ACCESS_KEY=...
   ALTAIR_STORAGE_SECRET_KEY=...
   ALTAIR_STORAGE_ENDPOINT=http://localhost:9000
   ALTAIR_STORAGE_BUCKET=altair-guidance
   ```

2. **Log warning:**

   ```
   WARN: OS keychain unavailable, using environment variables for storage credentials.
         This is less secure than OS keychain. For production, ensure keychain is available.
   ```

3. **Behavior:**
   - App works but logs security warning
   - Credentials not persisted across restarts (must set env vars each time)
   - Acceptable for CI, development, server deployments

### Implementation

```rust
use keyring::{Entry, Error};

pub fn load_storage_credentials(
    app_name: &str,
    user_id: &str,
) -> Result<StorageCredentials, StorageError> {
    let service = format!("com.altair.{}.storage", app_name);

    match Entry::new(&service, user_id) {
        Ok(entry) => match entry.get_password() {
            Ok(json) => {
                let creds: StorageCredentials = serde_json::from_str(&json)?;
                Ok(creds)
            }
            Err(Error::NoEntry) => {
                // Credentials not set, trigger first-run flow
                Err(StorageError::CredentialsNotFound)
            }
            Err(Error::PlatformFailure(msg)) => {
                // Keychain unavailable, try environment variables
                tracing::warn!(
                    "OS keychain unavailable ({}), falling back to environment variables",
                    msg
                );
                load_from_environment()
            }
            Err(e) => Err(StorageError::KeychainError(e.to_string())),
        },
        Err(e) => {
            tracing::error!("Failed to create keychain entry: {}", e);
            load_from_environment()
        }
    }
}

fn load_from_environment() -> Result<StorageCredentials, StorageError> {
    Ok(StorageCredentials {
        access_key_id: env::var("ALTAIR_STORAGE_ACCESS_KEY")
            .map_err(|_| StorageError::CredentialsNotFound)?,
        secret_access_key: env::var("ALTAIR_STORAGE_SECRET_KEY")
            .map_err(|_| StorageError::CredentialsNotFound)?,
        endpoint: env::var("ALTAIR_STORAGE_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:9000".to_string()),
        bucket: env::var("ALTAIR_STORAGE_BUCKET")
            .unwrap_or_else(|_| format!("altair-{}", "default")),
    })
}
```

## First-Run Credential Setup Flow

### Embedded Minio (Default)

For embedded Minio, credentials are auto-generated and stored:

1. **App Startup:**

   - Check keychain for existing credentials
   - If not found, generate new credentials

2. **Credential Generation:**

   ```rust
   use rand::Rng;

   fn generate_minio_credentials() -> (String, String) {
       let access_key = "altair"; // Fixed for simplicity
       let secret_key = generate_secure_password(32); // Random 32-char password
       (access_key, secret_key)
   }

   fn generate_secure_password(length: usize) -> String {
       use rand::distributions::Alphanumeric;
       rand::thread_rng()
           .sample_iter(&Alphanumeric)
           .take(length)
           .map(char::from)
           .collect()
   }
   ```

3. **Store in Keychain:**

   ```rust
   let credentials = StorageCredentials {
       access_key_id: "altair".to_string(),
       secret_access_key: generate_secure_password(32),
       endpoint: "http://localhost:9000".to_string(),
       bucket: format!("altair-{}", app_name),
   };

   let entry = Entry::new(&service, user_id)?;
   let json = serde_json::to_string(&credentials)?;
   entry.set_password(&json)?;
   ```

4. **Configure Minio:**
   - Pass generated credentials to Minio via environment variables
   - Start Minio process

**Flow Diagram:**

```
[App Start]
    ↓
[Check Keychain]
    ↓
[Credentials Exist?] -- No --> [Generate Credentials]
    ↓                               ↓
   Yes                         [Store in Keychain]
    ↓                               ↓
[Load Credentials] <----------------+
    ↓
[Start Minio with Credentials]
    ↓
[Initialize S3 Client]
```

### External Minio (Manual Configuration)

For external Minio (development, advanced users):

1. **User provides credentials via UI or CLI:**

   **Option A: UI (Future Enhancement)**

   - Settings dialog with fields:
     - Endpoint URL
     - Access Key ID
     - Secret Access Key
     - Bucket Name
   - "Test Connection" button
   - "Save to Keychain" button

   **Option B: Environment Variables (Current)**

   - User sets environment variables before starting app
   - App loads from env vars on startup

2. **Validation:**

   - Test connection to endpoint
   - Verify bucket exists or can be created
   - Test PUT/GET operations

3. **Storage:**
   - Save validated credentials to keychain
   - Use for all future sessions

### Migration from Environment Variables to Keychain

If app detects credentials in environment variables but not in keychain:

1. Offer to migrate:

   ```
   INFO: Found storage credentials in environment variables.
         Would you like to save these to the OS keychain for secure storage?
         [Yes] [No] [Don't ask again]
   ```

2. If user accepts:

   - Store credentials in keychain
   - Log success message
   - Continue using credentials

3. If user declines:
   - Continue using env vars
   - Log warning about reduced security
   - Remember preference

## Security Best Practices

### What We Do

✅ **Store credentials in OS keychain** (encrypted by OS)
✅ **Use per-app service names** (prevents conflicts)
✅ **Support multi-user** (separate credentials per user)
✅ **Generate strong passwords** (32 characters, alphanumeric)
✅ **Validate credentials before storage** (test connection)
✅ **Log security warnings** (when keychain unavailable)

### What We Don't Do

❌ **Never log credentials** (even in debug builds)
❌ **Never store credentials in config files** (plaintext)
❌ **Never transmit credentials** (only to localhost Minio)
❌ **Never share credentials** (across apps or users)

## Testing Strategy

### Unit Tests

See `backend/crates/altair-storage/tests/keychain_research.rs`:

- ✅ Store and retrieve credentials
- ✅ Overwrite existing credentials
- ✅ Delete credentials
- ✅ Handle missing credentials (NoEntry error)
- ✅ Multiple users with separate credentials
- ✅ JSON serialization/deserialization
- ✅ Platform-specific backends (macOS, Windows, Linux)

### Integration Tests

- ✅ End-to-end flow with embedded Minio
- ✅ Fallback to environment variables
- ✅ Credential validation
- ✅ Migration from env vars to keychain

### Manual Testing

| Platform      | Test                                     | Expected Result                |
| ------------- | ---------------------------------------- | ------------------------------ |
| macOS         | Open Keychain Access.app, verify entries | Entry visible and encrypted    |
| Windows       | Check Credential Manager                 | Entry visible in Generic Creds |
| Linux (GNOME) | Use Seahorse (keyring manager)           | Entry visible                  |

## Implementation Checklist

Phase 0.3 Tasks:

- [x] Validate store/retrieve S3 credentials
- [x] Document tested platforms
- [x] Design error handling for unavailable keychain
- [x] Design first-run credential setup flow

## References

- [keyring crate documentation](https://docs.rs/keyring/)
- [macOS Keychain Services](https://developer.apple.com/documentation/security/keychain_services)
- [Windows Credential Manager](https://docs.microsoft.com/en-us/windows/win32/secauthn/credential-manager)
- [Linux Secret Service API](https://specifications.freedesktop.org/secret-service/)

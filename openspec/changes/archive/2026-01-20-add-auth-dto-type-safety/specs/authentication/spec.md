## MODIFIED Requirements

### Requirement: Secure Token Storage (Client)

The system SHALL store authentication tokens securely on each client platform.

#### Scenario: Android token storage

- **WHEN** the Android client stores authentication tokens
- **THEN** tokens are stored using EncryptedSharedPreferences (Jetpack Security)
- **AND** tokens are encrypted with AES-256
- **AND** userId is stored as the Ulid string representation

#### Scenario: iOS token storage

- **WHEN** the iOS client stores authentication tokens
- **THEN** tokens are stored using Keychain Services
- **AND** tokens are protected by device security settings
- **AND** userId is stored as the Ulid string representation

#### Scenario: Desktop token storage with native credential store

- **WHEN** the desktop client stores authentication tokens
- **AND** a native credential store is available (macOS Keychain, Windows Credential Manager, or Linux Secret Service)
- **THEN** tokens are stored using the native credential store
- **AND** tokens benefit from OS-level security features
- **AND** userId is stored as the Ulid string representation

#### Scenario: Desktop token storage fallback

- **WHEN** the desktop client stores authentication tokens
- **AND** no native credential store is available or accessible
- **THEN** tokens are stored using AES-256-GCM encrypted Java Preferences
- **AND** the encryption key is derived from installation-specific data
- **AND** userId is stored as the Ulid string representation

#### Scenario: Desktop native store unavailable at runtime

- **WHEN** the native credential store becomes unavailable during application execution
- **THEN** the system falls back to encrypted Java Preferences storage
- **AND** a warning is logged for debugging purposes

#### Scenario: User ID retrieved from storage

- **WHEN** the client retrieves a stored userId
- **THEN** the string value is converted back to a Ulid type
- **AND** invalid Ulid strings result in authentication failure requiring re-login

## ADDED Requirements

### Requirement: Auth State Type Safety

The client-side authentication state SHALL use domain types for type-safe user identification.

#### Scenario: Authenticated state contains typed userId

- **WHEN** the user is authenticated
- **THEN** AuthState.Authenticated contains userId as Ulid type
- **AND** downstream code receives compile-time type safety guarantees

#### Scenario: Auth manager returns typed userId

- **WHEN** login or registration succeeds
- **THEN** the AuthManager returns the userId as Ulid type
- **AND** callers do not need to parse or validate the ID format

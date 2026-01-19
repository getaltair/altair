## MODIFIED Requirements

### Requirement: Secure Token Storage (Client)

The system SHALL store authentication tokens securely on each client platform.

#### Scenario: Android token storage

- **WHEN** the Android client stores authentication tokens
- **THEN** tokens are stored using EncryptedSharedPreferences (Jetpack Security)
- **AND** tokens are encrypted with AES-256

#### Scenario: iOS token storage

- **WHEN** the iOS client stores authentication tokens
- **THEN** tokens are stored using Keychain Services
- **AND** tokens are protected by device security settings

#### Scenario: Desktop token storage with native credential store

- **WHEN** the desktop client stores authentication tokens
- **AND** a native credential store is available (macOS Keychain, Windows Credential Manager, or Linux Secret Service)
- **THEN** tokens are stored using the native credential store
- **AND** tokens benefit from OS-level security features

#### Scenario: Desktop token storage fallback

- **WHEN** the desktop client stores authentication tokens
- **AND** no native credential store is available or accessible
- **THEN** tokens are stored using AES-256-GCM encrypted Java Preferences
- **AND** the encryption key is derived from installation-specific data

#### Scenario: Desktop native store unavailable at runtime

- **WHEN** the native credential store becomes unavailable during application execution
- **THEN** the system falls back to encrypted Java Preferences storage
- **AND** a warning is logged for debugging purposes

## ADDED Requirements

### Requirement: Native Credential Store Platform Support

The desktop client SHALL support native credential stores on major operating systems.

#### Scenario: macOS Keychain integration

- **WHEN** the desktop client runs on macOS
- **THEN** tokens are stored in the system Keychain via Security.framework
- **AND** credentials use service name "com.getaltair.altair"
- **AND** each token is stored as a separate generic password item

#### Scenario: Windows Credential Manager integration

- **WHEN** the desktop client runs on Windows
- **THEN** tokens are stored in Windows Credential Manager via advapi32.dll
- **AND** credentials use target name prefix "altair:"
- **AND** each token is stored as a separate generic credential

#### Scenario: Linux Secret Service integration

- **WHEN** the desktop client runs on Linux
- **AND** a Secret Service provider is available (GNOME Keyring or KDE Wallet)
- **THEN** tokens are stored via libsecret
- **AND** credentials use a schema with application attribute "com.getaltair.altair"
- **AND** each token is stored as a separate secret

### Requirement: Credential Store Discovery

The system SHALL automatically detect and use the best available credential storage mechanism.

#### Scenario: Native store detected and functional

- **WHEN** the desktop application starts
- **AND** a native credential store is detected for the current platform
- **AND** the native library can be loaded successfully
- **THEN** the system uses the native credential store for all token operations

#### Scenario: Native store detection failure

- **WHEN** the desktop application starts
- **AND** native credential store detection fails (missing library, permission denied, etc.)
- **THEN** the system logs a warning with the failure reason
- **AND** the system uses the fallback encrypted storage
- **AND** the application continues to function normally

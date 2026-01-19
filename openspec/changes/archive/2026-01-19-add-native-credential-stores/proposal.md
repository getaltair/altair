# Change: Add Native Credential Stores for Desktop Platform

**Related Issue:** [getaltair/altair#18](https://github.com/getaltair/altair/issues/18)

## Why

The current Desktop implementation of `SecureTokenStorage` uses AES-256-GCM encrypted Java Preferences. While functionally secure, this approach:
- Doesn't integrate with native OS credential managers (better UX)
- Requires users to trust a custom encryption implementation
- Doesn't benefit from OS-level security features (biometric unlocking, hardware-backed keys)
- May store tokens in less secure locations than dedicated credential stores

Native credential stores provide:
- Better security integration (hardware-backed keys where available)
- Consistent UX with other applications
- Automatic credential lifecycle management
- Potential biometric authentication support

## What Changes

- **NEW**: `CredentialStoreProvider` interface for platform-specific credential stores
- **NEW**: `MacOSCredentialStore` implementation using Keychain Services via JNA
- **NEW**: `WindowsCredentialStore` implementation using Windows Credential Manager via JNA
- **NEW**: `LinuxCredentialStore` implementation using libsecret (GNOME Keyring / KDE Wallet) via JNA
- **REPLACED**: `DesktopSecureTokenStorage` replaced by native stores with fallback
- **NEW**: Factory pattern with automatic platform detection and graceful fallback
- **NEW**: Add JNA dependency for native library access

## Impact

- **Affected specs:** `authentication` (Secure Token Storage requirement)
- **Affected code:**
  - `shared/src/jvmMain/kotlin/com/getaltair/altair/service/auth/DesktopSecureTokenStorage.kt`
  - `shared/build.gradle.kts` (new JNA dependency)
- **New files:**
  - `shared/src/jvmMain/kotlin/com/getaltair/altair/service/auth/CredentialStoreProvider.kt`
  - `shared/src/jvmMain/kotlin/com/getaltair/altair/service/auth/NativeCredentialStoreFactory.kt`
  - `shared/src/jvmMain/kotlin/com/getaltair/altair/service/auth/native/MacOSCredentialStore.kt`
  - `shared/src/jvmMain/kotlin/com/getaltair/altair/service/auth/native/WindowsCredentialStore.kt`
  - `shared/src/jvmMain/kotlin/com/getaltair/altair/service/auth/native/LinuxCredentialStore.kt`

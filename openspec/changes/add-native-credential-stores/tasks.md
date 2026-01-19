## 1. Setup

- [ ] 1.1 Add JNA dependency to `shared/build.gradle.kts` for jvmMain source set
- [ ] 1.2 Create `CredentialStoreProvider` interface in `shared/src/jvmMain/.../service/auth/`

## 2. Native Store Implementations

- [ ] 2.1 Implement `MacOSCredentialStore` using Keychain Services via JNA
  - Map Security.framework functions: SecKeychainAddGenericPassword, SecKeychainFindGenericPassword, SecKeychainItemDelete
  - Handle OSStatus error codes appropriately
- [ ] 2.2 Implement `WindowsCredentialStore` using Credential Manager via JNA
  - Map advapi32.dll functions: CredWriteW, CredReadW, CredDeleteW, CredFree
  - Handle DWORD error codes appropriately
- [ ] 2.3 Implement `LinuxCredentialStore` using libsecret via JNA
  - Map libsecret functions: secret_password_store_sync, secret_password_lookup_sync, secret_password_clear_sync
  - Define Secret.Schema for Altair credentials
  - Handle GError appropriately

## 3. Factory and Integration

- [ ] 3.1 Create `NativeCredentialStoreFactory` with platform detection
  - Detect OS via `System.getProperty("os.name")`
  - Attempt to load native library and verify functionality
  - Return appropriate store or fallback
- [ ] 3.2 Create `NativeSecureTokenStorage` that wraps `CredentialStoreProvider`
  - Implement `SecureTokenStorage` interface
  - Delegate to native store for each token operation
- [ ] 3.3 Update Koin DI module to use factory pattern
  - Try native store first
  - Fall back to `DesktopSecureTokenStorage` if unavailable

## 4. Testing

- [ ] 4.1 Add unit tests for `NativeCredentialStoreFactory` platform detection
- [ ] 4.2 Add unit tests for `NativeSecureTokenStorage` with mocked provider
- [ ] 4.3 Add integration tests for each platform (run on CI or manually)
  - macOS: Test Keychain integration
  - Windows: Test Credential Manager integration
  - Linux: Test libsecret integration (requires D-Bus and secret service)
- [ ] 4.4 Add fallback behavior tests (native store unavailable scenario)

## 5. Documentation

- [ ] 5.1 Update CLAUDE.md authentication section with native store information
- [ ] 5.2 Add debug logging for credential store type selection

## Dependencies

- Tasks 2.1, 2.2, 2.3 can be parallelized after 1.1 and 1.2 are complete
- Task 3.1 depends on 2.x (needs implementations to detect)
- Task 3.2 depends on 3.1
- Task 3.3 depends on 3.2
- Task 4.x depends on 3.x

## 1. Setup

- [x] 1.1 Add JNA dependency to `shared/build.gradle.kts` for jvmMain source set
- [x] 1.2 Create `CredentialStoreProvider` interface in `shared/src/jvmMain/.../service/auth/`

## 2. Native Store Implementations

- [x] 2.1 Implement `MacOSCredentialStore` using Keychain Services via JNA
  - Map Security.framework functions: SecKeychainAddGenericPassword, SecKeychainFindGenericPassword, SecKeychainItemDelete
  - Handle OSStatus error codes appropriately
- [x] 2.2 Implement `WindowsCredentialStore` using Credential Manager via JNA
  - Map advapi32.dll functions: CredWriteW, CredReadW, CredDeleteW, CredFree
  - Handle DWORD error codes appropriately
- [x] 2.3 Implement `LinuxCredentialStore` using secret-tool CLI
  - Uses secret-tool commands for reliable Secret Service access
  - Handles store, lookup, and clear operations via subprocess

## 3. Factory and Integration

- [x] 3.1 Create `NativeCredentialStoreFactory` with platform detection
  - Detect OS via `System.getProperty("os.name")`
  - Attempt to load native library and verify functionality
  - Return appropriate store or fallback
- [x] 3.2 Create `NativeSecureTokenStorage` that wraps `CredentialStoreProvider`
  - Implement `SecureTokenStorage` interface
  - Delegate to native store for each token operation
- [x] 3.3 Update Koin DI module to use factory pattern
  - Try native store first
  - Fall back to `DesktopSecureTokenStorage` if unavailable

## 4. Testing

- [x] 4.1 Add unit tests for `NativeCredentialStoreFactory` platform detection
- [x] 4.2 Add unit tests for `NativeSecureTokenStorage` with mocked provider
- [x] 4.3 Add integration tests for each platform (run on CI or manually)
  - macOS: Test Keychain integration
  - Windows: Test Credential Manager integration
  - Linux: Test secret-tool integration (requires secret-tool and secret service)
- [x] 4.4 Add fallback behavior tests (native store unavailable scenario)

## 5. Documentation

- [x] 5.1 Update CLAUDE.md authentication section with native store information
- [x] 5.2 Add debug logging for credential store type selection (using SLF4J)

## Dependencies

- Tasks 2.1, 2.2, 2.3 can be parallelized after 1.1 and 1.2 are complete
- Task 3.1 depends on 2.x (needs implementations to detect)
- Task 3.2 depends on 3.1
- Task 3.3 depends on 3.2
- Task 4.x depends on 3.x

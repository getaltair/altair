## Context

The Desktop platform currently uses a custom AES-256-GCM encryption layer over Java Preferences for token storage (`DesktopSecureTokenStorage`). This works but doesn't leverage OS-native credential stores that provide:
- Hardware-backed security (TPM, Secure Enclave)
- User-managed credential lifecycle
- Integration with system lock screens
- Potential biometric authentication

This change affects only the Desktop (JVM) platform. Android and iOS already use their respective native secure storage mechanisms (EncryptedSharedPreferences and Keychain Services).

## Goals / Non-Goals

**Goals:**
- Use native credential stores when available (macOS Keychain, Windows Credential Manager, Linux Secret Service)
- Provide graceful fallback when native stores are unavailable
- Minimize new dependencies

**Non-Goals:**
- Biometric authentication integration (future enhancement)
- Support for exotic platforms (FreeBSD, etc.)

## Decisions

### Decision: Use JNA for Native Access

JNA (Java Native Access) provides a simpler integration path than JNI:
- No native code compilation required
- Dynamic binding at runtime
- Good Kotlin interop
- Well-maintained library with broad platform support

**Alternative considered:** JNI
- Requires native code compilation per platform
- More complex build setup
- Better performance but unnecessary for credential store access

### Decision: Factory Pattern with Fallback Chain

```
NativeCredentialStoreFactory.create() returns:
1. Try native store for current platform
2. If unavailable/fails, return DesktopSecureTokenStorage (current impl)
```

This ensures:
- Best available security on each platform
- No breakage if native library loading fails
- Existing behavior preserved as fallback

### Decision: Keychain/Credential Manager API Mapping

| Platform | API | Credential Type |
|----------|-----|-----------------|
| macOS | Security.framework (SecKeychainAddGenericPassword) | Generic password |
| Windows | advapi32.dll (CredWrite/CredRead) | Generic credential |
| Linux | libsecret (secret_password_store) | Schema-based password |

All credentials stored with:
- Service name: "com.getaltair.altair"
- Account/label: key name (access_token, refresh_token, etc.)

### Decision: Store Individual Tokens Separately

Store each token as a separate credential rather than bundling:
- Simpler error handling (one failure doesn't lose all tokens)
- Matches current `SecureTokenStorage` interface
- Easier debugging and manual inspection if needed

## Risks / Trade-offs

**Risk:** Native library loading fails on some systems
- **Mitigation:** Comprehensive fallback to existing encryption-based storage
- **Mitigation:** Log warnings when falling back for debugging

**Risk:** Different platforms have different credential size limits
- **Mitigation:** JWTs are typically small (<4KB), well under limits
- **Mitigation:** If limits exceeded, fall back to encrypted storage

**Risk:** User credentials may be visible in system credential manager UI
- **Mitigation:** Use service-specific names that identify Altair
- **Trade-off:** This is actually a UX benefit (users can manage credentials)

**Risk:** JNA adds ~3MB to application size
- **Trade-off:** Acceptable for desktop applications; security benefit outweighs size

## Open Questions

1. **Should we expose credential store type in settings/debug UI?**
   - Low priority but could help with support tickets
   - Recommendation: Add to debug info panel, not settings

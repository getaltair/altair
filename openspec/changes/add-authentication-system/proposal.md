# Change: Implement Authentication & Multi-User System (Phase 6)

## Why

Altair requires proper authentication to support multi-user households on self-hosted servers.
Currently, the `AuthServiceImpl` is a stub that accepts any credentials and returns test tokens.
This phase implements real JWT authentication, Argon2 password hashing, invite-only registration,
and user-scoped data access as specified in ADR-012.

## What Changes

### Server Authentication Layer
- Implement JWT token generation with configurable secrets and expiration
- Add Argon2 password hashing for secure credential storage
- Replace stub `AuthServiceImpl` with real implementation using `UserRepository`
- Add password hash field to User entity and migration
- Create `InviteCodeRepository` and invite-based registration flow
- Add `RefreshTokenRepository` for secure token rotation and revocation

### User Scoping Middleware
- Create `AuthContext` interface for accessing authenticated user
- Implement Ktor authentication middleware extracting user from JWT
- Wire auth context into Koin's user-scoped repository module
- Ensure all repositories receive user context for query filtering

### Client Authentication Flow
- Create platform-specific secure token storage (Keychain/Keystore/SecureStorage)
- Add `AuthManager` for token lifecycle management in composeApp
- Implement auth state in Koin for session management
- Create basic Login and Registration UI screens

### Auth Service Enhancements
- Add `AuthService.generateInviteCode()` for admin users
- Add `AuthService.changePassword()` for authenticated users
- Add `AuthService.revokeAllSessions()` for security

## Impact

- **Affected specs**: `rpc-services` (AuthService modifications), new `authentication` spec
- **Affected code**:
  - `server/src/main/kotlin/com/getaltair/rpc/AuthServiceImpl.kt`
  - `server/src/main/kotlin/Security.kt`
  - `shared/src/commonMain/kotlin/com/getaltair/altair/domain/model/system/User.kt`
  - `shared/src/commonMain/kotlin/com/getaltair/altair/rpc/AuthService.kt`
  - `composeApp/src/commonMain/kotlin/.../auth/` (new)
  - Platform-specific token storage implementations

## References

- [ADR-012: Multi-User Data Isolation](../../docs/adr/012-multi-user-data-isolation.md)
- [Phase 6 in implementation-plan.md](../../docs/implementation-plan.md)

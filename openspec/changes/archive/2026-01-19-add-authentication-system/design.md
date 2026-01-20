## Context

Altair is a self-hosted, multi-user productivity application. Households may share a single server
instance while keeping data completely isolated. ADR-012 establishes the data isolation architecture;
this change implements the authentication layer that enforces it.

### Stakeholders
- **End users**: Need secure login without friction
- **Server admins**: Need user management without content access
- **Developers**: Need clean auth abstraction for all platforms

### Constraints
- Must work offline on desktop (local auth with sync later)
- Mobile uses server authentication exclusively
- Passwords must never be stored in plaintext
- Tokens must be securely stored per platform

## Goals / Non-Goals

### Goals
- Secure JWT-based authentication for client-server communication
- Argon2id password hashing (OWASP recommended)
- Invite-only registration to prevent unauthorized signups
- Platform-specific secure credential storage
- Token refresh without re-authentication
- Session revocation capability

### Non-Goals
- OAuth/OIDC federation (future consideration)
- Two-factor authentication (future phase)
- Password reset via email (requires email infrastructure)
- Desktop offline authentication (desktop uses embedded DB directly)

## Decisions

### Decision 1: JWT with HMAC-SHA256 signing
**What**: Use HS256 JWTs with server-side secret, short-lived access tokens (15 min), long-lived
refresh tokens (30 days).

**Why**: Simple to implement, sufficient for self-hosted scenarios. RS256 adds complexity without
benefit when there's only one server.

**Alternatives considered**:
- RS256/ES256 asymmetric signing: Overkill for single-server deployment
- Session cookies: Poor fit for mobile apps and RPC

### Decision 2: Argon2id for password hashing
**What**: Use Argon2id with recommended parameters (memory: 64MB, iterations: 3, parallelism: 4).

**Why**: OWASP-recommended, resistant to GPU/ASIC attacks, available via `de.mkammerer:argon2-jvm`.

**Alternatives considered**:
- bcrypt: Good but Argon2 is newer and recommended
- PBKDF2: Weaker against hardware attacks

### Decision 3: Refresh token rotation
**What**: Store refresh tokens in database with device metadata. Issue new refresh token on each
refresh, invalidating the old one.

**Why**: Limits damage from token theft, enables "revoke all sessions" functionality.

**Alternatives considered**:
- Stateless refresh tokens: Cannot revoke without blacklist
- No refresh tokens: Poor UX (frequent re-login)

### Decision 4: Platform-specific secure storage
**What**:
- Android: EncryptedSharedPreferences (Jetpack Security)
- iOS: Keychain Services
- Desktop: Java KeyStore or OS keyring via `java-keyring`

**Why**: Each platform has native secure storage; using it ensures credentials aren't accessible
to other apps.

### Decision 5: AuthContext via Koin scoping
**What**: Create `AuthContext` interface, implement as request-scoped object in Ktor, inject into
repositories via Koin user scope.

**Why**: Clean separation, testable, follows existing Koin patterns in the codebase.

## Component Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              CLIENT                                      │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────┐    ┌──────────────┐    ┌────────────────────────────┐ │
│  │  Login UI    │───▶│ AuthManager  │───▶│ SecureTokenStorage         │ │
│  │              │    │              │    │ (platform-specific)        │ │
│  │  Register UI │    │ - login()    │    │ - Android: EncryptedSP     │ │
│  └──────────────┘    │ - logout()   │    │ - iOS: Keychain            │ │
│                      │ - refresh()  │    │ - Desktop: KeyStore        │ │
│                      └──────┬───────┘    └────────────────────────────┘ │
│                             │                                            │
│                             ▼                                            │
│                      ┌──────────────┐                                   │
│                      │ AuthService  │ (RPC)                             │
│                      │ (interface)  │                                   │
│                      └──────┬───────┘                                   │
└─────────────────────────────┼───────────────────────────────────────────┘
                              │ kotlinx-rpc WebSocket
                              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                              SERVER                                      │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────────┐    ┌───────────────────┐    ┌──────────────────┐ │
│  │ AuthServiceImpl  │───▶│ JwtTokenService   │───▶│ JWT Generation   │ │
│  │                  │    │                   │    │ & Validation     │ │
│  │ - login()        │    │ - generateTokens()│    └──────────────────┘ │
│  │ - register()     │    │ - validateToken() │                         │
│  │ - refresh()      │    │ - extractClaims() │                         │
│  │ - logout()       │    └───────────────────┘                         │
│  └────────┬─────────┘                                                   │
│           │                                                              │
│           ▼                                                              │
│  ┌──────────────────┐    ┌───────────────────┐                         │
│  │ PasswordService  │    │ UserRepository    │                         │
│  │                  │    │                   │                         │
│  │ - hash()         │    │ - findByEmail()   │                         │
│  │ - verify()       │    │ - create()        │                         │
│  │ (Argon2id)       │    │ - updatePassword()│                         │
│  └──────────────────┘    └───────────────────┘                         │
│                                                                          │
│  ┌──────────────────┐    ┌───────────────────┐                         │
│  │ RefreshToken     │    │ InviteCode        │                         │
│  │ Repository       │    │ Repository        │                         │
│  │                  │    │                   │                         │
│  │ - store()        │    │ - create()        │                         │
│  │ - validate()     │    │ - consume()       │                         │
│  │ - revoke()       │    │ - validate()      │                         │
│  └──────────────────┘    └───────────────────┘                         │
│                                                                          │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                     Ktor Authentication Middleware                │  │
│  │                                                                    │  │
│  │  Request ──▶ JWT Validation ──▶ AuthContext ──▶ User-Scoped DI   │  │
│  └──────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
```

## Token Format

### Access Token Claims
```json
{
  "sub": "01HWUSER00000000000000001",
  "aud": "altair-client",
  "iss": "altair-server",
  "iat": 1705500000,
  "exp": 1705500900,
  "role": "member",
  "email": "user@example.com"
}
```

### Refresh Token (stored in DB)
```kotlin
data class RefreshToken(
    val id: Ulid,
    val userId: Ulid,
    val tokenHash: String,      // SHA-256 hash of actual token
    val deviceName: String?,
    val expiresAt: Instant,
    val createdAt: Instant,
    val revokedAt: Instant?
)
```

## Database Changes

### User Table Updates
```sql
-- Add password_hash column to user table
ALTER TABLE user ADD COLUMN password_hash STRING;
```

### New Tables
```sql
-- Refresh tokens for session management
DEFINE TABLE refresh_token SCHEMAFULL;
DEFINE FIELD user_id ON refresh_token TYPE record<user>;
DEFINE FIELD token_hash ON refresh_token TYPE string;
DEFINE FIELD device_name ON refresh_token TYPE option<string>;
DEFINE FIELD expires_at ON refresh_token TYPE datetime;
DEFINE FIELD created_at ON refresh_token TYPE datetime DEFAULT time::now();
DEFINE FIELD revoked_at ON refresh_token TYPE option<datetime>;
DEFINE INDEX idx_refresh_token_hash ON refresh_token FIELDS token_hash UNIQUE;

-- Invite codes for controlled registration
DEFINE TABLE invite_code SCHEMAFULL;
DEFINE FIELD code ON invite_code TYPE string;
DEFINE FIELD created_by ON invite_code TYPE record<user>;
DEFINE FIELD used_by ON invite_code TYPE option<record<user>>;
DEFINE FIELD expires_at ON invite_code TYPE datetime;
DEFINE FIELD created_at ON invite_code TYPE datetime DEFAULT time::now();
DEFINE FIELD used_at ON invite_code TYPE option<datetime>;
DEFINE INDEX idx_invite_code ON invite_code FIELDS code UNIQUE;
```

## Risks / Trade-offs

### Risk: Token theft enables impersonation
**Mitigation**: Short-lived access tokens (15 min), refresh token rotation, session revocation.

### Risk: Brute force attacks on login
**Mitigation**: Argon2 is slow by design, rate limiting at Ktor level (future: add explicit rate
limiter).

### Risk: First user bootstrap (no admin to create invite)
**Mitigation**: If no users exist, first registration creates admin without invite code.

### Trade-off: No offline desktop auth
**Rationale**: Desktop uses embedded SurrealDB directly, authentication happens via the database
itself. Server auth is for sync only.

## Migration Plan

1. Add `password_hash` column to user table (nullable initially)
2. Create `refresh_token` and `invite_code` tables
3. Update `AuthServiceImpl` with real implementation
4. Deploy server update (existing test tokens stop working)
5. Users must re-register (acceptable for pre-production)

### Rollback
- Revert server to stub `AuthServiceImpl`
- Test tokens resume working
- No data migration needed (new tables can remain)

## Open Questions

1. **Rate limiting strategy**: Should we implement per-IP rate limiting now or defer?
   - **Recommendation**: Defer to Phase 11 (Platform Polish) unless security audit requires earlier

2. **Password requirements**: What minimum complexity should we enforce?
   - **Recommendation**: Minimum 8 characters, no complexity rules (follow NIST guidelines)

3. **Desktop offline mode**: Should desktop store credentials locally for offline-first operation?
   - **Recommendation**: Desktop uses embedded DB directly, no server auth needed for local operation

# Implementation Plan: Local Authentication and User Management

**Branch**: `spec/core-010-auth-local` | **Date**: 2025-12-07 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/core-010-auth-local/spec.md`

## Summary

Implement a plugin-based local authentication system for the Altair desktop application. The solution uses `LocalAuthProvider` implementing the `AuthProvider` trait with Argon2id password hashing, OS keychain session storage via the `keyring` crate, and SurrealDB-backed session management. This enables single-user identity for data ownership, preference persistence, and future sync authorization—all working fully offline.

## Technical Context

**Language/Version**: Rust 2024 edition (rustc 1.91.1)
**Primary Dependencies**: `argon2` (password hashing), `keyring` (OS keychain), `getrandom` (secure tokens), `surrealdb` (sessions/credentials), `tauri` (commands)
**Storage**: SurrealDB embedded (users, sessions, credentials), OS keychain (session token)
**Testing**: `cargo test`, integration tests with embedded SurrealDB
**Target Platform**: Linux, macOS, Windows desktop (Tauri)
**Project Type**: Rust monorepo with workspace crates
**Performance Goals**: Password hashing < 500ms, session validation < 5ms per request
**Constraints**: Offline-capable, single-user, no external auth calls
**Scale/Scope**: Single user per installation, ~8 Tauri commands

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Gate                            | Status  | Notes                                                                                   |
| ------------------------------- | ------- | --------------------------------------------------------------------------------------- |
| Single responsibility per crate | ✅ PASS | Auth logic stays in `altair-auth`, DB ops in `altair-db`, commands in `altair-commands` |
| No hardcoded secrets            | ✅ PASS | Session tokens generated at runtime, no embedded secrets                                |
| Plugin architecture             | ✅ PASS | `AuthProvider` trait already defined, `LocalAuthProvider` implements it                 |
| Existing patterns               | ✅ PASS | Follows existing `CommandResponse` wrapper, `Result<T, Error>` pattern                  |
| Test coverage                   | ✅ PASS | Unit tests for each module, integration tests for full flow                             |

## Project Structure

### Documentation (this feature)

```text
specs/core-010-auth-local/
├── spec.md              # Feature specification (completed)
├── plan.md              # This file
└── tasks.md             # Phase 2 output (/spectrena.tasks command)
```

### Source Code (repository root)

```text
backend/crates/
├── altair-auth/                    # Auth provider implementations
│   ├── src/
│   │   ├── lib.rs                  # Re-exports, module organization
│   │   ├── provider.rs             # AuthProvider trait (enhanced)
│   │   ├── local/
│   │   │   ├── mod.rs              # LocalAuthProvider implementation
│   │   │   ├── password.rs         # Argon2id hashing
│   │   │   ├── session.rs          # Session management
│   │   │   └── keychain.rs         # OS keychain abstraction
│   │   ├── types.rs                # AuthResponse, Session, UserCredential
│   │   └── error.rs                # Auth-specific errors
│   └── Cargo.toml                  # Add: argon2, keyring, getrandom, hex
│
├── altair-db/                      # Database operations
│   ├── src/
│   │   ├── schema/
│   │   │   ├── session.rs          # NEW: Session entity
│   │   │   └── credential.rs       # NEW: UserCredential entity
│   │   └── queries/
│   │       ├── user.rs             # NEW: User CRUD operations
│   │       ├── session.rs          # NEW: Session CRUD operations
│   │       └── credential.rs       # NEW: Credential operations
│   └── tests/
│       └── auth_integration.rs     # NEW: Auth flow integration tests
│
├── altair-commands/                # Tauri command handlers
│   ├── src/
│   │   ├── auth.rs                 # NEW: Auth command module
│   │   └── lib.rs                  # Register auth module
│   └── Cargo.toml                  # Add: altair-auth dependency
│
└── altair-core/                    # Shared types and errors
    └── src/
        └── error.rs                # Enhance Auth error variants

backend/migrations/
└── 005_auth_tables.surql           # NEW: Session and UserCredential tables
```

**Structure Decision**: Extend existing crate structure. `altair-auth` handles all auth logic, `altair-db` provides persistence, `altair-commands` exposes Tauri commands.

## Implementation Phases

### Phase 1: Core Auth Infrastructure (altair-auth)

**Objective**: Replace placeholder implementations with production-ready auth logic

**Tasks**:

1.1. **Add crate dependencies**

- Add to `altair-auth/Cargo.toml`: `argon2 = "0.6"`, `keyring = "3"`, `getrandom = "0.3"`, `hex = "0.4"`, `chrono = "0.4"`
- Update `altair-core/Cargo.toml` if needed for new error types

  1.2. **Implement password hashing module** (`src/local/password.rs`)

- `hash_password(password: &str) -> Result<String>` - Argon2id with PHC string output
- `verify_password(password: &str, hash: &str) -> Result<bool>` - Constant-time comparison
- Parameters: time=3, memory=65536 (64MB), parallelism=4, output=32 bytes
- Unit tests: hash creation, verification success, verification failure, timing consistency

  1.3. **Implement session management** (`src/local/session.rs`)

- `Session` struct: token, user_id, expires_at, device_id, created_at
- `generate_token() -> String` - 256-bit random via `getrandom`, hex-encoded
- `is_expired(&self) -> bool` - Check against current time
- `should_refresh(&self) -> bool` - True if within 1 day of 7-day expiration
- Unit tests: token generation uniqueness, expiration logic, refresh window

  1.4. **Implement keychain abstraction** (`src/local/keychain.rs`)

- `KeychainStorage` struct with service name `com.altair.auth`
- `store_token(token: &str) -> Result<()>` - Save to OS keychain
- `get_token() -> Result<Option<String>>` - Retrieve from keychain
- `delete_token() -> Result<()>` - Remove from keychain
- Fallback detection: If keychain unavailable, return specific error for UI to handle
- Unit tests: store/retrieve/delete cycle (mock keychain in tests)

  1.5. **Define auth types** (`src/types.rs`)

- `UserCredential`: user_id, password_hash, updated_at
- `AuthResponse`: user, session_token, expires_at
- `AuthError`: InvalidCredentials, SessionExpired, KeychainUnavailable, UserNotFound, UserAlreadyExists
- Derive: Serialize, Deserialize, Clone, Debug

  1.6. **Enhance AuthProvider trait** (`src/provider.rs`)

- Add `refresh(&self, token: &str) -> Result<Session>` method
- Add `register(&self, ...) -> Result<AuthResponse>` method
- Add `get_current_user(&self, token: &str) -> Result<User>` method
- Keep backward compatibility with existing trait methods

  1.7. **Implement LocalAuthProvider** (`src/local/mod.rs`)

- Full implementation of all `AuthProvider` methods
- Integration with password, session, and keychain modules
- Inject DB client for persistence (via constructor)
- Background session refresh logic (optional auto-refresh timer)

**Deliverables**: Fully functional `altair-auth` crate with real password hashing, session tokens, and keychain storage.

**Exit Criteria**:

- `cargo test -p altair-auth` passes
- Password hashing < 500ms (benchmark test)
- Session tokens are 64-char hex strings
- Keychain operations work on dev machine

---

### Phase 2: Database Schema and Operations (altair-db)

**Objective**: Add persistence layer for sessions and credentials

**Tasks**:

2.1. **Create migration for auth tables** (`migrations/005_auth_tables.surql`)

- `session` table: id, token (unique indexed), user, expires_at, device_id, created_at
- `user_credential` table: id, user (unique indexed), password_hash, updated_at
- Both tables: `SCHEMAFULL`, `CHANGEFEED 7d`
- Index: `token_idx` on session.token, `user_cred_idx` on user_credential.user

  2.2. **Add Session schema type** (`src/schema/session.rs`)

- `Session` struct matching DB schema
- `SessionStatus` enum (Active, Expired, Invalidated) if needed
- Implement `Default` for new sessions

  2.3. **Add UserCredential schema type** (`src/schema/credential.rs`)

- `UserCredential` struct: id, user (Thing), password_hash, updated_at
- No Default - always created with explicit values

  2.4. **Implement user queries** (`src/queries/user.rs`)

- `create_user(db, user) -> Result<User>` - Insert new user
- `get_user_by_email(db, email) -> Result<Option<User>>` - Lookup by email
- `get_user_by_id(db, id) -> Result<User>` - Lookup by ID
- `update_user_preferences(db, id, prefs) -> Result<User>` - Merge preferences
- `user_exists(db) -> Result<bool>` - Check if any user exists (for first-launch detection)

  2.5. **Implement session queries** (`src/queries/session.rs`)

- `create_session(db, session) -> Result<Session>` - Insert session
- `get_session_by_token(db, token) -> Result<Option<Session>>` - Lookup
- `refresh_session(db, token, new_expiry) -> Result<Session>` - Update expiry
- `delete_session(db, token) -> Result<()>` - Remove session (logout)
- `delete_expired_sessions(db) -> Result<u64>` - Cleanup job

  2.6. **Implement credential queries** (`src/queries/credential.rs`)

- `create_credential(db, user_id, hash) -> Result<UserCredential>` - Store hash
- `get_credential_by_user(db, user_id) -> Result<Option<UserCredential>>` - Lookup
- `update_credential(db, user_id, hash) -> Result<UserCredential>` - Change password

  2.7. **Add module exports** (`src/lib.rs`, `src/schema/mod.rs`, `src/queries/mod.rs`)

- Export new types and query functions
- Update `altair-db` Cargo.toml if new dependencies needed

**Deliverables**: Database schema and query functions for auth entities.

**Exit Criteria**:

- Migration applies cleanly to fresh DB
- `cargo test -p altair-db` passes including new auth tests
- Queries return correct types

---

### Phase 3: Tauri Commands (altair-commands)

**Objective**: Expose auth operations to the Svelte frontend via Tauri IPC

**Tasks**:

3.1. **Add auth module** (`src/auth.rs`)

- Define input structs: `RegisterInput`, `LoginInput`, `UpdatePrefsInput`
- Define response types (or re-export from altair-auth)

  3.2. **Implement auth_check_setup command**

- Returns `bool`: true if user exists, false for first-launch
- Used by frontend to show setup wizard vs login

  3.3. **Implement auth_register command**

- Input: email (required), display_name (optional), password (optional)
- Creates user, optional credential, initial session
- Returns `AuthResponse` with token and user profile
- Stores token in keychain

  3.4. **Implement auth_login command**

- Input: email, password (optional for passwordless)
- Validates credentials (if password set)
- Creates session, stores in keychain
- Returns `AuthResponse`

  3.5. **Implement auth_logout command**

- Input: none (uses stored token)
- Deletes session from DB
- Removes token from keychain
- Returns `()` on success

  3.6. **Implement auth_validate command**

- Input: none (retrieves token from keychain)
- Validates session, returns `User` if valid
- Returns error if invalid/expired

  3.7. **Implement auth_refresh command**

- Input: none (uses stored token)
- Extends session expiry by 7 days
- Returns updated `Session`

  3.8. **Implement auth_get_user command**

- Input: none (uses stored token)
- Returns current `User` with preferences

  3.9. **Implement auth_update_prefs command**

- Input: partial `UserPreferences` object
- Merges with existing preferences
- Returns updated `User`

  3.10. **Register commands in lib.rs**

- Add `pub mod auth;` to module declarations
- Export command functions for Tauri registration

**Deliverables**: 8 Tauri commands for complete auth flow.

**Exit Criteria**:

- All commands compile and type-check
- Commands return correct response types
- `cargo test -p altair-commands` passes

---

### Phase 4: Integration and Testing

**Objective**: End-to-end validation and performance verification

**Tasks**:

4.1. **Integration test: First-time setup flow**

- Fresh DB → check_setup returns false → register → check_setup returns true
- Verify user created, session active, token in keychain

  4.2. **Integration test: Login/logout cycle**

- Register → logout → validate fails → login → validate succeeds
- Verify keychain token lifecycle

  4.3. **Integration test: Password verification**

- Register with password → logout → login with correct password succeeds
- Login with wrong password fails with generic error

  4.4. **Integration test: Passwordless auth**

- Register without password → logout → login without password succeeds
- Verify no credential record created

  4.5. **Integration test: Session expiry**

- Create session with past expiry → validate returns expired error
- Verify refresh extends expiry correctly

  4.6. **Integration test: Preferences CRUD**

- Register → update_prefs → get_user → verify preferences merged

  4.7. **Performance benchmarks**

- Password hashing: < 500ms (Argon2id parameters)
- Session validation: < 5ms overhead
- Token generation: < 1ms

  4.8. **Security tests**

- Timing attack resistance: verify_password takes same time for valid/invalid
- Error messages don't reveal which field was wrong
- Credentials never appear in logs (add tracing filters)

  4.9. **Keychain fallback test** (manual or CI-specific)

- Simulate keychain unavailable
- Verify specific error returned for UI handling

**Deliverables**: Comprehensive test suite with passing integration tests.

**Exit Criteria**:

- All integration tests pass
- Performance benchmarks meet targets
- Security tests validate constant-time operations

---

## Dependencies

### External Crates to Add

| Crate       | Version | Purpose                                                      | Crate Location         |
| ----------- | ------- | ------------------------------------------------------------ | ---------------------- |
| `argon2`    | 0.6     | Password hashing with Argon2id                               | altair-auth            |
| `keyring`   | 3.x     | OS keychain access (libsecret, Keychain, Credential Manager) | altair-auth            |
| `getrandom` | 0.3     | Cryptographically secure random bytes                        | altair-auth            |
| `hex`       | 0.4     | Hex encoding for tokens                                      | altair-auth            |
| `chrono`    | 0.4     | Timestamp handling                                           | altair-auth, altair-db |

### Internal Dependencies

- `altair-auth` depends on `altair-db` (for persistence)
- `altair-commands` depends on `altair-auth` (for auth operations)
- All crates depend on `altair-core` (for error types)

### Ordering Constraints

1. Phase 1 (altair-auth core) can proceed independently
2. Phase 2 (altair-db) can proceed in parallel with Phase 1
3. Phase 3 (altair-commands) requires Phase 1 + Phase 2 complete
4. Phase 4 (integration) requires all prior phases

## Risks and Mitigations

| Risk                                | Likelihood | Impact | Mitigation                                                                                                                                             |
| ----------------------------------- | ---------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Keychain unavailable on some Linux  | Medium     | Medium | Return specific `KeychainUnavailable` error; frontend prompts user to choose encrypted file fallback or fix keychain. Document libsecret requirements. |
| Argon2 too slow on weak hardware    | Low        | Low    | Make parameters configurable; document minimum specs                                                                                                   |
| Session token collision             | Very Low   | High   | 256-bit token space makes collision astronomically unlikely; add unique constraint in DB as safety net                                                 |
| Breaking existing placeholder tests | Low        | Low    | Update tests to match new implementation; maintain trait compatibility                                                                                 |

## Complexity Tracking

No constitution violations anticipated. The implementation:

- Extends existing crates rather than creating new ones
- Follows established patterns (CommandResponse, Result<T, Error>)
- Uses well-maintained external crates
- Single-responsibility: auth logic in altair-auth, persistence in altair-db, commands in altair-commands

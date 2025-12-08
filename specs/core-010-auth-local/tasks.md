# Tasks: core-010-auth-local

**Branch**: `spec/core-010-auth-local` | **Generated**: 2025-12-07
**Spec**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

---

## Phase 1: Core Auth Infrastructure (altair-auth)

**Objective**: Replace placeholder implementations with production-ready auth logic

### Task 1.1: Add crate dependencies

- [ ] Add dependencies to `backend/crates/altair-auth/Cargo.toml`
  - **Acceptance**: Dependencies include `argon2 = "0.6"`, `keyring = "3"`, `getrandom = "0.3"`, `hex = "0.4"`, `chrono = "0.4"`
  - **Files**: `backend/crates/altair-auth/Cargo.toml`
  - **Duration**: 15 minutes
  - **Verification**: `cargo check -p altair-auth` succeeds

### Task 1.2: Implement password hashing module

- [ ] Create `backend/crates/altair-auth/src/local/password.rs`
  - **Acceptance**:
    - `hash_password(password: &str) -> Result<String>` implemented with Argon2id (time=3, memory=65536, parallelism=4, output=32 bytes)
    - `verify_password(password: &str, hash: &str) -> Result<bool>` implemented with constant-time comparison
    - PHC string format output
    - Unit tests: hash creation, verification success, verification failure, timing consistency
  - **Files**: `backend/crates/altair-auth/src/local/password.rs`
  - **Duration**: 1 hour
  - **Verification**: `cargo test -p altair-auth password` passes, hashing < 500ms

### Task 1.3: Implement session management module

- [ ] Create `backend/crates/altair-auth/src/local/session.rs`
  - **Acceptance**:
    - `Session` struct with fields: token, user_id, expires_at, device_id, created_at
    - `generate_token() -> String` produces 256-bit random hex string
    - `is_expired(&self) -> bool` checks against current time
    - `should_refresh(&self) -> bool` returns true if within 1 day of expiration (7-day window)
    - Unit tests: token uniqueness, expiration logic, refresh window
  - **Files**: `backend/crates/altair-auth/src/local/session.rs`
  - **Duration**: 1 hour
  - **Verification**: `cargo test -p altair-auth session` passes, tokens are 64-char hex

### Task 1.4: Implement keychain abstraction module

- [ ] Create `backend/crates/altair-auth/src/local/keychain.rs`
  - **Acceptance**:
    - `KeychainStorage` struct with service name `com.altair.auth`
    - `store_token(token: &str) -> Result<()>` saves to OS keychain
    - `get_token() -> Result<Option<String>>` retrieves from keychain
    - `delete_token() -> Result<()>` removes from keychain
    - Fallback detection with `KeychainUnavailable` error
    - Unit tests with mocked keychain: store/retrieve/delete cycle
  - **Files**: `backend/crates/altair-auth/src/local/keychain.rs`
  - **Duration**: 1.5 hours
  - **Verification**: `cargo test -p altair-auth keychain` passes, manual test on dev machine works

### Task 1.5: Define auth types

- [ ] Create/update `backend/crates/altair-auth/src/types.rs`
  - **Acceptance**:
    - `UserCredential`: user_id, password_hash, updated_at
    - `AuthResponse`: user, session_token, expires_at
    - `AuthError` enum: InvalidCredentials, SessionExpired, KeychainUnavailable, UserNotFound, UserAlreadyExists
    - All types derive: Serialize, Deserialize, Clone, Debug
  - **Files**: `backend/crates/altair-auth/src/types.rs`
  - **Duration**: 30 minutes
  - **Verification**: Types compile and have required derives

### Task 1.6: Enhance AuthProvider trait

- [ ] Update `backend/crates/altair-auth/src/provider.rs`
  - **Acceptance**:
    - Add `refresh(&self, token: &str) -> Result<Session>` method
    - Add `register(&self, ...) -> Result<AuthResponse>` method
    - Add `get_current_user(&self, token: &str) -> Result<User>` method
    - Maintain backward compatibility with existing trait methods
  - **Files**: `backend/crates/altair-auth/src/provider.rs`
  - **Duration**: 30 minutes
  - **Verification**: Trait compiles, no breaking changes to existing implementations

### Task 1.7: Implement LocalAuthProvider

- [ ] Update `backend/crates/altair-auth/src/local/mod.rs`
  - **Acceptance**:
    - Full implementation of all `AuthProvider` trait methods
    - Integration with password, session, and keychain modules
    - Constructor accepts DB client for persistence
    - Optional background session refresh logic
  - **Files**: `backend/crates/altair-auth/src/local/mod.rs`
  - **Duration**: 2 hours
  - **Verification**: `cargo test -p altair-auth` passes completely

---

## Phase 2: Database Schema and Operations (altair-db)

**Objective**: Add persistence layer for sessions and credentials

### Task 2.1: Create migration for auth tables

- [ ] Create `backend/migrations/005_auth_tables.surql`
  - **Acceptance**:
    - `session` table: id, token (unique indexed), user, expires_at, device_id, created_at
    - `user_credential` table: id, user (unique indexed), password_hash, updated_at
    - Both tables: `SCHEMAFULL`, `CHANGEFEED 7d`
    - Indexes: `token_idx` on session.token, `user_cred_idx` on user_credential.user
  - **Files**: `backend/migrations/005_auth_tables.surql`
  - **Duration**: 45 minutes
  - **Verification**: Migration applies cleanly to fresh DB

### Task 2.2: Add Session schema type

- [ ] Create `backend/crates/altair-db/src/schema/session.rs`
  - **Acceptance**:
    - `Session` struct matching DB schema
    - Optional `SessionStatus` enum (Active, Expired, Invalidated) if needed
    - Implement `Default` for new sessions
  - **Files**: `backend/crates/altair-db/src/schema/session.rs`, `backend/crates/altair-db/src/schema/mod.rs`
  - **Duration**: 30 minutes
  - **Verification**: Type compiles and matches DB schema

### Task 2.3: Add UserCredential schema type

- [ ] Create `backend/crates/altair-db/src/schema/credential.rs`
  - **Acceptance**:
    - `UserCredential` struct: id, user (Thing), password_hash, updated_at
    - No Default implementation - always created with explicit values
  - **Files**: `backend/crates/altair-db/src/schema/credential.rs`, `backend/crates/altair-db/src/schema/mod.rs`
  - **Duration**: 20 minutes
  - **Verification**: Type compiles, no Default trait

### Task 2.4: Implement user queries

- [ ] Create `backend/crates/altair-db/src/queries/user.rs`
  - **Acceptance**:
    - `create_user(db, user) -> Result<User>` - Insert new user
    - `get_user_by_email(db, email) -> Result<Option<User>>` - Lookup by email
    - `get_user_by_id(db, id) -> Result<User>` - Lookup by ID
    - `update_user_preferences(db, id, prefs) -> Result<User>` - Merge preferences
    - `user_exists(db) -> Result<bool>` - Check if any user exists (first-launch detection)
  - **Files**: `backend/crates/altair-db/src/queries/user.rs`, `backend/crates/altair-db/src/queries/mod.rs`
  - **Duration**: 1.5 hours
  - **Verification**: Query functions compile and return correct types

### Task 2.5: Implement session queries

- [ ] Create `backend/crates/altair-db/src/queries/session.rs`
  - **Acceptance**:
    - `create_session(db, session) -> Result<Session>` - Insert session
    - `get_session_by_token(db, token) -> Result<Option<Session>>` - Lookup
    - `refresh_session(db, token, new_expiry) -> Result<Session>` - Update expiry
    - `delete_session(db, token) -> Result<()>` - Remove session (logout)
    - `delete_expired_sessions(db) -> Result<u64>` - Cleanup job
  - **Files**: `backend/crates/altair-db/src/queries/session.rs`, `backend/crates/altair-db/src/queries/mod.rs`
  - **Duration**: 1 hour
  - **Verification**: Query functions compile and return correct types

### Task 2.6: Implement credential queries

- [ ] Create `backend/crates/altair-db/src/queries/credential.rs`
  - **Acceptance**:
    - `create_credential(db, user_id, hash) -> Result<UserCredential>` - Store hash
    - `get_credential_by_user(db, user_id) -> Result<Option<UserCredential>>` - Lookup
    - `update_credential(db, user_id, hash) -> Result<UserCredential>` - Change password
  - **Files**: `backend/crates/altair-db/src/queries/credential.rs`, `backend/crates/altair-db/src/queries/mod.rs`
  - **Duration**: 45 minutes
  - **Verification**: Query functions compile and return correct types

### Task 2.7: Update module exports

- [ ] Update `backend/crates/altair-db/src/lib.rs`
  - **Acceptance**:
    - Export new schema types (Session, UserCredential)
    - Export new query modules (user, session, credential)
    - Update Cargo.toml if new dependencies needed
  - **Files**: `backend/crates/altair-db/src/lib.rs`, `backend/crates/altair-db/Cargo.toml`
  - **Duration**: 15 minutes
  - **Verification**: `cargo test -p altair-db` passes including new auth tests

---

## Phase 3: Tauri Commands (altair-commands)

**Objective**: Expose auth operations to the Svelte frontend via Tauri IPC

### Task 3.1: Create auth command module

- [ ] Create `backend/crates/altair-commands/src/auth.rs`
  - **Acceptance**:
    - Define input structs: `RegisterInput`, `LoginInput`, `UpdatePrefsInput`
    - Define or re-export response types from altair-auth
  - **Files**: `backend/crates/altair-commands/src/auth.rs`
  - **Duration**: 30 minutes
  - **Verification**: Module compiles with correct types

### Task 3.2: Implement auth_check_setup command

- [ ] Add `auth_check_setup` to `backend/crates/altair-commands/src/auth.rs`
  - **Acceptance**:
    - Returns `bool`: true if user exists, false for first-launch
    - Uses `user_exists` query from altair-db
    - Used by frontend to show setup wizard vs login
  - **Files**: `backend/crates/altair-commands/src/auth.rs`
  - **Duration**: 30 minutes
  - **Verification**: Command compiles and returns correct type

### Task 3.3: Implement auth_register command

- [ ] Add `auth_register` to `backend/crates/altair-commands/src/auth.rs`
  - **Acceptance**:
    - Input: email (required), display_name (optional), password (optional)
    - Creates user, optional credential, initial session
    - Returns `AuthResponse` with token and user profile
    - Stores token in keychain
  - **Files**: `backend/crates/altair-commands/src/auth.rs`
  - **Duration**: 1 hour
  - **Verification**: Command compiles and integrates with LocalAuthProvider

### Task 3.4: Implement auth_login command

- [ ] Add `auth_login` to `backend/crates/altair-commands/src/auth.rs`
  - **Acceptance**:
    - Input: email, password (optional for passwordless)
    - Validates credentials (if password set)
    - Creates session, stores in keychain
    - Returns `AuthResponse`
  - **Files**: `backend/crates/altair-commands/src/auth.rs`
  - **Duration**: 45 minutes
  - **Verification**: Command compiles and validates credentials

### Task 3.5: Implement auth_logout command

- [ ] Add `auth_logout` to `backend/crates/altair-commands/src/auth.rs`
  - **Acceptance**:
    - Input: none (uses stored token from keychain)
    - Deletes session from DB
    - Removes token from keychain
    - Returns `()` on success
  - **Files**: `backend/crates/altair-commands/src/auth.rs`
  - **Duration**: 30 minutes
  - **Verification**: Command compiles and cleans up session

### Task 3.6: Implement auth_validate command

- [ ] Add `auth_validate` to `backend/crates/altair-commands/src/auth.rs`
  - **Acceptance**:
    - Input: none (retrieves token from keychain)
    - Validates session, returns `User` if valid
    - Returns error if invalid/expired
  - **Files**: `backend/crates/altair-commands/src/auth.rs`
  - **Duration**: 30 minutes
  - **Verification**: Command compiles and validates sessions

### Task 3.7: Implement auth_refresh command

- [ ] Add `auth_refresh` to `backend/crates/altair-commands/src/auth.rs`
  - **Acceptance**:
    - Input: none (uses stored token from keychain)
    - Extends session expiry by 7 days
    - Returns updated `Session`
  - **Files**: `backend/crates/altair-commands/src/auth.rs`
  - **Duration**: 30 minutes
  - **Verification**: Command compiles and refreshes sessions

### Task 3.8: Implement auth_get_user command

- [ ] Add `auth_get_user` to `backend/crates/altair-commands/src/auth.rs`
  - **Acceptance**:
    - Input: none (uses stored token from keychain)
    - Returns current `User` with preferences
  - **Files**: `backend/crates/altair-commands/src/auth.rs`
  - **Duration**: 30 minutes
  - **Verification**: Command compiles and returns user data

### Task 3.9: Implement auth_update_prefs command

- [ ] Add `auth_update_prefs` to `backend/crates/altair-commands/src/auth.rs`
  - **Acceptance**:
    - Input: partial `UserPreferences` object
    - Merges with existing preferences
    - Returns updated `User`
  - **Files**: `backend/crates/altair-commands/src/auth.rs`
  - **Duration**: 45 minutes
  - **Verification**: Command compiles and merges preferences

### Task 3.10: Register commands in lib.rs

- [ ] Update `backend/crates/altair-commands/src/lib.rs`
  - **Acceptance**:
    - Add `pub mod auth;` to module declarations
    - Export all 8 auth command functions
    - Update Cargo.toml to add `altair-auth` dependency
  - **Files**: `backend/crates/altair-commands/src/lib.rs`, `backend/crates/altair-commands/Cargo.toml`
  - **Duration**: 15 minutes
  - **Verification**: `cargo test -p altair-commands` passes

---

## Phase 4: Integration and Testing

**Objective**: End-to-end validation and performance verification

### Task 4.1: Integration test - First-time setup flow

- [ ] Create `backend/crates/altair-db/tests/auth_integration.rs`
  - **Acceptance**:
    - Test: Fresh DB → check_setup returns false → register → check_setup returns true
    - Verify user created, session active, token in keychain
  - **Files**: `backend/crates/altair-db/tests/auth_integration.rs`
  - **Duration**: 1 hour
  - **Verification**: Test passes on fresh DB

### Task 4.2: Integration test - Login/logout cycle

- [ ] Add to `backend/crates/altair-db/tests/auth_integration.rs`
  - **Acceptance**:
    - Test: Register → logout → validate fails → login → validate succeeds
    - Verify keychain token lifecycle
  - **Files**: `backend/crates/altair-db/tests/auth_integration.rs`
  - **Duration**: 45 minutes
  - **Verification**: Test passes with token lifecycle

### Task 4.3: Integration test - Password verification

- [ ] Add to `backend/crates/altair-db/tests/auth_integration.rs`
  - **Acceptance**:
    - Test: Register with password → logout → login with correct password succeeds
    - Test: Login with wrong password fails with generic error (no user enumeration)
  - **Files**: `backend/crates/altair-db/tests/auth_integration.rs`
  - **Duration**: 45 minutes
  - **Verification**: Test passes with correct error handling

### Task 4.4: Integration test - Passwordless auth

- [ ] Add to `backend/crates/altair-db/tests/auth_integration.rs`
  - **Acceptance**:
    - Test: Register without password → logout → login without password succeeds
    - Verify no credential record created
  - **Files**: `backend/crates/altair-db/tests/auth_integration.rs`
  - **Duration**: 30 minutes
  - **Verification**: Test passes with no credentials

### Task 4.5: Integration test - Session expiry

- [ ] Add to `backend/crates/altair-db/tests/auth_integration.rs`
  - **Acceptance**:
    - Test: Create session with past expiry → validate returns expired error
    - Test: Refresh extends expiry correctly
  - **Files**: `backend/crates/altair-db/tests/auth_integration.rs`
  - **Duration**: 30 minutes
  - **Verification**: Test passes with expiry logic

### Task 4.6: Integration test - Preferences CRUD

- [ ] Add to `backend/crates/altair-db/tests/auth_integration.rs`
  - **Acceptance**:
    - Test: Register → update_prefs → get_user → verify preferences merged
  - **Files**: `backend/crates/altair-db/tests/auth_integration.rs`
  - **Duration**: 30 minutes
  - **Verification**: Test passes with preference merging

### Task 4.7: Performance benchmarks

- [ ] Create `backend/crates/altair-auth/benches/auth_bench.rs`
  - **Acceptance**:
    - Password hashing: < 500ms (Argon2id parameters)
    - Session validation: < 5ms overhead
    - Token generation: < 1ms
  - **Files**: `backend/crates/altair-auth/benches/auth_bench.rs`
  - **Duration**: 1 hour
  - **Verification**: All benchmarks meet targets

### Task 4.8: Security tests

- [ ] Create `backend/crates/altair-auth/tests/security.rs`
  - **Acceptance**:
    - Timing attack resistance: verify_password takes same time for valid/invalid
    - Error messages don't reveal which field was wrong
    - Credentials never appear in logs (add tracing filters)
  - **Files**: `backend/crates/altair-auth/tests/security.rs`
  - **Duration**: 1 hour
  - **Verification**: Security tests pass

### Task 4.9: Keychain fallback test

- [ ] Add to `backend/crates/altair-auth/tests/security.rs`
  - **Acceptance**:
    - Simulate keychain unavailable
    - Verify specific `KeychainUnavailable` error returned for UI handling
  - **Files**: `backend/crates/altair-auth/tests/security.rs`
  - **Duration**: 30 minutes
  - **Verification**: Test passes with correct error variant

---

## Summary

**Total Tasks**: 33 atomic tasks across 4 phases
**Estimated Duration**: ~22 hours of implementation time

### Dependency Order

1. Phase 1 (Tasks 1.1-1.7) can proceed independently
2. Phase 2 (Tasks 2.1-2.7) can proceed in parallel with Phase 1
3. Phase 3 (Tasks 3.1-3.10) requires Phase 1 + Phase 2 complete
4. Phase 4 (Tasks 4.1-4.9) requires all prior phases

### Critical Path

Phase 1 → Phase 3 → Phase 4 (Phase 2 can run parallel to Phase 1)

### Verification Checklist

- [ ] All unit tests pass: `cargo test`
- [ ] All integration tests pass: `cargo test --test auth_integration`
- [ ] Performance benchmarks meet targets: `cargo bench`
- [ ] Security tests pass: `cargo test security`
- [ ] Manual keychain test on dev machine works
- [ ] Migration applies to fresh DB cleanly

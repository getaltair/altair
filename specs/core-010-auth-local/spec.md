# Specification: Local Authentication and User Management

<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<!--
SPEC WEIGHT: [ ] LIGHTWEIGHT  [x] STANDARD  [ ] FORMAL

Weight Guidelines:
- LIGHTWEIGHT: Bug fixes, small enhancements, config changes (<2 days work)
  Required sections: Quick Reference, Problem/Solution, Requirements, Acceptance Criteria

- STANDARD: New features, integrations, moderate complexity (2-10 days work)
  Required sections: All LIGHTWEIGHT + Data Model, Security, Test Requirements

- FORMAL: Major systems, compliance-sensitive, cross-team impact (>10 days work)
  Required sections: All sections, full sign-off
-->
<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->

**Spec ID**: core-010-auth-local
**Component**: CORE
**Weight**: STANDARD
**Version**: 1.0
**Status**: DRAFT
**Created**: 2025-12-07
**Author**: Robert Hamilton

---

## Quick Reference

> Local authentication provides single-user identity and session management for the Altair desktop application, enabling secure data ownership and preference storage without requiring external identity providers.

**What**: Local user creation with password-based authentication, session management, and user preferences storage.
**Why**: Desktop apps need user identity for data ownership, sync authorization, and personalized preferences—without requiring cloud accounts.
**Impact**: Users can securely log in to their local Altair instance, with all data properly owned and preferences persisted.

**Success Metrics**:

| Metric                          | Target    | How Measured                      |
| ------------------------------- | --------- | --------------------------------- |
| User creation success rate      | 100%      | Unit tests for all creation paths |
| Password verification latency   | < 200ms   | Performance benchmark             |
| Session validation overhead     | < 5ms     | Per-request timing                |
| Auth trait implementation count | 1 (local) | Plugin registry inspection        |

---

## Problem Statement

### Current State

The Altair backend skeleton (core-003) provides application infrastructure but lacks user identity management. Currently:

- No user entity exists at runtime (schema defines it but no creation/management)
- No way to authenticate who is using the application
- No session management for maintaining authenticated state
- No secure storage for user credentials
- No mechanism to associate data (quests, notes, items) with an owner
- User preferences defined in schema but no way to persist/retrieve them

Without authentication, the application cannot enforce data ownership rules, cannot track user preferences, and cannot prepare for future multi-device sync (which requires user identity).

### Desired State

After implementing local authentication:

- A single-user account is automatically created on first launch (or through explicit setup)
- User can optionally set a password to protect their data
- Sessions persist across app restarts with secure token storage
- User preferences are stored and retrieved from the user entity
- All created entities (Quest, Note, Item, etc.) are automatically associated with the authenticated user
- The auth system is built as a plugin, allowing future OAuth/OIDC providers (core-040)
- Password hashing uses Argon2id for cryptographic security

### Why Now

Authentication is foundational infrastructure that other features depend on:

- **Gamification (platform-010)** needs user identity to track XP and achievements
- **Sync engine (core-013)** needs user identity for cloud authorization
- **Data ownership** must be established before creating production data
- **Early implementation** allows all subsequent features to build with proper user context

---

## Solution Overview

### Approach

Implement a plugin-based authentication system with `LocalAuthProvider` as the default (and initially only) provider. The architecture follows the `AuthProvider` trait defined in the technical architecture document, enabling future extensibility.

The local auth flow:

1. On first launch, detect no user exists and prompt for setup
2. Create user with email (identifier) and optional password
3. Hash password with Argon2id if provided, store securely
4. Generate session token, store in OS keychain
5. On subsequent launches, validate session or re-authenticate
6. Provide `validate()` method for request-level auth checks

### Scope

**In Scope**:

- `AuthProvider` trait definition with core methods
- `LocalAuthProvider` implementation
- User creation flow (single-user default)
- Password hashing with Argon2id
- Session token generation and validation
- Session persistence in OS keychain (via `keyring` crate)
- User preferences CRUD operations
- Tauri commands for auth operations
- Integration with existing `AppState`

**Out of Scope**:

- OAuth providers (Google, GitHub) — tracked in core-040
- Multi-user support / account switching — future consideration
- Password reset flow (no email service for local-only)
- Two-factor authentication — future consideration
- Cloud-based session sync — depends on core-013

**Future Considerations**:

- OAuth plugin registration once core-040 is implemented
- Session sync across devices when sync engine is ready
- Biometric authentication on mobile (core-030)

### Key Decisions

| Decision                          | Options Considered                        | Rationale                                                      |
| --------------------------------- | ----------------------------------------- | -------------------------------------------------------------- |
| Argon2id for password hashing     | bcrypt, scrypt, Argon2id                  | Argon2id is the current recommendation, resistant to GPU/ASIC  |
| OS keychain for session storage   | File-based, encrypted SQLite, OS keychain | OS keychain provides best security with minimal implementation |
| Optional password for single-user | Required password, no password, optional  | Optional balances security with convenience for local-only use |
| Plugin trait architecture         | Hardcoded local auth, plugin system       | Plugin allows future OAuth without refactoring                 |

---

## Requirements

### Functional Requirements

| ID     | Requirement                                                                                      | Priority | Notes                        |
| ------ | ------------------------------------------------------------------------------------------------ | -------- | ---------------------------- |
| FR-001 | System shall define an `AuthProvider` trait with authenticate, validate, refresh, logout methods | CRITICAL |                              |
| FR-002 | System shall implement `LocalAuthProvider` as the default auth plugin                            | CRITICAL |                              |
| FR-003 | System shall create a user account on first launch if none exists                                | CRITICAL | Single-user default          |
| FR-004 | System shall hash passwords using Argon2id with secure parameters                                | CRITICAL | Cost=3, Memory=64MB          |
| FR-005 | System shall generate cryptographically secure session tokens                                    | CRITICAL | 256-bit random tokens        |
| FR-006 | System shall store session tokens in OS-native keychain                                          | HIGH     | Uses `keyring` crate         |
| FR-007 | System shall validate sessions without requiring password on each request                        | CRITICAL |                              |
| FR-008 | System shall provide user preferences CRUD through authenticated context                         | HIGH     |                              |
| FR-009 | System shall expose Tauri commands for login, logout, register, get_current_user                 | CRITICAL |                              |
| FR-010 | System shall support optional (no password) authentication for local-only use                    | MEDIUM   | Password can be empty/none   |
| FR-011 | System shall invalidate sessions on explicit logout                                              | HIGH     |                              |
| FR-012 | System shall refresh session tokens before expiration                                            | MEDIUM   | 7-day default, 1-day refresh |

### Non-Functional Requirements

| ID      | Requirement                                             | Priority | Notes                       |
| ------- | ------------------------------------------------------- | -------- | --------------------------- |
| NFR-001 | Password hashing shall complete in < 500ms              | HIGH     | Argon2 tuned for UX         |
| NFR-002 | Session validation shall add < 5ms overhead per request | HIGH     | Critical for responsiveness |
| NFR-003 | Auth system shall work offline without network          | CRITICAL | Local-first requirement     |
| NFR-004 | Credentials shall never be logged or exposed in errors  | CRITICAL | Security requirement        |
| NFR-005 | System shall be extensible to additional auth providers | MEDIUM   | Plugin architecture         |

### User Stories

**US-001: First-Time Setup**

- **As** a new Altair user,
- **I** need to
  - Create my local account on first launch
  - Optionally set a password for protection
- **so** that I can start using the app with my data securely owned.

Acceptance:

- [ ] First launch detects no existing user
- [ ] Setup wizard prompts for display name and optional password
- [ ] Account is created and session established
- [ ] Subsequent launches skip setup

Independent Test: Fresh database → launch app → verify setup flow → verify user exists

**US-002: Authenticated Session**

- **As** a returning Altair user,
- **I** need to
  - Have my session automatically restored on app launch
  - Re-authenticate only when session expires or I explicitly log out
- **so** that I don't need to enter credentials repeatedly.

Acceptance:

- [ ] Session token retrieved from keychain on launch
- [ ] Valid session skips login, provides user context
- [ ] Expired session prompts for re-authentication
- [ ] Invalid session clears and prompts for login

Independent Test: Create session → close app → relaunch → verify auto-login

**US-003: Secure Logout**

- **As** an Altair user,
- **I** need to
  - Log out and clear my session
  - Have my credentials protected when I'm away
- **so** that others cannot access my data.

Acceptance:

- [ ] Logout command clears session from keychain
- [ ] App transitions to login state
- [ ] Protected data inaccessible until re-auth

Independent Test: Login → logout → verify session cleared → verify data access denied

**US-004: User Preferences**

- **As** an Altair user,
- **I** need to
  - Store my preferences (theme, focus duration, etc.)
  - Have them persist across sessions
- **so** that my experience is personalized.

Acceptance:

- [ ] Preferences stored on user entity
- [ ] Preferences retrieved on login
- [ ] Preference updates persist immediately
- [ ] Defaults applied for unset preferences

Independent Test: Set preference → close app → relaunch → verify preference retained

---

## Data Model

### Key Entities

- **User**: Identity with email, display name, preferences, role. Owns all domain entities.
- **Session**: Authentication state with token, expiration, device identifier.
- **UserCredential** (internal): Hashed password and salt, never exposed externally.

### Entity Details

**User**

- **Purpose**: Represents the authenticated identity owning all data in Altair
- **Key Attributes**:
  - `email` (unique identifier for the user)
  - `display_name` (shown in UI)
  - `role` (owner or viewer, defaults to owner)
  - `preferences` (flexible object for user settings)
  - `created_at`, `updated_at` (timestamps)
- **Relationships**: Owns Campaign, Quest, Note, Folder, Item, Location, Capture, UserProgress
- **Lifecycle**: Created once on setup, updated when preferences change, never deleted (archived)
- **Business Rules**:
  - Single user in local mode (multi-user requires cloud)
  - Email must be valid format (can be placeholder for local-only)
  - Role defaults to 'owner' for first user

**Session**

- **Purpose**: Tracks active authentication state for the user
- **Key Attributes**:
  - `token` (256-bit random, hex-encoded)
  - `user` (reference to User)
  - `expires_at` (datetime, default 7 days from creation)
  - `device_id` (identifies the device/app instance)
  - `created_at` (when session started)
- **Relationships**: Belongs to User
- **Lifecycle**: Created on login, refreshed periodically, deleted on logout or expiration
- **Business Rules**:
  - Only one active session per device
  - Session refresh extends expiration, doesn't create new token
  - Logout invalidates session immediately

**UserCredential** (Internal)

- **Purpose**: Securely stores password hash, never exposed via API
- **Key Attributes**:
  - `user` (reference to User)
  - `password_hash` (Argon2id output including salt)
  - `updated_at` (last password change)
- **Relationships**: One-to-one with User
- **Lifecycle**: Created with user, updated on password change
- **Business Rules**:
  - Never returned in any API response
  - Hash includes algorithm parameters for future-proofing
  - Null/empty hash means passwordless auth

### State Transitions

```
Session States:
CREATED → ACTIVE → EXPIRED
            ↓
         INVALIDATED (logout)
```

**Transition Rules**:

- CREATED → ACTIVE: Automatic on creation
- ACTIVE → EXPIRED: When `expires_at` passes current time
- ACTIVE → INVALIDATED: On explicit logout call
- EXPIRED/INVALIDATED: Cannot transition back, new session required

---

## Interfaces

### Operations

**authenticate**

- **Purpose**: Verify user credentials and establish session
- **Trigger**: User submits login form or passwordless auth
- **Inputs**:
  - `email` (required): User identifier
  - `password` (optional): User password, empty for passwordless
- **Outputs**: Session token and user profile on success
- **Behavior**:
  - Look up user by email
  - If password set, verify against stored hash
  - If no password set, allow passwordless auth
  - Generate session token, store in DB and keychain
  - Return session token and basic user info
- **Error Conditions**:
  - User not found: Return generic "invalid credentials"
  - Password mismatch: Return generic "invalid credentials"
  - Account locked: Return "account unavailable" (future)

**validate**

- **Purpose**: Verify session token is valid for authenticated operations
- **Trigger**: Every authenticated Tauri command
- **Inputs**:
  - `token` (required): Session token from keychain
- **Outputs**: User entity if valid, error if not
- **Behavior**:
  - Look up session by token
  - Check expiration
  - Return associated user
- **Error Conditions**:
  - Token not found: Session invalid
  - Token expired: Session expired, needs re-auth
  - User deleted: Session orphaned, invalid

**refresh**

- **Purpose**: Extend session expiration before it expires
- **Trigger**: Background job when session approaches expiration
- **Inputs**:
  - `token` (required): Current session token
- **Outputs**: Updated session with new expiration
- **Behavior**:
  - Validate current token
  - Update `expires_at` to 7 days from now
  - Return updated session
- **Error Conditions**:
  - Invalid token: Cannot refresh, re-auth required

**logout**

- **Purpose**: Invalidate current session
- **Trigger**: User clicks logout or closes app with "clear session" setting
- **Inputs**:
  - `token` (required): Session token to invalidate
- **Outputs**: Confirmation of logout
- **Behavior**:
  - Delete session from database
  - Remove token from keychain
  - Return success
- **Error Conditions**:
  - Token not found: Still return success (idempotent)

**register** (Initial Setup)

- **Purpose**: Create first user account
- **Trigger**: First app launch or explicit setup
- **Inputs**:
  - `email` (required): User email/identifier
  - `display_name` (optional): Display name
  - `password` (optional): Password to set
- **Outputs**: Created user and initial session
- **Behavior**:
  - Verify no user exists (single-user mode)
  - Create user entity
  - If password provided, hash and store credential
  - Create initial session
  - Store session token in keychain
- **Error Conditions**:
  - User already exists: Return error (single-user mode)
  - Invalid email format: Return validation error

**get_current_user**

- **Purpose**: Retrieve authenticated user's profile
- **Trigger**: App initialization, profile display
- **Inputs**:
  - `token` (required): Valid session token
- **Outputs**: User profile with preferences
- **Behavior**:
  - Validate session
  - Return user entity with preferences
- **Error Conditions**:
  - Invalid session: Return auth error

**update_preferences**

- **Purpose**: Modify user preferences
- **Trigger**: Settings changes
- **Inputs**:
  - `token` (required): Valid session token
  - `preferences` (required): Partial preferences object to merge
- **Outputs**: Updated user with new preferences
- **Behavior**:
  - Validate session
  - Merge preferences (not replace)
  - Update user entity
- **Error Conditions**:
  - Invalid session: Return auth error
  - Invalid preference key: Return validation error

### Tauri Commands

| Command             | Operation            | Returns                |
| ------------------- | -------------------- | ---------------------- |
| `auth_register`     | register             | `Result<AuthResponse>` |
| `auth_login`        | authenticate         | `Result<AuthResponse>` |
| `auth_logout`       | logout               | `Result<()>`           |
| `auth_validate`     | validate             | `Result<User>`         |
| `auth_refresh`      | refresh              | `Result<Session>`      |
| `auth_get_user`     | get_current_user     | `Result<User>`         |
| `auth_update_prefs` | update_preferences   | `Result<User>`         |
| `auth_check_setup`  | (checks user exists) | `Result<bool>`         |

---

## Security and Compliance

### Authorization

| Operation          | Required Permission | Notes                          |
| ------------------ | ------------------- | ------------------------------ |
| register           | None (first-time)   | Only works when no user exists |
| authenticate       | None                | Entry point                    |
| validate           | Valid session       | Token from keychain            |
| refresh            | Valid session       | Token must not be expired      |
| logout             | Valid session       | Clears own session             |
| get_current_user   | Valid session       | Returns own data only          |
| update_preferences | Valid session       | Updates own preferences        |

### Data Classification

| Data Element  | Classification | Handling Requirements                       |
| ------------- | -------------- | ------------------------------------------- |
| Email         | Internal       | Stored in DB, shown in UI                   |
| Password      | Confidential   | Never stored plain, only Argon2id hash      |
| Session Token | Confidential   | Stored in OS keychain, transmitted securely |
| Preferences   | Internal       | Stored in DB, no special handling           |

### Security Requirements

**Password Storage**:

- Argon2id with parameters: time=3, memory=65536 (64MB), parallelism=4
- Salt: 16 bytes random per password
- Output: 32 bytes hash
- Full PHC string format stored for algorithm agility

**Session Tokens**:

- Generated with `getrandom` crate (OS entropy)
- 256 bits (32 bytes), hex-encoded (64 chars)
- 7-day expiration default
- Single-use: logout invalidates immediately

**Keychain Storage**:

- Service name: `com.altair.auth`
- Account: `session_token`
- Secret: Raw token value
- Access: App-only (OS enforced)

### Audit Requirements

- Events to log: login success, login failure, logout, session refresh, password change
- Retention: Local logs, 30 days
- Access: Debug purposes only, no external transmission

---

## Test Requirements

### Success Criteria

| ID     | Criterion                                              | Measurement        |
| ------ | ------------------------------------------------------ | ------------------ |
| SC-001 | User can complete registration in under 3 seconds      | End-to-end timing  |
| SC-002 | Password verification completes in under 500ms         | Benchmark test     |
| SC-003 | Session validation adds < 5ms overhead                 | Per-request timing |
| SC-004 | Invalid credentials never reveal which field was wrong | Security review    |
| SC-005 | Session persists across app restarts                   | Integration test   |
| SC-006 | Logout completely clears session from all storage      | Integration test   |

### Acceptance Criteria

**Scenario**: First-time user registration _(maps to US-001)_

```gherkin
Given no user exists in the database
When the user opens the app for the first time
Then they see the setup wizard
And they can enter email and optional password
And clicking "Create Account" creates the user and logs them in
```

**Scenario**: Returning user auto-login _(maps to US-002)_

```gherkin
Given a user has previously logged in
And their session token is in the keychain
And the session is not expired
When they open the app
Then they are automatically logged in
And they see their dashboard without login prompt
```

**Scenario**: Password authentication _(maps to US-002)_

```gherkin
Given a user with password "secret123"
When they enter email and password "secret123"
Then they are logged in successfully
And a session token is stored in the keychain
```

**Scenario**: Invalid password rejection _(maps to US-002)_

```gherkin
Given a user with password "secret123"
When they enter email and password "wrongpassword"
Then they see "Invalid credentials"
And no session is created
And the error does not reveal which field was wrong
```

**Scenario**: Secure logout _(maps to US-003)_

```gherkin
Given a logged-in user
When they click logout
Then their session is invalidated
And the session token is removed from keychain
And they see the login screen
```

### Test Scenarios

| ID     | Scenario                              | Type        | Priority | Maps To |
| ------ | ------------------------------------- | ----------- | -------- | ------- |
| TS-001 | Register first user                   | Functional  | CRITICAL | US-001  |
| TS-002 | Login with password                   | Functional  | CRITICAL | US-002  |
| TS-003 | Login without password (passwordless) | Functional  | HIGH     | FR-010  |
| TS-004 | Auto-login with valid session         | Functional  | CRITICAL | US-002  |
| TS-005 | Reject expired session                | Functional  | CRITICAL | FR-007  |
| TS-006 | Logout clears session                 | Functional  | HIGH     | US-003  |
| TS-007 | Password hashing performance          | Performance | HIGH     | NFR-001 |
| TS-008 | Session validation performance        | Performance | HIGH     | NFR-002 |
| TS-009 | Invalid password timing attack resist | Security    | HIGH     | NFR-004 |
| TS-010 | Keychain storage security             | Security    | CRITICAL | FR-006  |
| TS-011 | Preferences CRUD                      | Functional  | MEDIUM   | US-004  |

### Performance Criteria

| Operation            | Metric   | Target  | Conditions            |
| -------------------- | -------- | ------- | --------------------- |
| Password hashing     | Duration | < 500ms | Argon2id, 64MB mem    |
| Session validation   | Overhead | < 5ms   | Per authenticated req |
| User lookup by email | Duration | < 10ms  | Indexed query         |
| Token generation     | Duration | < 1ms   | OS random source      |

---

## Constraints and Assumptions

### Technical Constraints

- **Single user mode**: Local auth supports only one user per installation
- **OS keychain dependency**: Requires functioning keychain (libsecret on Linux, Keychain on macOS)
- **No password reset**: No email service means no "forgot password" flow
- **Argon2 memory**: Requires 64MB RAM for password hashing

### Business Constraints

- **Privacy first**: No telemetry, no external auth calls, fully local
- **Offline capable**: Must work without network connectivity

### Assumptions

- User has a functioning OS keychain/credential manager
- Single-user per device is acceptable for MVP
- Users will not need to share Altair data between accounts on same device
- 7-day session expiration is acceptable (can be tuned)

### Dependencies

| Dependency         | Type     | Status   | Impact if Delayed          |
| ------------------ | -------- | -------- | -------------------------- |
| core-002 (schema)  | Internal | Complete | User table already defined |
| core-003 (backend) | Internal | Complete | AppState ready             |
| `argon2` crate     | External | Stable   | Use alternative hasher     |
| `keyring` crate    | External | Stable   | Fallback to encrypted file |

### Risks

| Risk                               | Likelihood | Impact | Mitigation                                             |
| ---------------------------------- | ---------- | ------ | ------------------------------------------------------ |
| Keychain unavailable on some Linux | Medium     | Medium | Encrypted file fallback                                |
| Argon2 too slow on weak hardware   | Low        | Low    | Tunable parameters, document recommendations           |
| Session token collision            | Very Low   | High   | 256-bit token space, collision astronomically unlikely |

---

## Open Questions

| #   | Question                                                | Location | Owner | Due         | Status |
| --- | ------------------------------------------------------- | -------- | ----- | ----------- | ------ |
| 1   | Should session auto-refresh or require explicit action? | FR-012   | Dev   | During plan | OPEN   |
| 2   | Keychain fallback behavior if unavailable               | FR-006   | Dev   | During plan | OPEN   |

### Clarifications Log

| Date | Question | Resolution | Decided By |
| ---- | -------- | ---------- | ---------- |
|      |          |            |            |

---

## References

### Internal

- [Technical Architecture - Auth Section](../docs/technical-architecture.md#authentication-plugin-based)
- [Domain Model - User Entity](../docs/domain-model.md#shared-domain)
- [core-002 Schema Migrations](./core-002-schema-migrations/spec.md)
- [core-003 Backend Skeleton](./core-003-backend-skeleton/spec.md)

### External

- [Argon2 IETF RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html)
- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- [keyring-rs crate](https://crates.io/crates/keyring)

---

## Approval

| Role             | Name            | Date | Status  |
| ---------------- | --------------- | ---- | ------- |
| Author           | Robert Hamilton |      | DRAFT   |
| Technical Review |                 |      | PENDING |

---

## Changelog

| Version | Date       | Author          | Changes               |
| ------- | ---------- | --------------- | --------------------- |
| 1.0     | 2025-12-07 | Robert Hamilton | Initial specification |

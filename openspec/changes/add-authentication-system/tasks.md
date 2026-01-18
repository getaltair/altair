# Implementation Tasks

## 1. Server Foundation

### 1.1 Add Dependencies
- [x] 1.1.1 Add `argon2-jvm` dependency to `server/build.gradle.kts`
- [x] 1.1.2 Add `java-jwt` (auth0) if not already present
- [x] 1.1.3 Verify Ktor auth dependencies are configured

### 1.2 Password Service
- [x] 1.2.1 Create `PasswordService` interface in shared module
- [x] 1.2.2 Implement `Argon2PasswordService` in server module
- [x] 1.2.3 Write unit tests for password hashing and verification
- [x] 1.2.4 Add Koin module registration

### 1.3 JWT Token Service
- [x] 1.3.1 Create `JwtTokenService` interface
- [x] 1.3.2 Implement token generation with configurable secret/expiration
- [x] 1.3.3 Implement token validation and claims extraction
- [x] 1.3.4 Create `JwtConfig` data class for configuration
- [x] 1.3.5 Load JWT config from environment variables
- [x] 1.3.6 Write unit tests for token lifecycle

## 2. Database Layer

### 2.1 User Table Updates
- [x] 2.1.1 Add `password_hash` field to User domain model (via UserWithCredentials wrapper)
- [x] 2.1.2 Create migration to add `password_hash` column (V2__authentication_tables.surql)
- [x] 2.1.3 Update `SurrealUserRepository` to handle password hash
- [x] 2.1.4 Add `findByEmailWithPassword()` method for login (findByEmailWithCredentials)

### 2.2 Refresh Token Repository
- [x] 2.2.1 Create `RefreshToken` domain model in shared module
- [x] 2.2.2 Create `RefreshTokenRepository` interface
- [x] 2.2.3 Create migration for `refresh_token` table (V2__authentication_tables.surql)
- [x] 2.2.4 Implement `SurrealRefreshTokenRepository`
- [x] 2.2.5 Write integration tests (covered via AuthIntegrationTest - token refresh, rotation, expiration)

### 2.3 Invite Code Repository
- [x] 2.3.1 Create `InviteCode` domain model in shared module
- [x] 2.3.2 Create `InviteCodeRepository` interface
- [x] 2.3.3 Create migration for `invite_code` table (V2__authentication_tables.surql)
- [x] 2.3.4 Implement `SurrealInviteCodeRepository`
- [x] 2.3.5 Write integration tests (covered via AuthIntegrationTest - valid/invalid/expired invite codes)

## 3. Auth Service Implementation

### 3.1 Replace Stub Implementation
- [x] 3.1.1 Implement `AuthServiceImpl.login()` with real credential validation
- [x] 3.1.2 Implement `AuthServiceImpl.register()` with invite code validation
- [x] 3.1.3 Implement `AuthServiceImpl.refresh()` with token rotation
- [x] 3.1.4 Implement `AuthServiceImpl.logout()` with token revocation (placeholder - requires AuthContext)
- [x] 3.1.5 Handle first-user bootstrap (admin without invite)

### 3.2 Auth Service Extensions
- [x] 3.2.1 Add `generateInviteCode()` to `AuthService` interface
- [x] 3.2.2 Add `changePassword()` to `AuthService` interface
- [x] 3.2.3 Add `revokeAllSessions()` to `AuthService` interface
- [x] 3.2.4 Implement server-side methods (placeholders for changePassword/revokeAllSessions - require AuthContext)
- [x] 3.2.5 Update DTOs in shared module

### 3.3 Integration Tests
Tests created in `AuthIntegrationTest.kt`. All tests passing.
- [x] 3.3.1 Test login with valid credentials
- [x] 3.3.2 Test login with invalid credentials
- [x] 3.3.3 Test registration with valid invite code
- [x] 3.3.4 Test registration without invite (first user)
- [x] 3.3.5 Test registration with invalid/expired invite
- [x] 3.3.6 Test token refresh flow
- [x] 3.3.7 Test logout and session invalidation

## 4. User Scoping Middleware

### 4.1 Auth Context
- [x] 4.1.1 Create `AuthContext` interface with `currentUserId` property
- [x] 4.1.2 Create `RequestAuthContext` implementation for Ktor
- [x] 4.1.3 Create `TestAuthContext` for unit testing

### 4.2 Ktor Middleware
- [x] 4.2.1 Update `Security.kt` to use real JWT validation
- [x] 4.2.2 Extract user claims and populate `AuthContext`
- [x] 4.2.3 Wire auth context into Koin request scope
- [x] 4.2.4 Update RPC endpoints to require authentication (except login/register)
  - Note: Split into `/rpc/auth` (public) and `/rpc` (authenticated) endpoints
  - AuthService methods requiring user context have limited functionality until kotlinx-rpc adds context support

### 4.3 Repository Integration
Tests created in `UserScopeIntegrationTest.kt`. All tests passing.
- [x] 4.3.1 Verify user-scoped repositories receive `AuthContext`
- [x] 4.3.2 Add integration test for cross-user data isolation
- [x] 4.3.3 Ensure RPC services use authenticated user context

## 5. Client Authentication

### 5.1 Secure Token Storage
- [x] 5.1.1 Create `SecureTokenStorage` interface in commonMain
- [x] 5.1.2 Implement Android: `AndroidSecureTokenStorage` (EncryptedSharedPreferences)
- [x] 5.1.3 Implement iOS: `IosSecureTokenStorage` (Keychain)
- [x] 5.1.4 Implement Desktop: `DesktopSecureTokenStorage` (AES-GCM encrypted preferences)
- [ ] 5.1.5 Write platform tests for each implementation

### 5.2 Auth Manager
- [x] 5.2.1 Create `AuthManager` class for token lifecycle
- [x] 5.2.2 Implement automatic token refresh before expiration
- [x] 5.2.3 Implement session state (logged in/out/expired)
- [x] 5.2.4 Add Koin module registration (authModule + platform-specific modules)
- [x] 5.2.5 Integrate with RPC client for auth headers (TokenProvider in RpcClientFactory)

### 5.3 Auth State in Koin
- [x] 5.3.1 Create `AuthState` sealed class (Authenticated, Unauthenticated, Loading) - In AuthManager.kt
- [x] 5.3.2 Create `AuthStateHolder` with StateFlow - AuthManager.authState StateFlow
- [x] 5.3.3 Wire into app-level Koin scope (via authModule and platform modules)
- [x] 5.3.4 Add navigation guard for authenticated routes (RootComponent observes authState)

## 6. UI Screens

### 6.1 Login Screen
- [x] 6.1.1 Create `LoginComponent` with Decompose
- [x] 6.1.2 Create `LoginScreen` composable
- [x] 6.1.3 Implement email/password input validation
- [x] 6.1.4 Handle login errors with user feedback
- [x] 6.1.5 Navigate to home on successful login

### 6.2 Registration Screen
- [x] 6.2.1 Create `RegisterComponent` with Decompose
- [x] 6.2.2 Create `RegisterScreen` composable
- [x] 6.2.3 Implement invite code field
- [x] 6.2.4 Implement password confirmation
- [x] 6.2.5 Handle registration errors

### 6.3 Navigation Integration
- [x] 6.3.1 Add `Config.Login` and `Config.Register` to navigation
- [x] 6.3.2 Update `RootComponent` to check auth state on startup
- [x] 6.3.3 Redirect unauthenticated users to login
- [x] 6.3.4 Redirect authenticated users away from login/register

## 7. Verification

### 7.1 Security Review
- [x] 7.1.1 Verify password hash is never logged or exposed (verified: only "invalid password" logged, no hash values)
- [x] 7.1.2 Verify tokens are stored securely on each platform (verified: AES-GCM/Keychain/EncryptedSharedPreferences)
- [x] 7.1.3 Verify JWT secret is loaded from environment, not hardcoded (verified: `JwtConfig.fromEnvironment()`)
- [x] 7.1.4 Verify user data isolation in repositories (verified: all queries filter by `user_id`)

### 7.2 End-to-End Testing
- [ ] 7.2.1 Test full registration flow on Android
- [ ] 7.2.2 Test full login flow on Desktop
- [ ] 7.2.3 Test token refresh after expiration
- [ ] 7.2.4 Test logout clears stored credentials
- [ ] 7.2.5 Test cross-user data isolation via API

### 7.3 Documentation
- [x] 7.3.1 Update CLAUDE.md with auth patterns (added Authentication Architecture section)
- [x] 7.3.2 Document environment variables for JWT configuration (documented in CLAUDE.md)
- [~] 7.3.3 Update docker-compose example with JWT secret (N/A - no docker-compose file exists yet)

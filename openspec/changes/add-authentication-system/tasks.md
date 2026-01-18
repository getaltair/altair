# Implementation Tasks

## 1. Server Foundation

### 1.1 Add Dependencies
- [ ] 1.1.1 Add `argon2-jvm` dependency to `server/build.gradle.kts`
- [ ] 1.1.2 Add `java-jwt` (auth0) if not already present
- [ ] 1.1.3 Verify Ktor auth dependencies are configured

### 1.2 Password Service
- [ ] 1.2.1 Create `PasswordService` interface in shared module
- [ ] 1.2.2 Implement `Argon2PasswordService` in server module
- [ ] 1.2.3 Write unit tests for password hashing and verification
- [ ] 1.2.4 Add Koin module registration

### 1.3 JWT Token Service
- [ ] 1.3.1 Create `JwtTokenService` interface
- [ ] 1.3.2 Implement token generation with configurable secret/expiration
- [ ] 1.3.3 Implement token validation and claims extraction
- [ ] 1.3.4 Create `JwtConfig` data class for configuration
- [ ] 1.3.5 Load JWT config from environment variables
- [ ] 1.3.6 Write unit tests for token lifecycle

## 2. Database Layer

### 2.1 User Table Updates
- [ ] 2.1.1 Add `password_hash` field to User domain model
- [ ] 2.1.2 Create migration to add `password_hash` column
- [ ] 2.1.3 Update `SurrealUserRepository` to handle password hash
- [ ] 2.1.4 Add `findByEmailWithPassword()` method for login

### 2.2 Refresh Token Repository
- [ ] 2.2.1 Create `RefreshToken` domain model in shared module
- [ ] 2.2.2 Create `RefreshTokenRepository` interface
- [ ] 2.2.3 Create migration for `refresh_token` table
- [ ] 2.2.4 Implement `SurrealRefreshTokenRepository`
- [ ] 2.2.5 Write integration tests

### 2.3 Invite Code Repository
- [ ] 2.3.1 Create `InviteCode` domain model in shared module
- [ ] 2.3.2 Create `InviteCodeRepository` interface
- [ ] 2.3.3 Create migration for `invite_code` table
- [ ] 2.3.4 Implement `SurrealInviteCodeRepository`
- [ ] 2.3.5 Write integration tests

## 3. Auth Service Implementation

### 3.1 Replace Stub Implementation
- [ ] 3.1.1 Implement `AuthServiceImpl.login()` with real credential validation
- [ ] 3.1.2 Implement `AuthServiceImpl.register()` with invite code validation
- [ ] 3.1.3 Implement `AuthServiceImpl.refresh()` with token rotation
- [ ] 3.1.4 Implement `AuthServiceImpl.logout()` with token revocation
- [ ] 3.1.5 Handle first-user bootstrap (admin without invite)

### 3.2 Auth Service Extensions
- [ ] 3.2.1 Add `generateInviteCode()` to `AuthService` interface
- [ ] 3.2.2 Add `changePassword()` to `AuthService` interface
- [ ] 3.2.3 Add `revokeAllSessions()` to `AuthService` interface
- [ ] 3.2.4 Implement server-side methods
- [ ] 3.2.5 Update DTOs in shared module

### 3.3 Integration Tests
- [ ] 3.3.1 Test login with valid credentials
- [ ] 3.3.2 Test login with invalid credentials
- [ ] 3.3.3 Test registration with valid invite code
- [ ] 3.3.4 Test registration without invite (first user)
- [ ] 3.3.5 Test registration with invalid/expired invite
- [ ] 3.3.6 Test token refresh flow
- [ ] 3.3.7 Test logout and session invalidation

## 4. User Scoping Middleware

### 4.1 Auth Context
- [ ] 4.1.1 Create `AuthContext` interface with `currentUserId` property
- [ ] 4.1.2 Create `RequestAuthContext` implementation for Ktor
- [ ] 4.1.3 Create `TestAuthContext` for unit testing

### 4.2 Ktor Middleware
- [ ] 4.2.1 Update `Security.kt` to use real JWT validation
- [ ] 4.2.2 Extract user claims and populate `AuthContext`
- [ ] 4.2.3 Wire auth context into Koin request scope
- [ ] 4.2.4 Update RPC endpoints to require authentication (except login/register)

### 4.3 Repository Integration
- [ ] 4.3.1 Verify user-scoped repositories receive `AuthContext`
- [ ] 4.3.2 Add integration test for cross-user data isolation
- [ ] 4.3.3 Ensure RPC services use authenticated user context

## 5. Client Authentication

### 5.1 Secure Token Storage
- [ ] 5.1.1 Create `SecureTokenStorage` interface in commonMain
- [ ] 5.1.2 Implement Android: `EncryptedSharedPreferencesTokenStorage`
- [ ] 5.1.3 Implement iOS: `KeychainTokenStorage`
- [ ] 5.1.4 Implement Desktop: `KeyStoreTokenStorage`
- [ ] 5.1.5 Write platform tests for each implementation

### 5.2 Auth Manager
- [ ] 5.2.1 Create `AuthManager` class for token lifecycle
- [ ] 5.2.2 Implement automatic token refresh before expiration
- [ ] 5.2.3 Implement session state (logged in/out/expired)
- [ ] 5.2.4 Add Koin module registration
- [ ] 5.2.5 Integrate with RPC client for auth headers

### 5.3 Auth State in Koin
- [ ] 5.3.1 Create `AuthState` sealed class (Authenticated, Unauthenticated, Loading)
- [ ] 5.3.2 Create `AuthStateHolder` with StateFlow
- [ ] 5.3.3 Wire into app-level Koin scope
- [ ] 5.3.4 Add navigation guard for authenticated routes

## 6. UI Screens

### 6.1 Login Screen
- [ ] 6.1.1 Create `LoginComponent` with Decompose
- [ ] 6.1.2 Create `LoginScreen` composable
- [ ] 6.1.3 Implement email/password input validation
- [ ] 6.1.4 Handle login errors with user feedback
- [ ] 6.1.5 Navigate to home on successful login

### 6.2 Registration Screen
- [ ] 6.2.1 Create `RegisterComponent` with Decompose
- [ ] 6.2.2 Create `RegisterScreen` composable
- [ ] 6.2.3 Implement invite code field
- [ ] 6.2.4 Implement password confirmation
- [ ] 6.2.5 Handle registration errors

### 6.3 Navigation Integration
- [ ] 6.3.1 Add `Config.Login` and `Config.Register` to navigation
- [ ] 6.3.2 Update `RootComponent` to check auth state on startup
- [ ] 6.3.3 Redirect unauthenticated users to login
- [ ] 6.3.4 Redirect authenticated users away from login/register

## 7. Verification

### 7.1 Security Review
- [ ] 7.1.1 Verify password hash is never logged or exposed
- [ ] 7.1.2 Verify tokens are stored securely on each platform
- [ ] 7.1.3 Verify JWT secret is loaded from environment, not hardcoded
- [ ] 7.1.4 Verify user data isolation in repositories

### 7.2 End-to-End Testing
- [ ] 7.2.1 Test full registration flow on Android
- [ ] 7.2.2 Test full login flow on Desktop
- [ ] 7.2.3 Test token refresh after expiration
- [ ] 7.2.4 Test logout clears stored credentials
- [ ] 7.2.5 Test cross-user data isolation via API

### 7.3 Documentation
- [ ] 7.3.1 Update CLAUDE.md with auth patterns
- [ ] 7.3.2 Document environment variables for JWT configuration
- [ ] 7.3.3 Update docker-compose example with JWT secret

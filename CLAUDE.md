# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

IMPORTANT: When applicable, prefer using intellij-index MCP tools for code navigation and refactoring.

## Project Overview

Altair is a life management ecosystem built with Kotlin Multiplatform and Compose Multiplatform.
It consists of three modules (Guidance, Knowledge, Tracking) unified by system-level features
(Universal Inbox, Initiatives, Routines). The system supports desktop (Windows/Linux/macOS),
mobile (Android/iOS), and a self-hosted server backend.

**Key References:**

- `@docs/architecture/overview.md` - System architecture and design principles
- `@docs/architecture/system-architecture.md` - Technical infrastructure details
- `@docs/architecture/domain-model.md` - Business entities and relationships
- `@docs/architecture/persistence.md` - Database strategy and multi-user isolation
- `@docs/architecture/event-bus.md` - Cross-module communication patterns
- `@docs/adr/` - Architectural Decision Records (read on demand for specific decisions)

## Module Structure

```
altair/
├── composeApp/     # Compose Multiplatform UI (Android, iOS, Desktop)
│   └── src/
│       ├── commonMain/    # Shared UI code
│       ├── androidMain/   # Android-specific code
│       ├── iosMain/       # iOS-specific code
│       └── jvmMain/       # Desktop-specific code
├── shared/         # Kotlin Multiplatform domain logic
│   └── src/
│       ├── commonMain/    # Platform-agnostic domain code
│       ├── androidMain/   # Android data layer (SQLite)
│       ├── iosMain/       # iOS data layer (SQLite)
│       └── jvmMain/       # Desktop/Server data layer (SurrealDB)
├── server/         # Ktor backend (sync, AI, auth)
└── iosApp/         # iOS application entry point
```

## Development Commands

### Build

```bash
./gradlew build                           # Full build (all platforms)
./gradlew :composeApp:assembleDebug       # Android debug APK
./gradlew :composeApp:run                 # Run desktop application
```

### Tests

```bash
./gradlew test                            # All unit tests
./gradlew :shared:jvmTest                 # Shared module JVM tests
./gradlew :composeApp:jvmTest             # ComposeApp JVM tests
./gradlew :composeApp:iosSimulatorArm64Test  # iOS simulator tests
./gradlew allTests                        # All targets + aggregated report
```

### Lint & Code Quality

```bash
./gradlew lint                            # Android lint (default variant)
./gradlew lintFix                         # Auto-fix lint issues
./gradlew check                           # All checks (lint + tests)
```

### Platform-Specific

```bash
# Android
./gradlew :composeApp:assembleDebug       # Build debug APK
./gradlew :composeApp:assembleRelease     # Build release APK
./gradlew :composeApp:connectedAndroidTest # Run instrumented tests

# iOS
# Use Xcode or: ./gradlew :composeApp:embedAndSignAppleFrameworkForXcode

# Desktop
./gradlew :composeApp:run                 # Run desktop app
./gradlew :composeApp:packageDmg          # macOS DMG
./gradlew :composeApp:packageMsi          # Windows MSI
./gradlew :composeApp:packageDeb          # Linux DEB

# Server
JWT_SECRET="your-secret-key" ./gradlew :server:run    # Run Ktor server
./gradlew :server:test                                # Server tests
```

## Environment Setup

### First-Time Setup

1. **Install required tools**:
   - JDK 17 or later
   - Gradle 8.14.3+ (or use included wrapper)
   - For iOS: Xcode 15+ and CocoaPods

2. **Configure server environment** (optional, for running server):
   ```bash
   # Required for server authentication
   export JWT_SECRET="your-secret-key-at-least-32-characters-long"

   # Optional JWT configuration
   export JWT_ISSUER="altair-server"           # Default
   export JWT_AUDIENCE="altair-client"         # Default
   export JWT_ACCESS_EXPIRY_MINUTES="15"       # Default
   export JWT_REFRESH_EXPIRY_DAYS="30"         # Default
   ```

3. **Verify setup**:
   ```bash
   ./gradlew build
   ```

### Running the Server

```bash
# From server module
cd server
JWT_SECRET="your-secret-key-here" ./gradlew run

# Or from root
JWT_SECRET="your-secret-key-here" ./gradlew :server:run
```

## Core Architecture Patterns

### Technology Stack

| Layer          | Technology                      | Version | Purpose                                          |
| -------------- | ------------------------------- | ------- | ------------------------------------------------ |
| UI             | Compose Multiplatform           | 1.10.0  | Cross-platform declarative UI                    |
| UI Components  | Compose Unstyled                | 1.49.6  | Headless primitives with custom Altair theme     |
| Navigation     | Decompose                       | 3.3.0   | UI-agnostic navigation with lifecycle management |
| Domain Logic   | Kotlin Multiplatform            | 2.3.0   | Shared business logic across all platforms       |
| DI             | Koin                            | 4.x     | Runtime dependency injection                     |
| Error Handling | Arrow                           | 2.x     | Typed errors with `Either<Error, Success>`       |
| Desktop DB     | SurrealDB (embedded)            | 2.0+    | Graph queries, vector search, full-text search   |
| Mobile DB      | SQLite (SQLDelight)             | 2.0+    | Lightweight, type-safe queries                   |
| Server         | Ktor + SurrealDB                | 3.1+    | Self-hosted backend, sync hub, AI services       |
| Client-Server  | kotlinx-rpc                     | latest  | Type-safe RPC over WebSocket                     |
| Testing        | Kotest + Mokkery + Turbine      | 5.9.1   | BDD testing, mocking, Flow testing               |

### Error Handling with Arrow

**Always use `Either<DomainError, T>` for operations that can fail.** Never throw exceptions for expected failures.

```kotlin
suspend fun getUser(id: UserId): Either<UserError, User> = either {
    val cached = cache.get(id).bind()
    cached ?: api.fetchUser(id).bind().also { cache.store(it) }
}
```

Chain operations with `flatMap`, `map`, and recover with `fold`.

### Navigation with Decompose

Use `ComponentContext` decomposition for navigation state:

```kotlin
class RootComponent(
    componentContext: ComponentContext,
    // other dependencies
) : ComponentContext by componentContext {
    // Component uses lifecycle and back handling
}
```

### Multi-User Data Isolation

**Every entity must include `user_id` field.** All queries must filter by authenticated user:

```kotlin
// SurrealDB example
SELECT * FROM quest WHERE user_id = $current_user AND status = 'active'

// SQLite example
SELECT * FROM note WHERE user_id = :userId
```

- Server enforces user scope on all queries
- No cross-user data access
- Admin users manage accounts but cannot view member content

### Event Bus (Desktop Only)

Modules communicate via event bus using Kotlin `SharedFlow`. Events are published after successful database operations:

```kotlin
// Publishing
eventBus.publish(GuidanceEvent.QuestCompleted(quest))

// Subscribing
eventBus.subscribe(GuidanceEvent.QuestCompleted::class, scope) { event ->
    showReflectionPrompt(event.quest)
}
```

See `@docs/architecture/event-bus.md` for all event types and patterns.

### Database Strategy

- **Desktop**: SurrealDB embedded for graph queries, vector search, semantic similarity
- **Mobile**: SQLite via SQLDelight for quick capture and reliability
- **Server**: SurrealDB for sync, conflict resolution, and central storage

All migrations live in platform-specific source sets:

- Desktop/Server: Manual SurrealDB schema evolution
- Mobile: SQLDelight migrations in `shared/src/commonMain/sqldelight/migrations/`

### API Layer

All network calls in `shared/src/commonMain/kotlin/api/` must:

- Use `Either<NetworkError, T>` return types
- Never expose Ktor types outside the API package
- Use `NetworkModule` for Ktor client configuration

See `.claude/rules/api-rules.md` for details.

### RPC Services (kotlinx-rpc)

Client-server communication uses **kotlinx-rpc** with WebSocket transport (kRPC protocol).

**Service interfaces** are defined in `shared/src/commonMain/kotlin/.../rpc/`:

```kotlin
@Rpc
interface AuthService {
    suspend fun login(request: AuthRequest): AuthResponse
    suspend fun refresh(refreshToken: String): TokenRefreshResponse
    suspend fun logout()
    suspend fun register(request: RegisterRequest): AuthResponse
}
```

**Server implementations** are in `server/src/main/kotlin/.../rpc/`:

```kotlin
class AuthServiceImpl : AuthService {
    override suspend fun login(request: AuthRequest): AuthResponse {
        // Implementation
    }
}
```

**Key patterns:**

- Use `@Rpc` annotation on service interfaces (no `RemoteService` base interface)
- Streaming uses `Flow<T>` return types (e.g., `AiService.complete()`)
- Server uses `Krpc` Ktor plugin with `rpc("/rpc")` routing
- Client uses `installKrpc()` on HttpClient with WebSocket support
- RPC services are registered via Koin in `composeApp/.../rpc/RpcModule.kt`

**Available services:**

- `SyncService` - Pull/push sync with optional streaming
- `PublicAuthService` - Login, register, token refresh (unauthenticated endpoint at `/rpc/auth`)
- `AuthService` - Logout, invite code generation (authenticated endpoint at `/rpc`)
- `AiService` - Embeddings, transcription, streaming completions

### Authentication Architecture

The authentication system uses JWT tokens with refresh token rotation:

**Server-side components** (`server/src/main/kotlin/`):

- `JwtConfig` - Configuration from environment variables (JWT_SECRET required)
- `JwtTokenServiceImpl` - JWT generation and validation with HMAC-SHA256
- `Argon2PasswordService` - Password hashing with Argon2id (OWASP-recommended parameters)
- `PublicAuthServiceImpl` - Handles login, registration, token refresh
- `AuthServiceImpl` - Authenticated operations (invite codes, logout)

**Client-side components** (`shared/src/`):

- `AuthManager` - Token lifecycle, auto-refresh, session state (StateFlow)
- `SecureTokenStorage` - Platform-specific secure storage interface
    - Android: `EncryptedSharedPreferences` with Android Keystore
    - iOS: Keychain Services with `kSecAttrAccessibleWhenUnlockedThisDeviceOnly`
    - Desktop: Native credential stores with encrypted preferences fallback
        - macOS: Keychain Services via JNA
        - Windows: Credential Manager (advapi32.dll) via JNA
        - Linux: Secret Service (libsecret / GNOME Keyring / KDE Wallet) via JNA
        - Fallback: AES-256-GCM encrypted Java Preferences when native stores unavailable

**Auth flow:**

```kotlin
// Login/Register -> store tokens -> navigate to Home
val response = publicAuthService.login(AuthRequest(email, password))
tokenStorage.saveAccessToken(response.accessToken)
tokenStorage.saveRefreshToken(response.refreshToken)

// RPC client automatically includes Authorization header
val factory: RpcClientFactory = get()
val syncService = factory.syncService() // Uses TokenProvider
```

**Required environment variables:**

- `JWT_SECRET` - HMAC secret (minimum 32 characters, required)
- `JWT_ISSUER` - Token issuer (default: altair-server)
- `JWT_AUDIENCE` - Token audience (default: altair-client)
- `JWT_ACCESS_EXPIRY_MINUTES` - Access token expiry (default: 15)
- `JWT_REFRESH_EXPIRY_DAYS` - Refresh token expiry (default: 30)

## Code Organization Principles

1. **Platform-specific code only where necessary**: Maximize `commonMain`, minimize platform-specific implementations
2. **Repository pattern**: UI never accesses databases directly; use repository interfaces
3. **ViewModels for state**: UI observes ViewModel state; dispatches intents for actions
4. **Event-driven integration**: Use event bus for cross-module reactions (desktop)
5. **Type-safe queries**: SQLDelight generates type-safe Kotlin for SQLite; SurrealDB uses parameterized queries

## Development Workflow

### Adding New Features

1. **Start with domain model** in `shared/src/commonMain/kotlin/`
2. **Define repository interface** in shared module
3. **Implement platform-specific repositories**:
    - Desktop: `shared/src/jvmMain/kotlin/` (SurrealDB)
    - Mobile: `shared/src/androidMain/` or `iosMain/` (SQLite)
4. **Create UI in composeApp** using Compose Multiplatform
5. **Add DI setup** in Koin modules
6. **Publish events** for cross-module integration (desktop)
7. **Write tests** in `commonTest` for shared logic

### OpenSpec Workflow (Experimental)

For major changes requiring design review, use the OpenSpec artifact-based workflow:

**Workflow progression**: explore → proposal → architecture → implementation → verification → archive

**Available commands**:
- `/openspec-explore` - Think through ideas before creating formal change
- `/openspec-new-change` - Start new change with structured artifacts
- `/openspec-continue-change` - Create next artifact in workflow
- `/openspec-apply-change` - Implement tasks from change artifacts
- `/openspec-verify-change` - Validate implementation matches design
- `/openspec-archive-change` - Finalize and archive completed change
- `/openspec-ff-change` - Fast-forward through all artifacts at once
- `/openspec-bulk-archive-change` - Archive multiple completed changes
- `/openspec-sync-specs` - Sync delta specs to main specs without archiving
- `/openspec-onboard` - Guided walkthrough of the OpenSpec workflow

**When to use**: Breaking changes, new modules, architectural decisions, cross-cutting features

See `openspec/project.md` for full documentation.

### Testing Strategy

**Framework**: All tests use Kotest 5.9.1 (BDD-style testing framework)

**Test organization**:

- **Unit tests**: `commonTest` for platform-agnostic logic
- **Platform tests**: `jvmTest`, `androidTest`, `iosTest` for platform-specific code
- **Integration tests**: Server module uses Testcontainers for database tests

**Spec styles** (choose based on test type):

- `BehaviorSpec`: BDD-style `given/when/then` for integration and behavioral tests
- `DescribeSpec`: RSpec-style `describe/it` for unit tests with hierarchical structure
- `FunSpec`: Simple `test("name") {}` for straightforward tests
- `withData`: Data-driven testing for parameterized tests

**Assertions**:

```kotlin
// Kotest matchers
result shouldBe expected
list shouldHaveSize 3
value.shouldBeNull()
flag.shouldBeTrue()

// Arrow Either matchers
result.shouldBeRight()
result.shouldBeLeft()
result.leftOrNull().shouldBeInstanceOf<DomainError.NotFoundError>()
```

**Async testing**:

```kotlin
// Use eventually for non-deterministic operations
eventually(5.seconds) {
    repository.getById(id).shouldBeRight()
}
```

**Lifecycle hooks**:

```kotlin
class MyTest : BehaviorSpec({
    beforeEach {
        // Setup before each test
    }

    afterEach {
        // Cleanup after each test
    }

    given("feature") {
        `when`("action") {
            then("outcome") {
                // test logic
            }
        }
    }
})
```

**Property-based testing**:

```kotlin
checkAll<String, Int> { str, num ->
    // Test invariants with random inputs
    str.length shouldBeGreaterThanOrEqual 0
}
```

**Additional tools**:

- **Mocking**: Use Mokkery for multiplatform mocking
- **Flow testing**: Use Turbine for testing Kotlin Flows
- **Skill reference**: Use `/kotest` skill for detailed examples and patterns

## Common Pitfalls

1. **Don't use exceptions for expected failures** - Use Arrow's `Either` instead
2. **Don't skip `user_id` filtering** - All queries must scope to authenticated user
3. **Don't expose platform types** - Keep SurrealDB/SQLite/Ktor details in data layer
4. **Don't create circular dependencies** - Use event bus for cross-module communication
5. **Don't forget iOS targets** - When adding dependencies, consider iOS compatibility

## Skills & Commands

### Available Skills

**Testing & Code Generation:**
- `/kotest` - Kotest testing framework patterns and examples
- `/gen-kmp-test` - Generate Kotest test specifications for Kotlin Multiplatform code
- `/new-migration` - Generate database migrations for SQLDelight (mobile) or SurrealDB (desktop/server)

**Functional Programming:**
- `/arrow-patterns` - Get help with Arrow functional error handling patterns

**OpenSpec Workflow:**
- `/openspec-explore` - Think through ideas before formal changes
- `/openspec-new-change` - Start new change with artifact workflow
- `/openspec-continue-change` - Create next artifact in workflow
- `/openspec-apply-change` - Implement tasks from change artifacts
- `/openspec-verify-change` - Validate implementation vs. design
- `/openspec-archive-change` - Archive completed change
- `/openspec-bulk-archive-change` - Archive multiple changes
- `/openspec-ff-change` - Fast-forward through all artifacts
- `/openspec-sync-specs` - Sync delta specs to main specs
- `/openspec-onboard` - Guided workflow walkthrough

**Version Control:**
- `/commit` - Create a git commit
- `/review-pr` - Comprehensive PR review

### Subagents

- `arrow-validator` - Validates Arrow Either error handling patterns in domain/data layers (use proactively after implementing repositories or services)

## Additional Resources

- `README.md` - Project setup and build instructions
- `.claude/AUTOMATIONS.md` - Complete automation reference (hooks, MCP servers, skills)
- `docs/implementation-plan.md` - Phased feature development roadmap
- `docs/requirements/` - Product requirements documents (PRDs)
- `openspec/project.md` - OpenSpec proposal system for major changes

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

<!-- OPENSPEC:START -->

# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:

- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:

- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

## Project Overview

Altair is a life management ecosystem built with Kotlin Multiplatform and Compose Multiplatform. It consists of three modules (Guidance, Knowledge, Tracking) unified by system-level features (Universal Inbox, Initiatives, Routines). The system supports desktop (Windows/Linux/macOS), mobile (Android/iOS), and a self-hosted server backend.

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
```

## Core Architecture Patterns

### Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| UI | Compose Multiplatform | Cross-platform declarative UI |
| UI Components | Compose Unstyled | Headless primitives with custom Altair theme |
| Navigation | Decompose | UI-agnostic navigation with lifecycle management |
| Domain Logic | Kotlin Multiplatform | Shared business logic across all platforms |
| DI | Koin | Runtime dependency injection |
| Error Handling | Arrow | Typed errors with `Either<Error, Success>` |
| Desktop DB | SurrealDB (embedded) | Graph queries, vector search, full-text search |
| Mobile DB | SQLite (SQLDelight) | Lightweight, type-safe queries |
| Server | Ktor + SurrealDB | Self-hosted backend, sync hub, AI services |
| Client-Server | kotlinx-rpc | Type-safe RPC over gRPC |

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

### Testing Strategy

- **Unit tests**: `commonTest` for platform-agnostic logic
- **Platform tests**: `jvmTest`, `androidTest`, `iosTest` for platform-specific code
- **Mocking**: Use Mokkery for multiplatform mocking
- **Flow testing**: Use Turbine for testing Kotlin Flows

## Common Pitfalls

1. **Don't use exceptions for expected failures** - Use Arrow's `Either` instead
2. **Don't skip `user_id` filtering** - All queries must scope to authenticated user
3. **Don't expose platform types** - Keep SurrealDB/SQLite/Ktor details in data layer
4. **Don't create circular dependencies** - Use event bus for cross-module communication
5. **Don't forget iOS targets** - When adding dependencies, consider iOS compatibility

## Skills & Commands

- `/arrow-patterns` - Get help with Arrow functional error handling patterns
- `/commit` - Create a git commit
- `/review-pr` - Comprehensive PR review

## Additional Resources

- `README.md` - Project setup and build instructions
- `docs/implementation-plan.md` - Phased feature development roadmap
- `docs/requirements/` - Product requirements documents (PRDs)
- `openspec/AGENTS.md` - OpenSpec proposal system for major changes

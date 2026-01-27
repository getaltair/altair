# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Altair is a Kotlin Multiplatform life management ecosystem with three modules: Guidance (task execution), Knowledge (PKM), and Tracking (inventory). Privacy-focused, self-hosted, designed for neurodivergent users.

## Build Commands

```bash
# Build all modules
./gradlew build

# Run desktop app (JVM)
./gradlew :composeApp:run

# Run server (Ktor on port 8080)
./gradlew :server:run

# Run tests
./gradlew test                           # All tests
./gradlew :shared:test                   # Shared module only
./gradlew :composeApp:testDebugUnitTest  # Android unit tests
./gradlew :server:test                   # Server tests

# Android
./gradlew :composeApp:assembleDebug

# iOS - build via Xcode (iosApp/iosApp.xcodeproj), consumes ComposeApp framework
```

## Architecture

```
altair/
├── composeApp/     # KMP Compose client (Android, iOS, Desktop)
├── server/         # Ktor backend (JVM only, port 8080)
└── shared/         # KMP shared domain models and logic
```

**Module Dependencies:**
- `composeApp` → `shared`
- `server` → `shared`

**Tech Stack:**
- Kotlin 2.3.0, Compose Multiplatform 1.10.0
- Ktor 3.0.3 (server)
- Koin (DI), Decompose (navigation), Arrow (error handling)
- SurrealDB (server/desktop), SQLite via SQLDelight (mobile)
- JWT + Argon2 (auth)

## Key Conventions

**Source locations:**
- Common client code: `composeApp/src/commonMain/kotlin/com/getaltair/altair/`
- Platform-specific: `composeApp/src/{androidMain,iosMain,jvmMain}/`
- Server: `server/src/main/kotlin/com/getaltair/altair/server/`
- Shared: `shared/src/commonMain/kotlin/com/getaltair/altair/shared/`

**Android targets:** minSdk 24, targetSdk 36, JVM 11
**JVM/Server:** Java 21

## Documentation

Read docs in this order for context:
1. `docs/architecture/overview.md` - What Altair is
2. `docs/implementation-plan.md` - 15-phase roadmap (Phase 0 complete)
3. `docs/architecture/domain-model.md` - Entities and relationships
4. `docs/adr/` - Architecture Decision Records

Key ADRs:
- ADR-001: Kotlin Multiplatform + Compose
- ADR-002: Hybrid Database (SurrealDB + SQLite)
- ADR-009: Core Library Stack (Koin, Decompose, Arrow, Mokkery)
- ADR-016: Ktor REST API (current API approach)

## Domain Concepts

- **Quest** - Task unit with energy cost, WIP=1 enforcement
- **Initiative** - Cross-module organization (Projects/Areas)
- **Universal Inbox** - Type-agnostic capture, triage to Quest/Note/Item
- **Routine** - Recurring template spawning instances on schedule
- **Energy** - Variable daily capacity for ADHD-aware planning

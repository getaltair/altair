# Altair

A privacy-focused, self-hosted life management ecosystem built with Kotlin Multiplatform.

## Overview

Altair externalizes executive function through three interconnected modules:

- **Guidance** — Quest-based task execution with energy-aware planning
- **Knowledge** — Personal knowledge management with bidirectional linking
- **Tracking** — Physical inventory management with location awareness

Designed for neurodivergent users, Altair enforces constraints that reduce cognitive load: WIP limits, energy budgets, and a universal inbox that defers type decisions until triage.

## Key Features

- **Universal Inbox** — Capture anything, decide what it becomes later
- **Initiatives** — Cross-module organization (Projects and Areas)
- **Routines** — Recurring templates that spawn Quest instances
- **Energy Budgets** — ADHD-aware daily capacity planning
- **Multi-user Support** — Complete data isolation on shared instances
- **Offline-first** — Full functionality without network; sync when connected

## Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Language | Kotlin 2.3 | Shared codebase across all platforms |
| UI | Compose Multiplatform 1.10 | Android, iOS, Desktop |
| Server | Ktor 3.4 | Self-hosted backend |
| DI | Koin 4.1 | Dependency injection |
| Navigation | Decompose 3.3 | UI-agnostic navigation |
| Errors | Arrow 2.1 | Typed error handling |
| Desktop/Server DB | SurrealDB | Graph database with vector search |
| Mobile DB | SQLDelight | SQLite for Android/iOS |
| Auth | JWT + Argon2 | Secure authentication |

## Project Structure

```
altair/
├── composeApp/     # KMP Compose client (Android, iOS, Desktop)
├── server/         # Ktor backend (JVM, port 8080)
├── shared/         # KMP shared domain models and logic
└── docs/           # Architecture docs and ADRs
```

## Prerequisites

- **JDK 21** (for server and desktop)
- **Android Studio** or IntelliJ IDEA with Kotlin Multiplatform plugin
- **Xcode 15+** (for iOS builds, macOS only)

## Getting Started

### Build All Modules

```bash
./gradlew build
```

### Run Desktop App

```bash
./gradlew :composeApp:run
```

### Run Server

```bash
./gradlew :server:run
# Server starts on http://localhost:8080
```

### Run Tests

```bash
./gradlew test                           # All tests
./gradlew :shared:test                   # Shared module
./gradlew :composeApp:testDebugUnitTest  # Android unit tests
./gradlew :server:test                   # Server tests
```

### Android

```bash
./gradlew :composeApp:assembleDebug
```

### iOS

Open `iosApp/iosApp.xcodeproj` in Xcode and build from there.

## Platform Targets

| Platform | Scope | Database |
|----------|-------|----------|
| Desktop (Win/Linux/macOS) | Full features | SurrealDB embedded |
| Android | Daily driver + capture | SQLite |
| iOS | Daily driver + capture | SQLite |
| Server (Docker) | Sync + AI + Auth | SurrealDB |

## Documentation

- [Architecture Overview](docs/architecture/overview.md) — System design entry point
- [Domain Model](docs/architecture/domain-model.md) — Entities and relationships
- [Implementation Plan](docs/implementation-plan.md) — 15-phase roadmap
- [ADRs](docs/adr/) — Architecture Decision Records

## Development Status

Currently implementing core infrastructure. See [Implementation Plan](docs/implementation-plan.md) for progress.

## License

*License TBD*

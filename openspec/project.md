# Project Context

## Purpose

**Altair** is a privacy-focused productivity suite designed as a Life Management Ecosystem. It helps users—especially neurodivergent users—externalize executive function with thoughtful defaults.

The system consists of three core modules:

- **Guidance**: Quest-based agile task execution with energy-based planning (WIP=1 default)
- **Knowledge**: Personal knowledge management (Markdown PKM with bidirectional linking)
- **Tracking**: Inventory and asset management with photo-first capture

Key organizing concepts:

- **Universal Inbox**: System-wide untyped capture point for triaging items
- **Initiatives**: Cross-cutting organizational units linking content across modules
- **Routines**: Recurring templates that spawn instances on schedule

Self-hosted architecture keeps all data on user infrastructure with complete multi-user support and data isolation.

## Tech Stack

### Languages & Frameworks

- **Kotlin 2.3.0** - Primary language across all platforms
- **Kotlin Multiplatform (KMP)** - Shared code across Android, iOS, Desktop, and Server
- **Compose Multiplatform 1.10.0** - Cross-platform UI framework
- **Ktor 3.3.3** - Server framework with Netty engine

### Core Libraries

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Dependency Injection | Koin 4.x | DI container |
| Navigation | Decompose 3.x | Composable navigation for KMP |
| Error Handling | Arrow 2.x | Typed errors, validation, optics |
| Testing | Kotest 5.9.1 | BDD testing framework with property-based testing |
| Mocking & Flows | Mokkery 3.x + Turbine | Mocking and Flow testing |
| Serialization | kotlinx.serialization 1.9.0 | JSON serialization |
| Async | kotlinx-coroutines 1.10.2 | Coroutines for concurrency |
| Date/Time | kotlinx-datetime 0.7.1 | Cross-platform date handling |

### Database Strategy (ADR-002)

- **Server**: SurrealDB (embedded or standalone)
- **Desktop**: SurrealDB embedded for offline support
- **Mobile**: SQLite via SQLDelight
- **Storage**: S3-compatible (local, MinIO, AWS S3, Backblaze)

### AI Services (ADR-006)

- **ONNX Runtime**: Embeddings generation
- **whisper.cpp**: Audio transcription
- **LLM Provider Proxy**: Support for Ollama, OpenAI, etc.

### Build & Deployment

- **Build System**: Gradle 8.12.3 with Kotlin DSL
- **Version Catalog**: `gradle/libs.versions.toml` for centralized dependency management
- **Android**: AGP 8.12.3, minSdk 24, targetSdk 36
- **iOS**: Xcode project with native Swift support
- **JVM**: Java 21 (via mise.toml)
- **Deployment**: Docker Compose (ADR-007)

## Project Conventions

### Code Style

- **Kotlin official style** (`kotlin.code.style=official` in gradle.properties)
- **JVM Target**: Java 11 for compatibility across all targets
- **Markdown linting**: max line length 120 characters (code blocks and tables exempt)

### Architecture Patterns

- **Monorepo structure** with three modules:
  - `composeApp/` - Multiplatform Compose UI (Android, iOS, Desktop)
  - `server/` - Ktor backend
  - `shared/` - KMP shared module (domain models, DTOs, repository interfaces)
- **Event Bus** for module communication (ADR-003)
- **Server-centralized AI services** (ADR-006)
- **Multi-user data isolation** (ADR-012)
- **Universal Inbox architecture** (ADR-010)

### Testing Strategy

**Framework**: All tests use **Kotest 5.9.1** (BDD-style testing framework)

**Test organization**:
- **Common tests**: `src/commonTest/kotlin/` for platform-agnostic logic
- **Platform-specific tests**: `src/jvmTest/`, `src/androidTest/`, `src/iosTest/`
- **Server tests**: `server/src/test/kotlin/` for integration tests
- **Android instrumented tests**: `composeApp/src/androidInstrumentedTest/` (uses JUnit runner with Kotest matchers)

**Spec styles** (choose based on test type):
- **BehaviorSpec**: BDD-style `given/when/then` for integration and behavioral tests
- **DescribeSpec**: RSpec-style `describe/it` for unit tests with hierarchical structure
- **FunSpec**: Simple `test("name") {}` for straightforward tests
- **withData**: Data-driven testing for parameterized tests

**Assertions**:
```kotlin
// Kotest matchers
result shouldBe expected
list shouldHaveSize 3
value.shouldBeNull()
flag.shouldBeTrue()

// Arrow Either matchers (kotest-assertions-arrow)
result.shouldBeRight()
result.shouldBeLeft()
result.leftOrNull().shouldBeInstanceOf<DomainError.NotFoundError>()
```

**Property-based testing**:
- Use `checkAll` with `Arb<T>` generators for testing invariants
- Custom generators in `shared/src/commonTest/kotlin/TestGenerators.kt`
- Seeds logged for reproducibility

**Async testing**:
```kotlin
// Testing coroutines
test("async operation").config(coroutineTestScope = true) {
    eventually(5.seconds) {
        repository.getUser(id).shouldBeRight()
    }
}
```

**Additional tools**:
- **Mocking**: Mokkery 3.x for multiplatform mocking
- **Flow testing**: Turbine for testing Kotlin Flows
- **Server tests**: Ktor `testApplication` DSL with SurrealDB Testcontainers
- **Testcontainers**: Kotest extension for SurrealDB integration tests

**Running tests**:
```bash
./gradlew test                           # All unit tests
./gradlew :shared:jvmTest                # Shared module JVM tests
./gradlew :server:test                   # Server integration tests
./gradlew :composeApp:jvmTest            # ComposeApp JVM tests
./gradlew :composeApp:connectedAndroidTest  # Android instrumented tests
./gradlew allTests                       # All targets + aggregated report
```

**Reference**: Use `/kotest` skill for detailed examples and patterns

### Git Workflow

- **Main branch**: Primary development branch
- **Commit conventions**:
  - `feat:` for new features
  - `fix:` for bug fixes
  - `chore:` for maintenance tasks
  - `docs:` for documentation changes
- **OpenSpec integration**: Spec-driven development with change proposals in `openspec/changes/`

## Domain Context

### Core Modules

1. **Guidance (Task Management)**
   - Quest-based task execution
   - Energy-based planning with WIP=1 default
   - Agile sprint-style workflow

2. **Knowledge (PKM)**
   - Markdown-based personal knowledge management
   - Bidirectional linking between notes
   - Source document architecture (ADR-014)

3. **Tracking (Asset Management)**
   - Photo-first capture for physical items
   - Inventory tracking and organization

### Key Architecture Decisions

Reference the `/docs/adr/` directory for detailed decisions:

- ADR-001: Kotlin Multiplatform + Compose Multiplatform
- ADR-002: Hybrid database (SurrealDB + SQLite)
- ADR-003: Event Bus for module communication
- ADR-005: kotlinx-rpc for client-server communication
- ADR-008: Compose Unstyled + Custom Altair theme
- ADR-009: Core library stack (Koin, Decompose, Arrow, Mokkery)

## Important Constraints

- **Privacy-first**: All data stored on user infrastructure (self-hosted)
- **Multi-user support**: Complete data isolation between users
- **Offline support**: Desktop and mobile must work offline with sync
- **Cross-platform consistency**: UI and behavior should be consistent across Android, iOS, and Desktop
- **Neurodivergent-friendly**: Thoughtful defaults to reduce decision fatigue

## External Dependencies

### Runtime Services

- **SurrealDB**: Primary database (standalone or embedded)
- **SQLite**: Mobile local database via SQLDelight
- **S3-Compatible Storage**: For attachments and media

### Optional AI Services

- **ONNX Runtime**: On-device AI inference for embeddings
- **whisper.cpp**: Audio transcription (local or server)
- **LLM APIs**: Ollama, OpenAI, or other providers

### Platform SDKs

- **Android SDK**: Configured via local.properties
- **iOS SDK**: Xcode project compiles to framework
- **JVM**: Java 21 runtime

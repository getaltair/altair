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
| Testing | Mokkery 3.x + Turbine | Mocking and Flow testing |
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

- **Unit Testing**: Kotlin Test with JUnit
- **Mocking**: Mokkery 3.x (KMP-compatible)
- **Flow Testing**: Turbine for testing Kotlin Flow
- **Server Tests**: Ktor test host with `testApplication` DSL

Test locations:

- Common tests: `src/commonTest/kotlin/`
- Server tests: `server/src/test/kotlin/`

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

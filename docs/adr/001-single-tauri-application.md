# ADR-001: Kotlin Multiplatform Application Architecture

| Field          | Value                                        |
| -------------- | -------------------------------------------- |
| **Status**     | Accepted                                     |
| **Date**       | 2026-01-09                                   |
| **Deciders**   | Robert Hamilton                              |
| **Supersedes** | ADR-001 (Single Tauri Application, original) |

## Context

Altair consists of three logical applications—Guidance (task management), Knowledge (PKM), and
Tracking (inventory)—that share data and need to communicate in real-time. The system must support:

1. **Desktop platforms**: Windows, Linux (hard requirements), macOS (nice to have)
2. **Mobile platforms**: Android (hard requirement), iOS (nice to have)
3. **Self-hosted cloud sync**: User-operated server for multi-device synchronization
4. **Absolutely no Electron**: Performance and resource usage are critical for ADHD users

Mobile scope is intentionally limited to "quick capture + view"—not full feature parity with
desktop.

## Decision

Build Altair using **Kotlin Multiplatform (KMP)** with **Compose Multiplatform** for the UI layer:

- **Desktop**: Full-featured Compose Multiplatform application with SurrealDB embedded
- **Mobile**: Lightweight Compose Multiplatform application with SQLite embedded (quick capture
  focus)
- **Server**: Ktor-based server with kotlinx-rpc endpoints, SurrealDB, and AI services

All three targets share a common Kotlin codebase for domain models, validation logic, and UI
components. Communication between clients and server uses kotlinx-rpc (gRPC-compatible protocol).

## Consequences

### Positive

- **90-96% code sharing**: Domain models, validation, and most UI shared across all platforms
- **Native performance**: Compose Multiplatform renders natively (Skia on desktop, native on
  Android, Metal on iOS)
- **Single language**: Kotlin across frontend, backend, and shared logic—no context switching
- **Production-proven**: McDonald's, Google Docs, Cash App, Forbes, Netflix use KMP in production
- **Mobile-ready**: Compose Multiplatform iOS stable since May 2025 (v1.8.0)
- **Type-safe IPC**: kotlinx-rpc provides compile-time checked client-server communication
- **JetBrains backing**: Active development with Google collaboration on build tooling

### Negative

- **Gradle complexity**: Build times 2-10 minutes vs 10-30s for pure Android projects
- **Learning curve**: 2-4 weeks for Kotlin, 1-2 weeks for KMP-specific patterns
- **Binary size**: iOS ~9MB overhead, Android 8-12% larger than pure native
- **Desktop distribution**: Requires bundling JVM runtime (~40-50MB for packaged app)
- **Smaller ecosystem**: Fewer KMP libraries than pure Android or web ecosystems

### Neutral

- Desktop apps use JVM; could explore GraalVM native-image later for smaller binaries
- iOS requires macOS + Xcode for building (standard for any iOS development)

## Alternatives Considered

### Alternative 1: Tauri 2 (Rust + Web)

Desktop-first framework using Rust backend and web frontend.

**Pros:**

- Excellent desktop support, small binaries (~5MB)
- Rust backend performance and safety
- Strong web ecosystem (Svelte, React, etc.)

**Rejected because:**

- Mobile support is "production-capable" but not "first-class citizen"
- Android requires SDK, NDK, 4 Rust target toolchains
- No shared logic between web frontend and Rust backend
- Real production mobile apps with Tauri are scarce

### Alternative 2: Flutter

Google's cross-platform UI framework with Dart.

**Pros:**

- Mature cross-platform (desktop stable since May 2022)
- Large ecosystem (55,000+ packages)
- Impeller rendering engine improvements

**Rejected because:**

- Active Linux rendering issues (backdrop filters #169508, shader failures #179185, crashes #172000)
- May 2024 layoffs affected Flutter team; mixed signals on commitment
- Dart is a single-purpose language with limited ecosystem outside Flutter
- Desktop performance lags behind mobile (Linux especially)

### Alternative 3: Qt/QML

Cross-platform C++ framework with QML declarative UI.

**Rejected because:**

- Complex licensing (€1M revenue threshold, LGPL v3 dynamic linking, iOS requires commercial)
- Steep learning curve (C++, QML, Qt concepts)
- APK sizes 18-55MB
- Dated developer experience compared to modern frameworks

### Alternative 4: Electron

Web technologies wrapped in Chromium.

**Rejected because:**

- Explicitly excluded by requirements ("Absolutely no Electron")
- Resource usage problematic for ADHD users who need responsive tools
- 150-200MB base memory footprint

## References

- [Compose Multiplatform Production Apps](https://www.jetbrains.com/lp/compose-multiplatform/) —
  McDonald's, Google Docs, Cash App examples
- [Compose Multiplatform 1.8.0 Release](https://blog.jetbrains.com/kotlin/2025/05/compose-multiplatform-1-8-0/)
  — iOS production stability
- [ADR-002: Hybrid Database Strategy](./002-surrealdb-embedded.md)
- [ADR-005: kotlinx-rpc Communication](./005-kotlinx-rpc-communication.md)
- PRD Core, Section 5: System Architecture

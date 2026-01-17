# Change: Add kotlinx-rpc Service Layer

## Why

Altair needs type-safe client-server communication for sync, authentication, and AI services.
Currently, the server has basic Ktor routing but no structured RPC mechanism. kotlinx-rpc (per
ADR-005) provides type-safe service definitions shared between client and server, native Kotlin
coroutines support, and built-in streaming for AI token delivery and real-time sync.

## What Changes

- Add kotlinx-rpc dependencies to version catalog and all modules (shared, server, composeApp)
- Define `@Rpc` service interfaces in shared module:
  - `SyncService` - Pull/push sync with optional streaming for real-time changes
  - `AuthService` - Login, token refresh, logout
  - `AiService` - Embeddings, transcription, completion streaming
- Implement service classes on server (stub implementations initially)
- Wire RPC into Ktor server configuration
- Create RPC client factory in composeApp for connecting to server

## Impact

- Affected specs: New `rpc-services` capability
- Affected code:
  - `gradle/libs.versions.toml` - New dependencies
  - `shared/build.gradle.kts` - RPC dependencies + KSP plugin
  - `server/build.gradle.kts` - RPC server dependencies
  - `composeApp/build.gradle.kts` - RPC client dependencies
  - `shared/src/commonMain/kotlin/.../rpc/` - Service interfaces (new)
  - `server/src/main/kotlin/.../rpc/` - Service implementations (new)
  - `server/src/main/kotlin/Application.kt` - RPC configuration
  - `composeApp/src/commonMain/kotlin/.../rpc/` - RPC client (new)

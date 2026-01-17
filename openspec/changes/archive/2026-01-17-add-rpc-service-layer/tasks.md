# Tasks: Add kotlinx-rpc Service Layer

## 1. Add Dependencies

- [x] 1.1 Add kotlinx-rpc version to `gradle/libs.versions.toml`
- [x] 1.2 Add kotlinx-rpc library entries (core, krpc-client, krpc-server, ktor integrations)
- [x] 1.3 Add kotlinx-rpc Gradle plugin entry
- [x] 1.4 Apply kotlinx-rpc plugin to `shared/build.gradle.kts`
- [x] 1.5 Add RPC dependencies to `shared/build.gradle.kts` (commonMain)
- [x] 1.6 Apply kotlinx-rpc plugin to `server/build.gradle.kts`
- [x] 1.7 Add RPC server dependencies to `server/build.gradle.kts`
- [x] 1.8 Apply kotlinx-rpc plugin to `composeApp/build.gradle.kts`
- [x] 1.9 Add RPC client dependencies to `composeApp/build.gradle.kts`
- [x] 1.10 Verify builds pass on all targets (`./gradlew build`)

## 2. Define RPC Service Interfaces

- [x] 2.1 Create `shared/.../rpc/` package directory
- [x] 2.2 Define `SyncService` interface with `@Rpc` annotation
  - `suspend fun pull(since: Long, entityTypes: Set<String>): SyncResponse`
  - `suspend fun push(changes: ChangeSet): PushResult`
  - `fun streamChanges(entityTypes: Set<String>): Flow<EntityChange>` (optional streaming)
- [x] 2.3 Define `AuthService` interface with `@Rpc` annotation
  - `suspend fun login(request: AuthRequest): AuthResponse`
  - `suspend fun refresh(refreshToken: String): TokenRefreshResponse`
  - `suspend fun logout()`
  - `suspend fun register(request: RegisterRequest): AuthResponse`
- [x] 2.4 Define `AiService` interface with `@Rpc` annotation
  - `suspend fun embed(texts: List<String>): List<List<Float>>`
  - `suspend fun transcribe(audioData: ByteArray, format: String): String`
  - `fun complete(request: CompletionRequest): Flow<String>`
- [x] 2.5 Create supporting DTOs for RPC (CompletionRequest, PushResult, etc.)
- [x] 2.6 Verify shared module compiles

## 3. Implement Server-Side Services

- [x] 3.1 Create `server/.../rpc/` package directory
- [x] 3.2 Implement `SyncServiceImpl` (stub returning empty/mock data)
- [x] 3.3 Implement `AuthServiceImpl` (stub with hardcoded test credentials)
- [x] 3.4 Implement `AiServiceImpl` (stub returning placeholder data)
- [x] 3.5 Verify server compiles

## 4. Wire RPC into Ktor

- [x] 4.1 Create `configureRpc()` extension function in server
- [x] 4.2 Register RPC services with Ktor routing
- [x] 4.3 Add `configureRpc()` call to `Application.module()`
- [x] 4.4 Configure RPC serialization (kotlinx.serialization)
- [x] 4.5 Verify server starts and RPC endpoints are exposed

## 5. Create RPC Client in composeApp

- [x] 5.1 Create `composeApp/.../rpc/` package directory
- [x] 5.2 Create `RpcClientFactory` for instantiating service stubs
- [x] 5.3 Create Koin module for RPC client dependencies
- [x] 5.4 Add RPC services to DI (SyncService, AuthService, AiService stubs)
- [x] 5.5 Verify composeApp compiles on all targets

## 6. Integration Testing

- [x] 6.1 Create basic round-trip test (client calls server, server responds)
- [x] 6.2 Test streaming endpoint (AiService.complete)
- [x] 6.3 Verify WebSocket transport works (if using kRPC)

## 7. Documentation

- [x] 7.1 Update CLAUDE.md with RPC patterns and conventions
- [x] 7.2 Add inline documentation to service interfaces

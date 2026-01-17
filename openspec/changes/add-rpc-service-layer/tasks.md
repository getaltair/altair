# Tasks: Add kotlinx-rpc Service Layer

## 1. Add Dependencies

- [ ] 1.1 Add kotlinx-rpc version to `gradle/libs.versions.toml`
- [ ] 1.2 Add kotlinx-rpc library entries (core, krpc-client, krpc-server, ktor integrations)
- [ ] 1.3 Add kotlinx-rpc Gradle plugin entry
- [ ] 1.4 Apply kotlinx-rpc plugin to `shared/build.gradle.kts`
- [ ] 1.5 Add RPC dependencies to `shared/build.gradle.kts` (commonMain)
- [ ] 1.6 Apply kotlinx-rpc plugin to `server/build.gradle.kts`
- [ ] 1.7 Add RPC server dependencies to `server/build.gradle.kts`
- [ ] 1.8 Apply kotlinx-rpc plugin to `composeApp/build.gradle.kts`
- [ ] 1.9 Add RPC client dependencies to `composeApp/build.gradle.kts`
- [ ] 1.10 Verify builds pass on all targets (`./gradlew build`)

## 2. Define RPC Service Interfaces

- [ ] 2.1 Create `shared/.../rpc/` package directory
- [ ] 2.2 Define `SyncService` interface with `@Rpc` annotation
  - `suspend fun pull(since: Long, entityTypes: Set<String>): SyncResponse`
  - `suspend fun push(changes: ChangeSet): PushResult`
  - `fun streamChanges(entityTypes: Set<String>): Flow<EntityChange>` (optional streaming)
- [ ] 2.3 Define `AuthService` interface with `@Rpc` annotation
  - `suspend fun login(request: AuthRequest): AuthResponse`
  - `suspend fun refresh(refreshToken: String): TokenRefreshResponse`
  - `suspend fun logout()`
  - `suspend fun register(request: RegisterRequest): AuthResponse`
- [ ] 2.4 Define `AiService` interface with `@Rpc` annotation
  - `suspend fun embed(texts: List<String>): List<List<Float>>`
  - `suspend fun transcribe(audioData: ByteArray, format: String): String`
  - `fun complete(request: CompletionRequest): Flow<String>`
- [ ] 2.5 Create supporting DTOs for RPC (CompletionRequest, PushResult, etc.)
- [ ] 2.6 Verify shared module compiles

## 3. Implement Server-Side Services

- [ ] 3.1 Create `server/.../rpc/` package directory
- [ ] 3.2 Implement `SyncServiceImpl` (stub returning empty/mock data)
- [ ] 3.3 Implement `AuthServiceImpl` (stub with hardcoded test credentials)
- [ ] 3.4 Implement `AiServiceImpl` (stub returning placeholder data)
- [ ] 3.5 Verify server compiles

## 4. Wire RPC into Ktor

- [ ] 4.1 Create `configureRpc()` extension function in server
- [ ] 4.2 Register RPC services with Ktor routing
- [ ] 4.3 Add `configureRpc()` call to `Application.module()`
- [ ] 4.4 Configure RPC serialization (kotlinx.serialization)
- [ ] 4.5 Verify server starts and RPC endpoints are exposed

## 5. Create RPC Client in composeApp

- [ ] 5.1 Create `composeApp/.../rpc/` package directory
- [ ] 5.2 Create `RpcClientFactory` for instantiating service stubs
- [ ] 5.3 Create Koin module for RPC client dependencies
- [ ] 5.4 Add RPC services to DI (SyncService, AuthService, AiService stubs)
- [ ] 5.5 Verify composeApp compiles on all targets

## 6. Integration Testing

- [ ] 6.1 Create basic round-trip test (client calls server, server responds)
- [ ] 6.2 Test streaming endpoint (AiService.complete)
- [ ] 6.3 Verify WebSocket transport works (if using kRPC)

## 7. Documentation

- [ ] 7.1 Update CLAUDE.md with RPC patterns and conventions
- [ ] 7.2 Add inline documentation to service interfaces

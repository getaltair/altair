# Design: Add kotlinx-rpc Service Layer

## Context

Altair requires client-server communication for:
- **Sync**: Offline-capable clients pushing/pulling entity changes
- **Auth**: JWT-based authentication with refresh tokens
- **AI**: Server-centralized AI services (embeddings, transcription, completions)

ADR-005 selected kotlinx-rpc over REST, GraphQL, and raw gRPC for its type safety, native
Kotlin streaming support, and minimal boilerplate for Kotlin-to-Kotlin communication.

**Stakeholders**: All client platforms (Android, iOS, Desktop), Server

## Goals / Non-Goals

**Goals:**
- Establish type-safe RPC infrastructure between clients and server
- Enable streaming for AI token delivery and optional real-time sync
- Keep service interfaces in shared module for compile-time safety
- Stub implementations to validate infrastructure before business logic

**Non-Goals:**
- Full business logic implementation (Phase 5+ concerns)
- Database integration (repository implementations come later)
- Production-ready error handling (stubs may throw for now)
- Non-Kotlin client support (no protobuf schema generation)

## Decisions

### Decision 1: Use kRPC (WebSocket-based) Transport

**What**: Use `kotlinx-rpc-krpc-*` artifacts with Ktor WebSocket integration.

**Why**:
- Native bidirectional streaming for `Flow<T>` return types
- Works well for mobile (persistent connection, efficient binary protocol)
- Ktor 3.x has first-class kRPC support

**Alternatives considered**:
- gRPC transport: More battle-tested but requires protobuf schema, less idiomatic
- Custom WebSocket protocol: Reinventing wheel, maintenance burden

### Decision 2: Service Interfaces in `shared/src/commonMain`

**What**: Define `@Rpc` interfaces in shared module's common source set.

**Why**:
- Single source of truth for API contract
- Compiler enforces client-server compatibility
- iOS, Android, Desktop all use same interface

**Structure**:
```
shared/src/commonMain/kotlin/com/getaltair/altair/rpc/
├── SyncService.kt
├── AuthService.kt
├── AiService.kt
└── dto/
    ├── CompletionRequest.kt
    └── PushResult.kt
```

### Decision 3: Stub Implementations First

**What**: Server implementations return mock/hardcoded data initially.

**Why**:
- Validates RPC infrastructure before database integration
- Allows client development to proceed in parallel
- Clear separation between transport layer and business logic

**Example stub**:
```kotlin
class AuthServiceImpl : AuthService {
    override suspend fun login(request: AuthRequest): AuthResponse {
        // Stub: Accept any credentials, return test token
        return AuthResponse(
            accessToken = "test-token",
            refreshToken = "test-refresh",
            expiresIn = 3600,
            userId = "01HWTEST000000000000000001",
            displayName = "Test User",
            role = "member"
        )
    }
}
```

### Decision 4: Koin Integration for RPC Clients

**What**: Register RPC service stubs in Koin modules for DI.

**Why**:
- Consistent with existing DI strategy (ADR-009)
- Easy to swap stubs for real implementations
- Testable via Koin test utilities

**Structure**:
```kotlin
val rpcModule = module {
    single { createRpcClient(get<ServerConfig>()) }
    single<SyncService> { get<RpcClient>().withService() }
    single<AuthService> { get<RpcClient>().withService() }
    single<AiService> { get<RpcClient>().withService() }
}
```

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| kotlinx-rpc is relatively new | Pin to stable version, fallback to REST if critical issues arise |
| iOS binary size increase | Monitor, optimize with R8/ProGuard if needed |
| Version coupling between client/server | Version RPC interfaces carefully, maintain compatibility |
| Streaming complexity | Start with request-response, add streaming incrementally |

## Migration Plan

1. Add dependencies (non-breaking, just new libs)
2. Define interfaces (no runtime impact until wired)
3. Wire server (new endpoints, existing routes unchanged)
4. Wire client (opt-in usage, existing code unchanged)
5. Gradual adoption in features

**Rollback**: Remove RPC configuration, revert to REST endpoints if needed.

## Open Questions

1. **Authentication flow**: Should RPC endpoints require auth middleware, or handle auth in
   AuthService only? (Leaning toward: auth middleware for SyncService/AiService, none for
   AuthService.login/register)

2. **Error serialization**: How should domain errors (e.g., `AuthError.InvalidCredentials`)
   serialize over RPC? (Options: sealed class with `@Serializable`, exception mapping, or
   `Either` wrapper in DTOs)

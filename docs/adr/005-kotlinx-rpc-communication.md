# ADR-005: kotlinx-rpc for Client-Server Communication

| Field             | Value                                           |
| ----------------- | ----------------------------------------------- |
| **Status**   | Superseded by [ADR-016](./016-ktor-rest-api.md) |
| **Date**     | 2026-01-26                                      |
| **Deciders** | Robert Hamilton                                 |

## Reason for Supersession

kotlinx-rpc's `registerService` lambda does not expose the underlying
`ApplicationCall`, making it impossible to:

- Inject authenticated user identity into service instances
- Create user-scoped repository instances per request
- Implement proper multi-user data isolation (ADR-012)

See [ADR-016: Ktor REST API](./016-ktor-rest-api.md) for the replacement architecture.

## Context

Altair's architecture includes:

- Desktop and mobile clients (Compose Multiplatform)
- Self-hosted server (Ktor)
- Real-time sync, AI service proxying, and standard CRUD operations

We need a communication protocol that:

1. Works efficiently for both request-response and streaming (AI completions, sync deltas)
2. Provides type safety between Kotlin client and Kotlin server
3. Supports bidirectional communication for real-time features
4. Has reasonable tooling and documentation

## Decision

Use **kotlinx-rpc** with gRPC transport for all client-server communication.

kotlinx-rpc is JetBrains' official RPC framework for Kotlin, shipping with Ktor 3.1.0 (Feb 2025).
For Kotlin-to-Kotlin communication, it provides type-safe service definitions without requiring
Protocol Buffer schemas.

```kotlin
// Shared module (commonMain)
@Rpc
interface SyncService : RemoteService {
    suspend fun pull(since: Long): SyncResponse
    suspend fun push(changes: List<ChangeSet>): PushResult
    fun streamChanges(): Flow<ChangeEvent>  // Server-sent events
}

// Server (Ktor)
class SyncServiceImpl : SyncService { ... }

// Client (Compose Multiplatform)
val syncService = rpcClient.withService<SyncService>()
val changes = syncService.pull(lastSyncVersion)
```

## Consequences

### Positive

- **Type safety**: Service interfaces shared between client and server; compiler catches mismatches
- **Streaming support**: `Flow<T>` return types for server-sent events (sync deltas, AI token
  streaming)
- **No protobuf**: Pure Kotlin definitions for Kotlin-to-Kotlin communication
- **Ktor integration**: First-class support in Ktor 3.1+; natural fit for server
- **gRPC interop**: Can expose gRPC protocol if non-Kotlin clients ever needed
- **Coroutines native**: Built on Kotlin coroutines; natural suspend functions and Flows

### Negative

- **JetBrains dependency**: kotlinx-rpc is relatively new; less battle-tested than raw gRPC
- **Learning curve**: Different mental model from REST APIs
- **Debugging**: Binary protocol harder to inspect than JSON REST (though tooling exists)
- **Version coupling**: Client and server must use compatible kotlinx-rpc versions

### Neutral

- Can fall back to raw HTTP/JSON for specific endpoints if needed
- Mobile clients use same protocol as desktop; no special handling required

## Service Definitions

### Core Services

```kotlin
@Rpc
interface SyncService : RemoteService {
    suspend fun pull(since: Long, entityTypes: Set<EntityType>): SyncResponse
    suspend fun push(changes: List<ChangeSet>): PushResult
    fun streamChanges(entityTypes: Set<EntityType>): Flow<ChangeEvent>
}

@Rpc
interface AiService : RemoteService {
    suspend fun embed(texts: List<String>): List<FloatArray>
    suspend fun transcribe(audioData: ByteArray, format: AudioFormat): String
    fun complete(request: CompletionRequest): Flow<String>  // Token streaming
}

@Rpc
interface AuthService : RemoteService {
    suspend fun login(credentials: Credentials): AuthToken
    suspend fun refresh(token: AuthToken): AuthToken
    suspend fun logout(token: AuthToken)
}
```

## Alternatives Considered

### Alternative 1: REST with OpenAPI

Traditional HTTP/JSON with generated clients from OpenAPI specs.

**Pros:**

- Universal, well-understood
- Easy to debug with curl/Postman
- Massive tooling ecosystem

**Rejected because:**

- No native streaming (would need SSE or WebSocket alongside REST)
- Type safety requires code generation from separate spec file
- More boilerplate for Kotlin-to-Kotlin communication

### Alternative 2: GraphQL

Query language with schema-first design.

**Pros:**

- Flexible queries, client specifies needed fields
- Strong typing with schema

**Rejected because:**

- Overkill for single-client-per-user application
- Streaming support (subscriptions) more complex than gRPC streams
- Additional runtime complexity (query parsing, validation)

### Alternative 3: Raw gRPC (protobuf)

Standard gRPC with Protocol Buffer schemas.

**Pros:**

- Mature, battle-tested
- Language-agnostic
- Excellent streaming support

**Rejected because:**

- Requires maintaining .proto files separate from Kotlin code
- Generated code less idiomatic than kotlinx-rpc
- More tooling friction for pure-Kotlin project

### Alternative 4: Ktor WebSockets + Custom Protocol

Hand-rolled binary or JSON protocol over WebSockets.

**Rejected because:**

- Must implement all framing, error handling, reconnection logic
- No type safety without significant custom tooling
- Reinventing wheel that kotlinx-rpc already provides

## References

- [kotlinx-rpc Documentation](https://kotlin.github.io/kotlinx-rpc/)
- [Ktor 3.1.0 Release Notes](https://ktor.io/changelog/3.1/) — gRPC via kotlinx-rpc
- [ADR-001: Kotlin Multiplatform Architecture](./001-single-tauri-application.md)
- [ADR-006: Server-Centralized AI Services](./006-server-centralized-ai.md)

# ADR-016: Ktor REST API (Supersedes kotlinx-rpc)

| Field             | Value                                           |
| ----------------- | ----------------------------------------------- |
| **Status**        | Accepted                                        |
| **Date**          | 2026-01-26                                      |
| **Deciders**      | Robert Hamilton                                 |
| **Supersedes**    | [ADR-005](./005-kotlinx-rpc-communication.md)   |
| **Superseded by** | —                                               |

## Context

ADR-005 selected kotlinx-rpc for client-server communication based on its promise of type-safe RPC with shared interfaces. During implementation, we discovered a fundamental limitation that blocks our multi-user architecture.

### The Blocking Issue

kotlinx-rpc's `registerService` lambda does not expose the underlying `ApplicationCall` or WebSocket session context:

```kotlin
// What we need
registerService<SyncService> { call ->
    val authContext = call.extractAuthContext()  // ❌ 'call' not available
    SyncServiceImpl(authContext, userScopedRepositories)
}

// What kotlinx-rpc provides
registerService<SyncService> { 
    SyncServiceImpl()  // No way to inject per-request auth context
}
```

This means we cannot:
- Inject authenticated user identity into service instances
- Create user-scoped repository instances per request
- Implement proper multi-user data isolation (ADR-012)

### Workarounds Considered

| Workaround | Problem |
|------------|---------|
| Pass token as parameter to every method | Verbose, validation overhead per call, poor DX |
| Coroutine context injection | kotlinx-rpc doesn't propagate custom context elements |
| Hybrid REST + RPC | Two patterns to maintain, defeats simplicity goal |

None of these workarounds are acceptable for a production multi-user application.

## Decision

**Replace kotlinx-rpc with pure Ktor REST API.**

All client-server communication will use standard HTTP REST endpoints with JSON serialization. The `shared` module will contain DTOs (request/response data classes) rather than RPC service interfaces.

## Architecture

### Server Side

```kotlin
// server/src/main/kotlin/com/getaltair/server/routes/SyncRoutes.kt
fun Route.syncRoutes(syncService: SyncService) {
    route("/api/sync") {
        post("/pull") {
            val userId = call.userId  // Extension from JWT principal
            val request = call.receive<SyncPullRequest>()
            val response = syncService.pull(userId, request)
            call.respond(response)
        }
        
        post("/push") {
            val userId = call.userId
            val request = call.receive<SyncPushRequest>()
            val response = syncService.push(userId, request)
            call.respond(response)
        }
    }
}
```

### Shared DTOs

```kotlin
// shared/src/commonMain/kotlin/com/getaltair/altair/shared/dto/sync/
@Serializable
data class SyncPullRequest(
    val since: Instant,
    val entityTypes: List<EntityType>? = null  // Optional filter
)

@Serializable
data class SyncPullResponse(
    val changes: List<EntityChange>,
    val serverTime: Instant,
    val hasMore: Boolean
)

@Serializable
data class SyncPushRequest(
    val changes: List<EntityChange>
)

@Serializable
data class SyncPushResponse(
    val accepted: List<EntityId>,
    val conflicts: List<ConflictInfo>,
    val serverTime: Instant
)
```

### Client Side

```kotlin
// composeApp/src/commonMain/kotlin/com/getaltair/altair/api/SyncApi.kt
class SyncApi(private val httpClient: HttpClient) {
    
    suspend fun pull(since: Instant): Either<ApiError, SyncPullResponse> = either {
        httpClient.post("/api/sync/pull") {
            contentType(ContentType.Application.Json)
            setBody(SyncPullRequest(since))
        }.bodyOrError<SyncPullResponse>().bind()
    }
    
    suspend fun push(changes: List<EntityChange>): Either<ApiError, SyncPushResponse> = either {
        httpClient.post("/api/sync/push") {
            contentType(ContentType.Application.Json)
            setBody(SyncPushRequest(changes))
        }.bodyOrError<SyncPushResponse>().bind()
    }
}
```

### HTTP Client Configuration

```kotlin
// composeApp/src/commonMain/kotlin/com/getaltair/altair/api/HttpClientFactory.kt
fun createHttpClient(tokenProvider: TokenProvider): HttpClient {
    return HttpClient {
        install(ContentNegotiation) {
            json(Json {
                ignoreUnknownKeys = true
                encodeDefaults = true
            })
        }
        
        install(Auth) {
            bearer {
                loadTokens {
                    tokenProvider.getTokens()?.let { 
                        BearerTokens(it.accessToken, it.refreshToken) 
                    }
                }
                refreshTokens {
                    tokenProvider.refresh()?.let {
                        BearerTokens(it.accessToken, it.refreshToken)
                    }
                }
            }
        }
        
        defaultRequest {
            url(tokenProvider.serverUrl)
            contentType(ContentType.Application.Json)
        }
    }
}
```

## API Endpoint Structure

```
/api
├── /auth
│   ├── POST /register     # Create account with invite code
│   ├── POST /login        # Get access + refresh tokens
│   ├── POST /refresh      # Refresh access token
│   └── POST /logout       # Invalidate refresh token
│
├── /sync
│   ├── POST /pull         # Get changes since timestamp
│   └── POST /push         # Push local changes
│
├── /inbox
│   ├── GET  /             # List inbox items
│   ├── POST /             # Create inbox item
│   ├── POST /triage/{id}  # Convert to Quest/Note/Item
│   └── DELETE /{id}       # Delete inbox item
│
├── /guidance
│   ├── GET  /quests       # List quests (with filters)
│   ├── POST /quests       # Create quest
│   ├── GET  /quests/{id}  # Get quest detail
│   ├── PUT  /quests/{id}  # Update quest
│   ├── DELETE /quests/{id}# Delete quest
│   └── ...                # Epics, Checkpoints, etc.
│
├── /knowledge
│   ├── GET  /notes        # List notes
│   ├── POST /notes        # Create note
│   ├── GET  /notes/{id}   # Get note detail
│   ├── PUT  /notes/{id}   # Update note
│   └── ...                # Folders, Tags, etc.
│
├── /tracking
│   ├── GET  /items        # List items
│   ├── POST /items        # Create item
│   └── ...                # Locations, Containers, etc.
│
├── /initiatives
│   ├── GET  /             # List initiatives
│   ├── POST /             # Create initiative
│   └── ...
│
├── /routines
│   ├── GET  /             # List routines
│   ├── POST /             # Create routine
│   └── ...
│
└── /ai
    ├── POST /transcribe   # Audio → text
    ├── POST /embed        # Text → embedding
    └── POST /complete     # LLM completion
```

## Consequences

### Positive

- **Full auth context access** — `ApplicationCall` available in every route handler
- **Single communication pattern** — No mental context switching between REST and RPC
- **Conventional API** — Easy to debug with curl, Postman, browser dev tools
- **OpenAPI compatible** — Can generate API documentation and client SDKs
- **Mature ecosystem** — Ktor's auth, rate limiting, caching plugins work naturally
- **Simpler dependencies** — Remove kotlinx-rpc from version catalog entirely
- **Better testability** — Standard HTTP testing patterns

### Negative

- **No auto-generated routing** — Must manually define routes (minimal overhead)
- **Interface duplication** — Server service interfaces separate from client API classes
- **Slightly more verbose** — Explicit route definitions vs. `registerService<T>`

### Neutral

- **Same type safety** — Shared DTOs + kotlinx.serialization provides compile-time safety
- **Same performance** — JSON over HTTP is sufficient for our use case
- **Same coroutine support** — Ktor is fully coroutine-native

## Migration from ADR-005

### Remove Dependencies

From `gradle/libs.versions.toml`:
```diff
- kotlinx-rpc = "0.4.0"
- [libraries]
- kotlinx-rpc-core = { module = "org.jetbrains.kotlinx:kotlinx-rpc-core", version.ref = "kotlinx-rpc" }
- kotlinx-rpc-krpc-client = { module = "org.jetbrains.kotlinx:kotlinx-rpc-krpc-client", version.ref = "kotlinx-rpc" }
- kotlinx-rpc-krpc-server = { module = "org.jetbrains.kotlinx:kotlinx-rpc-krpc-server", version.ref = "kotlinx-rpc" }
- kotlinx-rpc-krpc-ktor-client = { module = "org.jetbrains.kotlinx:kotlinx-rpc-krpc-ktor-client", version.ref = "kotlinx-rpc" }
- kotlinx-rpc-krpc-ktor-server = { module = "org.jetbrains.kotlinx:kotlinx-rpc-krpc-ktor-server", version.ref = "kotlinx-rpc" }
```

### Update Shared Module

Replace RPC service interfaces with DTO classes:
```diff
- // shared/.../rpc/SyncService.kt
- @Rpc
- interface SyncService : RemoteService {
-     suspend fun pull(since: Long): SyncResponse
- }

+ // shared/.../dto/sync/SyncDtos.kt
+ @Serializable data class SyncPullRequest(val since: Instant)
+ @Serializable data class SyncPullResponse(val changes: List<EntityChange>, ...)
```

### Update Server

Replace RPC registration with route definitions:
```diff
- import kotlinx.rpc.krpc.ktor.server.rpc
- 
- fun Application.configureRpc() {
-     rpc("/rpc") {
-         registerService<SyncService> { SyncServiceImpl() }
-     }
- }

+ fun Application.configureRouting() {
+     routing {
+         authenticate("jwt") {
+             syncRoutes(get<SyncService>())
+             guidanceRoutes(get<GuidanceService>())
+             // ...
+         }
+     }
+ }
```

### Update Client

Replace RPC client with HTTP API classes:
```diff
- val syncService = rpcClient.withService<SyncService>()
- val response = syncService.pull(since)

+ val syncApi = SyncApi(httpClient)
+ val response = syncApi.pull(since)
```

## References

- [ADR-005: kotlinx-rpc Communication](./005-kotlinx-rpc-communication.md) (superseded)
- [ADR-012: Multi-User Data Isolation](./012-multi-user-data-isolation.md)
- [Ktor Server Documentation](https://ktor.io/docs/server-create-http-client.html)
- [Ktor Client Documentation](https://ktor.io/docs/client-create-multiplatform-application.html)

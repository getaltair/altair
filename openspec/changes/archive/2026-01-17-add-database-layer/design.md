# Design: Hybrid Database Layer

## Context

Altair uses a hybrid database strategy (ADR-002):
- **Server**: SurrealDB for primary storage, sync hub, conflict resolution
- **Desktop**: SurrealDB embedded for full offline support with graph queries, vector search
- **Mobile**: SQLite via SQLDelight for quick capture and proven reliability

The shared module already defines 18 repository interfaces with Arrow `Either` return types. This change implements those interfaces for each platform.

## Goals / Non-Goals

**Goals:**
- Implement all repository interfaces for server using SurrealDB
- Create SQLDelight schema for mobile with type-safe queries
- Enable desktop to run offline with full database functionality
- Establish migration infrastructure for schema evolution
- Maintain user isolation across all queries

**Non-Goals:**
- Sync protocol implementation (Phase 9)
- Full-text search optimization (future enhancement)
- Vector search/embeddings integration (Phase 10)
- Data migration tooling from other sources

## Decisions

### Decision 1: surrealdb-java SDK

**What:** Use official `surrealdb.java` library for all JVM-based SurrealDB access.

**Why:**
- Official SDK maintained by SurrealDB team
- Supports both network and embedded modes
- JNI bindings to native Rust for performance
- Kotlin coroutine support via suspend functions

**Alternatives considered:**
- Raw HTTP client: More control but significant boilerplate
- surrealdb-kotlin: Unofficial, less mature

### Decision 2: SQLDelight for Mobile

**What:** Use SQLDelight with generated Kotlin code for mobile database access.

**Why:**
- Compile-time SQL verification catches errors early
- Type-safe generated Kotlin matches domain model
- Native driver support for Android and iOS
- Excellent KMP integration

**Alternatives considered:**
- Room: Android-only, no iOS support
- Realm: Heavy dependency, complex sync model
- Raw SQLite: No type safety, manual mapping

### Decision 3: Shared Repository Implementations for Server and Desktop

**What:** Server and Desktop share the same SurrealDB repository implementations.

**Why:**
- Both use surrealdb-java
- Reduces code duplication
- Same query logic across platforms
- Only difference is connection config (network vs embedded)

**Implementation:**
- Repository implementations live in `jvmMain` source set
- `SurrealDbClient` abstraction handles connection differences
- `DatabaseConfig` sealed class for network vs embedded config

### Decision 4: User Scoping at Repository Level

**What:** All repository implementations automatically filter by authenticated user ID.

**Why:**
- Prevents cross-user data access
- Consistent enforcement across all queries
- Auth context injected via constructor
- Matches existing repository interface contracts

**Implementation:**
```kotlin
class SurrealQuestRepository(
    private val db: SurrealDbClient,
    private val userId: String  // From auth context
) : QuestRepository {
    override suspend fun findById(id: String) = either {
        db.query<Quest>(
            "SELECT * FROM quest WHERE id = \$id AND user_id = \$userId",
            mapOf("id" to id, "userId" to userId)
        ).firstOrNull() ?: raise(QuestError.NotFound(id))
    }
}
```

### Decision 5: Schema Migration Strategy

**What:** Version-based migrations with forward-only evolution.

**SurrealDB:**
- Migrations stored in `server/src/main/resources/migrations/`
- Each file: `V{version}__{description}.surql`
- Migration table tracks applied versions
- Run at application startup

**SQLDelight:**
- Migrations in `.sqm` files in `sqldelight/migrations/`
- SQLDelight handles version tracking automatically
- Schema version in `Database.sq`

### Decision 6: Connection Pooling (Server)

**What:** Use connection pooling for server SurrealDB connections.

**Why:**
- Multiple concurrent requests
- Avoid connection overhead per request
- Ktor handles many concurrent connections

**Implementation:**
- HikariCP-style pool management
- Configurable pool size via environment variables
- Health check endpoint validates connectivity

## Risks / Trade-offs

### Risk: SurrealDB Stability

SurrealDB is newer than traditional databases.

**Mitigation:**
- Use stable 2.x release
- Comprehensive integration tests
- Server can switch to PostgreSQL if needed (interfaces abstract implementation)

### Risk: Mobile-Desktop Feature Gap

Mobile SQLite lacks graph queries and vector search.

**Mitigation:**
- Document desktop-only features clearly
- Mobile handles quick capture, desktop does heavy lifting
- Sync brings desktop-computed results to mobile

### Risk: Schema Drift

SurrealDB and SQLite schemas could diverge.

**Mitigation:**
- Single source of truth in persistence.md
- Schema tests verify consistency
- Automated schema comparison in CI

## Migration Plan

**Phase 1 (This change):**
1. Add dependencies to build files
2. Create initial schema (v1) for both engines
3. Implement repositories
4. Wire into application startup

**Phase 2 (Future):**
- Add migration runner to CI
- Create schema comparison tests
- Document migration procedures

**Rollback:**
- Remove database dependencies from build files
- Delete repository implementations
- Revert to stub/mock implementations if needed

## Open Questions

1. **Pool size defaults:** What's the optimal connection pool size for typical self-hosted server?
   - Proposed: Start with 10, make configurable

2. **Embedded DB location:** Where should desktop store SurrealDB files?
   - Proposed: `$APP_DATA/altair/db/` per persistence.md

3. **SQLDelight database name:** What schema name for mobile?
   - Proposed: `AltairDatabase`

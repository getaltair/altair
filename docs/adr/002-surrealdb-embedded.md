# ADR-002: Hybrid Database Strategy

| Field          | Value                                       |
| -------------- | ------------------------------------------- |
| **Status**     | Accepted                                    |
| **Date**       | 2026-01-09                                  |
| **Deciders**   | Robert Hamilton                             |
| **Supersedes** | ADR-002 (SurrealDB Embedded only, original) |

## Context

Altair requires a database strategy that supports:

1. **Desktop**: Full-featured offline-capable operation with graph relationships, full-text search,
   vector search
2. **Mobile**: Quick capture and basic viewing—not full feature parity
3. **Server**: Primary data store for sync, shared across all user devices
4. **Cross-device sync**: User-operated server synchronizes data between desktop and mobile clients

The mobile "quick capture" use case doesn't require graph traversal or complex queries—just fast
text entry and basic reads. Desktop does the heavy lifting with full search and relationship
features.

## Decision

Use a **hybrid database architecture**:

| Platform | Database           | Purpose                                        |
| -------- | ------------------ | ---------------------------------------------- |
| Desktop  | SurrealDB embedded | Full graph queries, vector search, FTS, events |
| Mobile   | SQLite embedded    | Quick capture, basic CRUD, proven reliability  |
| Server   | SurrealDB          | Primary store, sync hub, conflict resolution   |

A **custom sync bridge** handles bidirectional synchronization between SurrealDB (desktop/server)
and SQLite (mobile). Conflict resolution happens server-side with last-write-wins semantics for
simple fields and custom merge logic for complex structures.

## Consequences

### Positive

- **Mobile simplicity**: SQLite is battle-tested on mobile (15+ years), tiny footprint, zero
  configuration
- **Desktop power**: SurrealDB graph traversal for note↔quest↔item relationships, native vector
  search
- **Appropriate complexity**: Mobile doesn't pay for features it doesn't use
- **Sync flexibility**: Server controls conflict resolution; clients are thin
- **Offline-first**: Both desktop and mobile work fully offline; sync when connected

### Negative

- **Sync bridge complexity**: Must maintain schema mapping between SurrealDB and SQLite
- **Feature asymmetry**: Mobile can't do semantic search or complex graph queries locally
- **Two database libraries**: Kotlin codebases need both SurrealDB and SQLite dependencies
- **Testing overhead**: Must test sync behavior between different database engines

### Neutral

- Mobile-created data syncs to server immediately when online; desktop sees it on next sync
- Desktop-only features (semantic search, graph traversal) are clearly documented as such

## Database Details

### SurrealDB (Desktop/Server)

- **Version**: 2.0+ with SurrealKV storage engine
- **Access**: surrealdb.java with JNI bindings to native Rust (Android support included)
- **Schema**: Singular snake_case tables, ULID IDs, soft delete with `deleted_at`
- **Features used**: RELATE for graph edges, vector fields for embeddings, FTS for search

### SQLite (Mobile)

- **Library**: SQLDelight for Kotlin Multiplatform (type-safe generated queries)
- **Schema**: Mirrors SurrealDB tables with foreign keys instead of graph relations
- **Features used**: Basic CRUD, simple indexes, FTS5 for basic text search

### Sync Protocol

- Clients track `sync_version` per entity
- Server maintains authoritative `sync_version` sequence
- Pull: Client requests changes since last known version
- Push: Client sends local changes with base version for conflict detection
- Conflicts: Server resolves and returns merged result

## Alternatives Considered

### Alternative 1: SurrealDB Only (All Platforms)

Use SurrealDB embedded on mobile via surrealdb.java JNI bindings.

**Rejected because:**

- SurrealDB's built-in sync is not yet implemented (GitHub discussions show founder exploring CRDTs,
  nothing shipped)
- JNI adds complexity and potential crash vectors on mobile
- Mobile "quick capture" doesn't need graph features—overkill

### Alternative 2: SQLite Only (All Platforms)

Use SQLite everywhere with cr-sqlite or ElectricSQL for sync.

**Rejected because:**

- Graph relationships require complex JOIN chains or recursive CTEs
- Vector search via sqlite-vec extension adds deployment complexity
- Would lose SurrealDB's elegant graph query syntax on desktop

### Alternative 3: CouchDB/PouchDB

Document database with built-in sync protocol.

**Rejected because:**

- JavaScript-based (PouchDB); poor fit for Kotlin backend
- No native graph relationships
- Limited vector search support
- Would require Node.js sidecar or complex bridging

## References

- [SurrealDB Java SDK](https://github.com/surrealdb/surrealdb.java) — Android support via JNI
- [SQLDelight](https://cashapp.github.io/sqldelight/) — Kotlin Multiplatform SQL
- [ADR-001: Kotlin Multiplatform Architecture](./001-single-tauri-application.md)
- [ADR-005: kotlinx-rpc Communication](./005-kotlinx-rpc-communication.md)
- PRD Core, Section 8: Cross-Application Integration

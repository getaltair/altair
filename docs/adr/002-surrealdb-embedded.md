# ADR-002: SurrealDB for Persistence

| Field        | Value           |
| ------------ | --------------- |
| **Status**   | Accepted        |
| **Date**     | 2026-01-08      |
| **Deciders** | Robert Hamilton |

## Context

Altair requires a database that supports:

1. **Local-first operation**: Full functionality offline with optional cloud sync
2. **Graph relationships**: Notes link to quests link to items; bidirectional traversal required
3. **Embedded deployment**: No separate database server process
4. **Full-text search**: Search across notes, quests, and items
5. **Vector search**: Semantic similarity for auto-discovery features
6. **Rust integration**: Native client library for Tauri backend

## Decision

Use **SurrealDB 2.0+** in embedded mode as the primary datastore, with optional SurrealDB Cloud for cross-device sync.

SurrealDB runs as a library within the Tauri Rust backend—no separate process. Data stored in local files with SurrealKV storage engine.

## Consequences

### Positive

- **Native graph queries**: `RELATE` and graph traversal built into query language; no ORM gymnastics for note↔quest↔item relationships
- **Embedded mode**: Single process, no database server to manage
- **Full-text search**: Built-in `@@` operator with analyzers; no separate search index
- **Vector search**: Native vector fields and KNN queries for semantic similarity (FR-K-125, FR-T-038)
- **Optional cloud sync**: SurrealDB Cloud provides managed sync without changing query code
- **Rust-native**: `surrealdb` crate with async support; natural fit for Tauri backend
- **Schemaless flexibility**: Can evolve data model without rigid migrations during early development
- **Multi-model**: Document, graph, and relational patterns in one database

### Negative

- **Less mature**: Smaller community than SQLite/PostgreSQL; fewer Stack Overflow answers
- **Migration tooling**: Schema migration tools less developed than established databases
- **Sync complexity**: Cloud sync is newer feature; conflict resolution semantics still evolving
- **Learning curve**: SurrealQL is powerful but unfamiliar to most developers
- **Binary size**: Adds to application size (though acceptable for desktop app)

### Neutral

- Performance characteristics still being benchmarked for Altair's specific workloads
- May need to implement custom conflict resolution for complex sync scenarios

## Alternatives Considered

### Alternative 1: SQLite + Extensions

SQLite with FTS5 for full-text search and sqlite-vec for vector search.

**Pros:**

- Battle-tested, massive community
- Excellent Rust support (rusqlite)
- Tiny binary footprint

**Rejected because:**

- Graph relationships require complex JOIN chains or recursive CTEs
- Vector search via extension adds deployment complexity
- No built-in cloud sync; would need separate sync layer (e.g., cr-sqlite, ElectricSQL)
- Relational model fights against document-like note/quest structures

### Alternative 2: SQLite + libSQL (Turso)

libSQL fork with vector search and Turso cloud sync.

**Pros:**

- SQLite compatibility with modern features
- Turso provides managed sync

**Rejected because:**

- Still fundamentally relational; graph queries remain awkward
- Turso sync model less flexible than SurrealDB for complex conflict resolution
- Smaller ecosystem than either SQLite or SurrealDB

### Alternative 3: PouchDB/CouchDB

JavaScript document database with built-in sync.

**Rejected because:**

- JavaScript-based; poor fit for Rust backend
- No native graph relationships
- Limited vector search support
- Would require Node.js sidecar or WebAssembly shim

### Alternative 4: Datomic-style (Local)

Immutable, append-only database with time-travel queries.

**Rejected because:**

- No production-ready embedded Rust implementation
- Overkill for single-user desktop application
- Storage growth concerns for append-only model

## References

- [SurrealDB Documentation](https://surrealdb.com/docs)
- [SurrealDB Rust SDK](https://github.com/surrealdb/surrealdb/tree/main/lib)
- PRD Core, Section 8: Cross-Application Integration
- FR-K-111 through FR-K-124: Search requirements
- FR-K-125 through FR-K-130: Auto-discovery requirements

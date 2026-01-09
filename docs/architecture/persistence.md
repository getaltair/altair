# Persistence

## Purpose

This document describes how Altair stores and manages data, including the hybrid database strategy, schema design
patterns, query strategies, and sync considerations.

---

## Database Architecture

### Hybrid Strategy

Altair uses different databases optimized for each platform:

| Platform | Database          | Rationale                                            |
| -------- | ----------------- | ---------------------------------------------------- |
| Desktop  | SurrealDB embedded | Graph queries, vector search, full-text search       |
| Mobile   | SQLite (SQLDelight) | Proven reliability, minimal footprint, quick capture |
| Server   | SurrealDB          | Primary store, sync hub, conflict resolution         |

See [ADR-002](../adr/002-surrealdb-embedded.md) for the full decision rationale.

### Desktop: SurrealDB Embedded

SurrealDB runs in-process using the SurrealKV storage engine via surrealdb.java JNI bindings.

| Aspect         | Configuration                   |
| -------------- | ------------------------------- |
| Mode           | Embedded (in-process)           |
| Storage Engine | SurrealKV                       |
| Location       | `$APP_DATA/altair/db/`          |
| Concurrency    | Single writer, multiple readers |
| Access         | surrealdb.java (Kotlin)         |

**Why SurrealDB for Desktop:**

| Need                | SurrealDB Capability                              |
| ------------------- | ------------------------------------------------- |
| Graph relationships | Native graph queries with `->` and `<-` operators |
| Full-text search    | Built-in search indexes                           |
| Vector similarity   | Native vector type and KNN search                 |
| Embedded operation  | No external process required                      |
| Flexible schema     | Schemaless with optional schema enforcement       |

### Mobile: SQLite (SQLDelight)

SQLite runs embedded via SQLDelight for type-safe Kotlin queries.

| Aspect         | Configuration                          |
| -------------- | -------------------------------------- |
| Mode           | Embedded                               |
| Location       | Platform-standard app data directory   |
| Access         | SQLDelight generated Kotlin code       |
| Query Safety   | Compile-time verified SQL              |

**Why SQLite for Mobile:**

| Need                | SQLite Capability                          |
| ------------------- | ------------------------------------------ |
| Quick capture       | Fast writes, minimal overhead              |
| Reliability         | 15+ years of mobile production use         |
| Battery efficiency  | No background processes                    |
| Minimal footprint   | ~500KB library size                        |
| Basic search        | FTS5 for text search                       |

### Server: SurrealDB

Server runs SurrealDB as a container alongside the Ktor application.

| Aspect         | Configuration                          |
| -------------- | -------------------------------------- |
| Mode           | Server (networked)                     |
| Storage        | File-backed (`/data/altair.db`)        |
| Access         | surrealdb.java (Kotlin)                |
| Volume         | Docker volume for persistence          |

---

## Schema Design

### Namespace Structure (SurrealDB)

```text
namespace: altair
├── database: main
│   ├── tables: epic, quest, checkpoint, energy_budget
│   ├── tables: note, note_link, folder, tag, attachment
│   └── tables: item, custom_field, location, container, template, field_def
└── database: sync
    └── tables: sync_state, conflict_log
```

### SQLite Schema (Mobile)

```sql
-- Mirrors SurrealDB tables with foreign keys instead of graph relations
CREATE TABLE quest (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'backlog',
    energy_cost INTEGER NOT NULL DEFAULT 1,
    epic_id TEXT REFERENCES epic(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE note (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT,
    folder_id TEXT REFERENCES folder(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);

-- ... similar patterns for other entities
```

### Table Conventions

Both databases follow consistent conventions:

- Table names are `snake_case` singular (e.g., `quest`, `note`, `item`)
- Record IDs use ULID strings: `01HXK3...`
- Timestamps stored as ISO 8601 strings
- Soft-deleted records have `deleted_at` field set
- All entities include `sync_version` for sync tracking

### Indexes

**SurrealDB (Desktop/Server):**

| Table | Index         | Fields           | Purpose                  |
| ----- | ------------- | ---------------- | ------------------------ |
| quest | status_idx    | status           | Filter by status         |
| quest | epic_idx      | epic_id          | Quests in an Epic        |
| note  | folder_idx    | folder_id        | Notes in a Folder        |
| note  | title_idx     | folder_id, title | Unique title per folder  |
| note  | search_idx    | content          | Full-text search         |
| note  | embedding_idx | embedding        | Vector similarity (HNSW) |
| item  | location_idx  | location_id      | Items at a Location      |
| item  | container_idx | container_id     | Items in a Container     |

**SQLite (Mobile):**

| Table | Index             | Columns          | Purpose                 |
| ----- | ----------------- | ---------------- | ----------------------- |
| quest | quest_status_idx  | status           | Filter by status        |
| quest | quest_epic_idx    | epic_id          | Quests in an Epic       |
| note  | note_folder_idx   | folder_id        | Notes in a Folder       |
| note  | note_title_idx    | folder_id, title | Unique title per folder |
| item  | item_location_idx | location_id      | Items at a Location     |

### Relations

**SurrealDB (Desktop/Server):**

Uses two relation patterns:

- **Record Links** for hierarchical relationships: `quest.epic_id = epic:xyz`
- **Graph Edges** for cross-module relationships: `note:abc ->links-> note:xyz`

**SQLite (Mobile):**

Uses foreign keys for all relationships:

```sql
epic_id TEXT REFERENCES epic(id)
folder_id TEXT REFERENCES folder(id)
```

Cross-module links stored in junction tables:

```sql
CREATE TABLE quest_note (
    quest_id TEXT REFERENCES quest(id),
    note_id TEXT REFERENCES note(id),
    PRIMARY KEY (quest_id, note_id)
);
```

---

## Query Patterns

### Desktop (SurrealDB)

```kotlin
// Basic CRUD
suspend fun getQuest(id: String): Quest? =
    db.query("SELECT * FROM quest WHERE id = \$id", mapOf("id" to id))
        .firstOrNull()

// Graph traversal
suspend fun getQuestNotes(questId: String): List<Note> =
    db.query("SELECT ->references->note.* FROM quest WHERE id = \$id", mapOf("id" to questId))
        .flatten()

// Vector similarity
suspend fun findSimilarNotes(embedding: FloatArray, limit: Int = 5): List<Note> =
    db.query("""
        SELECT *, vector::similarity::cosine(embedding, \$vec) AS score
        FROM note
        WHERE embedding IS NOT NULL
        ORDER BY score DESC
        LIMIT \$limit
    """, mapOf("vec" to embedding, "limit" to limit))
```

### Mobile (SQLDelight)

```kotlin
// Generated from .sq file
val getQuest: Quest? = questQueries.getById(id).executeAsOneOrNull()

// Junction table for cross-module
val questNotes: List<Note> = questNoteQueries.getNotesForQuest(questId).executeAsList()

// Full-text search
val searchResults: List<Note> = noteQueries.search(query).executeAsList()
```

---

## Sync Protocol

### Version Tracking

All entities include `sync_version`:

- Monotonically increasing counter per entity
- Server maintains authoritative version sequence
- Clients track last known version per entity type

### Pull Flow

```
Client                          Server
   │                               │
   │─── Pull(since: 1000) ────────>│
   │                               │
   │<── Changes(v1001..v1050) ─────│
   │                               │
   │    Apply changes locally      │
   │    Update last_sync = 1050    │
```

### Push Flow

```
Client                          Server
   │                               │
   │─── Push(changes, base: 1050)─>│
   │                               │
   │    Validate base version      │
   │    Apply changes              │
   │    Resolve conflicts          │
   │                               │
   │<── Result(v1051, conflicts)───│
   │                               │
   │    Apply server decisions     │
```

### Conflict Resolution

Server-side resolution with last-write-wins for simple fields:

| Scenario              | Resolution                                |
| --------------------- | ----------------------------------------- |
| Same field changed    | Latest `updated_at` wins                  |
| Different fields      | Merge both changes                        |
| Entity deleted        | Delete wins (soft delete propagates)      |
| Complex structures    | Custom merge logic (e.g., note content)   |

---

## Migrations

### SurrealDB Migrations

```kotlin
data class Migration(
    val version: Int,
    val name: String,
    val up: String  // SurrealQL
)

val migrations = listOf(
    Migration(1, "create_quest_table", """
        DEFINE TABLE quest SCHEMAFULL;
        DEFINE FIELD title ON quest TYPE string;
        DEFINE FIELD status ON quest TYPE string DEFAULT 'backlog';
        -- ...
    """),
    Migration(2, "add_sync_version", """
        DEFINE FIELD sync_version ON quest TYPE int DEFAULT 0;
        -- ...
    """)
)
```

### SQLDelight Migrations

Stored in `.sqm` files with version numbers:

```
src/commonMain/sqldelight/
├── com/getaltair/altair/
│   ├── Quest.sq
│   ├── Note.sq
│   └── migrations/
│       ├── 1.sqm
│       └── 2.sqm
```

---

## Backup and Recovery

### Desktop

- Export: Dump all tables to JSON via SurrealQL `SELECT * FROM ...`
- Import: Transaction-wrapped inserts
- Location: User-selected file

### Server

- Volume backup: `docker compose exec surrealdb surreal export`
- Scheduled: Configurable cron job
- Retention: User-configured

### Mobile

- Sync to server is the primary backup mechanism
- SQLite database included in platform backup (iCloud, Google backup)

---

## References

- [SurrealDB Documentation](https://surrealdb.com/docs)
- [SQLDelight Documentation](https://cashapp.github.io/sqldelight/)
- [ADR-002: Hybrid Database Strategy](../adr/002-surrealdb-embedded.md)
- [ADR-005: kotlinx-rpc Communication](../adr/005-kotlinx-rpc-communication.md)

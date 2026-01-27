# Persistence

## Purpose

This document describes how Altair stores and manages data, including the hybrid database strategy,
schema design patterns, multi-user data isolation, query strategies, and sync considerations.

---

## Database Architecture

### Hybrid Strategy

Altair uses different databases optimized for each platform:

| Platform | Database            | Rationale                                            |
| -------- | ------------------- | ---------------------------------------------------- |
| Desktop  | SurrealDB embedded  | Graph queries, vector search, full-text search       |
| Mobile   | SQLite (SQLDelight) | Proven reliability, minimal footprint, quick capture |
| Server   | SurrealDB           | Primary store, sync hub, conflict resolution         |

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

| Aspect       | Configuration                        |
| ------------ | ------------------------------------ |
| Mode         | Embedded                             |
| Location     | Platform-standard app data directory |
| Access       | SQLDelight generated Kotlin code     |
| Query Safety | Compile-time verified SQL            |

**Why SQLite for Mobile:**

| Need               | SQLite Capability                  |
| ------------------ | ---------------------------------- |
| Quick capture      | Fast writes, minimal overhead      |
| Reliability        | 15+ years of mobile production use |
| Battery efficiency | No background processes            |
| Minimal footprint  | ~500KB library size                |
| Basic search       | FTS5 for text search               |

### Server: SurrealDB

Server runs SurrealDB as a container alongside the Ktor application.

| Aspect  | Configuration                   |
| ------- | ------------------------------- |
| Mode    | Server (networked)              |
| Storage | File-backed (`/data/altair.db`) |
| Access  | surrealdb.java (Kotlin)         |
| Volume  | Docker volume for persistence   |

---

## Multi-User Data Isolation

### User Scoping

All user-generated content is scoped by `user_id`:

```sql
-- Every entity query includes user filter
SELECT * FROM quest WHERE user_id = $current_user AND status = 'active';
SELECT * FROM note WHERE user_id = $current_user;
SELECT * FROM item WHERE user_id = $current_user AND location_id = $loc;
```

**Isolation Rules:**

- Every entity (Initiative, Quest, Note, Item, etc.) has a `user_id` field
- All queries filter by authenticated user's ID
- No cross-user data access in v1
- Admin users cannot view member data (only manage accounts)
- Future collaboration will use explicit sharing, not implicit access

### Server-Side Enforcement

The server enforces isolation at the repository level:

```kotlin
class QuestRepository(
    private val db: Database,
    private val auth: AuthContext
) {
    suspend fun findAll(): List<Quest> =
        db.query(
            "SELECT * FROM quest WHERE user_id = \$userId",
            mapOf("userId" to auth.currentUserId)
        )
    
    suspend fun create(quest: QuestCreate): Quest {
        val created = quest.copy(userId = auth.currentUserId)
        return db.create("quest", created)
    }
}
```

### Client-Side Context

Mobile and desktop clients include user context in all sync requests:

- JWT token contains user ID
- Server validates token and extracts user scope
- All synced data filtered to authenticated user

---

## Schema Design

### Namespace Structure (SurrealDB)

```text
namespace: altair
├── database: main
│   ├── tables: user, initiative, inbox_item, routine
│   ├── tables: epic, quest, checkpoint, energy_budget
│   ├── tables: note, note_link, folder, tag, attachment
│   └── tables: item, custom_field, location, container, template, field_def
└── database: sync
    └── tables: sync_state, conflict_log, device
```

### System-Level Tables (SurrealDB)

```surql
-- User (server-side only, not synced to clients)
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD email ON user TYPE option<string>;
DEFINE FIELD password_hash ON user TYPE string;
DEFINE FIELD role ON user TYPE string DEFAULT 'member';
DEFINE FIELD status ON user TYPE string DEFAULT 'active';
DEFINE FIELD storage_used ON user TYPE int DEFAULT 0;
DEFINE FIELD storage_quota ON user TYPE option<int>;
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD last_login_at ON user TYPE option<datetime>;
DEFINE INDEX username_idx ON user FIELDS username UNIQUE;
DEFINE INDEX email_idx ON user FIELDS email UNIQUE;

-- Initiative
DEFINE TABLE initiative SCHEMAFULL;
DEFINE FIELD user_id ON initiative TYPE record<user>;
DEFINE FIELD name ON initiative TYPE string;
DEFINE FIELD description ON initiative TYPE option<string>;
DEFINE FIELD parent_id ON initiative TYPE option<record<initiative>>;
DEFINE FIELD ongoing ON initiative TYPE bool DEFAULT false;
DEFINE FIELD target_date ON initiative TYPE option<datetime>;
DEFINE FIELD status ON initiative TYPE string DEFAULT 'active';
DEFINE FIELD focused ON initiative TYPE bool DEFAULT false;
DEFINE FIELD created_at ON initiative TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON initiative TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON initiative TYPE option<datetime>;
DEFINE FIELD sync_version ON initiative TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON initiative FIELDS user_id;
DEFINE INDEX status_idx ON initiative FIELDS user_id, status;

-- Inbox Item
DEFINE TABLE inbox_item SCHEMAFULL;
DEFINE FIELD user_id ON inbox_item TYPE record<user>;
DEFINE FIELD content ON inbox_item TYPE string;
DEFINE FIELD source ON inbox_item TYPE string;
DEFINE FIELD created_at ON inbox_item TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON inbox_item TYPE option<datetime>;
DEFINE FIELD sync_version ON inbox_item TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON inbox_item FIELDS user_id;

-- Routine
DEFINE TABLE routine SCHEMAFULL;
DEFINE FIELD user_id ON routine TYPE record<user>;
DEFINE FIELD name ON routine TYPE string;
DEFINE FIELD description ON routine TYPE option<string>;
DEFINE FIELD schedule ON routine TYPE string;
DEFINE FIELD time_of_day ON routine TYPE option<string>;
DEFINE FIELD energy_cost ON routine TYPE int DEFAULT 1;
DEFINE FIELD initiative_id ON routine TYPE option<record<initiative>>;
DEFINE FIELD active ON routine TYPE bool DEFAULT true;
DEFINE FIELD next_due ON routine TYPE option<datetime>;
DEFINE FIELD created_at ON routine TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON routine TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON routine TYPE option<datetime>;
DEFINE FIELD sync_version ON routine TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON routine FIELDS user_id;
DEFINE INDEX active_idx ON routine FIELDS user_id, active;

-- SourceDocument
DEFINE TABLE source_document SCHEMAFULL;
DEFINE FIELD user_id ON source_document TYPE record<user>;
DEFINE FIELD title ON source_document TYPE string;
DEFINE FIELD source_type ON source_document TYPE string;
DEFINE FIELD source_path ON source_document TYPE string;
DEFINE FIELD mime_type ON source_document TYPE string;
DEFINE FIELD content_hash ON source_document TYPE string;
DEFINE FIELD extracted_text ON source_document TYPE option<string>;
DEFINE FIELD embedding ON source_document TYPE option<array<float>>;
DEFINE FIELD status ON source_document TYPE string DEFAULT 'pending';
DEFINE FIELD error_message ON source_document TYPE option<string>;
DEFINE FIELD initiative_id ON source_document TYPE option<record<initiative>>;
DEFINE FIELD watched_folder_id ON source_document TYPE option<record<watched_folder>>;
DEFINE FIELD last_synced_at ON source_document TYPE option<datetime>;
DEFINE FIELD created_at ON source_document TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON source_document TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON source_document TYPE option<datetime>;
DEFINE FIELD sync_version ON source_document TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON source_document FIELDS user_id;
DEFINE INDEX status_idx ON source_document FIELDS user_id, status;
DEFINE INDEX hash_idx ON source_document FIELDS user_id, content_hash;
DEFINE INDEX search_idx ON source_document FIELDS extracted_text SEARCH ANALYZER snowball(ENGLISH);
DEFINE INDEX embedding_idx ON source_document FIELDS embedding MTREE DIMENSION 384;

-- SourceAnnotation
DEFINE TABLE source_annotation SCHEMAFULL;
DEFINE FIELD user_id ON source_annotation TYPE record<user>;
DEFINE FIELD source_document_id ON source_annotation TYPE record<source_document>;
DEFINE FIELD anchor_type ON source_annotation TYPE string;
DEFINE FIELD anchor_value ON source_annotation TYPE option<string>;
DEFINE FIELD anchor_fingerprint ON source_annotation TYPE option<object>;
DEFINE FIELD content ON source_annotation TYPE string;
DEFINE FIELD created_at ON source_annotation TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON source_annotation TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON source_annotation TYPE option<datetime>;
DEFINE FIELD sync_version ON source_annotation TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON source_annotation FIELDS user_id;
DEFINE INDEX document_idx ON source_annotation FIELDS user_id, source_document_id;

-- WatchedFolder
DEFINE TABLE watched_folder SCHEMAFULL;
DEFINE FIELD user_id ON watched_folder TYPE record<user>;
DEFINE FIELD path ON watched_folder TYPE string;
DEFINE FIELD include_patterns ON watched_folder TYPE option<array<string>>;
DEFINE FIELD exclude_patterns ON watched_folder TYPE option<array<string>>;
DEFINE FIELD initiative_id ON watched_folder TYPE option<record<initiative>>;
DEFINE FIELD scan_interval ON watched_folder TYPE int DEFAULT 60;
DEFINE FIELD status ON watched_folder TYPE string DEFAULT 'active';
DEFINE FIELD last_scanned_at ON watched_folder TYPE option<datetime>;
DEFINE FIELD created_at ON watched_folder TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON watched_folder TYPE datetime DEFAULT time::now();
DEFINE INDEX user_idx ON watched_folder FIELDS user_id;
DEFINE INDEX path_idx ON watched_folder FIELDS user_id, path UNIQUE;

-- ExtractionJob (server-side only)
DEFINE TABLE extraction_job SCHEMAFULL;
DEFINE FIELD source_document_id ON extraction_job TYPE record<source_document>;
DEFINE FIELD status ON extraction_job TYPE string DEFAULT 'queued';
DEFINE FIELD error_message ON extraction_job TYPE option<string>;
DEFINE FIELD created_at ON extraction_job TYPE datetime DEFAULT time::now();
DEFINE FIELD started_at ON extraction_job TYPE option<datetime>;
DEFINE FIELD completed_at ON extraction_job TYPE option<datetime>;
DEFINE INDEX document_idx ON extraction_job FIELDS source_document_id;
DEFINE INDEX status_idx ON extraction_job FIELDS status;
```

### Module Tables (SurrealDB)

```surql
-- Epic (Guidance)
DEFINE TABLE epic SCHEMAFULL;
DEFINE FIELD user_id ON epic TYPE record<user>;
DEFINE FIELD title ON epic TYPE string;
DEFINE FIELD description ON epic TYPE option<string>;
DEFINE FIELD status ON epic TYPE string DEFAULT 'active';
DEFINE FIELD initiative_id ON epic TYPE option<record<initiative>>;
DEFINE FIELD created_at ON epic TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON epic TYPE datetime DEFAULT time::now();
DEFINE FIELD completed_at ON epic TYPE option<datetime>;
DEFINE FIELD deleted_at ON epic TYPE option<datetime>;
DEFINE FIELD sync_version ON epic TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON epic FIELDS user_id;
DEFINE INDEX initiative_idx ON epic FIELDS user_id, initiative_id;

-- Quest (Guidance)
DEFINE TABLE quest SCHEMAFULL;
DEFINE FIELD user_id ON quest TYPE record<user>;
DEFINE FIELD title ON quest TYPE string;
DEFINE FIELD description ON quest TYPE option<string>;
DEFINE FIELD status ON quest TYPE string DEFAULT 'backlog';
DEFINE FIELD energy_cost ON quest TYPE int DEFAULT 1;
DEFINE FIELD epic_id ON quest TYPE option<record<epic>>;
DEFINE FIELD routine_id ON quest TYPE option<record<routine>>;
DEFINE FIELD created_at ON quest TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON quest TYPE datetime DEFAULT time::now();
DEFINE FIELD started_at ON quest TYPE option<datetime>;
DEFINE FIELD completed_at ON quest TYPE option<datetime>;
DEFINE FIELD deleted_at ON quest TYPE option<datetime>;
DEFINE FIELD sync_version ON quest TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON quest FIELDS user_id;
DEFINE INDEX status_idx ON quest FIELDS user_id, status;
DEFINE INDEX epic_idx ON quest FIELDS user_id, epic_id;
DEFINE INDEX routine_idx ON quest FIELDS user_id, routine_id;

-- Note (Knowledge) with Initiative link
DEFINE TABLE note SCHEMAFULL;
DEFINE FIELD user_id ON note TYPE record<user>;
DEFINE FIELD title ON note TYPE string;
DEFINE FIELD content ON note TYPE option<string>;
DEFINE FIELD folder_id ON note TYPE option<record<folder>>;
DEFINE FIELD initiative_id ON note TYPE option<record<initiative>>;
DEFINE FIELD embedding ON note TYPE option<array<float>>;
DEFINE FIELD created_at ON note TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON note TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON note TYPE option<datetime>;
DEFINE FIELD sync_version ON note TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON note FIELDS user_id;
DEFINE INDEX folder_idx ON note FIELDS user_id, folder_id;
DEFINE INDEX initiative_idx ON note FIELDS user_id, initiative_id;
DEFINE INDEX title_idx ON note FIELDS user_id, folder_id, title UNIQUE;
DEFINE INDEX search_idx ON note FIELDS content SEARCH ANALYZER snowball(ENGLISH);
DEFINE INDEX embedding_idx ON note FIELDS embedding MTREE DIMENSION 384;

-- Item (Tracking) with Initiative link
DEFINE TABLE item SCHEMAFULL;
DEFINE FIELD user_id ON item TYPE record<user>;
DEFINE FIELD name ON item TYPE string;
DEFINE FIELD description ON item TYPE option<string>;
DEFINE FIELD quantity ON item TYPE int DEFAULT 1;
DEFINE FIELD template_id ON item TYPE option<record<item_template>>;
DEFINE FIELD location_id ON item TYPE option<record<location>>;
DEFINE FIELD container_id ON item TYPE option<record<container>>;
DEFINE FIELD initiative_id ON item TYPE option<record<initiative>>;
DEFINE FIELD created_at ON item TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON item TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON item TYPE option<datetime>;
DEFINE FIELD sync_version ON item TYPE int DEFAULT 0;
DEFINE INDEX user_idx ON item FIELDS user_id;
DEFINE INDEX location_idx ON item FIELDS user_id, location_id;
DEFINE INDEX container_idx ON item FIELDS user_id, container_id;
DEFINE INDEX initiative_idx ON item FIELDS user_id, initiative_id;
```

### SQLite Schema (Mobile)

```sql
-- User reference (synced from server, read-only on mobile)
CREATE TABLE user_info (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    storage_quota INTEGER,
    sync_version INTEGER NOT NULL DEFAULT 0
);

-- Initiative
CREATE TABLE initiative (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    parent_id TEXT REFERENCES initiative(id),
    ongoing INTEGER NOT NULL DEFAULT 0,
    target_date TEXT,
    status TEXT NOT NULL DEFAULT 'active',
    focused INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX initiative_user_idx ON initiative(user_id);
CREATE INDEX initiative_status_idx ON initiative(user_id, status);

-- Inbox Item
CREATE TABLE inbox_item (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    content TEXT NOT NULL,
    source TEXT NOT NULL,
    created_at TEXT NOT NULL,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX inbox_user_idx ON inbox_item(user_id);

-- Routine
CREATE TABLE routine (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    schedule TEXT NOT NULL,
    time_of_day TEXT,
    energy_cost INTEGER NOT NULL DEFAULT 1,
    initiative_id TEXT REFERENCES initiative(id),
    active INTEGER NOT NULL DEFAULT 1,
    next_due TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX routine_user_idx ON routine(user_id);
CREATE INDEX routine_active_idx ON routine(user_id, active);

-- SourceDocument
CREATE TABLE source_document (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    title TEXT NOT NULL,
    source_type TEXT NOT NULL,
    source_path TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    extracted_text TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    error_message TEXT,
    initiative_id TEXT REFERENCES initiative(id),
    watched_folder_id TEXT,
    last_synced_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX source_document_user_idx ON source_document(user_id);
CREATE INDEX source_document_status_idx ON source_document(user_id, status);
CREATE INDEX source_document_hash_idx ON source_document(user_id, content_hash);

-- SourceAnnotation
CREATE TABLE source_annotation (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    source_document_id TEXT NOT NULL REFERENCES source_document(id),
    anchor_type TEXT NOT NULL,
    anchor_value TEXT,
    anchor_fingerprint TEXT,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX source_annotation_user_idx ON source_annotation(user_id);
CREATE INDEX source_annotation_document_idx ON source_annotation(user_id, source_document_id);

-- Quest with routine_id
CREATE TABLE quest (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'backlog',
    energy_cost INTEGER NOT NULL DEFAULT 1,
    epic_id TEXT REFERENCES epic(id),
    routine_id TEXT REFERENCES routine(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    started_at TEXT,
    completed_at TEXT,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX quest_user_idx ON quest(user_id);
CREATE INDEX quest_status_idx ON quest(user_id, status);
CREATE INDEX quest_routine_idx ON quest(user_id, routine_id);

-- Note with initiative_id
CREATE TABLE note (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT,
    folder_id TEXT REFERENCES folder(id),
    initiative_id TEXT REFERENCES initiative(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX note_user_idx ON note(user_id);
CREATE INDEX note_initiative_idx ON note(user_id, initiative_id);

-- Item with initiative_id
CREATE TABLE item (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    quantity INTEGER NOT NULL DEFAULT 1,
    template_id TEXT REFERENCES item_template(id),
    location_id TEXT REFERENCES location(id),
    container_id TEXT REFERENCES container(id),
    initiative_id TEXT REFERENCES initiative(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX item_user_idx ON item(user_id);
CREATE INDEX item_initiative_idx ON item(user_id, initiative_id);
```

### Table Conventions

Both databases follow consistent conventions:

- Table names are `snake_case` singular (e.g., `quest`, `note`, `item`)
- Record IDs use ULID strings: `01HXK3...`
- Timestamps stored as ISO 8601 strings
- Soft-deleted records have `deleted_at` field set
- All entities include `sync_version` for sync tracking
- All user-owned entities include `user_id` for data isolation

---

## Query Patterns

### Desktop (SurrealDB)

```kotlin
// Basic CRUD with user scope
suspend fun getQuest(id: String): Quest? =
    db.query(
        "SELECT * FROM quest WHERE id = \$id AND user_id = \$userId",
        mapOf("id" to id, "userId" to auth.currentUserId)
    ).firstOrNull()

// Initiative-filtered query
suspend fun getQuestsForInitiative(initiativeId: String): List<Quest> =
    db.query("""
        SELECT * FROM quest 
        WHERE user_id = \$userId 
        AND epic_id IN (SELECT id FROM epic WHERE initiative_id = \$initId)
    """, mapOf("userId" to auth.currentUserId, "initId" to initiativeId))

// Graph traversal
suspend fun getQuestNotes(questId: String): List<Note> =
    db.query(
        "SELECT ->references->note.* FROM quest WHERE id = \$id AND user_id = \$userId",
        mapOf("id" to questId, "userId" to auth.currentUserId)
    ).flatten()

// Vector similarity
suspend fun findSimilarNotes(embedding: FloatArray, limit: Int = 5): List<Note> =
    db.query("""
        SELECT *, vector::similarity::cosine(embedding, \$vec) AS score
        FROM note
        WHERE user_id = \$userId AND embedding IS NOT NULL
        ORDER BY score DESC
        LIMIT \$limit
    """, mapOf("vec" to embedding, "limit" to limit, "userId" to auth.currentUserId))
```

### Mobile (SQLDelight)

```kotlin
// Generated from .sq file with user scope
val getQuest: Quest? = questQueries.getById(id, currentUserId).executeAsOneOrNull()

// Initiative filtering
val initiativeItems: List<Item> = itemQueries.getByInitiative(initiativeId, currentUserId).executeAsList()

// Routine instances due today
val todayRoutines: List<Routine> = routineQueries.getDueToday(today, currentUserId).executeAsList()
```

---

## Sync Protocol

### Version Tracking

All entities include `sync_version`:

- Monotonically increasing counter per entity
- Server maintains authoritative version sequence
- Clients track last known version per entity type

### Multi-User Sync

Each user's data syncs independently:

- Sync requests include JWT token with user ID
- Server filters all sync data by authenticated user
- No cross-user data leakage in sync responses

### Pull Flow

```
Client                          Server
   │                               │
   │─── Pull(since: 1000) ────────>│
   │    (JWT token in header)      │
   │                               │
   │    Filter by user_id          │
   │                               │
   │<── Changes(v1001..v1050) ─────│
   │    (user's data only)         │
   │                               │
   │    Apply changes locally      │
   │    Update last_sync = 1050    │
```

### Push Flow

```
Client                          Server
   │                               │
   │─── Push(changes, base: 1050)─>│
   │    (JWT token in header)      │
   │                               │
   │    Validate user_id on all    │
   │    Apply changes              │
   │    Resolve conflicts          │
   │                               │
   │<── Result(v1051, conflicts)───│
   │                               │
   │    Apply server decisions     │
```

### Conflict Resolution

Server-side resolution strategy:

| Scenario                  | Resolution                                        |
| ------------------------- | ------------------------------------------------- |
| Same field changed        | Latest `updated_at` wins                          |
| Different fields changed  | Merge both changes                                |
| Entity deleted            | Delete wins (soft delete propagates)              |
| Long text conflicts       | Keep both versions, flag for user resolution      |
| Additive changes          | Merge both (new links, new items)                 |

Complex conflicts (divergent note content) create conflict snapshots for deferred user resolution.

---

## Storage Architecture

### S3-Compatible Backend

Attachments stored in S3-compatible storage:

| Configuration | Purpose                                |
| ------------- | -------------------------------------- |
| Local         | Default, files in `$DATA_DIR/storage/` |
| MinIO         | Self-hosted S3 alternative             |
| AWS S3        | Cloud storage option                   |
| Backblaze B2  | Cost-effective cloud option            |

**Object Key Pattern:**
```
{user_id}/{content_hash}/{filename}
```

### Storage Quota Enforcement

```kotlin
suspend fun uploadAttachment(userId: String, file: ByteArray): Result<Attachment> {
    val user = userRepository.getById(userId)
    val currentUsage = user.storageUsed
    val quota = user.storageQuota ?: Long.MAX_VALUE
    
    if (currentUsage + file.size > quota) {
        return Result.failure(StorageQuotaExceeded(
            required = file.size,
            available = quota - currentUsage
        ))
    }
    
    // Proceed with upload
    val attachment = storageService.upload(userId, file)
    userRepository.updateStorageUsed(userId, currentUsage + file.size)
    return Result.success(attachment)
}
```

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
    Migration(1, "create_user_table", """
        DEFINE TABLE user SCHEMAFULL;
        DEFINE FIELD username ON user TYPE string;
        -- ...
    """),
    Migration(2, "create_initiative_table", """
        DEFINE TABLE initiative SCHEMAFULL;
        DEFINE FIELD user_id ON initiative TYPE record<user>;
        -- ...
    """),
    Migration(3, "add_initiative_to_entities", """
        DEFINE FIELD initiative_id ON epic TYPE option<record<initiative>>;
        DEFINE FIELD initiative_id ON note TYPE option<record<initiative>>;
        DEFINE FIELD initiative_id ON item TYPE option<record<initiative>>;
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
│   ├── Initiative.sq
│   ├── Routine.sq
│   └── migrations/
│       ├── 1.sqm
│       ├── 2.sqm
│       └── 3.sqm
```

---

## Backup and Recovery

### Desktop

- Export: Dump all tables to JSON via SurrealQL `SELECT * FROM ...`
- Import: Transaction-wrapped inserts
- Location: User-selected file

### Server

- Volume backup: `docker compose exec surrealdb surreal export`
- S3 backup: Separate backup of attachment storage
- Scheduled: Configurable cron job
- Retention: User-configured

### Mobile

- Sync to server is the primary backup mechanism
- SQLite database included in platform backup (iCloud, Google backup)

### User Data Export

Disabled users can export all their data:

```kotlin
suspend fun exportUserData(userId: String): ExportPackage {
    return ExportPackage(
        initiatives = initiativeRepo.getAllForUser(userId),
        epics = epicRepo.getAllForUser(userId),
        quests = questRepo.getAllForUser(userId),
        notes = noteRepo.getAllForUser(userId),
        items = itemRepo.getAllForUser(userId),
        routines = routineRepo.getAllForUser(userId),
        attachments = attachmentRepo.getAllForUser(userId)
    )
}
```

---

## References

- [SurrealDB Documentation](https://surrealdb.com/docs)
- [SQLDelight Documentation](https://cashapp.github.io/sqldelight/)
- [ADR-002: Hybrid Database Strategy](../adr/002-surrealdb-embedded.md)
- [ADR-005: kotlinx-rpc Communication](../adr/005-kotlinx-rpc-communication.md)
- [Domain Model](./domain-model.md) — Entity definitions

---

_Last updated: January 14, 2026_

# Persistence

## Purpose

This document describes how Altair stores and manages data using SurrealDB, including schema design patterns, query
strategies, sync considerations, and migration approach.

---

## Database Architecture

### Embedded Mode

SurrealDB runs in-process using the SurrealKV storage engine. No separate database server to install or manage.

| Aspect         | Configuration                   |
| -------------- | ------------------------------- |
| Mode           | Embedded (in-process)           |
| Storage Engine | SurrealKV                       |
| Location       | `$APP_DATA/altair/db/`          |
| Concurrency    | Single writer, multiple readers |

### Why SurrealDB

| Need                | SurrealDB Capability                              |
| ------------------- | ------------------------------------------------- |
| Graph relationships | Native graph queries with `->` and `<-` operators |
| Full-text search    | Built-in search indexes                           |
| Vector similarity   | Native vector type and KNN search                 |
| Embedded operation  | No external process required                      |
| Flexible schema     | Schemaless with optional schema enforcement       |
| Rust-native         | First-class Rust SDK                              |

See [ADR-002](../adr/002-surrealdb-embedded.md) for the full decision rationale.

---

## Schema Design

### Namespace Structure

```text
namespace: altair
├── database: main
│   ├── tables: epic, quest, checkpoint, energy_budget
│   ├── tables: note, note_link, folder, tag, attachment
│   └── tables: item, custom_field, location, container, template, field_def
```

### Table Conventions

- Table names are `snake_case` singular (e.g., `quest`, `note`, `item`)
- Record IDs use ULID strings: `quest:01HXK3...`
- Timestamps stored as ISO 8601 strings
- Soft-deleted records have `deleted_at` field set

### Indexes

Each table has indexes based on common query patterns:

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
| item  | template_idx  | template_id      | Items by type            |

### Relations

SurrealDB supports two relation patterns:

**Record Links** (simple references):

```text
quest:abc.epic_id = epic:xyz
```

**Graph Edges** (rich relationships):

```text
note:abc ->links-> note:xyz
quest:abc ->references-> note:xyz
```

Altair uses:

- Record links for hierarchical relationships (Quest→Epic, Note→Folder, Item→Location)
- Graph edges for cross-module relationships (Quest↔Note, Note↔Item)

---

## Query Patterns

### Basic CRUD

Queries go through a typed repository layer, not raw SurrealQL in business logic.

**Create**: Insert with generated ULID
**Read**: Select by ID or query with filters
**Update**: Merge to preserve unspecified fields
**Delete**: Set `deleted_at` for soft delete

### Filtering Deleted Records

All queries exclude soft-deleted records by default:

```sql
WHERE deleted_at IS NONE
```

The "Trash" view explicitly includes them:

```sql
WHERE deleted_at IS NOT NONE
  AND deleted_at > time::now() - 30d
```

### Graph Traversal

Finding all Notes linked from a Note (outgoing):

```sql
SELECT ->links->note FROM note:abc
```

Finding all Notes that link to a Note (incoming):

```sql
SELECT <-links<-note FROM note:abc
```

Two-hop traversal (notes linked from notes linked from this note):

```sql
SELECT ->links->note->links->note FROM note:abc
```

### Full-Text Search

Notes use a search index on content:

```sql
SELECT * FROM note
WHERE content @@ 'search terms'
ORDER BY search::score(content) DESC
```

### Vector Similarity

Semantic search uses embeddings:

```sql
SELECT * FROM note
WHERE embedding <|10,COSINE|> $query_vector
```

Returns 10 nearest neighbors by cosine similarity.

---

## Transactions

### ACID Guarantees

SurrealDB provides ACID transactions. Operations that modify multiple records use explicit transactions:

- Creating a Quest with Checkpoints
- Moving an Item (update Item, log history)
- Deleting a Folder (reassign Notes)

### Optimistic Concurrency

For conflict detection on concurrent edits:

1. Read record with `updated_at` timestamp
2. User makes changes
3. Update with condition: `WHERE updated_at = $original_timestamp`
4. If no rows affected, another edit occurred—prompt user to resolve

---

## Data Lifecycle

### Creation

1. Generate ULID for new record
2. Set `created_at` to current timestamp
3. Set `updated_at` equal to `created_at`
4. Insert record

### Modification

1. Update relevant fields
2. Set `updated_at` to current timestamp
3. (For audited entities) Append to change history

### Soft Delete

1. Set `deleted_at` to current timestamp
2. Record remains in database but hidden from queries
3. After retention period (30 days), eligible for hard delete

### Hard Delete

1. Purge job runs periodically (daily)
2. Delete records where `deleted_at < now() - 30d`
3. Cascade delete orphaned relations
4. Vacuum storage to reclaim space

---

## Migrations

### Strategy

Schema changes are handled through versioned migrations:

1. Each migration has an ID (incrementing integer) and timestamp
2. `_migration` table tracks applied migrations
3. On startup, apply any pending migrations in order
4. Migrations are forward-only (no automatic rollback)

### Migration Structure

Each migration defines:

- **ID**: Unique sequential number
- **Name**: Descriptive slug (e.g., `add_note_embedding_index`)
- **Up**: SurrealQL statements to apply the change

### Common Migration Types

| Type          | Example                                       |
| ------------- | --------------------------------------------- |
| Add table     | `DEFINE TABLE new_table SCHEMAFULL`           |
| Add field     | `DEFINE FIELD new_field ON table TYPE string` |
| Add index     | `DEFINE INDEX idx ON table FIELDS field`      |
| Backfill data | `UPDATE table SET new_field = computed_value` |
| Rename field  | Add new, copy data, remove old                |

### Breaking Changes

For changes that can't be done in-place:

1. Add new structure alongside old
2. Migrate data in batches
3. Update application to use new structure
4. Remove old structure in future migration

---

## Sync Strategy (Future)

Altair is local-first, but future sync is designed for:

### Conflict Resolution

- Last-write-wins for simple fields
- CRDT-based merge for text content (if needed)
- User prompt for structural conflicts (moved to different folder by two devices)

### Sync Metadata

Records include sync fields (inactive until sync implemented):

- `_sync_id`: Globally unique identifier
- `_sync_version`: Logical clock / vector clock
- `_sync_modified`: Timestamp of last local change

### Sync-Aware Deletion

Soft delete is essential for sync:

- Deleted records sync as tombstones
- Tombstones retained until all devices acknowledge
- Hard delete only after sync confirmation

---

## Backup and Recovery

### Automatic Backups

- Daily backup of database directory
- Retained for 7 days locally
- User can configure backup location (e.g., cloud folder)

### Manual Export

Users can export all data as:

- JSON (structured, for migration)
- Markdown (Notes only, for portability)

### Recovery

If database corrupts:

1. Attempt SurrealDB repair
2. Restore from most recent backup
3. Re-import from export if backup unavailable

---

## Performance Considerations

### Index Strategy

- Index fields used in WHERE clauses
- Compound indexes for multi-field filters
- Vector index (HNSW) for embedding search
- Full-text index for content search

### Query Optimization

- Fetch only needed fields (avoid `SELECT *` in production)
- Paginate large result sets
- Use graph queries instead of multiple round trips
- Cache frequently-accessed records in memory

### Storage Growth

| Entity     | Estimated Size | Notes                    |
| ---------- | -------------- | ------------------------ |
| Note       | 5-50 KB        | Content + embedding      |
| Item       | 1-5 KB         | Metadata + custom fields |
| Quest      | 0.5-2 KB       | Small records            |
| Attachment | Reference only | Files stored separately  |

Embedding vectors (384 dimensions × 4 bytes = 1.5 KB per Note) are the largest per-record cost.

---

## References

- [ADR-002: SurrealDB for Persistence](../adr/002-surrealdb-embedded.md)
- [Domain Model](./domain-model.md) — Entity definitions
- [SurrealDB Documentation](https://surrealdb.com/docs)

# SQLite Conventions

Applies to: Android local database (`apps/android/`)

## Role

SQLite is the local offline database on Android, managed through Room. It provides immediate read/write access while the device is offline. PowerSync handles bidirectional sync with the server PostgreSQL database.

## Room Configuration

- Single `@Database` class (`AltairDatabase`) as the entry point
- Version migrations via `Migration` objects, not `fallbackToDestructiveMigration`
- `@TypeConverter` for non-primitive column types (UUIDs, enums, timestamps)

## Entity Design

- Room entities in `data/local/entity/` with `Entity` suffix (e.g., `QuestEntity`)
- Domain models in `domain/entity/` without suffix (e.g., `Quest`)
- Mappers in `data/local/mapper/` convert between entity and domain layers
- All entities require `id` (String/UUID) as `@PrimaryKey`
- Use `@ColumnInfo(name = "snake_case")` to match PostgreSQL column names

## DAO Pattern

- One DAO per entity in `data/local/dao/`
- Return `Flow<List<T>>` for observable queries
- Use `suspend` functions for write operations
- Batch operations via `@Transaction`
- Avoid raw queries; use Room's annotation-based query builder

## Schema Alignment

- Local SQLite schema must mirror the synced subset of the PostgreSQL schema
- Column names must match exactly (snake_case) for PowerSync compatibility
- Include `created_at` and `updated_at` text columns for sync metadata
- Soft-deleted rows retained locally until sync confirms server-side deletion

## Performance

- Index columns used in WHERE clauses and ORDER BY
- Use `@Relation` for pre-fetched joins, but avoid deep nesting
- Prefer `Flow` over repeated single queries for reactive UI
- WAL mode enabled for concurrent read/write

## Testing

- Room in-memory database for unit tests
- Verify migrations with `MigrationTestHelper`
- Test DAO queries with realistic data volumes

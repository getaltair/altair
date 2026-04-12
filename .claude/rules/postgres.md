# PostgreSQL Conventions

Applies to: server-side database (accessed from `apps/server/`)

## Role

PostgreSQL is the main database and source of truth. All clients sync through PowerSync; the server writes directly via sqlx.

## Schema Design

- All tables require a `id` primary key (UUID)
- All tables require `created_at` and `updated_at` timestamps
- `updated_at` auto-maintained via PL/pgSQL trigger
- Soft deletion via `deleted_at` timestamp where applicable
- Foreign keys with appropriate `ON DELETE` behavior (CASCADE, SET NULL, or RESTRICT)
- Strategic indexing on columns used in WHERE clauses and JOIN conditions

## Naming

- Tables: `snake_case`, plural (e.g., `quests`, `daily_checkins`, `entity_relations`)
- Columns: `snake_case`
- Indexes: `idx_{table}_{column(s)}`
- Foreign keys: `fk_{table}_{referenced_table}`
- Constraints: `chk_{table}_{description}`

## Migrations

- All migrations must include rollback procedures
- One migration per logical change
- Migration ordering dependencies tracked explicitly
- Test migrations against a fresh database and an existing one
- Never modify a migration that has been applied to any environment

## Security

- Row Level Security (RLS) on all user-facing tables
- User isolation policies to prevent cross-user data access
- Parameterized queries only; never interpolate user input into SQL
- Connection pooling for concurrent load

## Performance

- Avoid N+1 query patterns; use JOINs or batch queries
- Index coverage for common query patterns
- Use `EXPLAIN ANALYZE` to validate query plans for complex queries
- Connection pooling via pgbouncer or sqlx pool configuration

## PowerSync Integration

- Tables must conform to the PowerSync sync rules schema
- `updated_at` column is required for change detection
- Soft-deleted rows must remain queryable for sync reconciliation

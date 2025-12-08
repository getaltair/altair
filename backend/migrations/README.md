# Database Migrations

This directory contains SurrealDB schema migrations for the Altair project.

## Migration File Naming Convention

Migration files must follow the pattern:

```
NNN_description.surql
```

Where:

- `NNN` is a zero-padded three-digit sequential number (e.g., `001`, `002`, `010`, `100`)
- `description` is a brief, lowercase description using underscores (e.g., `initial_schema`, `add_indexes`, `seed_data`)
- Extension must be `.surql`

### Examples

```
001_initial_schema.surql
002_edge_tables.surql
003_indexes.surql
004_seed_data.surql
```

## Migration Execution

Migrations are automatically applied in numerical order by the `MigrationRunner` when the backend starts. The runner:

1. Creates a `_migrations` tracking table (if it doesn't exist)
2. Discovers all `.surql` files matching the `NNN_*.surql` pattern
3. Checks which migrations have already been applied
4. Applies pending migrations in order
5. Records each successful migration in `_migrations`

## Writing Migrations

Each migration file should:

- Start with `USE NS altair DB main;` to ensure correct namespace/database
- Use `DEFINE TABLE ... SCHEMAFULL;` for strict schema enforcement
- Include `CHANGEFEED 7d;` on all tables that need sync support
- Be idempotent where possible (use `IF NOT EXISTS` clauses)

### Example Migration

```surql
-- 001_initial_schema.surql
USE NS altair DB main;

DEFINE TABLE user SCHEMAFULL CHANGEFEED 7d;
DEFINE FIELD email ON TABLE user TYPE string ASSERT $value != NONE;
DEFINE FIELD display_name ON TABLE user TYPE string;
DEFINE FIELD created_at ON TABLE user TYPE datetime DEFAULT time::now();
DEFINE INDEX idx_user_email_unique ON TABLE user FIELDS email UNIQUE;
```

## Development Workflow

1. Create a new migration file with the next sequential number
2. Write your schema changes using SurrealQL
3. Test locally by restarting the backend (migrations run on startup)
4. Verify changes with `surreal sql` or through the application
5. Commit the migration file with your code changes

## Migration History

The `_migrations` table tracks applied migrations:

```surql
SELECT * FROM _migrations ORDER BY version;
```

Each record contains:

- `version`: The migration number (e.g., `1`, `2`, `10`)
- `name`: The migration filename (e.g., `001_initial_schema.surql`)
- `applied_at`: Timestamp when the migration was applied

## Troubleshooting

If a migration fails:

1. Check the backend logs for SurrealDB error messages
2. Fix the migration file
3. If the migration partially applied, you may need to manually rollback changes
4. For development, you can reset the database by deleting `~/.local/share/altair/db` and restarting

**Note**: Migrations are forward-only. There is no automatic rollback mechanism.

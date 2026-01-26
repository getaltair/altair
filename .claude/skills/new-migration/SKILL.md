---
name: new-migration
description: Generate database migrations for SQLDelight (mobile) or SurrealDB (desktop/server)
user-invocable: true
---

# Database Migration Generator

Generate type-safe database migrations for Altair's dual database strategy:
- **Mobile**: SQLDelight (SQLite) for Android/iOS
- **Desktop/Server**: SurrealDB for JVM

## Usage

```
/new-migration <platform> <description>
```

**Arguments**:
- `platform`: `mobile` (SQLDelight) or `desktop` (SurrealDB)
- `description`: Short kebab-case description (e.g., "add-user-preferences-table")

**Examples**:
```
/new-migration mobile add-quest-tags-table
/new-migration desktop create-knowledge-nodes-graph
```

## Process

### For Mobile Platform (SQLDelight)

#### 1. Determine Next Version
- Check existing migrations in `shared/src/commonMain/sqldelight/migrations/`
- Find the highest version number
- Increment by 1 for the new migration

#### 2. Create Migration File
**Path**: `shared/src/commonMain/sqldelight/migrations/<version>_<description>.sqm`

**Example**: `shared/src/commonMain/sqldelight/migrations/3_add_user_preferences.sqm`

#### 3. Migration Template

```sql
-- Migration: <description>
-- Version: <version>
-- Created: <date>

-- IMPORTANT: All tables MUST include user_id for multi-user isolation
-- IMPORTANT: All queries MUST filter by user_id

-- Create table
CREATE TABLE IF NOT EXISTS user_preference (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    preference_key TEXT NOT NULL,
    preference_value TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE,
    UNIQUE(user_id, preference_key)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_user_preference_user_id
    ON user_preference(user_id);

CREATE INDEX IF NOT EXISTS idx_user_preference_key
    ON user_preference(user_id, preference_key);

-- Sample query (add to appropriate .sq file)
-- getUserPreferences:
-- SELECT * FROM user_preference
-- WHERE user_id = :userId
-- ORDER BY preference_key;
```

#### 4. SQLite Migration Rules

**Must include**:
- ✅ `user_id` column on all tables (except lookup tables)
- ✅ Foreign key constraint: `FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE`
- ✅ Index on `user_id`: `CREATE INDEX idx_<table>_user_id ON <table>(user_id)`
- ✅ NOT NULL constraints on required fields
- ✅ Default values where appropriate
- ✅ Unique constraints for natural keys
- ✅ Proper data types: TEXT (ids, strings), INTEGER (timestamps, numbers), REAL (decimals)

**Example with relationships**:
```sql
CREATE TABLE IF NOT EXISTS quest (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS quest_tag (
    quest_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    PRIMARY KEY (quest_id, tag_id),
    FOREIGN KEY (quest_id) REFERENCES quest(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tag(id) ON DELETE CASCADE
);
```

---

### For Desktop Platform (SurrealDB)

#### 1. Determine Next Version
- Check existing migrations in `shared/src/jvmMain/kotlin/com/getaltair/altair/db/migrations/`
- Find the highest version in migration class names
- Increment by 1 for the new migration

#### 2. Create Migration Class
**Path**: `shared/src/jvmMain/kotlin/com/getaltair/altair/db/migrations/V<version>_<Description>.kt`

**Example**: `shared/src/jvmMain/kotlin/com/getaltair/altair/db/migrations/V003_AddUserPreferences.kt`

#### 3. Migration Template

```kotlin
package com.getaltair.altair.db.migrations

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.domain.error.DomainError
import com.surrealdb.Surreal

/**
 * Migration: <description>
 * Version: <version>
 * Created: <date>
 *
 * IMPORTANT: All tables MUST define user_id field
 * IMPORTANT: All queries MUST filter by user_id = $auth.id
 */
class V003_AddUserPreferences : Migration {
    override val version: Int = 3
    override val description: String = "Add user preferences table"

    override suspend fun up(db: Surreal): Either<DomainError, Unit> = either {
        // Define table with schema
        db.query(
            """
            DEFINE TABLE user_preference SCHEMAFULL
                PERMISSIONS
                    FOR select, create, update, delete
                    WHERE user_id = \$auth.id;

            DEFINE FIELD user_id ON user_preference TYPE record<user>
                ASSERT \$value != NONE;

            DEFINE FIELD preference_key ON user_preference TYPE string
                ASSERT \$value != NONE AND string::len(\$value) > 0;

            DEFINE FIELD preference_value ON user_preference TYPE string
                ASSERT \$value != NONE;

            DEFINE FIELD created_at ON user_preference TYPE datetime
                VALUE \$value OR time::now()
                DEFAULT time::now();

            DEFINE FIELD updated_at ON user_preference TYPE datetime
                VALUE time::now();

            DEFINE INDEX idx_user_preference_key ON user_preference
                FIELDS user_id, preference_key UNIQUE;
            """.trimIndent()
        ).bind("auth", mapOf("id" to "user:system"))

        Unit
    }

    override suspend fun down(db: Surreal): Either<DomainError, Unit> = either {
        // Rollback: Remove table and indexes
        db.query(
            """
            REMOVE INDEX idx_user_preference_key ON user_preference;
            REMOVE TABLE user_preference;
            """.trimIndent()
        )

        Unit
    }
}
```

#### 4. SurrealDB Migration Rules

**Must include**:
- ✅ `SCHEMAFULL` table definition
- ✅ `PERMISSIONS` clause filtering by `user_id = $auth.id`
- ✅ `user_id` field defined as `TYPE record<user>` with `ASSERT $value != NONE`
- ✅ Field validations using `ASSERT`
- ✅ Timestamps with `DEFAULT time::now()`
- ✅ Indexes for query performance
- ✅ Unique constraints where needed
- ✅ Rollback query in `down()` method

**Example with graph relations**:
```kotlin
override suspend fun up(db: Surreal): Either<DomainError, Unit> = either {
    db.query(
        """
        DEFINE TABLE knowledge_node SCHEMAFULL
            PERMISSIONS
                FOR select, create, update, delete
                WHERE user_id = \$auth.id;

        DEFINE FIELD user_id ON knowledge_node TYPE record<user>
            ASSERT \$value != NONE;

        DEFINE FIELD title ON knowledge_node TYPE string
            ASSERT \$value != NONE AND string::len(\$value) > 0;

        DEFINE FIELD content ON knowledge_node TYPE string;

        -- Graph relation table
        DEFINE TABLE links SCHEMAFULL TYPE RELATION
            IN knowledge_node OUT knowledge_node
            PERMISSIONS
                FOR select, create, update, delete
                WHERE in.user_id = \$auth.id AND out.user_id = \$auth.id;

        DEFINE FIELD strength ON links TYPE number DEFAULT 1.0;

        DEFINE FIELD created_at ON links TYPE datetime
            VALUE time::now();
        """.trimIndent()
    )

    Unit
}
```

---

## Migration Checklist

Before finalizing a migration:

### Security
- [ ] All tables have `user_id` field (except system/lookup tables)
- [ ] All tables enforce user isolation in queries
- [ ] No hardcoded user references
- [ ] Permissions properly restrict data access

### Data Integrity
- [ ] NOT NULL constraints on required fields
- [ ] Foreign keys defined with proper ON DELETE behavior
- [ ] Unique constraints on natural keys
- [ ] Default values for timestamps and status fields

### Performance
- [ ] Indexes on `user_id` for filtering
- [ ] Indexes on foreign keys
- [ ] Indexes on commonly queried fields
- [ ] Composite indexes for multi-column queries

### Rollback
- [ ] `down()` method properly reverses changes
- [ ] Rollback tested (if possible)
- [ ] Data migration has inverse operation

---

## Testing Migrations

### Mobile (SQLDelight)
```bash
# Run migration tests
./gradlew :shared:testDebugUnitTest --tests "*Migration*"

# Verify schema
# Check generated code in: shared/build/generated/sqldelight/
```

### Desktop (SurrealDB)
```bash
# Run migration tests
./gradlew :shared:jvmTest --tests "*Migration*"

# Manual verification
# Start embedded SurrealDB and run migration
./gradlew :composeApp:run
```

---

## Common Patterns

### Adding a New Module Table
```sql
-- Mobile
CREATE TABLE IF NOT EXISTS <module>_item (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);
CREATE INDEX idx_<module>_item_user_id ON <module>_item(user_id);
CREATE INDEX idx_<module>_item_status ON <module>_item(user_id, status);
```

### Adding a Column
```sql
-- Mobile
ALTER TABLE quest ADD COLUMN priority INTEGER NOT NULL DEFAULT 0;
CREATE INDEX idx_quest_priority ON quest(user_id, priority);
```

### Creating a Junction Table
```sql
-- Mobile
CREATE TABLE IF NOT EXISTS quest_tag (
    quest_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (quest_id, tag_id),
    FOREIGN KEY (quest_id) REFERENCES quest(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tag(id) ON DELETE CASCADE
);
```

---

## Notes

- Always test migrations locally before committing
- Consider data migration for existing records
- Document breaking changes in migration comments
- Keep migrations small and focused
- Never edit existing migrations - create new ones
- Coordinate mobile and desktop schema changes

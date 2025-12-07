# Tasks: CORE-002-SCHEMA-MIGRATIONS

**Spec**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md) | **Branch**: `spec/core-002-schema-migrations`

## Phase 1: Migration Infrastructure

**Goal**: Create the migration runner and `_migrations` tracking table.

- [ ] **1.1**: Add surrealdb dependency to altair-db

  - **Acceptance**: `backend/crates/altair-db/Cargo.toml` includes `surrealdb = "2.x"`, `tokio = { version = "1", features = ["full"] }`, `tracing = "0.1"`
  - **Files**: `backend/crates/altair-db/Cargo.toml`
  - **Verify**: `cargo check -p altair-db` passes

- [ ] **1.2**: Create migrations directory with README

  - **Acceptance**: `backend/migrations/README.md` exists and documents migration file naming convention (NNN_description.surql)
  - **Files**: `backend/migrations/README.md`
  - **Verify**: File readable and follows project markdown style

- [ ] **1.3**: Implement MigrationRunner struct

  - **Acceptance**: `MigrationRunner` struct with `new()` and `run()` methods exists
  - **Files**: `backend/crates/altair-db/src/migration.rs`
  - **Verify**: `cargo check -p altair-db` passes, struct is public

- [ ] **1.4**: Implement version tracking (`_migrations` table)

  - **Acceptance**: `MigrationRunner::ensure_migrations_table()` creates `_migrations` table with fields: `version: int`, `name: string`, `applied_at: datetime`
  - **Files**: `backend/crates/altair-db/src/migration.rs`
  - **Verify**: Method creates table on first run, idempotent on subsequent runs

- [ ] **1.5**: Implement file discovery (NNN\_\*.surql pattern)

  - **Acceptance**: `MigrationRunner::discover_migrations()` returns Vec of migration files sorted by version number
  - **Files**: `backend/crates/altair-db/src/migration.rs`
  - **Verify**: Returns correct files from `backend/migrations/`, ignores non-surql files

- [ ] **1.6**: Implement idempotent apply logic

  - **Acceptance**: `MigrationRunner::apply()` checks `_migrations` table, only applies pending migrations, records versions
  - **Files**: `backend/crates/altair-db/src/migration.rs`
  - **Verify**: Running twice applies migrations only once

- [ ] **1.7**: Create SurrealDB client wrapper

  - **Acceptance**: `DatabaseClient` struct with `connect()`, `execute()`, and `select()` methods exists
  - **Files**: `backend/crates/altair-db/src/client.rs`
  - **Verify**: `cargo check -p altair-db` passes, client can connect to embedded SurrealKV

- [ ] **1.8**: Add integration test for migration runner
  - **Acceptance**: Test creates in-memory DB, runs empty migration, verifies `_migrations` table exists
  - **Files**: `backend/crates/altair-db/tests/migration_test.rs`
  - **Verify**: `cargo test -p altair-db migration_test` passes

---

## Phase 2: Core Entity Tables

**Goal**: Define all 15+ entity tables with SCHEMAFULL constraints and CHANGEFEED.

- [ ] **2.1**: Create 001_initial_schema.surql with namespace/database

  - **Acceptance**: File defines `DEFINE NAMESPACE altair;` and `DEFINE DATABASE main;` and `USE NS altair DB main;`
  - **Files**: `backend/migrations/001_initial_schema.surql`
  - **Verify**: File parses as valid SurrealQL

- [ ] **2.2**: Define `user` table with auth fields

  - **Acceptance**: `DEFINE TABLE user SCHEMAFULL CHANGEFEED 7d;` with fields: id, email (string), display_name (string), avatar_url (option<string>), preferences (object), created_at, updated_at
  - **Files**: `backend/migrations/001_initial_schema.surql`
  - **Verify**: Migration applies without errors

- [ ] **2.3**: Define Quest domain tables (campaign, quest, focus_session, energy_checkin)

  - **Acceptance**: 4 tables defined with SCHEMAFULL, CHANGEFEED 7d, all required fields per domain model
  - **Files**: `backend/migrations/001_initial_schema.surql`
  - **Verify**: Tables created with correct fields (campaign: title, description, status, owner; quest: title, description, column, energy_cost, status, owner, campaign_id; focus_session: quest_id, started_at, ended_at; energy_checkin: level, timestamp, owner)

- [ ] **2.4**: Define Knowledge domain tables (note, folder, daily_note)

  - **Acceptance**: 3 tables defined with SCHEMAFULL, CHANGEFEED 7d, all required fields per domain model
  - **Files**: `backend/migrations/001_initial_schema.surql`
  - **Verify**: Tables created with correct fields (note: title, content, owner, folder_id; folder: name, owner, parent_id; daily_note: date, content, owner)

- [ ] **2.5**: Define Inventory domain tables (item, location, reservation, maintenance_schedule)

  - **Acceptance**: 4 tables defined with SCHEMAFULL, CHANGEFEED 7d, all required fields per domain model
  - **Files**: `backend/migrations/001_initial_schema.surql`
  - **Verify**: Tables created with correct fields (item: name, description, quantity, owner; location: name, description, owner; reservation: item_id, quest_id, quantity, reserved_at; maintenance_schedule: item_id, next_date, frequency)

- [ ] **2.6**: Define Capture table (multi-modal input)

  - **Acceptance**: `capture` table with fields: id, content, capture_type, source_app, owner, captured_at, processed (bool), processed_entity_id
  - **Files**: `backend/migrations/001_initial_schema.surql`
  - **Verify**: Table supports capture_type enum: text, voice, image, link

- [ ] **2.7**: Define Gamification tables (user_progress, achievement, streak)

  - **Acceptance**: 3 tables defined with SCHEMAFULL, CHANGEFEED 7d (user_progress: owner, xp, level, current_energy; achievement: name, description, icon, unlock_criteria; streak: owner, metric, current_count, longest_count, last_updated)
  - **Files**: `backend/migrations/001_initial_schema.surql`
  - **Verify**: Tables created with correct relationships to user table

- [ ] **2.8**: Define shared tables (attachment, tag)

  - **Acceptance**: 2 tables defined (attachment: entity_id, file_path, content_type, size; tag: name, color, owner)
  - **Files**: `backend/migrations/001_initial_schema.surql`
  - **Verify**: Tables support attachment of multiple entity types

- [ ] **2.9**: Add field assertions for enums (column, energy_level, status)

  - **Acceptance**: ASSERT clauses enforce valid enum values (quest.column IN ['backlog', 'wip', 'done'], quest.energy_cost IN ['tiny', 'small', 'medium', 'large', 'huge'], all entities status IN ['active', 'archived'])
  - **Files**: `backend/migrations/001_initial_schema.surql`
  - **Verify**: INSERT with invalid enum value fails with error

- [ ] **2.10**: Verify CHANGEFEED 7d on all tables
  - **Acceptance**: Integration test confirms all 15+ entity tables have CHANGEFEED enabled
  - **Files**: `backend/crates/altair-db/tests/migration_test.rs`
  - **Verify**: `INFO FOR TABLE` shows `changefeed` field for each table

---

## Phase 3: Graph Edge Tables

**Goal**: Define the 10 relationship edge tables for graph queries.

- [ ] **3.1**: Create 002_edge_tables.surql

  - **Acceptance**: File exists with `USE NS altair DB main;` header
  - **Files**: `backend/migrations/002_edge_tables.surql`
  - **Verify**: File parses as valid SurrealQL

- [ ] **3.2**: Define `contains` edge (Campaign→Quest, Folder→Note)

  - **Acceptance**: `DEFINE TABLE contains SCHEMAFULL;` with fields: in (record), out (record), created_at
  - **Files**: `backend/migrations/002_edge_tables.surql`
  - **Verify**: Edge supports both campaign→quest and folder→note relationships

- [ ] **3.3**: Define `references` edge (Quest→Note)

  - **Acceptance**: `DEFINE TABLE references SCHEMAFULL;` with fields: in (record<quest>), out (record<note>), created_at
  - **Files**: `backend/migrations/002_edge_tables.surql`
  - **Verify**: Graph query `SELECT ->references->note FROM quest:xyz` works

- [ ] **3.4**: Define `requires` edge (Quest→Item)

  - **Acceptance**: `DEFINE TABLE requires SCHEMAFULL;` with fields: in (record<quest>), out (record<item>), quantity (int), created_at
  - **Files**: `backend/migrations/002_edge_tables.surql`
  - **Verify**: Edge stores quantity of item required for quest

- [ ] **3.5**: Define `links_to` edge (Note→Note, bidirectional)

  - **Acceptance**: `DEFINE TABLE links_to SCHEMAFULL;` with fields: in (record<note>), out (record<note>), created_at
  - **Files**: `backend/migrations/002_edge_tables.surql`
  - **Verify**: Bidirectional queries work (both `->links_to->` and `<-links_to<-`)

- [ ] **3.6**: Define `stored_in` edge (Item→Location)

  - **Acceptance**: `DEFINE TABLE stored_in SCHEMAFULL;` with fields: in (record<item>), out (record<location>), created_at
  - **Files**: `backend/migrations/002_edge_tables.surql`
  - **Verify**: Query `SELECT ->stored_in->location FROM item:xyz` returns location

- [ ] **3.7**: Define `documents` edge (Note→Item)

  - **Acceptance**: `DEFINE TABLE documents SCHEMAFULL;` with fields: in (record<note>), out (record<item>), created_at
  - **Files**: `backend/migrations/002_edge_tables.surql`
  - **Verify**: Notes can document multiple items, items can be documented by multiple notes

- [ ] **3.8**: Define `reserved_for` edge (Reservation→Quest)

  - **Acceptance**: `DEFINE TABLE reserved_for SCHEMAFULL;` with fields: in (record<reservation>), out (record<quest>), created_at
  - **Files**: `backend/migrations/002_edge_tables.surql`
  - **Verify**: Query links reservation to quest

- [ ] **3.9**: Define `blocks` edge (Quest→Quest)

  - **Acceptance**: `DEFINE TABLE blocks SCHEMAFULL;` with fields: in (record<quest>), out (record<quest>), created_at
  - **Files**: `backend/migrations/002_edge_tables.surql`
  - **Verify**: Quest can block another quest (dependency relationship)

- [ ] **3.10**: Define `has_attachment` and `tagged` edges
  - **Acceptance**: Both edge tables defined (has_attachment: any entity→attachment; tagged: any entity→tag)
  - **Files**: `backend/migrations/002_edge_tables.surql`
  - **Verify**: Edges support polymorphic relationships (multiple entity types)

---

## Phase 4: Indexes and Performance

**Goal**: Create indexes for common query patterns.

- [ ] **4.1**: Create 003_indexes.surql

  - **Acceptance**: File exists with `USE NS altair DB main;` header
  - **Files**: `backend/migrations/003_indexes.surql`
  - **Verify**: File parses as valid SurrealQL

- [ ] **4.2**: Add owner index on all entity tables

  - **Acceptance**: `DEFINE INDEX idx_owner ON TABLE [table] FIELDS owner;` for all 15+ entity tables
  - **Files**: `backend/migrations/003_indexes.surql`
  - **Verify**: Query `SELECT * FROM [table] WHERE owner = user:xyz` uses index

- [ ] **4.3**: Add status index on all entity tables

  - **Acceptance**: `DEFINE INDEX idx_status ON TABLE [table] FIELDS status;` for all entity tables with status field
  - **Files**: `backend/migrations/003_indexes.surql`
  - **Verify**: Query `SELECT * FROM [table] WHERE status = 'active'` uses index

- [ ] **4.4**: Add full-text search index on note.content

  - **Acceptance**: `DEFINE INDEX idx_note_content_search ON TABLE note FIELDS content SEARCH ANALYZER ascii BM25;`
  - **Files**: `backend/migrations/003_indexes.surql`
  - **Verify**: Full-text search query `SELECT * FROM note WHERE content @@ 'search term'` works

- [ ] **4.5**: Add full-text search index on quest.title

  - **Acceptance**: `DEFINE INDEX idx_quest_title_search ON TABLE quest FIELDS title SEARCH ANALYZER ascii BM25;`
  - **Files**: `backend/migrations/003_indexes.surql`
  - **Verify**: Full-text search query `SELECT * FROM quest WHERE title @@ 'search term'` works

- [ ] **4.6**: Reserve vector index structure for note.embedding (HNSW)

  - **Acceptance**: Comment in migration reserves `DEFINE INDEX idx_note_embedding ON TABLE note FIELDS embedding MTREE DIMENSION 384;` for core-012
  - **Files**: `backend/migrations/003_indexes.surql`
  - **Verify**: Comment documents vector index will be added later

- [ ] **4.7**: Add unique index on user.email

  - **Acceptance**: `DEFINE INDEX idx_user_email_unique ON TABLE user FIELDS email UNIQUE;`
  - **Files**: `backend/migrations/003_indexes.surql`
  - **Verify**: Inserting duplicate email fails with unique constraint error

- [ ] **4.8**: Add date index on daily_note.date
  - **Acceptance**: `DEFINE INDEX idx_daily_note_date ON TABLE daily_note FIELDS date;`
  - **Files**: `backend/migrations/003_indexes.surql`
  - **Verify**: Query `SELECT * FROM daily_note WHERE date = '2025-12-06'` uses index

---

## Phase 5: Rust Type Definitions

**Goal**: Create Rust structs that mirror the SurrealDB schema for type-safe operations.

- [ ] **5.1**: Create schema module structure

  - **Acceptance**: `backend/crates/altair-db/src/schema/mod.rs` exists and re-exports all domain modules
  - **Files**: `backend/crates/altair-db/src/schema/mod.rs`, updated `lib.rs`
  - **Verify**: `cargo check -p altair-db` passes

- [ ] **5.2**: Define Quest domain types (Campaign, Quest, FocusSession, EnergyCheckIn)

  - **Acceptance**: 4 structs with `#[derive(Debug, Clone, Serialize, Deserialize)]` matching SurrealDB fields exactly
  - **Files**: `backend/crates/altair-db/src/schema/quest.rs`
  - **Verify**: Structs compile, serde works for SurrealDB roundtrip

- [ ] **5.3**: Define Knowledge domain types (Note, Folder, DailyNote)

  - **Acceptance**: 3 structs with correct field types (String, Option<T>, DateTime, etc.)
  - **Files**: `backend/crates/altair-db/src/schema/note.rs`
  - **Verify**: Structs match SurrealDB schema exactly

- [ ] **5.4**: Define Inventory domain types (Item, Location, Reservation, MaintenanceSchedule)

  - **Acceptance**: 4 structs with ownership and relationship fields
  - **Files**: `backend/crates/altair-db/src/schema/item.rs`
  - **Verify**: Structs compile and serialize correctly

- [ ] **5.5**: Define Capture type

  - **Acceptance**: `Capture` struct with capture_type enum, processed flag
  - **Files**: `backend/crates/altair-db/src/schema/capture.rs`
  - **Verify**: CaptureType enum matches SurrealDB constraint

- [ ] **5.6**: Define Gamification types (UserProgress, Achievement, Streak)

  - **Acceptance**: 3 structs with XP, level, streak tracking fields
  - **Files**: `backend/crates/altair-db/src/schema/gamification.rs`
  - **Verify**: Types support gamification logic

- [ ] **5.7**: Define shared types (User, Attachment, Tag)

  - **Acceptance**: 3 structs with auth and metadata fields
  - **Files**: `backend/crates/altair-db/src/schema/shared.rs`
  - **Verify**: User type includes email, display_name, preferences

- [ ] **5.8**: Define enum types (QuestColumn, EnergyLevel, CaptureType, etc.)

  - **Acceptance**: Enums match SurrealDB ASSERT constraints exactly (QuestColumn: Backlog/Wip/Done, EnergyLevel: Tiny/Small/Medium/Large/Huge, EntityStatus: Active/Archived)
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs`
  - **Verify**: `#[serde(rename_all = "lowercase")]` matches SurrealDB enum storage

- [ ] **5.9**: Add serde derives for SurrealDB serialization
  - **Acceptance**: All types have `#[derive(Serialize, Deserialize)]` and appropriate serde attributes
  - **Files**: All schema files
  - **Verify**: Test roundtrip: struct → SurrealDB → struct preserves data

---

## Phase 6: Seed Data and Testing

**Goal**: Create optional seed data and comprehensive tests.

- [ ] **6.1**: Create 004_seed_data.surql (optional, for dev)

  - **Acceptance**: File exists with `USE NS altair DB main;` header, marked as optional in README
  - **Files**: `backend/migrations/004_seed_data.surql`
  - **Verify**: Migration runner can skip this file in production mode

- [ ] **6.2**: Add sample user with preferences

  - **Acceptance**: INSERT sample user with realistic data (email, display_name, preferences object)
  - **Files**: `backend/migrations/004_seed_data.surql`
  - **Verify**: User can be queried after migration

- [ ] **6.3**: Add sample campaign with quests

  - **Acceptance**: INSERT 1 campaign and 3 quests with different columns, energy costs, `contains` edges linking them
  - **Files**: `backend/migrations/004_seed_data.surql`
  - **Verify**: Graph query `SELECT ->contains->quest FROM campaign:sample` returns 3 quests

- [ ] **6.4**: Add sample notes with links

  - **Acceptance**: INSERT 3 notes with wiki-links via `links_to` edges (bidirectional relationships)
  - **Files**: `backend/migrations/004_seed_data.surql`
  - **Verify**: Graph traversal returns linked notes

- [ ] **6.5**: Test: Fresh migration creates all tables (SC-001, SC-002)

  - **Acceptance**: Integration test runs all migrations on empty DB, counts tables, verifies 15+ entity + 10 edge tables exist
  - **Files**: `backend/crates/altair-db/tests/migration_test.rs`
  - **Verify**: `cargo test -p altair-db test_fresh_migration` passes

- [ ] **6.6**: Test: CHANGEFEED enabled on all tables (SC-003)

  - **Acceptance**: Test queries `INFO FOR TABLE` for each entity table, asserts changefeed is present
  - **Files**: `backend/crates/altair-db/tests/migration_test.rs`
  - **Verify**: `cargo test -p altair-db test_changefeed` passes

- [ ] **6.7**: Test: Field assertions reject invalid data (SC-004)

  - **Acceptance**: Test attempts INSERT with invalid enum values (e.g., quest.column = 'invalid'), asserts error returned
  - **Files**: `backend/crates/altair-db/tests/migration_test.rs`
  - **Verify**: `cargo test -p altair-db test_field_assertions` passes

- [ ] **6.8**: Test: Running migrations twice is idempotent (SC-006)

  - **Acceptance**: Test runs migrations, then runs again, verifies no errors and `_migrations` table unchanged
  - **Files**: `backend/crates/altair-db/tests/migration_test.rs`
  - **Verify**: `cargo test -p altair-db test_idempotent_migrations` passes

- [ ] **6.9**: Test: CHANGEFEED captures INSERT/UPDATE/DELETE (US-003)
  - **Acceptance**: Test inserts quest, updates it, deletes it, queries changefeed, verifies 3 events captured
  - **Files**: `backend/crates/altair-db/tests/migration_test.rs`
  - **Verify**: `cargo test -p altair-db test_changefeed_events` passes

---

## Phase 7: Verification and Documentation

**Goal**: Final verification and update documentation.

- [ ] **7.1**: Verify migration applies in < 5 seconds

  - **Acceptance**: Benchmark test runs full migration suite, asserts total time < 5000ms
  - **Files**: `backend/crates/altair-db/benches/migration_bench.rs`
  - **Verify**: `cargo bench -p altair-db` shows migration time

- [ ] **7.2**: Verify all 15+ entity tables created

  - **Acceptance**: Manual or automated verification via `INFO FOR DB` lists all expected tables
  - **Files**: N/A (verification step)
  - **Verify**: Console or test output shows complete table list

- [ ] **7.3**: Verify all 10 edge tables created

  - **Acceptance**: `INFO FOR DB` shows all graph edge tables (contains, references, requires, links_to, stored_in, documents, reserved_for, blocks, has_attachment, tagged)
  - **Files**: N/A (verification step)
  - **Verify**: All edges queryable via graph syntax

- [ ] **7.4**: Verify indexes created correctly

  - **Acceptance**: For each indexed table, `INFO FOR TABLE [name]` shows expected indexes
  - **Files**: N/A (verification step)
  - **Verify**: owner, status, full-text, unique indexes present

- [ ] **7.5**: Update technical-architecture.md if needed

  - **Acceptance**: Review current tech arch doc, add section on migration system if missing, ensure schema matches domain model
  - **Files**: `docs/technical-architecture.md`
  - **Verify**: Doc accurately reflects implemented schema

- [ ] **7.6**: Commit and push all changes
  - **Acceptance**: All migration files, Rust code, and tests committed on `spec/core-002-schema-migrations` branch
  - **Files**: N/A (git operations)
  - **Verify**: `git log` shows clean commit history, `git push` succeeds

---

## Task Summary

| Phase     | Tasks  | Completion |
| --------- | ------ | ---------- |
| Phase 1   | 8      | 0/8        |
| Phase 2   | 10     | 0/10       |
| Phase 3   | 10     | 0/10       |
| Phase 4   | 8      | 0/8        |
| Phase 5   | 9      | 0/9        |
| Phase 6   | 9      | 0/9        |
| Phase 7   | 6      | 0/6        |
| **Total** | **60** | **0/60**   |

## Next Steps

1. Execute `/spectrena.implement` to start implementation
2. Begin with Phase 1 tasks (Migration Infrastructure)
3. Update task checkboxes as work progresses
4. Run `cargo test -p altair-db` after each phase to validate
5. Create pull request when all phases complete

## Dependencies

- **Blocks**: CORE-003 (backend Tauri commands), GUIDANCE-001, KNOWLEDGE-001, TRACKING-001
- **Depends on**: CORE-001 (monorepo setup) ✅ Complete

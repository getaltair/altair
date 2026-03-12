# Phase 2 — Complete execution checklist

## Goal

Get to a working Postgres schema with:

* `sqlx` as the migration tool
* sequential migration files
* migrations owned by `apps/server`
* reference SQL and seeds in `packages/db`
* a repeatable dev workflow

---

# 0. Final structure to use

Use this layout:

```text
apps/
  server/
    migrations/

packages/
  db/
    schema/
    seeds/
    views/
    triggers/

scripts/
tests/
```

## Meaning

* `apps/server/migrations/` = **executable sqlx migration history**
* `packages/db/schema/` = **reference SQL docs/canonical grouped SQL**
* `packages/db/seeds/` = **dev/test seed SQL**
* `packages/db/views/` = optional SQL views later
* `packages/db/triggers/` = optional trigger/reference SQL later

---

# 1. Install the migration tool

## Step 1.1

Install `sqlx-cli` once:

```bash
cargo install sqlx-cli
```

## Done when

* `sqlx` is available in your shell

---

# 2. Stand up local Postgres

## Step 2.1

Make sure your local Postgres instance is running.

This can be Docker Compose, podman, system postgres, whatever you already chose.

## Step 2.2

Make sure you know the actual connection string.

You need a working `DATABASE_URL`.

Example shape:

```text
postgres://user:password@localhost:5432/altair
```

## Step 2.3

Verify you can connect to the DB manually before touching migrations.

## Done when

* Postgres is running
* database exists
* you have a known-good `DATABASE_URL`

---

# 3. Create the database directories

## Step 3.1

Create this if it does not already exist:

```text
apps/server/migrations/
```

## Step 3.2

Create these if they do not already exist:

```text
packages/db/schema/
packages/db/seeds/
packages/db/views/
packages/db/triggers/
```

## Done when

* all DB-related directories exist in the correct places

---

# 4. Decide the migration execution workflow

## Rule

Use:

* **Cargo** to install Rust-side tooling
* **sqlx CLI** to run migrations
* **Bun** only as a future repo command wrapper if you want convenience

So for now, your real migration commands are `sqlx` commands.

## Done when

* you stop expecting Bun to be the DB migration engine itself

---

# 5. Lock the migration file sequence

Create these files manually in:

```text
apps/server/migrations/
```

## Initial migration list

```text
0001_users.sql
0002_households.sql
0003_household_memberships.sql
0004_initiatives.sql
0005_guidance_core.sql
0006_knowledge_core.sql
0007_tracking_core.sql
0008_tags.sql
0009_attachments.sql
0010_entity_relations.sql
```

## Important

Do **not** use `sqlx migrate add` for these, because that creates timestamp-style names, not sequential names.

## Done when

* all 10 empty migration files exist with the correct sequential names

---

# 6. Create the reference SQL files

These are not the executable migration history. They are grouped reference files.

Create these in:

```text
packages/db/schema/
```

## Suggested reference files

```text
core.sql
guidance.sql
knowledge.sql
tracking.sql
tags.sql
attachments.sql
entity_relations.sql
```

## Purpose

These help you keep the schema readable at the domain level, while migrations remain the historical execution record.

## Done when

* the schema area exists and has the grouped domain reference files you want to maintain

---

# 7. Create the seed file location

In:

```text
packages/db/seeds/
```

Create at least:

```text
dev_seed.sql
```

You do **not** need to fully populate it yet, but the place should exist now.

## Done when

* seed location exists and is clearly separated from migrations

---

# 8. Set your shell environment for migration runs

Before running migrations, export `DATABASE_URL` in the shell session you are using.

## Done when

* `sqlx migrate run` can find the DB without guessing

---

# 9. Migration-by-migration workflow

This is the exact workflow you should repeat for **every migration file**.

## For each migration

### Step A

Open the migration file.

### Step B

Write only the SQL for that migration’s scope.

### Step C

Run the migration from `apps/server`:

```bash
cd apps/server
sqlx migrate run
```

### Step D

Check migration status:

```bash
sqlx migrate info
```

### Step E

Inspect the DB manually to confirm the objects were created correctly.

### Step F

Only then move to the next migration.

## Done when

* each migration is applied cleanly before the next one starts

---

# 10. Contents checklist for each migration

## `0001_users.sql`

### Include

* `users` table
* PK
* email
* unique email constraint
* display name
* timezone
* active flag
* created/updated timestamps
* optional soft delete field

### Done when

* users table exists and email uniqueness works

---

## `0002_households.sql`

### Include

* `households` table
* PK
* owner user FK
* name
* slug if using one
* description
* timestamps
* optional soft delete

### Done when

* households table exists
* owner FK works

---

## `0003_household_memberships.sql`

### Include

* `household_memberships`
* household FK
* user FK
* role
* active flag
* timestamps
* unique `(household_id, user_id)`

### Done when

* one user cannot be added twice to the same household

---

## `0004_initiatives.sql`

### Include

* `initiatives`
* owner user FK
* optional household FK
* title
* slug
* description
* status
* start date
* target date
* timestamps
* optional soft delete

### Done when

* initiatives can be personal or household-scoped

---

## `0005_guidance_core.sql`

### Include

* `guidance_epics`
* `guidance_quests`
* `guidance_routines`
* `guidance_focus_sessions`
* `guidance_daily_checkins`

### Done when

* Guidance tables exist with their basic FKs and status fields

---

## `0006_knowledge_core.sql`

### Include

* `knowledge_notes`
* `knowledge_note_snapshots`

### Done when

* note hierarchy and note snapshot linkage work

---

## `0007_tracking_core.sql`

### Include

* `tracking_locations`
* `tracking_categories`
* `tracking_items`
* `tracking_item_events`
* `tracking_shopping_lists`
* `tracking_shopping_list_items`

### Done when

* tracking supports both current item state and event history

---

## `0008_tags.sql`

### Include

* `tags`
* `initiative_tags`
* `quest_tags`
* `note_tags`
* `item_tags`

### Done when

* tags can be linked to all main entity types explicitly

---

## `0009_attachments.sql`

### Include

* `attachments`
* `quest_attachments`
* `note_attachments`
* `item_attachments`

### Done when

* attachment metadata exists and entity link tables exist

---

## `0010_entity_relations.sql`

### Include

* `entity_relations`
* from/to entity type + id
* relation type
* source type
* status
* confidence
* evidence JSON
* ownership/scope columns
* indexes for from/to lookups

### Done when

* cross-domain relationships can be represented explicitly

---

# 11. Commands you will actually use

## Preferred working directory

Use:

```bash
cd apps/server
```

Then run:

### Run all pending migrations

```bash
sqlx migrate run
```

### Show migration status

```bash
sqlx migrate info
```

### Revert the most recent migration

```bash
sqlx migrate revert
```

---

## If you insist on staying at repo root

Use:

### Run migrations

```bash
sqlx migrate run --source apps/server/migrations
```

### Status

```bash
sqlx migrate info --source apps/server/migrations
```

### Revert

```bash
sqlx migrate revert --source apps/server/migrations
```

---

# 12. Where Bun fits

Bun is **not** the migration engine.

Use Bun later only to create convenient repo commands like:

* `bun run db:migrate`
* `bun run db:migrate:status`
* `bun run db:migrate:revert`

Those would just wrap the `sqlx` commands above.

So for Phase 2, the real commands are still:

* `cargo install sqlx-cli`
* `sqlx migrate run`
* `sqlx migrate info`
* `sqlx migrate revert`

---

# 13. Validation after each migration

After each migration, verify all three:

## 13.1 Migration tracking

Run:

```bash
sqlx migrate info
```

Confirm the migration shows as applied.

## 13.2 Schema inspection

Check the table/constraints/indexes in Postgres directly.

## 13.3 Dependency sanity

Make sure the next migration can legally depend on what you just created.

---

# 14. Reference SQL workflow

After finishing a migration, update the matching reference file in:

```text
packages/db/schema/
```

Example:

* after `0001_users.sql`, update `packages/db/schema/core.sql`
* after `0005_guidance_core.sql`, update `packages/db/schema/guidance.sql`

## Rule

* **migrations** = executable history
* **schema files** = readable canonical grouped reference

Do not confuse the two.

---

# 15. Seed workflow

Do **not** seed during the first migration pass unless you need it immediately.

First:

* get schema stable

Then:

* add `packages/db/seeds/dev_seed.sql`

After the 10 migrations exist, load seeds explicitly.

---

# 16. What “Phase 2 complete” means

Phase 2 is complete when all of these are true:

* [ ] `sqlx-cli` installed
* [ ] local Postgres running
* [ ] `apps/server/migrations/` exists
* [ ] `packages/db/schema/` exists
* [ ] `packages/db/seeds/` exists
* [ ] 10 sequential migration files exist
* [ ] all 10 migrations apply cleanly
* [ ] `sqlx migrate info` shows expected applied state
* [ ] grouped reference SQL exists in `packages/db/schema/`
* [ ] schema supports user / household / initiative scopes
* [ ] you are ready to add seed data next

---

# 17. Recommended exact order of execution

Do this in order:

1. install `sqlx-cli`
2. verify local Postgres + `DATABASE_URL`
3. create DB directories
4. create all 10 empty migration files
5. create grouped reference schema files
6. write `0001_users.sql`
7. run migration
8. verify
9. write `0002_households.sql`
10. run migration
11. verify
12. continue sequentially through `0010_entity_relations.sql`
13. update grouped reference SQL as you go
14. only after all that, start seed work

---

# Key Points

* Use `apps/server/migrations/` for executable `sqlx` history.
* Use `packages/db/schema/` for grouped reference SQL.
* Use `sqlx` commands directly; Bun is only a future wrapper.

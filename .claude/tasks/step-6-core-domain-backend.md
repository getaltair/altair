# Plan: Step 6 - Core Domain Backend

## Task Description

Implement the shared core domain entities that all three product domains (Guidance, Knowledge, Tracking) depend on. This includes: initiatives, tags, attachments metadata, and the entity_relations table. These are the foundational building blocks referenced throughout the Altair system - no domain-specific data yet, just the shared foundation.

Work happens on a new feature branch `feat/step-6-core-domain-backend` off `main`.

## Objective

When complete, the Rust backend will have:

- Full CRUD API for initiatives (user-scoped and household-scoped)
- Full CRUD API for tags (user-scoped and household-scoped)
- Entity relations API with registry-validated create/query/update
- Attachments metadata table (binary upload deferred to Step 16)
- All endpoints authenticated and authorized
- All registry values (entity types, relation types, source types, status) validated at write time

## Problem Statement

The backend currently has auth (users/sessions) and households, but no domain entities. Steps 8-11 (Guidance, Knowledge, Tracking, Relationships features) all depend on these shared core tables and APIs existing first. This step bridges auth/identity to domain features.

## Solution Approach

Follow the established patterns from the households module (models.rs / service.rs / handlers.rs per domain) and extend to four new domains. Create a contracts module with canonical registry values so validation is centralized. Use SQL migrations for schema, sqlx for queries, and the existing AppError/AuthenticatedUser patterns for error handling and auth.

## Relevant Files

Use these files to understand existing patterns and complete the task:

- `apps/server/src/core/households/models.rs` - Pattern for domain models, request DTOs, validation derives
- `apps/server/src/core/households/service.rs` - Pattern for service functions (async, PgPool, AppError)
- `apps/server/src/core/households/handlers.rs` - Pattern for axum handlers (AuthenticatedUser, State, Json, validation)
- `apps/server/src/core/households/mod.rs` - Module re-exports pattern
- `apps/server/src/core/mod.rs` - Where new domain modules get registered
- `apps/server/src/api/mod.rs` - Router with route registration and AppState
- `apps/server/src/error.rs` - AppError enum (may need no changes)
- `apps/server/src/auth/middleware.rs` - AuthenticatedUser extractor
- `apps/server/Cargo.toml` - Dependencies (should need no changes)
- `docs/altair-implementation-plan.md` (lines 505-568) - Step 6 spec with tables, endpoints, done criteria
- `docs/sync/altair-entity-type-registry.md` - Canonical entity types, relation types, source types, status values
- `docs/altair-shared-contracts-spec.md` - Shared contracts design (sections 5-8)
- `docs/adr/ADR-004-relationship-modeling-strategy.md` - Entity relations record shape and rationale
- `docs/schema/altair-initial-schema.sql` - Reference SQL schema
- `apps/server/migrations/` - Existing migration files (5 exist: users, sessions, households, memberships, role check)

### New Files

- `apps/server/migrations/20260326100001_create_initiatives.sql`
- `apps/server/migrations/20260326100002_create_tags.sql`
- `apps/server/migrations/20260326100003_create_attachments.sql`
- `apps/server/migrations/20260326100004_create_entity_relations.sql`
- `apps/server/src/contracts.rs` - Canonical registry constants and validation helpers
- `apps/server/src/core/initiatives/mod.rs`
- `apps/server/src/core/initiatives/models.rs`
- `apps/server/src/core/initiatives/service.rs`
- `apps/server/src/core/initiatives/handlers.rs`
- `apps/server/src/core/tags/mod.rs`
- `apps/server/src/core/tags/models.rs`
- `apps/server/src/core/tags/service.rs`
- `apps/server/src/core/tags/handlers.rs`
- `apps/server/src/core/relations/mod.rs`
- `apps/server/src/core/relations/models.rs`
- `apps/server/src/core/relations/service.rs`
- `apps/server/src/core/relations/handlers.rs`
- `apps/server/src/core/attachments/mod.rs`
- `apps/server/src/core/attachments/models.rs`

## Implementation Phases

### Phase 1: Foundation

- Create feature branch `feat/step-6-core-domain-backend` from `main`
- Write 4 SQL migration files for initiatives, tags, attachments, entity_relations tables
- Create `src/contracts.rs` with canonical constants for entity types, relation types, source types, and status values, plus validation functions
- Register `mod contracts;` in `main.rs`

### Phase 2: Core Implementation

Build four domain modules in parallel, each following the households pattern:

**Initiatives** (`src/core/initiatives/`):

- `models.rs`: Initiative struct (FromRow), CreateInitiativeRequest, UpdateInitiativeRequest with validation
- `service.rs`: create, list (by user + by household), get by id, update, soft-delete
- `handlers.rs`: CRUD handlers with auth + household membership authorization

**Tags** (`src/core/tags/`):

- `models.rs`: Tag struct (FromRow), CreateTagRequest, UpdateTagRequest
- `service.rs`: create, list (by user + by household), update, delete
- `handlers.rs`: CRUD handlers with auth + household authorization

**Entity Relations** (`src/core/relations/`):

- `models.rs`: EntityRelation struct (FromRow) with all ADR-004 columns, CreateRelationRequest, UpdateRelationStatusRequest
- `service.rs`: create (with registry validation), query (by from-entity, to-entity, or both), update status
- `handlers.rs`: Create/query/update handlers with auth, registry validation returning 400 for unknown types

**Attachments** (`src/core/attachments/`):

- `models.rs`: Attachment struct (FromRow) - metadata only, no handlers yet (binary upload deferred to Step 16)

### Phase 3: Integration & Polish

- Register all new modules in `src/core/mod.rs`
- Add all routes to `src/api/mod.rs` `create_router()`
- Verify `cargo build` succeeds
- Verify `cargo test` passes
- Verify `cargo clippy` is clean

## Team Orchestration

- You operate as the team lead and orchestrate the team to execute the plan.
- You're responsible for deploying the right team members with the right context to execute the plan.
- IMPORTANT: You NEVER operate directly on the codebase. You use `Task` and `Task*` tools to deploy team members to the building, validating, testing, deploying, and other tasks.
  - This is critical. Your job is to act as a high level director of the team, not a builder.
  - Your role is to validate all work is going well and make sure the team is on track to complete the plan.
  - You'll orchestrate this by using the Task\* Tools to manage coordination between the team members.
  - Communication is paramount. You'll use the Task\* Tools to communicate with the team members and ensure they're on track to complete the plan.
- Take note of the session id of each team member. This is how you'll reference them.

### Team Members

Available specialist agents: `frontend-specialist`, `backend-engineer`, `supabase-specialist`, `security-auditor`, `performance-optimizer`, `quality-engineer`, `general-purpose`

- Specialist
  - Name: builder-foundation
  - Role: Create feature branch, write SQL migrations, and create the contracts module with canonical registry constants
  - Agent Type: backend-engineer
  - Resume: true
- Specialist
  - Name: builder-initiatives
  - Role: Build the initiatives domain module (models, service, handlers) following households patterns
  - Agent Type: backend-engineer
  - Resume: true
- Specialist
  - Name: builder-tags
  - Role: Build the tags domain module (models, service, handlers) following households patterns
  - Agent Type: backend-engineer
  - Resume: true
- Specialist
  - Name: builder-relations
  - Role: Build the entity_relations domain module (models, service, handlers) with registry validation
  - Agent Type: backend-engineer
  - Resume: true
- Specialist
  - Name: builder-integration
  - Role: Register all modules in core/mod.rs, wire all routes in api/mod.rs, create attachments model, verify compilation
  - Agent Type: backend-engineer
  - Resume: true
- Quality Engineer (Validator)
  - Name: validator
  - Role: Validate completed work against acceptance criteria (read-only inspection mode)
  - Agent Type: quality-engineer
  - Resume: false

## Step by Step Tasks

- IMPORTANT: Execute every step in order, top to bottom. Each task maps directly to a `TaskCreate` call.
- Before you start, run `TaskCreate` to create the initial task list that all team members can see and execute.

### 1. Create Feature Branch and SQL Migrations

- **Task ID**: create-branch-and-migrations
- **Depends On**: none
- **Assigned To**: builder-foundation
- **Agent Type**: backend-engineer
- **Parallel**: false
- Create branch `feat/step-6-core-domain-backend` from `main`
- Create migration `20260326100001_create_initiatives.sql`:
  ```sql
  CREATE TABLE IF NOT EXISTS initiatives (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      user_id UUID NOT NULL REFERENCES users(id),
      household_id UUID REFERENCES households(id),
      name TEXT NOT NULL,
      description TEXT,
      status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'paused', 'completed', 'archived')),
      created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
      updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
  );
  CREATE INDEX idx_initiatives_user ON initiatives(user_id);
  CREATE INDEX idx_initiatives_household ON initiatives(household_id);
  ```
- Create migration `20260326100002_create_tags.sql`:
  ```sql
  CREATE TABLE IF NOT EXISTS tags (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      user_id UUID NOT NULL REFERENCES users(id),
      household_id UUID REFERENCES households(id),
      name TEXT NOT NULL,
      color TEXT,
      created_at TIMESTAMPTZ NOT NULL DEFAULT now()
  );
  CREATE INDEX idx_tags_user ON tags(user_id);
  ```
- Create migration `20260326100003_create_attachments.sql`:
  ```sql
  CREATE TABLE IF NOT EXISTS attachments (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      entity_type TEXT NOT NULL,
      entity_id UUID NOT NULL,
      filename TEXT NOT NULL,
      content_type TEXT NOT NULL,
      storage_key TEXT NOT NULL,
      size_bytes BIGINT NOT NULL,
      processing_state TEXT NOT NULL DEFAULT 'pending' CHECK (processing_state IN ('pending', 'processing', 'ready', 'failed')),
      created_at TIMESTAMPTZ NOT NULL DEFAULT now()
  );
  CREATE INDEX idx_attachments_entity ON attachments(entity_type, entity_id);
  ```
- Create migration `20260326100004_create_entity_relations.sql`:
  ```sql
  CREATE TABLE IF NOT EXISTS entity_relations (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      from_entity_type TEXT NOT NULL,
      from_entity_id UUID NOT NULL,
      to_entity_type TEXT NOT NULL,
      to_entity_id UUID NOT NULL,
      relation_type TEXT NOT NULL,
      source_type TEXT NOT NULL,
      status TEXT NOT NULL DEFAULT 'accepted',
      confidence NUMERIC CHECK (confidence >= 0.0 AND confidence <= 1.0),
      evidence_json JSONB,
      created_by_user_id UUID REFERENCES users(id),
      created_by_process TEXT,
      created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
      updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
      last_confirmed_at TIMESTAMPTZ
  );
  CREATE INDEX idx_relations_from ON entity_relations(from_entity_type, from_entity_id);
  CREATE INDEX idx_relations_to ON entity_relations(to_entity_type, to_entity_id);
  CREATE INDEX idx_relations_status ON entity_relations(status);
  ```

### 2. Create Contracts Module

- **Task ID**: create-contracts-module
- **Depends On**: create-branch-and-migrations
- **Assigned To**: builder-foundation
- **Agent Type**: backend-engineer
- **Parallel**: false
- Create `apps/server/src/contracts.rs` with:
  - `ENTITY_TYPES: &[&str]` containing all canonical entity types from `docs/sync/altair-entity-type-registry.md` (user, household, initiative, tag, attachment, guidance_epic, guidance_quest, guidance_routine, guidance_focus_session, guidance_daily_checkin, knowledge_note, knowledge_note_snapshot, tracking_location, tracking_category, tracking_item, tracking_item_event, tracking_shopping_list, tracking_shopping_list_item)
  - `RELATION_TYPES: &[&str]` (references, supports, requires, related_to, depends_on, duplicates, similar_to, generated_from)
  - `SOURCE_TYPES: &[&str]` (user, ai, import, rule, migration, system)
  - `RELATION_STATUSES: &[&str]` (accepted, suggested, dismissed, rejected, expired)
  - `INITIATIVE_STATUSES: &[&str]` (active, paused, completed, archived)
  - `ATTACHMENT_STATES: &[&str]` (pending, processing, ready, failed)
  - Validation helper functions: `is_valid_entity_type()`, `is_valid_relation_type()`, `is_valid_source_type()`, `is_valid_relation_status()`
  - Unit tests for all validation functions
- Add `mod contracts;` to `main.rs` (after `mod core;`)

### 3. Build Initiatives Module

- **Task ID**: build-initiatives
- **Depends On**: create-contracts-module
- **Assigned To**: builder-initiatives
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside build-tags and build-relations)
- Read `apps/server/src/core/households/` for pattern reference
- Create `apps/server/src/core/initiatives/mod.rs` exposing handlers, models, service
- Create `apps/server/src/core/initiatives/models.rs`:
  - `Initiative` struct (id, user_id, household_id Option, name, description Option, status, created_at, updated_at) with `sqlx::FromRow` + `Serialize`
  - `CreateInitiativeRequest` with validator: name (1-200 chars required), description (optional), household_id (optional), status (optional, defaults to active)
  - `UpdateInitiativeRequest`: name (optional), description (optional), status (optional)
  - Unit tests for request validation
- Create `apps/server/src/core/initiatives/service.rs`:
  - `create_initiative(pool, user_id, req)` - Insert with user_id, optional household_id. If household_id provided, verify user is a member via `crate::core::households::service::is_member()`
  - `list_initiatives(pool, user_id, household_id: Option)` - List by user or by household (if household_id, verify membership)
  - `get_initiative(pool, id, user_id)` - Get by id, verify ownership or household membership
  - `update_initiative(pool, id, user_id, req)` - Update fields, verify ownership or household membership
  - `soft_delete_initiative(pool, id, user_id)` - Set status to 'archived', verify ownership
- Create `apps/server/src/core/initiatives/handlers.rs`:
  - `create_initiative` - POST handler, validate body, delegate to service
  - `list_initiatives` - GET handler with optional `household_id` query param
  - `get_initiative` - GET handler with path param `:id`
  - `update_initiative` - PUT handler with path param `:id`
  - `delete_initiative` - DELETE handler with path param `:id` (soft delete)
  - All handlers use `AuthenticatedUser` extractor

### 4. Build Tags Module

- **Task ID**: build-tags
- **Depends On**: create-contracts-module
- **Assigned To**: builder-tags
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside build-initiatives and build-relations)
- Read `apps/server/src/core/households/` for pattern reference
- Create `apps/server/src/core/tags/mod.rs` exposing handlers, models, service
- Create `apps/server/src/core/tags/models.rs`:
  - `Tag` struct (id, user_id, household_id Option, name, color Option, created_at) with `sqlx::FromRow` + `Serialize`
  - `CreateTagRequest` with validator: name (1-50 chars required), color (optional), household_id (optional)
  - `UpdateTagRequest`: name (optional), color (optional)
- Create `apps/server/src/core/tags/service.rs`:
  - `create_tag(pool, user_id, req)` - Insert, verify household membership if household_id provided
  - `list_tags(pool, user_id, household_id: Option)` - List by user or by household
  - `update_tag(pool, id, user_id, req)` - Update, verify ownership
  - `delete_tag(pool, id, user_id)` - Hard delete, verify ownership
- Create `apps/server/src/core/tags/handlers.rs`:
  - `create_tag` - POST handler
  - `list_tags` - GET handler with optional `household_id` query param
  - `update_tag` - PUT handler with path param `:id`
  - `delete_tag` - DELETE handler with path param `:id`
  - All handlers use `AuthenticatedUser` extractor

### 5. Build Entity Relations Module

- **Task ID**: build-relations
- **Depends On**: create-contracts-module
- **Assigned To**: builder-relations
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside build-initiatives and build-tags)
- Read `apps/server/src/core/households/` for pattern reference
- Read `docs/adr/ADR-004-relationship-modeling-strategy.md` for full column spec
- Read `docs/sync/altair-entity-type-registry.md` for all canonical values
- Create `apps/server/src/core/relations/mod.rs` exposing handlers, models, service
- Create `apps/server/src/core/relations/models.rs`:
  - `EntityRelation` struct with ALL ADR-004 columns: id, from_entity_type, from_entity_id, to_entity_type, to_entity_id, relation_type, source_type, status, confidence (Option f64), evidence_json (Option serde_json::Value), created_by_user_id (Option Uuid), created_by_process (Option String), created_at, updated_at, last_confirmed_at (Option). Use `sqlx::FromRow` + `Serialize`
  - `CreateRelationRequest`: from_entity_type, from_entity_id, to_entity_type, to_entity_id, relation_type, source_type (optional, defaults to "user"), confidence (optional), evidence_json (optional)
  - `UpdateRelationStatusRequest`: status (required), last_confirmed_at (optional)
  - `RelationQuery` for GET filters: from_entity_type (optional), from_entity_id (optional), to_entity_type (optional), to_entity_id (optional), relation_type (optional), status (optional)
- Create `apps/server/src/core/relations/service.rs`:
  - `create_relation(pool, user_id, req)` - Validate entity_type, relation_type, source_type against contracts module. Reject unknown values with AppError::BadRequest. Insert with created_by_user_id = user_id
  - `query_relations(pool, query: RelationQuery)` - Build dynamic WHERE clause from provided filters. At least one filter required (from or to entity)
  - `update_relation_status(pool, id, user_id, req)` - Update status and optionally last_confirmed_at. Validate status against contracts
- Create `apps/server/src/core/relations/handlers.rs`:
  - `create_relation` - POST handler, validate registry values, delegate to service
  - `query_relations` - GET handler with query params for filtering
  - `update_relation_status` - PUT handler with path param `:id`
  - All handlers use `AuthenticatedUser` extractor

### 6. Integration - Wire Routes and Register Modules

- **Task ID**: wire-routes
- **Depends On**: build-initiatives, build-tags, build-relations
- **Assigned To**: builder-integration
- **Agent Type**: backend-engineer
- **Parallel**: false
- Create `apps/server/src/core/attachments/mod.rs` with `pub mod models;`
- Create `apps/server/src/core/attachments/models.rs` with Attachment struct (id, entity_type, entity_id, filename, content_type, storage_key, size_bytes i64, processing_state, created_at) using `sqlx::FromRow` + `Serialize`
- Update `apps/server/src/core/mod.rs` to add: `pub mod attachments;`, `pub mod initiatives;`, `pub mod tags;`, `pub mod relations;`
- Update `apps/server/src/api/mod.rs`:
  - Add imports for initiative_handlers, tag_handlers, relation_handlers
  - Register initiative routes:
    - `POST /core/initiatives` + `GET /core/initiatives`
    - `GET /core/initiatives/{id}` + `PUT /core/initiatives/{id}` + `DELETE /core/initiatives/{id}`
  - Register tag routes:
    - `POST /core/tags` + `GET /core/tags`
    - `PUT /core/tags/{id}` + `DELETE /core/tags/{id}`
  - Register relation routes:
    - `POST /core/relations` + `GET /core/relations`
    - `PUT /core/relations/{id}`
- Run `cargo build` to verify compilation
- Run `cargo test` to verify all tests pass
- Run `cargo clippy` to verify no warnings

### 7. Final Validation

- **Task ID**: validate-all
- **Depends On**: wire-routes
- **Assigned To**: validator
- **Agent Type**: quality-engineer
- **Parallel**: false
- Verify all acceptance criteria are met (see below)
- Check that all endpoints require authentication via `AuthenticatedUser`
- Check that household-scoped operations verify membership
- Check that relation creation validates entity_type, relation_type, source_type, status against contracts
- Check that unknown entity types return 400
- Check that the attachments metadata table exists with correct columns
- Verify `cargo build` succeeds with no errors
- Verify `cargo test` passes
- Verify `cargo clippy` produces no warnings
- Operate in validation mode: inspect and report only, do not modify files

## Acceptance Criteria

- [ ] Initiatives CRUD works with user and household scoping
- [ ] Tags CRUD works with user and household scoping
- [ ] `entity_relations` table exists with all columns from ADR-004
- [ ] Relation create endpoint validates entity_type, relation_type, source_type, status against registry
- [ ] Unknown entity types are rejected with 400
- [ ] Relations can be queried by from-entity, to-entity, or both
- [ ] Relation status can be updated (accept/reject/dismiss)
- [ ] Attachment metadata table exists (binary upload deferred to Step 16)
- [ ] All endpoints require authentication and enforce user/household authorization
- [ ] `cargo build` succeeds
- [ ] `cargo test` passes
- [ ] `cargo clippy` is clean

## Validation Commands

Execute these commands to validate the task is complete:

- `cd apps/server && cargo build 2>&1` - Verify the project builds without errors
- `cd apps/server && cargo test 2>&1` - Run the test suite
- `cd apps/server && cargo clippy -- -D warnings 2>&1` - Verify no clippy warnings
- `ls apps/server/migrations/` - Verify all 4 new migration files exist
- `ls apps/server/src/core/initiatives/ apps/server/src/core/tags/ apps/server/src/core/relations/ apps/server/src/core/attachments/` - Verify all domain module directories exist

## Notes

- Follow exact patterns from `apps/server/src/core/households/` for module structure, service signatures, and handler patterns
- The `AuthenticatedUser` extractor from `apps/server/src/auth/middleware.rs` provides `user_id` and `session_id`
- Household membership checks should reuse `crate::core::households::service::is_member()` and `get_member_role()`
- No new Cargo dependencies needed - existing deps (sqlx, serde, serde_json, validator, uuid, chrono) cover everything
- Soft-delete for initiatives means setting status to 'archived', not removing the row
- Tags use hard delete since they're lightweight labels
- The attachments module is metadata-only in this step - no upload/download handlers (deferred to Step 16)
- Entity relations `confidence` field is nullable (NULL for user-created relations, 0.0-1.0 for AI-suggested)
- The `evidence_json` column uses JSONB in Postgres and maps to `serde_json::Value` in Rust

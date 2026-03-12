# Altair Phase 0–4 Execution Checklist

| Field | Value |
|---|---|
| **Document** | Altair Phase 0–4 Execution Checklist |
| **Version** | 1.0 |
| **Status** | Draft |
| **Last Updated** | 2026-03-11 |
| **Related Docs** | `altair-implementation-plan.md`, architecture spec, ADR-002/003/004, schema design spec, PowerSync sync spec, shared contracts spec |

---

# 1. Purpose

This document converts **Phases 0–4** of the implementation plan into an execution-ready checklist.

It is optimized for:

- turning architecture into work items
- preserving dependency order
- reducing hidden prerequisite chaos
- making “what do I build next?” obvious

This covers:

- **Phase 0:** Project Setup & Decision Lock
- **Phase 1:** Shared Contracts Foundation
- **Phase 2:** Database & Schema Foundation
- **Phase 3:** Backend Core Foundation
- **Phase 4:** PowerSync Foundation

---

# 2. Execution Rules

## Rule 1

Do not start broad feature work before **contracts, schema, auth, and sync basics** are proven.

## Rule 2

Every task should produce either:

- a working artifact
- a test
- a decision
- or a documented failure mode

## Rule 3

When a task reveals structural mismatch, fix the structure before layering more code on top.

That is less glamorous, but also less stupid.

---

# 3. Status Legend

- **[ ]** Not started
- **[~]** In progress
- **[x]** Complete
- **[!]** Blocked

---

# 4. High-Level Dependency Chain

```mermaid
flowchart TB
    Repo[Phase 0 Repo Setup] --> Contracts[Phase 1 Contracts]
    Contracts --> Schema[Phase 2 Schema]
    Schema --> Backend[Phase 3 Backend]
    Backend --> Sync[Phase 4 PowerSync]
    Contracts --> Backend
    Contracts --> Sync
```

---

# 5. Phase 0 — Project Setup & Decision Lock

## Goal

Create the repo, tooling, and baseline project structure so the rest of the work can proceed without constant reinvention.

## Exit Criteria

- monorepo exists
- app/package layout exists
- baseline CI exists
- formatting/linting exists
- docs are in repo
- team can clone and run baseline tooling

---

## P0-001 — Create Monorepo Skeleton

**Depends on:** nothing  
**Owner:** platform/dev setup

### Checklist

- [x] Create repo root structure:
  - [x] `apps/server`
  - [x] `apps/web`
  - [x] `apps/desktop`
  - [x] `apps/android`
  - [x] `apps/worker`
  - [x] `packages/`
  - [x] `infra/`
  - [x] `docs/prd`
  - [x] `docs/architecture`
  - [x] `docs/adr`

### Acceptance Criteria

- repo layout matches agreed architecture
- paths are documented in root README

---

## P0-002 — Define Workspace Tooling

**Depends on:** P0-001  
**Owner:** platform/dev setup

### Checklist

- [x] Choose JS workspace manager (decided: bun)
- [x] Add root workspace config
- [x] Add Rust workspace config if needed
- [x] Confirm Android Gradle structure
- [x] Decide root task runner conventions

### Acceptance Criteria

- one documented command path exists for:
  - [x] web install/build
  - [x] server build/test
  - [x] Android build/test
  - [x] shared scripts

---

## P0-003 — Add Formatting and Linting

**Depends on:** P0-001  
**Owner:** platform/dev setup

### Checklist

- [x] Add `.editorconfig`
- [x] Add TS formatter/lint config
- [x] Add Rust fmt/clippy config or scripts
- [x] Add Kotlin formatter/lint config or scripts
- [x] Add root README section for developer setup

### Acceptance Criteria

- [x] formatting can be run locally
- [x] lint commands exist for each active language

---

## P0-004 — Add Baseline CI Skeleton

**Depends on:** P0-002, P0-003  
**Owner:** platform/dev setup

### Checklist

- [ ] Add GitHub Actions workflow for:
  - [ ] TypeScript install/build placeholder
  - [ ] Rust build/test placeholder
  - [ ] Kotlin/Gradle build placeholder
  - [ ] contract validation placeholder
- [ ] Ensure CI runs on PRs

### Acceptance Criteria

- PRs trigger CI
- CI can succeed on baseline repo state

---

## P0-005 — Check In Current Architecture Artifacts

**Depends on:** P0-001  
**Owner:** architecture/docs

### Checklist

- [X] Add PRDs to `docs/prd`
- [X] Add architecture spec to `docs/architecture`
- [X] Add ADRs to `docs/adr`
- [X] Add schema design spec
- [X] Add PowerSync sync spec
- [X] Add shared contracts spec
- [X] Add implementation plan

### Acceptance Criteria

- all current decisions are version-controlled in repo

---

## Phase 0 Review Gate

- [x] Repo structure approved
- [x] Tooling conventions approved
- [ ] CI skeleton green
- [x] Current architecture docs committed

---

# 6. Phase 1 — Shared Contracts Foundation

## Goal

Prevent string drift and identifier chaos before backend and clients proliferate.

## Exit Criteria

- registry files exist
- generated bindings exist
- contract tests pass
- CI enforces generation + validation

---

## P1-001 — Add Registry JSON Files

**Depends on:** P0-001  
**Owner:** contracts/platform

### Checklist

- [ ] Create `packages/contracts/registry/entity-types.json`
- [ ] Create `packages/contracts/registry/relation-types.json`
- [ ] Create `packages/contracts/registry/sync-streams.json`
- [ ] Populate with current canonical values
- [ ] Add README explaining source-of-truth policy

### Acceptance Criteria

- registry files contain all canonical values currently approved
- no duplicate identifiers exist

---

## P1-002 — Add Shared Schema Files

**Depends on:** P1-001  
**Owner:** contracts/platform

### Checklist

- [ ] Add `RelationRecord` JSON schema
- [ ] Add `AttachmentRecord` JSON schema
- [ ] Add `EntityRef` JSON schema
- [ ] Add optional starter `SyncSubscriptionRequest` schema

### Acceptance Criteria

- schemas validate basic cross-platform payload shapes

---

## P1-003 — Add Codegen Script

**Depends on:** P1-001  
**Owner:** contracts/platform

### Checklist

- [ ] Add generator script under `scripts/`
- [ ] Generator reads registry JSON
- [ ] Generator emits:
  - [ ] TypeScript bindings
  - [ ] Kotlin bindings
  - [ ] Rust bindings
- [ ] Document generator usage

### Acceptance Criteria

- running generator creates outputs without manual edits

---

## P1-004 — Commit Generated Bindings

**Depends on:** P1-003  
**Owner:** contracts/platform

### Checklist

- [ ] Add generated TS constants
- [ ] Add generated Kotlin enums/data classes
- [ ] Add generated Rust enums/structs

### Acceptance Criteria

- generated artifacts are committed and readable
- identifiers match registry exactly

---

## P1-005 — Add Validation Tests

**Depends on:** P1-003, P1-004  
**Owner:** contracts/platform

### Checklist

- [ ] Add registry shape tests
- [ ] Add duplicate-value tests
- [ ] Add generated TS value tests
- [ ] Add generated Kotlin value tests
- [ ] Add generated Rust value tests

### Acceptance Criteria

- tests fail when registry and generated code drift

---

## P1-006 — Wire Contracts CI

**Depends on:** P1-005, P0-004  
**Owner:** platform/dev setup

### Checklist

- [ ] Add contracts generation workflow
- [ ] Run generator in CI
- [ ] Fail if `git diff --exit-code` finds changes
- [ ] Run contract validation tests in CI

### Acceptance Criteria

- PR fails if generated artifacts are stale
- PR fails if registry values are invalid

---

## P1-007 — Replace Magic Strings in Existing Artifacts

**Depends on:** P1-004  
**Owner:** platform/backend/client leads

### Checklist

- [ ] Update backend placeholders/docs to reference canonical entity types
- [ ] Update PowerSync spec docs to reference canonical stream names
- [ ] Update future client scaffolds to import generated constants

### Acceptance Criteria

- no new shared identifier is introduced outside contracts package

---

## Phase 1 Review Gate

- [ ] Registry files approved
- [ ] Generated bindings checked in
- [ ] Validation tests green
- [ ] CI enforcement green

---

# 7. Phase 2 — Database & Schema Foundation

## Goal

Stand up Postgres schema and migration flow consistent with contracts and sync needs.

## Exit Criteria

- migrations run cleanly
- seed dataset loads cleanly
- schema supports ownership/scopes
- schema reviewed for sync friendliness

---

## P2-001 — Select and Wire Migration Tooling

**Depends on:** P0-002  
**Owner:** backend/platform

### Checklist

- [ ] Choose migration tool for Rust/Postgres stack
- [ ] Add migrations folder structure
- [ ] Add local migration commands
- [ ] Document migration workflow

### Acceptance Criteria

- dev can apply/reset migrations locally

---

## P2-002 — Stand Up Local Postgres in Dev Compose

**Depends on:** P0-001  
**Owner:** infra/platform

### Checklist

- [ ] Add Postgres service to local compose
- [ ] Add persistent volume
- [ ] Add env/config for local credentials
- [ ] Document startup command

### Acceptance Criteria

- local Postgres boots reliably from compose

---

## P2-003 — Implement Baseline Core Schema

**Depends on:** P2-001, P2-002, P1-001  
**Owner:** backend/data

### Checklist

- [ ] Add migrations for:
  - [ ] `users`
  - [ ] `households`
  - [ ] `household_memberships`
  - [ ] `initiatives`
  - [ ] `tags`
  - [ ] `attachments`
  - [ ] `entity_relations`
- [ ] Add timestamps / soft delete fields
- [ ] Add ownership/scope columns
- [ ] Add baseline constraints

### Acceptance Criteria

- core tables exist and apply cleanly from empty DB

---

## P2-004 — Implement Guidance Schema

**Depends on:** P2-003  
**Owner:** backend/data

### Checklist

- [ ] Add migrations for:
  - [ ] `guidance_epics`
  - [ ] `guidance_quests`
  - [ ] `guidance_routines`
  - [ ] `guidance_focus_sessions`
  - [ ] `guidance_daily_checkins`
- [ ] Add relevant indexes
- [ ] Add status/priority/energy check constraints

### Acceptance Criteria

- Guidance schema supports current-state + basic history tables

---

## P2-005 — Implement Knowledge Schema

**Depends on:** P2-003  
**Owner:** backend/data

### Checklist

- [ ] Add migrations for:
  - [ ] `knowledge_notes`
  - [ ] `knowledge_note_snapshots`
- [ ] Add note hierarchy FK
- [ ] Add slug uniqueness rules
- [ ] Add text fields needed for future search

### Acceptance Criteria

- note model supports personal/shared/initiative-scoped notes

---

## P2-006 — Implement Tracking Schema

**Depends on:** P2-003  
**Owner:** backend/data

### Checklist

- [ ] Add migrations for:
  - [ ] `tracking_locations`
  - [ ] `tracking_categories`
  - [ ] `tracking_items`
  - [ ] `tracking_item_events`
  - [ ] `tracking_shopping_lists`
  - [ ] `tracking_shopping_list_items`
- [ ] Add barcode/indexing basics
- [ ] Add quantity constraints where appropriate

### Acceptance Criteria

- item current-state + item event history both exist

---

## P2-007 — Implement Join Tables

**Depends on:** P2-004, P2-005, P2-006  
**Owner:** backend/data

### Checklist

- [ ] Add `initiative_tags`
- [ ] Add `quest_tags`
- [ ] Add `note_tags`
- [ ] Add `item_tags`
- [ ] Add `quest_attachments`
- [ ] Add `note_attachments`
- [ ] Add `item_attachments`

### Acceptance Criteria

- all join tables apply cleanly and support expected FK behavior

---

## P2-008 — Add Updated-At Triggers / DB Helpers

**Depends on:** P2-003  
**Owner:** backend/data

### Checklist

- [ ] Add `set_updated_at()` function
- [ ] Apply triggers to mutable tables
- [ ] Confirm inserts/updates behave as expected

### Acceptance Criteria

- updated timestamps are automatic and consistent

---

## P2-009 — Load Development Seed Dataset

**Depends on:** P2-003 through P2-007  
**Owner:** backend/data

### Checklist

- [ ] Add deterministic seed SQL or seeding tool
- [ ] Seed:
  - [ ] one primary user
  - [ ] one secondary household member
  - [ ] one household
  - [ ] one personal initiative
  - [ ] one household initiative
  - [ ] shared chores
  - [ ] inventory items
  - [ ] notes
  - [ ] relation records
  - [ ] shopping list
- [ ] Validate seed idempotence or documented reset flow

### Acceptance Criteria

- seed loads successfully into fresh local DB
- seed data exercises all major domain scopes

---

## P2-010 — Sync Scope Schema Review

**Depends on:** P2-009, P1-001  
**Owner:** backend + sync design

### Checklist

- [ ] Review each table for:
  - [ ] user ownership
  - [ ] household scope
  - [ ] initiative scope
  - [ ] whether auto-sync vs on-demand vs server-only
- [ ] Mark high-volume tables
- [ ] Identify tables needing selective replication

### Acceptance Criteria

- every syncable table has an explicit replication strategy note

---

## Phase 2 Review Gate

- [ ] Migrations apply from scratch
- [ ] Seed dataset loads
- [ ] Core scopes are present in schema
- [ ] Sync review completed

---

# 8. Phase 3 — Backend Core Foundation

## Goal

Stand up Axum server with auth, config, core APIs, and reusable authorization boundaries.

## Exit Criteria

- local server boots
- auth works
- core APIs work against real DB
- authorization boundaries are enforced
- integration tests cover access basics

---

## P3-001 — Create Server Skeleton

**Depends on:** P0-001, P2-002  
**Owner:** backend

### Checklist

- [ ] Create Rust app under `apps/server`
- [ ] Add config loader
- [ ] Add DB pool
- [ ] Add health endpoint
- [ ] Add structured logging
- [ ] Add error handling baseline

### Acceptance Criteria

- server starts locally and exposes `/health`

---

## P3-002 — Add Server Module Structure

**Depends on:** P3-001  
**Owner:** backend

### Checklist

- [ ] Add modules for:
  - [ ] auth
  - [ ] core
  - [ ] guidance
  - [ ] knowledge
  - [ ] tracking
  - [ ] relations
  - [ ] attachments
- [ ] Add shared API/router conventions

### Acceptance Criteria

- module layout mirrors architecture spec

---

## P3-003 — Implement Auth Foundation

**Depends on:** P2-003, P3-001  
**Owner:** backend/auth

### Checklist

- [ ] Add password hashing
- [ ] Add login endpoint
- [ ] Add session/token model
- [ ] Add current-user extractor
- [ ] Add auth middleware

### Acceptance Criteria

- test user can authenticate and call protected endpoints

---

## P3-004 — Implement Authorization Helpers

**Depends on:** P3-003, P2-009  
**Owner:** backend/auth

### Checklist

- [ ] Add helper for user-owned records
- [ ] Add helper for household membership checks
- [ ] Add helper for initiative visibility
- [ ] Add helper for attachment ownership/scope

### Acceptance Criteria

- reusable authorization checks exist and are not duplicated in every handler

---

## P3-005 — Implement Core APIs

**Depends on:** P3-004  
**Owner:** backend/core

### Checklist

- [ ] `GET /users/me`
- [ ] household list/detail endpoints
- [ ] household membership list endpoints
- [ ] initiatives CRUD (starter)
- [ ] tags CRUD (starter)

### Acceptance Criteria

- authenticated user can fetch personal/household core data from seed DB

---

## P3-006 — Implement Relation APIs

**Depends on:** P3-004, P2-003  
**Owner:** backend/relations

### Checklist

- [ ] create relation endpoint
- [ ] list relations by entity endpoint
- [ ] accept relation endpoint
- [ ] dismiss/reject relation endpoint
- [ ] validate entity type/relation type inputs against contracts

### Acceptance Criteria

- relation records can be created and queried safely

---

## P3-007 — Add Backend Integration Tests

**Depends on:** P3-005, P3-006  
**Owner:** backend/test

### Checklist

- [ ] auth success/failure tests
- [ ] users/me access test
- [ ] household membership boundary test
- [ ] initiative visibility test
- [ ] relation create/list authorization tests

### Acceptance Criteria

- integration tests run locally and in CI

---

## P3-008 — Add API Documentation Stubs

**Depends on:** P3-005  
**Owner:** backend/docs

### Checklist

- [ ] document auth endpoints
- [ ] document core entity payloads
- [ ] document relation payloads
- [ ] note which APIs are provisional

### Acceptance Criteria

- first client implementation does not require reverse-engineering handler code

---

## Phase 3 Review Gate

- [ ] Server boots locally
- [ ] Auth works
- [ ] Core APIs work on seeded DB
- [ ] Authorization helpers exist
- [ ] Integration tests are green

---

# 9. Phase 4 — PowerSync Foundation

## Goal

Validate baseline Postgres → SQLite sync for personal and household data.

## Exit Criteria

- PowerSync runs locally
- baseline streams work
- auth wiring works
- at least one client can sync baseline data
- one on-demand stream works

---

## P4-001 — Add Local PowerSync Environment

**Depends on:** P2-002, P3-001  
**Owner:** infra/sync

### Checklist

- [ ] Add PowerSync service to local compose or dev setup
- [ ] Add env/config wiring
- [ ] Document startup steps
- [ ] Confirm service health

### Acceptance Criteria

- local sync environment boots consistently

---

## P4-002 — Add Starter Sync Streams Config

**Depends on:** P1-001, P2-010, P4-001  
**Owner:** sync/backend

### Checklist

- [ ] Add streams for:
  - [ ] `my_profile`
  - [ ] `my_memberships`
  - [ ] `my_personal_data`
  - [ ] `my_household_data`
  - [ ] `my_relations`
  - [ ] `my_attachment_metadata`
- [ ] Ensure names match contracts registry exactly

### Acceptance Criteria

- config parses and stream names are canonical

---

## P4-003 — Wire Auth Identity into PowerSync

**Depends on:** P3-003, P4-002  
**Owner:** sync/backend

### Checklist

- [ ] propagate user identity into sync context
- [ ] confirm `auth.user_id()` equivalent works as intended
- [ ] validate unauthorized user cannot see other users’ data

### Acceptance Criteria

- stream queries return only authorized rows

---

## P4-004 — Validate Auto-Subscribed Streams

**Depends on:** P4-003, P2-009  
**Owner:** sync/test

### Checklist

- [ ] sync current user profile
- [ ] sync memberships
- [ ] sync personal initiatives/tags/routines/assigned quests
- [ ] sync household initiatives/quests/items/lists
- [ ] sync relations
- [ ] sync attachment metadata only

### Acceptance Criteria

- baseline working set lands in client SQLite correctly

---

## P4-005 — Add One On-Demand Stream

**Depends on:** P4-004  
**Owner:** sync/backend

### Checklist

- [ ] implement `initiative_detail`
- [ ] validate parameter-based subscription
- [ ] ensure server-side authorization still applies
- [ ] verify epics/quests/notes/items for initiative scope

### Acceptance Criteria

- a client can request one initiative’s detail set successfully

---

## P4-006 — Build Sync Verification Harness

**Depends on:** P4-004  
**Owner:** sync/test

### Checklist

- [ ] define test sequence:
  - [ ] first sync
  - [ ] offline local update
  - [ ] reconnect
  - [ ] upstream propagation
  - [ ] second client receives change
- [ ] capture test steps in repo docs
- [ ] run against seeded household data

### Acceptance Criteria

- repeatable sync smoke test exists

---

## P4-007 — Instrument Sync Diagnostics

**Depends on:** P4-004  
**Owner:** sync/backend

### Checklist

- [ ] log stream subscription names
- [ ] log sync failures
- [ ] capture missing-row debugging notes
- [ ] document common sync debugging commands

### Acceptance Criteria

- sync failures are diagnosable without blind guessing

---

## P4-008 — Connect One Thin Client for Proof

**Depends on:** P4-006  
**Owner:** Android or desktop/web prototype

### Checklist

- [ ] choose first proof client
- [ ] initialize local SQLite
- [ ] subscribe to baseline streams
- [ ] display synced data:
  - [ ] profile
  - [ ] one household
  - [ ] one shared quest
  - [ ] one tracking item
- [ ] verify local read path works

### Acceptance Criteria

- at least one real client can sync and display baseline personal + household data

---

## Phase 4 Review Gate

- [ ] PowerSync is running locally
- [ ] baseline streams sync authorized data
- [ ] one on-demand stream works
- [ ] one thin client proves local SQLite sync works
- [ ] sync diagnostics exist

---

# 10. Suggested Execution Order (Strict)

## Week/Block 1

- [ ] P0-001
- [ ] P0-002
- [ ] P0-003
- [ ] P0-005
- [ ] P0-004

## Week/Block 2

- [ ] P1-001
- [ ] P1-002
- [ ] P1-003
- [ ] P1-004
- [ ] P1-005
- [ ] P1-006
- [ ] P1-007

## Week/Block 3

- [ ] P2-001
- [ ] P2-002
- [ ] P2-003
- [ ] P2-004
- [ ] P2-005
- [ ] P2-006
- [ ] P2-007
- [ ] P2-008
- [ ] P2-009
- [ ] P2-010

## Week/Block 4

- [ ] P3-001
- [ ] P3-002
- [ ] P3-003
- [ ] P3-004
- [ ] P3-005
- [ ] P3-006
- [ ] P3-007
- [ ] P3-008

## Week/Block 5

- [ ] P4-001
- [ ] P4-002
- [ ] P4-003
- [ ] P4-004
- [ ] P4-005
- [ ] P4-006
- [ ] P4-007
- [ ] P4-008

This is not a calendar promise. It is an ordering recommendation so dependencies stop playing hide-and-seek.

---

# 11. Parallelizable Work

## Safe parallel work during Phase 1

- generator script
- registry docs
- validation tests

## Safe parallel work during Phase 2

- Guidance schema
- Knowledge schema
- Tracking schema  
Only after core tables are in place.

## Safe parallel work during Phase 3

- core APIs
- relation APIs
- integration tests/docs  
Only after auth/authorization foundation exists.

## Work to avoid parallelizing too early

- PowerSync auth wiring before backend auth exists
- client work before contracts are wired
- feature UI before one thin sync proof exists

---

# 12. Deliverables Checklist

## By end of Phase 0

- [ ] monorepo exists
- [ ] CI exists
- [ ] docs committed

## By end of Phase 1

- [ ] contracts package enforced
- [ ] generated bindings checked in
- [ ] contract CI green

## By end of Phase 2

- [ ] schema migration set exists
- [ ] seed dataset exists
- [ ] sync review notes exist

## By end of Phase 3

- [ ] auth + core server works
- [ ] authorization boundaries tested
- [ ] relation APIs work

## By end of Phase 4

- [ ] PowerSync baseline works
- [ ] one client sync proof exists
- [ ] initiative detail on-demand stream works

---

# 13. Immediate Next Actions

1. **Create the repo skeleton and commit the current docs.**
2. **Install the contracts package and CI enforcement immediately.**
3. **Stand up Postgres + migrations before touching feature APIs.**
4. **Implement auth and household boundaries before sync.**
5. **Prove one thin PowerSync-backed client before building broad UI.**

That is the shortest path to discovering whether the architecture is solid or just very eloquent.

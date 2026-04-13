# Feature 002: Shared Contracts

| Field | Value |
|---|---|
| **Feature** | 002-SharedContracts |
| **Status** | Draft |
| **Last Updated** | 2026-04-12 |
| **Plan Reference** | `docs/specs/10-PLAN-001-v1.md` — Step 2 |

---

## Overview

Shared Contracts establishes `packages/contracts/` as the single source of truth for cross-platform identifiers and schema primitives used by all Altair clients. It populates three canonical JSON registries (entity types, relation types, sync stream names) and produces typed language bindings for TypeScript, Kotlin, and Rust. A CI validation script asserts that all three bindings stay in sync with the registries.

---

## Problem Statement

Without a canonical registry, each client stack must independently maintain lists of entity type strings, relation type strings, and sync stream names. This creates drift: a string renamed in one stack silently breaks serialization or sync filtering in another. Contract invariants C-1, C-2, and C-3 require that these identifiers come from a single canonical source. The `packages/contracts/` registries fulfill that requirement; the language bindings enforce it at compile time per stack.

---

## User Stories

- As a backend engineer, I want a Rust enum for entity types so that I can validate incoming entity type strings against a compile-time-checked set and reject unknown types at write time.
- As an Android engineer, I want a Kotlin enum for entity types so that I can reference entity types without inline magic strings throughout the codebase.
- As a web engineer, I want a TypeScript const object for entity types so that IDE autocomplete catches typos and unknown types at development time.
- As any engineer, I want CI to verify that the language bindings match the JSON registry so that a stale or missing binding is caught before it reaches main.
- As a future maintainer, I want relation types defined consistently in all stacks so that cross-domain links created on any platform round-trip correctly.

---

## Requirements

### Must Have

- `packages/contracts/entity-types.json` — canonical list of all 18 entity type identifiers, matching `02-domain-model.md`
- `packages/contracts/relation-types.json` — canonical list of all 8 relation type identifiers
- `packages/contracts/sync-streams.json` — canonical list of PowerSync stream name constants (v1 streams; stubs acceptable if Step 4 design is incomplete)
- **TypeScript bindings** in `apps/web/src/lib/contracts/`:
  - `entityTypes.ts` — exports a const object with all entity type values
  - `relationTypes.ts` — exports a const object with all relation type values
  - `syncStreams.ts` — exports a const object with all stream name values
- **Kotlin bindings** in `apps/android/app/src/main/kotlin/com/getaltair/altair/contracts/`:
  - `EntityType.kt` — enum class with all entity type entries
  - `RelationType.kt` — enum class with all relation type entries
  - `SyncStream.kt` — enum class or const object with all stream name values
- **Rust bindings** in `apps/server/server/src/`:
  - `contracts.rs` — module containing `EntityType`, `RelationType`, and `SyncStream` enums with `serde` `rename` attributes matching the JSON registry strings
- **Shared DTOs** defined in all three languages:
  - `EntityRef` — `entity_type: String`, `entity_id: UUID`
  - `RelationRecord` — mirrors `entity_relations` table schema from `05-erd.md`
  - `AttachmentRecord` — mirrors `attachments` table schema from `05-erd.md`
  - `SyncSubscriptionRequest` — parameters for PowerSync stream subscription
- **CI validation script** (`packages/contracts/scripts/validate.ts` or equivalent) that:
  - Reads each JSON registry
  - Compares registry values against constants extracted from each language binding
  - Fails with a diff if any binding is missing or has an extra value
- CI pipeline runs the validation script on every push to main

### Should Have

- Each binding file contains a doc comment pointing back to the source registry file (e.g. `// Generated from packages/contracts/entity-types.json`)
- JSON registry entries include a `description` field alongside the identifier string
- A top-level `contracts_version` field in each registry file to support C-4 versioning rules
- `RelationRecord` and `AttachmentRecord` include all nullable fields from the ERD (not just required fields)

### Won't Have (this iteration)

- Automated code generation — hand-written bindings are acceptable; the registry is small and stable enough that manual authoring + CI validation provides sufficient safety
- iOS or WearOS bindings — deferred platforms per ADR-001
- C-5 AI pipeline validation — enforced at server write-time using the Rust `EntityType` enum; no separate validation layer needed in this step
- `packages/api-contracts/` — request/response DTOs beyond the four shared DTOs are deferred to Step 3 (Server Core)

---

## Testable Assertions

| ID    | Assertion                                                                                                                                        | Verification                                                                                      |
|-------|--------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| A-001 | `entity-types.json` contains exactly 18 entity type identifiers matching the canonical list in `02-domain-model.md` § Enumerations              | Count registry entries; diff against spec list                                                    |
| A-002 | `relation-types.json` contains exactly 8 relation type identifiers matching `02-domain-model.md` § Relation Types                               | Count registry entries; diff against spec list                                                    |
| A-003 | TypeScript `EntityType` const contains every identifier from `entity-types.json` and no additional values                                       | CI validation script: diff JSON registry keys vs. exported TS values                             |
| A-004 | Kotlin `EntityType` enum contains every identifier from `entity-types.json` and no additional values                                            | CI validation script: diff JSON registry keys vs. Kotlin enum entries                            |
| A-005 | Rust `EntityType` enum contains every identifier from `entity-types.json` and no additional values (accounting for serde rename attributes)      | CI validation script: diff JSON registry keys vs. serde-renamed Rust enum variants               |
| A-006 | CI validation covers relation types: same consistency check as A-003/A-004/A-005 applied to `relation-types.json`                               | CI script output includes relation-type coverage                                                  |
| A-007 | CI validation covers sync streams: same consistency check applied to `sync-streams.json`                                                        | CI script output includes sync-stream coverage                                                    |
| A-008 | Adding an entry to `entity-types.json` without updating any language binding causes CI validation to fail with a non-zero exit code              | Test: add a dummy entry to registry, run validation script, verify failure; revert                |
| A-009 | `EntityRef` is defined in TypeScript, Kotlin, and Rust with fields `entity_type` (string) and `entity_id` (UUID/string)                         | Compile each stack; inspect type definitions                                                      |
| A-010 | `RelationRecord` is defined in all three languages with fields matching the `entity_relations` table schema in `05-erd.md`                       | Compile each stack; field-level comparison against ERD                                            |
| A-011 | `AttachmentRecord` is defined in all three languages with fields matching the `attachments` table schema in `05-erd.md`                          | Compile each stack; field-level comparison against ERD                                            |
| A-012 | `SyncSubscriptionRequest` is defined in all three languages and is imported in at least one sync-related module per stack                        | Code import audit; compile each stack                                                             |
| A-013 | Rust server rejects a write request referencing an unknown entity type with a 422 Unprocessable Entity response                                  | Integration test: POST entity relation with `entity_type = "unknown_fake_type"`; assert 422      |
| A-014 | Entity type constants are imported and used in at least one Rust service file, one Kotlin Repository or ViewModel, and one SvelteKit route/store | Import audit via grep or LSP; confirm no stack has zero usages                                    |

---

## Open Questions

- [x] **OQ-001 — Binding location**: Write bindings into each app's source tree (`apps/web/src/lib/contracts/`, `apps/android/.../contracts/`, `apps/server/server/src/contracts.rs`). Cross-workspace imports add non-trivial build plumbing (Cargo dependency, Gradle module, bun workspace ref) for ~30 constants. The CI validation script enforces consistency regardless of location. Follows the existing server convention.
- [x] **OQ-002 — Sync stream content**: Define provisional stream name constants now (e.g., `user_data`, `household_data`, `guidance`, `knowledge`, `tracking`). Stream name constants can be stabilized without the Step 4 SQL bucket queries. If Step 4 renames a stream, updating the JSON registry + three binding files is a small, CI-enforced change. Deferring would require Step 4 to backfill all three stacks mid-implementation.
- [x] **OQ-003 — api-contracts scope**: All four DTOs (`EntityRef`, `RelationRecord`, `AttachmentRecord`, `SyncSubscriptionRequest`) live in `packages/contracts/` (written into app source trees per OQ-001) for v1. `packages/api-contracts/` is created in Step 3 (Server Core) when real endpoint request/response types appear.

---

## Dependencies

| Dependency | Reason |
|---|---|
| `docs/specs/02-domain-model.md` | Canonical source for entity type and relation type identifier lists |
| `docs/specs/05-erd.md` | Field-level schema for `EntityRef`, `RelationRecord`, `AttachmentRecord` DTO definitions |
| Step 4 — Sync Engine | Informs `sync-streams.json` stream name content; OQ-002 |
| `apps/server/server/src/` | Target location for Rust `contracts.rs`; no stub exists yet |

---

## Revision History

| Date       | Change       | ADR |
|------------|--------------|-----|
| 2026-04-12 | Initial spec | —   |
| 2026-04-12 | Resolved OQ-001, OQ-002, OQ-003 | —   |

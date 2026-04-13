# Implementation Steps: Shared Contracts

**Spec:** `Context/Features/002-SharedContracts/Spec.md`
**Tech:** `Context/Features/002-SharedContracts/Tech.md`

---

## Progress
- **Status:** Complete
- **Completed:** 2026-04-13
- **Last milestone:** M4 — Final validation passed (2026-04-13)

---

## Team Orchestration

### Team Members

- **builder-infra**
  - Role: JSON registries, CI validation script, CI workflow wiring
  - Agent Type: general-purpose
  - Resume: false

- **builder-rust**
  - Role: Rust contracts module — enums, DTOs, serde wiring
  - Agent Type: general-purpose
  - Resume: false

- **builder-web**
  - Role: TypeScript contracts — const objects, DTOs, barrel exports
  - Agent Type: general-purpose
  - Resume: false

- **builder-android**
  - Role: Kotlin contracts — enum classes, DTOs
  - Agent Type: general-purpose
  - Resume: false

- **validator**
  - Role: Quality validation — read-only inspection of all outputs
  - Agent Type: general-purpose
  - Resume: false

---

## Tasks

### Phase 1: JSON Registries

Create the three canonical registry files in `packages/contracts/`. These are the source of truth for all language bindings.

- [ ] S001: Create `packages/contracts/entity-types.json` with all 18 entity type identifiers and descriptions
  - **Assigned:** builder-infra
  - **Depends:** none
  - **Parallel:** true
  - **Content:** `contracts_version: "1.0.0"` field; `values` array with entries `{ "id": "<type>", "description": "<text>" }`. Canonical list from `docs/specs/02-domain-model.md` § Enumerations: Core (user, household, initiative, tag, attachment), Guidance (guidance_epic, guidance_quest, guidance_routine, guidance_focus_session, guidance_daily_checkin), Knowledge (knowledge_note, knowledge_note_snapshot), Tracking (tracking_location, tracking_category, tracking_item, tracking_item_event, tracking_shopping_list, tracking_shopping_list_item).

- [ ] S002: Create `packages/contracts/relation-types.json` with all 8 relation type identifiers and descriptions
  - **Assigned:** builder-infra
  - **Depends:** none
  - **Parallel:** true
  - **Content:** Same structure as S001. Values: `references`, `supports`, `requires`, `related_to`, `depends_on`, `duplicates`, `similar_to`, `generated_from`.

- [ ] S003: Create `packages/contracts/sync-streams.json` with provisional v1 stream name constants
  - **Assigned:** builder-infra
  - **Depends:** none
  - **Parallel:** true
  - **Content:** Values: `user_data`, `household`, `guidance`, `knowledge`, `tracking`. Include a top-level `"provisional": true` field and a comment-equivalent `note` field: `"Stream SQL bucket definitions are designed in Step 4 (Sync Engine); these names may be revised."`.

---

🏁 **MILESTONE 1: Registries complete**
Verify: A-001 (entity-types.json has exactly 18 entries), A-002 (relation-types.json has exactly 8 entries)
**Contracts:**
- `packages/contracts/entity-types.json` — canonical entity type identifiers; consumed by all three binding tasks
- `packages/contracts/relation-types.json` — canonical relation type identifiers
- `packages/contracts/sync-streams.json` — provisional sync stream name constants

---

### Phase 2: Language Bindings

All three binding tasks run in parallel after Milestone 1. Each stack targets a different directory — no file conflicts.

- [ ] S004: Create Rust contracts module `apps/server/server/src/contracts.rs` with `EntityType` (18 variants), `RelationType` (8 variants), `SyncStream` (5 variants) enums and `EntityRef`, `RelationRecord`, `AttachmentRecord`, `SyncSubscriptionRequest` structs
  - **Assigned:** builder-rust
  - **Depends:** S001, S002, S003
  - **Parallel:** true
  - **Details:** Each enum variant uses `#[serde(rename = "<registry-id>")]` (per-variant, not rename_all). Derive: `Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize` on enums; `Debug, Clone, Serialize, Deserialize` on structs. UUID fields use `uuid::Uuid`. Timestamp fields use `chrono::DateTime<chrono::Utc>`. Optional fields use `Option<T>`. `RelationRecord` and `AttachmentRecord` mirror the full `entity_relations` and `attachments` table schemas from `docs/specs/05-erd.md` — include all columns including nullable ones. `SyncSubscriptionRequest`: `streams: Vec<SyncStream>`, `user_id: uuid::Uuid`. Add `mod contracts;` and `pub use contracts::*;` to `main.rs`. File must include a top comment: `// Source of truth: packages/contracts/{entity,relation}-types.json`.

- [ ] S004-T: Rust serde round-trip and struct tests for `contracts.rs`
  - **Assigned:** builder-rust
  - **Depends:** S004
  - **Parallel:** false
  - **Scenarios:** (1) `EntityType::GuidanceEpic` serializes to `"guidance_epic"` and deserializes back; (2) `EntityType::User` serializes to `"user"`; (3) unknown string `"unknown_fake_type"` fails serde deserialization with an error; (4) `RelationType::RelatedTo` serializes to `"related_to"`; (5) `EntityRef` round-trips through serde with field names `entity_type` and `entity_id`

- [ ] S005: Create TypeScript contracts in `apps/web/src/lib/contracts/` — `entityTypes.ts`, `relationTypes.ts`, `syncStreams.ts`, `dtos.ts`, `index.ts`
  - **Assigned:** builder-web
  - **Depends:** S001, S002, S003
  - **Parallel:** true
  - **Details:** Each binding file uses `as const` object pattern: `export const EntityType = { User: 'user', Household: 'household', ... } as const; export type EntityTypeValue = typeof EntityType[keyof typeof EntityType];`. DTOs in `dtos.ts`: `EntityRef`, `RelationRecord`, `AttachmentRecord`, `SyncSubscriptionRequest` as TypeScript `interface` types. UUID fields typed as `string`, timestamps as `string` (ISO 8601), optionals as `T | null`. `index.ts` re-exports everything from all four files. Each file begins with: `// Source of truth: packages/contracts/<registry-file>.json`.

- [ ] S005-T: TypeScript binding correctness tests (`apps/web/src/lib/contracts/contracts.spec.ts`)
  - **Assigned:** builder-web
  - **Depends:** S005
  - **Parallel:** false
  - **Scenarios:** (1) `Object.values(EntityType)` has length 18; (2) `EntityType.GuidanceEpic === 'guidance_epic'`; (3) `Object.values(RelationType)` has length 8; (4) `RelationType.RelatedTo === 'related_to'`; (5) `bun run check` (TypeScript type check) passes with no errors — run as part of the test task by invoking `bun run check` in `apps/web/`

- [ ] S006: Create Kotlin contracts in `apps/android/app/src/main/java/com/getaltair/altair/contracts/` — `EntityType.kt`, `RelationType.kt`, `SyncStream.kt`, `Dtos.kt`
  - **Assigned:** builder-android
  - **Depends:** S001, S002, S003
  - **Parallel:** true
  - **Details:** `enum class EntityType(val value: String)` with all 18 entries in SCREAMING_SNAKE_CASE (e.g., `GUIDANCE_EPIC("guidance_epic")`), plus `companion object { fun fromValue(value: String): EntityType? = entries.find { it.value == value } }`. Same pattern for `RelationType` (8 entries) and `SyncStream` (5 entries). `Dtos.kt` defines `EntityRef`, `RelationRecord`, `AttachmentRecord`, `SyncSubscriptionRequest` as plain `data class` with camelCase field names; no serialization annotations yet (deferred to Step 8 when JSON library is chosen). Each file includes a top comment: `// Source of truth: packages/contracts/<registry-file>.json`. Package: `com.getaltair.altair.contracts`.

- [ ] S006-T: Kotlin unit tests for `EntityType.fromValue()` and enum completeness
  - **Assigned:** builder-android
  - **Depends:** S006
  - **Parallel:** false
  - **Scenarios:** (1) `EntityType.fromValue("guidance_epic")` returns `EntityType.GUIDANCE_EPIC`; (2) `EntityType.fromValue("unknown_fake_type")` returns null; (3) `EntityType.entries` has size 18; (4) all entries have non-blank `value` strings; (5) `RelationType.fromValue("related_to")` returns `RelationType.RELATED_TO`

---

🏁 **MILESTONE 2: Language bindings complete**
Verify: A-003 (TS EntityType values match registry), A-004 (Kotlin EntityType entries match registry), A-005 (Rust EntityType variants match registry), A-006 (relation types consistent across all three bindings), A-007 (sync streams consistent across all three bindings), A-008 (EntityRef defined in all three languages), A-009 (RelationRecord defined in all three languages), A-010 (AttachmentRecord defined in all three languages), A-011 (SyncSubscriptionRequest defined in all three languages)
**Contracts:**
- `apps/server/server/src/contracts.rs` — Rust enums + DTOs; Step 3 adds `use crate::contracts::EntityType` in domain handlers
- `apps/web/src/lib/contracts/index.ts` — TypeScript barrel; Step 9 imports via `$lib/contracts`
- `apps/android/app/src/main/java/com/getaltair/altair/contracts/EntityType.kt` — Kotlin enum; Step 8 imports in repositories

---

### Phase 3: CI Validation

Sequential: script must exist before the CI job can reference it.

- [ ] S007: Create `packages/contracts/scripts/validate.ts` — Bun script that reads all three JSON registries and validates each language binding file
  - **Assigned:** builder-infra
  - **Depends:** S004, S005, S006
  - **Parallel:** false
  - **Details:** Script reads `entity-types.json`, `relation-types.json`, `sync-streams.json`. For TypeScript: dynamically imports `apps/web/src/lib/contracts/entityTypes.ts` etc. using `bun` and compares `Object.values(EntityType)` against registry `values[].id` arrays. For Kotlin: reads `EntityType.kt`, `RelationType.kt`, `SyncStream.kt` as text and extracts quoted string values from enum entries using regex `/"([^"]+)"\)/g`; diffs against registry. For Rust: reads `contracts.rs` as text, extracts `serde(rename = "...")` values using regex; diffs against registry. Exits 0 if all bindings match; exits 1 with a human-readable diff report if any binding is missing a value or has an extra value not in the registry.

- [ ] S007-T: Test the validation script against known-good and known-bad states
  - **Assigned:** builder-infra
  - **Depends:** S007
  - **Parallel:** false
  - **Scenarios:** (1) Run script with current correct bindings — assert exit code 0; (2) Temporarily add a dummy entry `{ "id": "test_fake_type", "description": "Test" }` to `entity-types.json`, run script — assert exit code 1 and diff output names `test_fake_type`; (3) Revert the dummy entry — assert exit code 0 again; (4) Remove one entry from a binding file, run script — assert exit code 1

- [ ] S008: Add `validate-contracts` job to `.github/workflows/ci.yml` and update `smoke-test` dependencies
  - **Assigned:** builder-infra
  - **Depends:** S007
  - **Parallel:** false
  - **Details:** New job `validate-contracts` runs on `ubuntu-latest`, installs Bun via `oven-sh/setup-bun@v2`, installs deps with `bun install` in `apps/web/` (needed for TypeScript module resolution), then runs `bun run packages/contracts/scripts/validate.ts` from repo root. Update `smoke-test` job's `needs:` from `[rust, web]` to `[rust, web, android, validate-contracts]`.

- [ ] S008-D: Update `packages/contracts/README.md` with validation script usage and binding locations
  - **Assigned:** builder-infra
  - **Depends:** S008
  - **Parallel:** false
  - **Content:** Replace placeholder README with: registry file descriptions (what each contains), binding file locations for each stack, how to run the validation script locally (`bun run packages/contracts/scripts/validate.ts`), how to add a new type (update JSON registry → update all three binding files → run validate script → open PR), versioning rules per C-4 (additive = minor bump, rename/remove = breaking change).

---

🏁 **MILESTONE 3: CI validation complete**
Verify: A-012 (adding registry entry without updating binding causes CI failure), A-007 (sync-streams validated by script)
**Note:** A-013 (server rejects unknown entity type with 422) requires Step 3 HTTP handlers — partially satisfied here by the Rust `EntityType` enum rejecting unknown values at serde deserialization; the HTTP status code mapping is Step 3 work.
**Contracts:**
- `packages/contracts/scripts/validate.ts` — validation script; Step 3+ engineers run this locally when adding domain entity types
- `.github/workflows/ci.yml` — updated with validate-contracts job

---

### Phase 4: Usage Verification

Prove all three binding modules compile and are importable in a real file (satisfies A-014 groundwork). All three tasks are independent.

- [ ] S009: Import and use `EntityType` in Rust `apps/server/server/src/routes/health.rs`
  - **Assigned:** builder-rust
  - **Depends:** S004
  - **Parallel:** true
  - **Details:** Add `use crate::contracts::EntityType;` to `health.rs`. Add a private constant or a comment-free usage that the Rust compiler cannot optimize away as dead code — e.g., a private `fn _assert_contracts() { let _: EntityType = EntityType::Initiative; }` marked `#[allow(dead_code)]`, or preferably integrate it into the health response by including `"entity_types": EntityType::User` in the JSON response as a version hint. Ensure `cargo build` passes with no warnings. Intent: prove the import path `crate::contracts` resolves correctly from a route module.

- [ ] S010: Re-export contracts from `apps/web/src/lib/index.ts` and use `EntityType` in a new placeholder file
  - **Assigned:** builder-web
  - **Depends:** S005
  - **Parallel:** true
  - **Details:** Add `export * from './contracts';` to `apps/web/src/lib/index.ts`. Create `apps/web/src/lib/contracts/constants.ts` that imports `EntityType` and exports a typed array `export const SYNCED_ENTITY_TYPES: EntityTypeValue[] = [EntityType.Initiative, EntityType.GuidanceQuest, EntityType.KnowledgeNote, EntityType.TrackingItem];` — this is a real utility (the subset of entity types that sync through PowerSync). Ensure `bun run check` passes.

- [ ] S011: Import and use `EntityType` in Android `apps/android/app/src/main/java/com/getaltair/altair/MainActivity.kt`
  - **Assigned:** builder-android
  - **Depends:** S006
  - **Parallel:** true
  - **Details:** Add `import com.getaltair.altair.contracts.EntityType` to `MainActivity.kt`. Add a `private val coreEntityTypes = listOf(EntityType.USER, EntityType.HOUSEHOLD, EntityType.INITIATIVE)` property — dead-code-free and Kotlin-idiomatic. Ensure `./gradlew assembleDebug` passes without warnings about the import.

---

🏁 **MILESTONE FINAL: Feature complete**
Verify all assertions:
- A-001 ✓ entity-types.json has 18 entries
- A-002 ✓ relation-types.json has 8 entries
- A-003 ✓ TypeScript EntityType values match registry
- A-004 ✓ Kotlin EntityType entries match registry
- A-005 ✓ Rust EntityType variants match registry (serde renames)
- A-006 ✓ Relation types consistent across all three bindings
- A-007 ✓ Sync streams consistent across all three bindings
- A-008 ✓ EntityRef defined in all three languages
- A-009 ✓ RelationRecord defined in all three languages
- A-010 ✓ AttachmentRecord defined in all three languages
- A-011 ✓ SyncSubscriptionRequest defined in all three languages
- A-012 ✓ CI fails when binding drifts from registry
- A-013 ⚠️ Partial — Rust enum rejects unknown serde values; HTTP 422 response requires Step 3 handler wiring
- A-014 ✓ EntityType imported and used in at least one file per stack

### Phase 5: Validation

- [ ] S012: Final read-only inspection of all created files
  - **Assigned:** validator
  - **Depends:** S009, S010, S011, S008-D
  - **Parallel:** false
  - **Checks:** (1) All 18 entity types present in each of the three binding files; (2) All 8 relation types present in each binding; (3) No TODO/FIXME stubs remaining in any created file; (4) Rust `cargo build` passes; (5) TypeScript `bun run check` passes; (6) Android `./gradlew assembleDebug` passes; (7) `bun run packages/contracts/scripts/validate.ts` exits 0; (8) CI workflow includes `validate-contracts` job with correct `needs` dependencies; (9) README accurately describes the binding locations and add-a-type workflow.

---

### Phase 6: Post-Review Additions (2026-04-12)

Tasks identified during code review that were missed in the original implementation.

- [ ] S013: Implement S007-T drift-detection test scenarios
  - **Assigned:** builder-infra
  - **Depends:** S007
  - **Parallel:** false
  - **Detail:** S007-T test scenarios were planned but not shipped. Implement all four:
    (1) Run script with correct bindings — assert exit code 0;
    (2) Temporarily add `{ "id": "test_fake_type", "description": "Test" }` to entity-types.json, run script — assert exit code 1 and diff output names `test_fake_type`;
    (3) Revert dummy entry — assert exit code 0 again;
    (4) Remove one entry from a binding file, run script — assert exit code 1.
  - **Relates to:** P2-009, A-008, S007-T

- [ ] S014: Add throwing fromValue() companion to all three Kotlin enums
  - **Assigned:** builder-android
  - **Depends:** S006
  - **Parallel:** false
  - **Detail:** Rename existing `fromValue()` to `fromValueOrNull()` on EntityType, RelationType, and SyncStream. Add a throwing variant: `fun fromValue(value: String): EntityType = fromValueOrNull(value) ?: throw IllegalArgumentException("Unknown EntityType: '$value'")`. This prevents `!!` usage at Step 3/8 callsites per kotlin-android.md convention.
  - **Relates to:** P2-010

- [ ] S015: Add entry-count tests for RelationType and SyncStream in Kotlin
  - **Assigned:** builder-android
  - **Depends:** S006
  - **Parallel:** false
  - **Detail:** Add `assertEquals(8, RelationType.entries.size)` and `assertEquals(5, SyncStream.entries.size)` assertions. Add to EntityTypeTest.kt or create RelationTypeTest.kt. Satisfies A-002 fully in Kotlin.
  - **Relates to:** P2-011, A-002

---

## Acceptance Criteria

- [ ] All testable assertions from Spec.md verified (A-001 through A-012, A-014; A-013 partial per note above)
- [ ] All tests passing (`cargo test`, `bun test`, Android unit tests)
- [ ] CI `validate-contracts` job green on push
- [ ] No TODO/FIXME stubs in created files
- [ ] `packages/contracts/README.md` updated with binding locations and add-a-type workflow

---

## Validation Commands

```bash
# Verify registries (A-001, A-002)
jq '.values | length' packages/contracts/entity-types.json   # expect 18
jq '.values | length' packages/contracts/relation-types.json  # expect 8

# Run CI validation script locally (A-003–A-007, A-012)
bun run packages/contracts/scripts/validate.ts

# Rust build + tests (A-005, A-008–A-011)
cd apps/server && cargo build && cargo test

# TypeScript type check + tests (A-003, A-008–A-011)
cd apps/web && bun run check && bun test

# Android build (A-004, A-006, A-008–A-011)
cd apps/android && ./gradlew assembleDebug

# Usage import check (A-014)
grep -r "EntityType" apps/server/server/src/routes/health.rs
grep -r "EntityType" apps/web/src/lib/contracts/constants.ts
grep -r "EntityType" apps/android/app/src/main/java/com/getaltair/altair/MainActivity.kt
```

---

## Notes

- **A-013 partial:** Full 422 rejection test requires Step 3 (Server Core) to add domain write handlers. The foundation (Rust `EntityType` enum + serde deserialization) is laid here; Step 3 adds the HTTP status code mapping.
- **Kotlin serialization annotations:** `RelationRecord` and `AttachmentRecord` DTOs are plain `data class` without JSON annotations. Gson/Moshi/kotlinx.serialization choice is deferred to Step 8 (Android Client); annotations are added then.
- **Sync stream names provisional:** `sync-streams.json` includes `"provisional": true`. Step 4 (Sync Engine) will finalize stream bucket designs; if names change, update the registry and run the validation script to catch binding drift.

# Tasks: CORE-004 Type Generation

**Branch**: `spec/core-004-type-generation` | **Date**: 2025-12-07
**Spec**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)

---

## Phase 1: SurrealDB Thing Type Handler

### Task 1.1: Create serde helpers module

- [ ] Create `backend/crates/altair-db/src/schema/serde_helpers.rs`
  - **Acceptance**: File exists with module declaration in `schema/mod.rs`
  - **Files**: `backend/crates/altair-db/src/schema/serde_helpers.rs`, `backend/crates/altair-db/src/schema/mod.rs`

### Task 1.2: Implement Thing serialization

- [ ] Add custom serde serializer for `Thing` → `{ tb, id }` object format
  - **Acceptance**: Serializer converts `Thing { tb: "quest", id: String("123") }` to JSON `{"tb":"quest","id":"123"}`
  - **Files**: `backend/crates/altair-db/src/schema/serde_helpers.rs`
  - **Implementation**: Use `#[serde(with = "thing_serde")]` pattern

### Task 1.3: Implement specta type definition for Thing

- [ ] Create specta type definition for Thing as TypeScript `{ tb: string; id: string }`
  - **Acceptance**: specta outputs TypeScript interface matching `{ tb: string; id: string }`
  - **Files**: `backend/crates/altair-db/src/schema/serde_helpers.rs`

### Task 1.4: Implement NaiveTime serialization

- [ ] Add custom serde serializer for `NaiveTime` → `"HH:MM"` string format
  - **Acceptance**: Serializer converts `NaiveTime::from_hms(14, 30, 0)` to `"14:30"`
  - **Files**: `backend/crates/altair-db/src/schema/serde_helpers.rs`

### Task 1.5: Implement specta type definition for NaiveTime

- [ ] Create specta type definition for NaiveTime as TypeScript `string` (HH:MM format)
  - **Acceptance**: specta outputs TypeScript `string` type with JSDoc comment explaining HH:MM format
  - **Files**: `backend/crates/altair-db/src/schema/serde_helpers.rs`

### Task 1.6: Add unit tests for serde helpers

- [ ] Write unit tests verifying Thing and NaiveTime serialization
  - **Acceptance**: Tests pass for serialization and deserialization of both types
  - **Files**: `backend/crates/altair-db/src/schema/serde_helpers.rs` (inline tests)
  - **Test cases**:
    - Thing serialization to JSON
    - NaiveTime serialization to HH:MM
    - Round-trip deserialization

---

## Phase 2: Add specta Feature to altair-db

### Task 2.1: Update altair-db Cargo.toml

- [ ] Add specta dependency and feature flag
  - **Acceptance**: `cargo check -p altair-db --features specta` succeeds
  - **Files**: `backend/crates/altair-db/Cargo.toml`
  - **Changes**:
    - Add `specta = { workspace = true, optional = true }` to dependencies
    - Add `[features]` section with `specta = ["dep:specta"]`

### Task 2.2: Enable specta feature in Tauri apps

- [ ] Update workspace Cargo.toml to enable specta feature for altair-db in all apps
  - **Acceptance**: All Tauri apps build with altair-db specta feature enabled
  - **Files**:
    - `backend/apps/guidance/src-tauri/Cargo.toml`
    - `backend/apps/knowledge/src-tauri/Cargo.toml`
    - `backend/apps/tracking/src-tauri/Cargo.toml`
    - `backend/apps/mobile/src-tauri/Cargo.toml`
  - **Changes**: Add `features = ["specta"]` to altair-db dependency

---

## Phase 3: Add specta Derives to Enums

### Task 3.1: Add specta derive to QuestColumn enum

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `QuestColumn`
  - **Acceptance**: Enum compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:QuestColumn`

### Task 3.2: Add specta derive to EnergyCost enum

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `EnergyCost`
  - **Acceptance**: Enum compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:EnergyCost`

### Task 3.3: Add specta derive to EnergyLevel newtype

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `EnergyLevel`
  - **Acceptance**: Newtype compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:EnergyLevel`

### Task 3.4: Add specta derive to EntityStatus enum

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `EntityStatus`
  - **Acceptance**: Enum compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:EntityStatus`

### Task 3.5: Add specta derive to ItemStatus enum

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `ItemStatus`
  - **Acceptance**: Enum compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:ItemStatus`

### Task 3.6: Add specta derive to ReservationStatus enum

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `ReservationStatus`
  - **Acceptance**: Enum compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:ReservationStatus`

### Task 3.7: Add specta derive to CaptureStatus enum

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `CaptureStatus`
  - **Acceptance**: Enum compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:CaptureStatus`

### Task 3.8: Add specta derive to CaptureType enum

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `CaptureType`
  - **Acceptance**: Enum compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:CaptureType`

### Task 3.9: Add specta derive to CaptureSource enum

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `CaptureSource`
  - **Acceptance**: Enum compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:CaptureSource`

### Task 3.10: Add specta derive to StreakType enum

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `StreakType`
  - **Acceptance**: Enum compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:StreakType`

### Task 3.11: Add specta derive to MediaType enum

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `MediaType`
  - **Acceptance**: Enum compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:MediaType`

### Task 3.12: Add specta derive to UserRole enum

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `UserRole`
  - **Acceptance**: Enum compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:UserRole`

### Task 3.13: Add specta derive to FocusSessionStatus enum

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to `FocusSessionStatus`
  - **Acceptance**: Enum compiles with specta feature enabled
  - **Files**: `backend/crates/altair-db/src/schema/enums.rs:FocusSessionStatus`

---

## Phase 4: Add specta Derives to Domain Structs

### Task 4.1: quest.rs - Campaign struct

- [ ] Add specta derive to `Campaign` struct with Thing field serde handling
  - **Acceptance**: Struct compiles with specta, `id` and `owner` fields use `thing_serde`
  - **Files**: `backend/crates/altair-db/src/schema/quest.rs:Campaign`
  - **Special handling**: Add `#[serde(with = "thing_serde")]` to `id` and `owner` fields

### Task 4.2: quest.rs - Quest struct

- [ ] Add specta derive to `Quest` struct with Thing field serde handling
  - **Acceptance**: Struct compiles with specta, Thing fields use `thing_serde`
  - **Files**: `backend/crates/altair-db/src/schema/quest.rs:Quest`
  - **Special handling**: Add `#[serde(with = "thing_serde")]` to `id`, `owner`, `campaign_id` fields

### Task 4.3: quest.rs - FocusSession struct

- [ ] Add specta derive to `FocusSession` struct with Thing field serde handling
  - **Acceptance**: Struct compiles with specta
  - **Files**: `backend/crates/altair-db/src/schema/quest.rs:FocusSession`

### Task 4.4: quest.rs - EnergyCheckIn struct

- [ ] Add specta derive to `EnergyCheckIn` struct with NaiveDate handling
  - **Acceptance**: Struct compiles with specta, `date` field serializes correctly
  - **Files**: `backend/crates/altair-db/src/schema/quest.rs:EnergyCheckIn`

### Task 4.5: note.rs - Note struct

- [ ] Add specta derive to `Note` struct with Vec<f32> embedding field
  - **Acceptance**: Struct compiles with specta, embedding generates as `number[]`
  - **Files**: `backend/crates/altair-db/src/schema/note.rs:Note`
  - **Verification**: Check generated TypeScript has `embedding: number[]`

### Task 4.6: note.rs - Folder struct

- [ ] Add specta derive to `Folder` struct
  - **Acceptance**: Struct compiles with specta
  - **Files**: `backend/crates/altair-db/src/schema/note.rs:Folder`

### Task 4.7: note.rs - DailyNote struct

- [ ] Add specta derive to `DailyNote` struct with NaiveDate handling
  - **Acceptance**: Struct compiles with specta, `date` field serializes correctly
  - **Files**: `backend/crates/altair-db/src/schema/note.rs:DailyNote`

### Task 4.8: item.rs - Item struct

- [ ] Add specta derive to `Item` struct with JsonValue custom_fields
  - **Acceptance**: Struct compiles with specta, `custom_fields` generates as TypeScript `any` or `unknown`
  - **Files**: `backend/crates/altair-db/src/schema/item.rs:Item`
  - **Documentation**: Add JSDoc comment explaining custom_fields is dynamic JSON

### Task 4.9: item.rs - Location struct

- [ ] Add specta derive to `Location` struct
  - **Acceptance**: Struct compiles with specta
  - **Files**: `backend/crates/altair-db/src/schema/item.rs:Location`

### Task 4.10: item.rs - GeoPoint struct

- [ ] Add specta derive to `GeoPoint` struct
  - **Acceptance**: Struct compiles with specta, generates as `{ latitude: number; longitude: number }`
  - **Files**: `backend/crates/altair-db/src/schema/item.rs:GeoPoint`

### Task 4.11: item.rs - Reservation struct

- [ ] Add specta derive to `Reservation` struct
  - **Acceptance**: Struct compiles with specta
  - **Files**: `backend/crates/altair-db/src/schema/item.rs:Reservation`

### Task 4.12: item.rs - MaintenanceSchedule struct

- [ ] Add specta derive to `MaintenanceSchedule` struct
  - **Acceptance**: Struct compiles with specta
  - **Files**: `backend/crates/altair-db/src/schema/item.rs:MaintenanceSchedule`

### Task 4.13: capture.rs - Capture struct

- [ ] Add specta derive to `Capture` struct
  - **Acceptance**: Struct compiles with specta
  - **Files**: `backend/crates/altair-db/src/schema/capture.rs:Capture`

### Task 4.14: gamification.rs - UserProgress struct

- [ ] Add specta derive to `UserProgress` struct
  - **Acceptance**: Struct compiles with specta
  - **Files**: `backend/crates/altair-db/src/schema/gamification.rs:UserProgress`

### Task 4.15: gamification.rs - Achievement struct

- [ ] Add specta derive to `Achievement` struct
  - **Acceptance**: Struct compiles with specta
  - **Files**: `backend/crates/altair-db/src/schema/gamification.rs:Achievement`

### Task 4.16: gamification.rs - Streak struct

- [ ] Add specta derive to `Streak` struct
  - **Acceptance**: Struct compiles with specta
  - **Files**: `backend/crates/altair-db/src/schema/gamification.rs:Streak`

### Task 4.17: shared.rs - User struct

- [ ] Add specta derive to `User` struct
  - **Acceptance**: Struct compiles with specta
  - **Files**: `backend/crates/altair-db/src/schema/shared.rs:User`

### Task 4.18: shared.rs - UserPreferences struct

- [ ] Add specta derive to `UserPreferences` struct with NaiveTime handling
  - **Acceptance**: Struct compiles with specta, `weekly_harvest_time` uses custom serializer
  - **Files**: `backend/crates/altair-db/src/schema/shared.rs:UserPreferences`
  - **Special handling**: Add `#[serde(with = "naive_time_serde")]` to `weekly_harvest_time` field

### Task 4.19: shared.rs - Attachment struct

- [ ] Add specta derive to `Attachment` struct
  - **Acceptance**: Struct compiles with specta
  - **Files**: `backend/crates/altair-db/src/schema/shared.rs:Attachment`

### Task 4.20: shared.rs - Tag struct

- [ ] Add specta derive to `Tag` struct
  - **Acceptance**: Struct compiles with specta
  - **Files**: `backend/crates/altair-db/src/schema/shared.rs:Tag`

---

## Phase 5: Add specta Derives to altair-core Types

### Task 5.1: Investigate type duplication

- [ ] Document duplicate types between altair-core and altair-db
  - **Acceptance**: Create issue or document decision on which types to keep
  - **Files**: Create `specs/core-004-type-generation/type-duplication.md`
  - **Types to investigate**: `EnergyCost`, `EntityStatus`

### Task 5.2: Add specta derives to altair-core types

- [ ] Add `#[cfg_attr(feature = "specta", derive(specta::Type))]` to UserId, EntityId
  - **Acceptance**: Types compile with specta feature enabled
  - **Files**: `backend/crates/altair-core/src/types.rs`
  - **Note**: May deprecate duplicate EnergyCost/EntityStatus if Phase 5.1 decides to consolidate

---

## Phase 6: Update Tauri App Bindings Generation

### Task 6.1: Update Guidance app bindings

- [ ] Modify `apps/guidance/src-tauri/src/lib.rs` to export domain types
  - **Acceptance**: Build generates `packages/bindings/src/guidance.ts` with all Guidance types
  - **Files**: `backend/apps/guidance/src-tauri/src/lib.rs`
  - **Types to export**: Quest, Campaign, FocusSession, EnergyCheckIn, UserProgress, Achievement, Streak, QuestColumn, EnergyCost, FocusSessionStatus

### Task 6.2: Update Knowledge app bindings

- [ ] Modify `apps/knowledge/src-tauri/src/lib.rs` to export domain types
  - **Acceptance**: Build generates `packages/bindings/src/knowledge.ts` with all Knowledge types
  - **Files**: `backend/apps/knowledge/src-tauri/src/lib.rs`
  - **Types to export**: Note, Folder, DailyNote

### Task 6.3: Update Tracking app bindings

- [ ] Modify `apps/tracking/src-tauri/src/lib.rs` to export domain types
  - **Acceptance**: Build generates `packages/bindings/src/tracking.ts` with all Tracking types
  - **Files**: `backend/apps/tracking/src-tauri/src/lib.rs`
  - **Types to export**: Item, Location, GeoPoint, Reservation, MaintenanceSchedule, ItemStatus, ReservationStatus

### Task 6.4: Update Mobile app bindings

- [ ] Modify `apps/mobile/src-tauri/src/lib.rs` to export all domain types
  - **Acceptance**: Build generates `packages/bindings/src/mobile.ts` with complete type set
  - **Files**: `backend/apps/mobile/src-tauri/src/lib.rs`
  - **Types to export**: All domain types from all apps (unified mobile experience)

### Task 6.5: Export shared types in all apps

- [ ] Ensure all apps export shared types (User, Capture, Attachment, Tag, etc.)
  - **Acceptance**: All app binding files include shared types
  - **Files**: All `src-tauri/src/lib.rs` files
  - **Shared types**: EntityStatus, UserRole, User, UserPreferences, Attachment, Tag, Capture, CaptureStatus, CaptureType, CaptureSource

---

## Phase 7: Update Bindings Package Structure

### Task 7.1: Update bindings index.ts with namespace exports

- [ ] Refactor `packages/bindings/src/index.ts` for better developer experience
  - **Acceptance**: Developers can import with `import { guidance } from '@altair/bindings'`
  - **Files**: `packages/bindings/src/index.ts`
  - **Pattern**:
    ```typescript
    export * as guidance from './guidance';
    export * as knowledge from './knowledge';
    export * as tracking from './tracking';
    export * from './shared-types'; // Direct exports for common types
    ```

### Task 7.2: Create shared-types re-export file

- [ ] Create `packages/bindings/src/shared-types.ts` for common type re-exports
  - **Acceptance**: Shared types can be imported directly without namespace
  - **Files**: `packages/bindings/src/shared-types.ts`
  - **Exports**: EntityStatus, EnergyCost, UserRole, User, UserPreferences, Attachment, Tag, Capture types

### Task 7.3: Verify package.json exports

- [ ] Ensure `packages/bindings/package.json` exports all TypeScript files correctly
  - **Acceptance**: TypeScript resolution works for all import patterns
  - **Files**: `packages/bindings/package.json`

---

## Phase 8: CI Binding Freshness Check

### Task 8.1: Create GitHub Actions workflow for binding validation

- [ ] Add binding freshness check to `.github/workflows/ci.yml`
  - **Acceptance**: CI fails when bindings are stale (uncommitted changes detected)
  - **Files**: `.github/workflows/ci.yml`
  - **Steps**:
    1. Checkout code
    2. Setup Rust toolchain
    3. Install pnpm
    4. Run `pnpm build` (triggers bindings generation)
    5. Run `git diff --exit-code packages/bindings/`
    6. Fail if exit code != 0

### Task 8.2: Add Rust compilation caching

- [ ] Configure Rust cache in CI workflow to reduce build time
  - **Acceptance**: Second CI run completes faster due to caching
  - **Files**: `.github/workflows/ci.yml`
  - **Use**: `Swatinem/rust-cache@v2` action

### Task 8.3: Test CI workflow with intentional stale bindings

- [ ] Verify CI correctly detects stale bindings
  - **Acceptance**: Push commit with modified Rust type but stale TypeScript, CI fails
  - **Files**: N/A (validation test)

---

## Phase 9: Developer Documentation

### Task 9.1: Document type generation pattern in CLAUDE.md

- [ ] Add "Adding a New Rust Type to TypeScript Bindings" section to CLAUDE.md
  - **Acceptance**: Documentation clearly explains the 5-step process
  - **Files**: `CLAUDE.md`
  - **Location**: Under "Development Patterns" section

### Task 9.2: Document CI binding check failure resolution

- [ ] Add troubleshooting section for binding CI failures
  - **Acceptance**: Developers know to run `pnpm build` and commit bindings
  - **Files**: `CLAUDE.md`
  - **Content**: Explain how to fix "Bindings are stale" CI error

### Task 9.3: Document Thing and NaiveTime serialization

- [ ] Add documentation for custom serde helpers usage
  - **Acceptance**: Developers know when and how to use `thing_serde` and `naive_time_serde`
  - **Files**: `CLAUDE.md`
  - **Content**: Explain the `#[serde(with = "...")]` pattern for SurrealDB and chrono types

---

## Validation Checklist

After completing all tasks, verify:

- [ ] All 14 enums appear in generated TypeScript files
- [ ] All 15+ domain structs appear in generated TypeScript files
- [ ] `Thing` fields serialize as `{ tb: string, id: string }` in TypeScript
- [ ] `NaiveTime` fields serialize as `string` (HH:MM format) in TypeScript
- [ ] Run `pnpm build` completes without errors
- [ ] Run `pnpm typecheck` passes in all apps
- [ ] CI workflow correctly fails when bindings are stale
- [ ] Run `pnpm build` twice produces identical output (determinism test)
- [ ] Import patterns work: `import { guidance } from '@altair/bindings'`
- [ ] Import patterns work: `import { EntityStatus } from '@altair/bindings'`

---

## Notes

- **Dependencies**: Phase 1 is critical path - complete before Phase 4
- **Testing**: Add tests incrementally as each phase completes
- **Commits**: Commit after each phase completion for atomic changes
- **Performance**: Phase 8 includes caching to meet NFR-001 (< 5s generation time)

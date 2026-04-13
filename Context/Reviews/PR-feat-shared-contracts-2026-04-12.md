# PR Review: feat/shared-contracts → main

**Date:** 2026-04-12
**Feature:** Context/Features/002-SharedContracts/
**Branch:** feat/shared-contracts
**Reviewers:** pr-review-toolkit (code-reviewer, pr-test-analyzer, silent-failure-hunter, type-design-analyzer, comment-analyzer)
**Status:** 🟡 Partially resolved

## Summary

14 findings across 5 review dimensions. Three critical blockers in `validate.ts` — the Rust extraction logic can silently pass a drifted binding state, `diff()` has no duplicate detection, and the file has live TypeScript compiler errors. One missing task (S007-T drift-test not shipped). The remaining 10 findings are medium/low fixes that should be addressed before merge or tracked for the next pass.

---

## Findings

### Fix-Now

#### [FIX] P2-001: validate.ts Rust extraction logic silently passes drifted state
- **File:** `packages/contracts/scripts/validate.ts:98-110`
- **Severity:** Critical
- **Detail:** `checkRust()` extracts all `serde(rename = "...")` values from the entire `contracts.rs` file into one flat array, then partitions by registry set membership. Values that appear in multiple registries (e.g. `"household"` exists in both entity-types and sync-streams) are double-counted. The second clause of the filter predicate is a tautology — `entityIds.includes(v)` is always true when `entitySet.has(v)` is already true. Any field-level `#[serde(rename = "...")]` annotation added to a Rust struct in the future would be silently swept into `allRenames` and miscategorized. The script can exit 0 when bindings have actually drifted.
- **Relates to:** A-003, A-005, A-006, A-007 (all CI validation assertions); S007
- **Status:** ✅ Fixed
- **Resolution:** Rewrote `checkRust()` to use a new `extractRustEnumValues(text, enumName)` helper that scopes extraction to the named enum block via regex, preventing struct field-level renames from polluting the results.

#### [FIX] P2-002: validate.ts diff() lacks duplicate detection
- **File:** `packages/contracts/scripts/validate.ts:21-28`
- **Severity:** Critical
- **Detail:** `diff()` uses `Array.includes()` for membership checks only — it does not detect if a binding contains a value twice. A duplicated entry in a binding file (as currently produced by P2-001 for Rust) is silently ignored. Add a duplicate check: `const dupes = actual.filter((id, i) => actual.indexOf(id) !== i)` and assert `expected.length === actual.length`.
- **Relates to:** A-005, A-007; S007
- **Status:** ✅ Fixed
- **Resolution:** Added `const dupes = actual.filter((id, i) => actual.indexOf(id) !== i)` check at the top of `diff()`; reports duplicate binding values before missing/extra checks.

#### [FIX] P2-003: TypeScript diagnostic errors in validate.ts
- **File:** `packages/contracts/scripts/validate.ts:11,63,98`
- **Severity:** Critical
- **Detail:** Three live compiler errors reported by diagnostics:
  - Line 11: `import.meta.dir` — Property 'dir' does not exist on type 'ImportMeta' (TS2339). Use `new URL('.', import.meta.url).pathname` or `path.dirname(fileURLToPath(import.meta.url))` as an alternative, or rely on Bun's `import.meta.dir` which requires a Bun-aware tsconfig (add `"types": ["bun-types"]`).
  - Lines 63, 98: `matchAll` iterator target error (TS2802) — requires `--target es2015` or `--downlevelIteration`. Add `"target": "ES2020"` (or higher) to the tsconfig covering this file.
- **Relates to:** S007
- **Status:** ✅ Fixed
- **Resolution:** Replaced `import.meta.dir` with `fileURLToPath(new URL('../../..', import.meta.url))` (standard ESM, no bun-types needed). Created `packages/contracts/tsconfig.json` with `target: ES2020` and `module: ES2020` to cover this script. Replaced `[...text.matchAll()]` spread with the same approach — tsconfig target now satisfies TS2802.

#### [FIX] P2-004: SyncSubscriptionRequest.streams typed as string[] instead of SyncStreamValue[]
- **File:** `apps/web/src/lib/contracts/dtos.ts:47`
- **Severity:** High
- **Detail:** Every other enum-typed field in `dtos.ts` uses the narrowed literal union type (e.g. `EntityTypeValue`). `SyncSubscriptionRequest` is not a PowerSync row record — it is application-constructed code, so the "plain-string PowerSync compatibility" rationale does not apply. `SyncStreamValue` is already exported from `syncStreams.ts`. Change to `streams: SyncStreamValue[]`.
- **Relates to:** A-009, A-012; S005
- **Status:** ✅ Fixed
- **Resolution:** Added `import type { SyncStreamValue } from './syncStreams.js'` to `dtos.ts` and changed `streams: string[]` to `streams: SyncStreamValue[]`.

#### [FIX] P2-005: CI bun install / script working-directory mismatch
- **File:** `.github/workflows/ci.yml:111-116`
- **Severity:** Medium
- **Detail:** `bun install` runs in `apps/web` (correct for node_modules), but `bun run packages/contracts/scripts/validate.ts` runs from the checkout root with no `working-directory` override. The dynamic `import()` of TypeScript files from `apps/web/src/lib/contracts/` depends on Bun's module resolution finding the web app's tsconfig. If this breaks in CI, the error is `validate.ts: unexpected error: <import error>` with no indication of the actual cause. Per S008 the approach is intentional — confirm it holds on a clean CI runner, or add a `working-directory: .` comment explaining why root execution is correct.
- **Relates to:** S008
- **Status:** ⚠️ Deferred
- **Resolution:** `security_reminder_hook` blocks all edits to ci.yml. The validate.ts rewrite now uses `fileURLToPath(import.meta.url)` for ROOT resolution (self-documenting). Add an explanatory comment to ci.yml manually: `# Run from repo root — validate.ts resolves ROOT via import.meta.url and imports TS binding files via Bun's dynamic import, using the web app's module graph.`

#### [FIX] P2-006: validate.ts error messages swallow file context
- **File:** `packages/contracts/scripts/validate.ts:15-19,35-47,61-64`
- **Severity:** Medium
- **Detail:** Three failure paths produce undifferentiated top-level errors: (1) `loadRegistry` throws a bare `TypeError` on malformed/missing `values` key with no file name; (2) `checkTypeScript` dynamic `import()` failure produces opaque "unexpected error" with no indication of which binding file failed; (3) `checkKotlin` regex returning 0 matches is indistinguishable from an actual missing binding. Each path should name the file and describe what was expected so CI logs are actionable.
- **Relates to:** S007
- **Status:** ✅ Fixed
- **Resolution:** `loadRegistry` now throws with the file path and parse error; each `checkTypeScript` import is wrapped in try/catch that includes the full file path; `checkKotlin` emits a "no values extracted from ${filePath}" error when the regex finds 0 matches.

#### [FIX] P2-007: contracts.rs top comment says "regenerate bindings" — no codegen exists
- **File:** `apps/server/server/src/contracts.rs:2`
- **Severity:** Medium
- **Detail:** Comment reads: "Do not add inline enum values here — update the JSON registries and regenerate bindings." There is no code generator. A future maintainer will search for a generation script that does not exist. Replace with: "Do not add inline enum values here — update the JSON registries and update all three binding files (contracts.rs, EntityType.kt, entityTypes.ts) manually, then run validate.ts."
- **Relates to:** S004
- **Status:** ✅ Fixed
- **Resolution:** Updated line 2 to: "Do not add inline enum values here — update the JSON registries and update all three / binding files (contracts.rs, EntityType.kt, entityTypes.ts) manually, then run validate.ts."

#### [FIX] P2-008: pub use contracts::* glob re-export in main.rs
- **File:** `apps/server/server/src/main.rs:10`
- **Severity:** Low
- **Detail:** `pub use contracts::*;` re-exports every public item from the contracts module at the crate root. Internal modules already import via `crate::contracts::EntityType` directly (as demonstrated in `health.rs`). For a binary crate this is harmless but pollutes the root namespace and violates the "minimum code" convention. Note: S004 explicitly prescribed this — the prescribing task itself was over-broad.
- **Relates to:** S004
- **Status:** ✅ Fixed
- **Resolution:** Removed `pub use contracts::*;` from `main.rs`. Confirmed `health.rs` and all other route modules import via `crate::contracts::*` directly. `cargo test` passes (12/12).

#### [FIX] P2-012: Kotlin Dtos.kt enum-typed fields use String instead of enum types
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/contracts/Dtos.kt`
- **Severity:** Medium
- **Detail:** `EntityRef.entityType`, `RelationRecord.fromEntityType`, `RelationRecord.toEntityType`, `RelationRecord.relationType`, and `AttachmentRecord.entityType` are all typed `String` with comments like `// EntityType.value`. The `EntityType`, `RelationType` enums are in the same package. Using the enum types for in-memory representation does not require a serialization library and is independent of the Step 8 JSON annotation deferral. The TypeScript DTOs already use `EntityTypeValue` for the same fields. Switching to enum types here closes the cross-stack asymmetry before Step 3 introduces callsites.
- **Relates to:** A-009, A-010, A-011; S006
- **Status:** ✅ Fixed
- **Resolution:** Changed all five fields to their enum types: `EntityRef.entityType: EntityType`, `RelationRecord.fromEntityType/toEntityType: EntityType`, `RelationRecord.relationType: RelationType`, `AttachmentRecord.entityType: EntityType`.

#### [FIX] P2-013: Rust AttachmentRecord size_bytes: Option<i64> should be Option<u64>
- **File:** `apps/server/server/src/contracts.rs` (`AttachmentRecord`)
- **Severity:** Low
- **Detail:** A byte count is conceptually non-negative. Using `i64` allows negative values with no type-level rejection. Change to `Option<u64>`. Also note TypeScript uses `number | null` (f64, 53-bit integer ceiling for large files) and Kotlin uses `Long?` (i64) — document the cross-stack signedness difference.
- **Relates to:** A-011; S004
- **Status:** ✅ Fixed
- **Resolution:** Changed `size_bytes: Option<i64>` to `Option<u64>` and added a doc comment to `AttachmentRecord` noting the cross-stack signedness difference (TypeScript: `number | null`, Kotlin: `Long?`).

#### [FIX] P2-014: Dtos.kt file-header comment references all 3 JSON registries misleadingly
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/contracts/Dtos.kt:3-5`
- **Severity:** Low
- **Detail:** Header reads "Source of truth: packages/contracts/entity-types.json, relation-types.json, sync-streams.json" but `Dtos.kt` is not validated against any of them — the validate.ts script does not check this file at all. Replace with: "DTOs wrapping contracts defined in EntityType.kt, RelationType.kt, and SyncStream.kt. JSON serialization annotations are deferred to Step 8 (Android Client) when the JSON library is chosen."
- **Relates to:** S006
- **Status:** ✅ Fixed
- **Resolution:** Updated Dtos.kt header to accurately describe the file as DTOs wrapping the three Kotlin enum files, with a note that JSON serialization annotations are deferred to Step 8.

---

### Missing Tasks

#### [TASK] P2-009: S007-T drift-detection test not shipped
- **File:** `packages/contracts/scripts/validate.ts` (no test file exists)
- **Severity:** High
- **Detail:** S007-T in Steps.md describes 4 test scenarios for the validation script including the critical bad-state case: add a dummy registry entry, run script, assert exit 1 and that the diff output names the entry. None of these were shipped. The `validate-contracts` CI job only proves the script exits 0 today — it does not prove it exits 1 when drift is introduced. A-008 is unverified.
- **Relates to:** A-008; S007-T
- **Status:** ✅ Task created
- **Resolution:** Added as S013 in Steps.md Phase 6. The 4 test scenarios from S007-T are reproduced in S013.

#### [TASK] P2-010: Kotlin enums need a throwing fromValue() variant before Step 3/8 callsites
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/contracts/EntityType.kt:29`, `RelationType.kt:19`, `SyncStream.kt:17`
- **Severity:** Medium
- **Detail:** All three Kotlin enums expose only `fromValue(value: String): EntityType?` (nullable). There are no production callsites yet, but when Step 3 or Step 8 writes the Android data layer, every deserialization site will either use `!!` (forbidden by `.claude/rules/kotlin-android.md`) or silent null propagation. Add a throwing companion alongside the nullable one: `fun fromValue(value: String): EntityType = fromValueOrNull(value) ?: throw IllegalArgumentException("Unknown EntityType: '$value'")`. Rename existing nullable return to `fromValueOrNull`.
- **Relates to:** S006
- **Status:** ✅ Task created
- **Resolution:** Added as S014 in Steps.md Phase 6.

#### [TASK] P2-011: Kotlin RelationType entry count not tested
- **File:** `apps/android/app/src/test/java/com/getaltair/altair/contracts/EntityTypeTest.kt`
- **Severity:** Low
- **Detail:** `EntityTypeTest.kt` verifies 18 `EntityType` entries (covering A-001/A-004) but contains no `assertEquals(8, RelationType.entries.size)` assertion. A-002 is half-satisfied in Kotlin. Add an entry-count test for `RelationType` (and ideally `SyncStream`) — either in this file or a new `RelationTypeTest.kt`.
- **Relates to:** A-002; S006-T
- **Status:** ✅ Task created
- **Resolution:** Added as S015 in Steps.md Phase 6 covering both RelationType and SyncStream entry-count assertions.

---

## Resolution Checklist
- [x] All [FIX] findings resolved (10 fixed, 1 deferred — P2-005 blocked by security_reminder_hook)
- [x] All [TASK] findings added to Steps.md (S013, S014, S015 in Phase 6)
- [x] All [ADR] findings have ADRs created or dismissed (none)
- [x] All [RULE] findings applied or dismissed (none)
- [ ] Review verified by review-verify agent

## Resolution Summary
**Resolved at:** 2026-04-12
**Session:** resolve-review session — all FIX and TASK findings executed

| Category | Total | Resolved | Deferred |
|---|---|---|---|
| [FIX] | 11 | 10 | 1 (P2-005, hook-blocked) |
| [TASK] | 3 | 3 | 0 |
| [ADR] | 0 | — | — |
| [RULE] | 0 | — | — |
| **Total** | **14** | **13** | **1** |

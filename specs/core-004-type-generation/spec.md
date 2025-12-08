# Specification: Type-Safe Rust ↔ TypeScript Boundary

<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<!--
SPEC WEIGHT: [x] LIGHTWEIGHT  [ ] STANDARD  [ ] FORMAL

Weight Guidelines:
- LIGHTWEIGHT: Bug fixes, small enhancements, config changes (<2 days work)
  Required sections: Quick Reference, Problem/Solution, Requirements, Acceptance Criteria
-->
<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->

**Spec ID**: CORE-004-type-generation
**Component**: CORE
**Weight**: LIGHTWEIGHT
**Version**: 1.0
**Status**: DRAFT
**Created**: 2025-12-07
**Author**: Claude (AI Assistant)

---

## Quick Reference

> Type generation pipeline using tauri-specta to automatically generate TypeScript
> bindings from Rust types, ensuring compile-time type safety across the IPC boundary.

**What**: Automated TypeScript type generation from Rust types via tauri-specta
**Why**: Eliminates runtime type errors and manual type synchronization between Rust and TypeScript
**Impact**: Zero type mismatches between frontend and backend; reduced development time

**Success Metrics**:

| Metric                  | Target | How Measured                               |
| ----------------------- | ------ | ------------------------------------------ |
| Type coverage           | 100%   | All Tauri commands have generated bindings |
| Build-time type errors  | 0      | TypeScript compilation succeeds            |
| Manual type definitions | 0      | No hand-written IPC types in frontend      |

---

## Problem Statement

### Current State

The Altair monorepo has tauri-specta already integrated in each Tauri app (guidance,
knowledge, tracking, mobile). Each app generates its own TypeScript bindings to
`packages/bindings/src/{app}.ts`. The basic infrastructure works—the `health_check`
command is type-safe.

However, the current implementation lacks:

1. **Shared types** — Domain types (Quest, Note, Item) defined in `altair-db` and
   `altair-core` are not exported to TypeScript. Only command-level types are generated.
2. **Build integration** — Type generation happens ad-hoc during debug builds, not as
   part of a reliable build pipeline.
3. **Type conventions** — No documented patterns for ensuring all new types are
   specta-compatible.

### Desired State

When complete:

1. All domain types (`Quest`, `Note`, `Item`, enums like `EnergyCost`, `QuestStatus`)
   are available in TypeScript with correct types.
2. Running `pnpm build` automatically regenerates bindings if Rust types changed.
3. Developers have clear guidance on adding new types to the generation pipeline.
4. TypeScript consumers import from `@altair/bindings` with full intellisense.

### Why Now

Core-003 established the backend skeleton with tauri-specta integration. Core-002
defined the domain schema in SurrealDB with corresponding Rust types. This spec
bridges those two—making the Rust domain types available to the Svelte frontend
before any domain commands (guidance-001, knowledge-001) are implemented.

---

## Solution Overview

### Approach

Extend the existing tauri-specta setup to:

1. **Add specta derive to domain types** — Add `#[cfg_attr(feature = "specta", derive(specta::Type))]`
   to types in `altair-db/src/schema/` and `altair-core/src/`.
2. **Create a shared types module** — Export commonly-used types (enums, IDs, timestamps)
   that are referenced across commands.
3. **Improve bindings package** — Add re-exports and index file for clean imports.
4. **Document the pattern** — Add developer documentation for adding new types.

### Scope

**In Scope**:

- Adding `specta::Type` derive to domain types in `altair-db` schema module
- Adding `specta::Type` derive to shared types in `altair-core`
- Updating `packages/bindings/` structure for better organization
- Build script or turbo task to verify types are current
- Developer documentation in CLAUDE.md or CONTRIBUTING.md

**Out of Scope**:

- Implementing domain CRUD commands — tracked in guidance-001, knowledge-001, etc.
- Event types for real-time sync — tracked in core-013-sync-engine
- Complex nested type handling — defer until needed

**Future Considerations**:

- Automatic changelog for generated types — if breaking changes become common
- Versioned type exports — if mobile/desktop need different versions

### Key Decisions

| Decision                     | Options Considered                        | Rationale                                                           |
| ---------------------------- | ----------------------------------------- | ------------------------------------------------------------------- |
| Feature-flag specta derives  | Always derive vs. feature-gated           | Feature-gated avoids compile overhead for non-Tauri builds          |
| Per-app vs. unified bindings | One file per app vs. single bindings file | Per-app maintains separation; each app only gets types it uses      |
| Build-time vs. check-only    | Regenerate in build vs. check freshness   | CI regenerates + fails on git diff; debug regenerates automatically |

---

## Requirements

### Functional Requirements

| ID     | Requirement                                                                | Priority | Notes                              |
| ------ | -------------------------------------------------------------------------- | -------- | ---------------------------------- |
| FR-001 | Domain enums (EnergyCost, QuestStatus, etc.) generate as TypeScript unions | CRITICAL | Foundation for all domain commands |
| FR-002 | Domain structs (Quest, Note, Item) generate as TypeScript interfaces       | CRITICAL | Enable type-safe CRUD commands     |
| FR-003 | Shared types (RecordId, DateTime) have correct TypeScript representations  | HIGH     | SurrealDB IDs, chrono timestamps   |
| FR-004 | Generated bindings are importable from `@altair/bindings`                  | HIGH     | Clean import path for consumers    |
| FR-005 | Build fails if generated types are out of sync with Rust                   | MEDIUM   | Prevents runtime type mismatches   |

### Non-Functional Requirements

| ID      | Requirement                                     | Priority | Notes                             |
| ------- | ----------------------------------------------- | -------- | --------------------------------- |
| NFR-001 | Type generation completes in < 5 seconds        | MEDIUM   | Part of development feedback loop |
| NFR-002 | Generated files are deterministic (no diffs)    | HIGH     | Avoid spurious git changes        |
| NFR-003 | Specta feature adds < 2s to clean backend build | MEDIUM   | Developer experience              |

### User Stories

**US-001: Frontend Developer Uses Domain Types**

- **As** a frontend developer,
- **I** need to
  - Import domain types from `@altair/bindings`
  - Get TypeScript intellisense for Quest, Note, Item fields
  - Have compile-time errors if I use wrong field names
- **so** that I can build UI components with confidence.

Acceptance:

- [ ] `import { Quest, QuestStatus } from '@altair/bindings'` works
- [ ] TypeScript shows correct field types on hover
- [ ] Typos in field names cause TS errors

Independent Test: Import types in a test .ts file and verify compilation

**US-002: Backend Developer Adds New Type**

- **As** a backend developer,
- **I** need to
  - Add `specta::Type` derive to my new Rust type
  - See it appear in TypeScript bindings after rebuild
- **so** that I can expose new types without manual TypeScript work.

Acceptance:

- [ ] Adding derive macro causes TypeScript file to update on build
- [ ] Documentation explains the process

Independent Test: Create test struct, add derive, verify generation

---

## Test Requirements

### Success Criteria

| ID     | Criterion                                                  | Measurement         |
| ------ | ---------------------------------------------------------- | ------------------- |
| SC-001 | All domain enums from altair-db/schema are in TypeScript   | Grep for enum names |
| SC-002 | All domain structs from altair-db/schema are in TypeScript | Grep for type names |
| SC-003 | TypeScript compilation succeeds with generated types       | `pnpm build` passes |
| SC-004 | Svelte components can use generated types without errors   | App builds succeed  |

### Acceptance Criteria

**Scenario**: Import domain types in Svelte component _(maps to US-001)_

```gherkin
Given the packages/bindings package is built
When a Svelte component imports Quest and EnergyCost
Then TypeScript compilation succeeds
And the types have correct field definitions
```

**Scenario**: Add specta derive to new type _(maps to US-002)_

```gherkin
Given a new struct in altair-db with specta::Type derive
When the Guidance app is built in debug mode
Then the new type appears in packages/bindings/src/guidance.ts
```

### Test Scenarios

| ID     | Scenario                                       | Type       | Priority | Maps To |
| ------ | ---------------------------------------------- | ---------- | -------- | ------- |
| TS-001 | All domain enums have TypeScript equivalents   | Functional | CRITICAL | FR-001  |
| TS-002 | All domain structs have TypeScript equivalents | Functional | CRITICAL | FR-002  |
| TS-003 | RecordId maps to string in TypeScript          | Functional | HIGH     | FR-003  |
| TS-004 | DateTime maps to string (ISO 8601)             | Functional | HIGH     | FR-003  |
| TS-005 | Clean build generates deterministic output     | Functional | HIGH     | NFR-002 |

---

## Constraints and Assumptions

### Technical Constraints

- **SurrealDB RecordId (`Thing`)**: Serialize as object `{ tb: string, id: string }` preserving table/id structure
- **Chrono DateTime**: Serializes as ISO 8601 string via serde
- **Chrono NaiveTime**: Serializes as `HH:MM` format string (e.g., `"18:00"`)
- **specta version**: Locked to 2.0.0-rc.20 (workspace dependency)

### Assumptions

- Domain types in altair-db/schema are stable enough to expose
- Feature-gated specta derives don't cause conditional compilation issues
- Tauri apps are built in debug mode during development (triggers generation)

### Dependencies

| Dependency       | Type     | Status   | Impact if Delayed            |
| ---------------- | -------- | -------- | ---------------------------- |
| core-003-backend | Internal | Complete | Blocks: tauri-specta not set |
| core-002-schema  | Internal | Complete | Blocks: no domain types      |

---

## Clarifications

### Session 2025-12-07

- Q: How should SurrealDB `Thing` (record ID) be represented in TypeScript? → A: Object `{ tb: string, id: string }` - preserve structure as interface
- Q: How should `chrono::NaiveTime` be represented in TypeScript? → A: String in `HH:MM` format (e.g., `"18:00"`) - omit seconds
- Q: How should CI detect stale TypeScript bindings? → A: Regenerate bindings in CI, fail if `git diff` shows changes

---

## References

### Internal

- `docs/technical-architecture.md` §Type Safety Across Boundaries
- `backend/crates/altair-db/src/schema/` — Domain type definitions
- `backend/crates/altair-commands/src/lib.rs` — Current specta usage

### External

- [tauri-specta documentation](https://github.com/oscartbeaumont/tauri-specta)
- [specta documentation](https://docs.rs/specta)

---

## Changelog

| Version | Date       | Author | Changes               |
| ------- | ---------- | ------ | --------------------- |
| 1.0     | 2025-12-07 | Claude | Initial specification |

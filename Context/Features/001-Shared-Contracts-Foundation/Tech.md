# Tech Plan: Shared Contracts Foundation

**Spec:** Context/Features/001-Shared-Contracts-Foundation/Spec.md
**Stacks involved:** Python 3 (code generation), TypeScript, Kotlin, Rust (generated bindings), GitHub Actions (CI enforcement)

## Architecture Overview

The shared contracts foundation establishes a **registry-first** architecture where JSON files in `packages/contracts/registry/` serve as the source of truth for all canonical identifiers. A Python code generation script reads these registries and emits language-specific bindings (TypeScript, Kotlin, Rust) into `packages/contracts/generated/`. These generated bindings are committed to the repo and imported by all platforms (web, desktop, Android, server, worker).

```text
packages/contracts/
├── registry/                          # Source of truth (never edit elsewhere)
│   ├── entity-types.json              # Canonical entity type identifiers
│   ├── relation-types.json            # Canonical relation/source/status types
│   └── sync-streams.json              # Canonical sync stream names
├── schemas/                           # Cross-platform DTO definitions
│   ├── relation-record.schema.json     # JSON Schema for RelationRecord
│   └── attachment-record.schema.json   # JSON Schema for AttachmentRecord
├── generated/                         # Derived artifacts (commit to repo)
│   ├── typescript/
│   │   └── contracts.ts             # TS constants + types
│   ├── kotlin/
│   │   └── Contracts.kt              # Kotlin enums
│   └── rust/
│       └── contracts.rs               # Rust enums
├── scripts/
│   └── generate_contracts.py           # Code generation script
└── tests/
    ├── test_registry_shapes.py          # Validate registry structure
    ├── test_generated_typescript.py     # Verify TS bindings
    ├── test_generated_kotlin.py        # Verify Kotlin bindings
    └── test_generated_rust.py          # Verify Rust bindings
```

**CI enforcement workflow:**
1. On PR to contracts package or root
2. Checkout and run Python generator script
3. Run validation tests (pytest)
4. Check git diff on generated/ directory
5. Fail if any generated files are stale or tests fail

This ensures that any changes to registry JSON must include updated generated bindings, and that no PR can add new magic strings outside the contracts package without being caught by CI.

## Key Decisions

### Decision 1: Code Generation Language

**Options considered:**
- Option A: Python — Pros: Already provided in tooling bundle, mature JSON handling, pytest ecosystem for tests, simple dependency management. Cons: Another language in monorepo.
- Option B: TypeScript/Node — Pros: Existing workspace tooling (bun), familiar to frontend team. Cons: Would need to be part of web app or separate script, heavier runtime.
- Option C: Rust — Pros: Already in stack, strong typing. Cons: JSON handling more verbose, test ecosystem (cargo test) different from other test frameworks.

**Chosen:** Option A — Python 3

**Rationale:** The provided tooling bundle (`altair-contracts-tooling-bundle.zip`) includes a complete Python generator script with pytest-based validation tests. This is production-ready code that handles all three target languages (TypeScript, Kotlin, Rust) correctly. Reimplementing in another language would be duplication without benefit. Python is lightweight, has excellent JSON support, and pytest integrates cleanly with CI.

**Related ADRs:** None (infrastructure decision not addressed by existing ADRs)

### Decision 2: CI Enforcement Strategy

**Options considered:**
- Option A: Generate in CI, check git diff, fail on staleness — Pros: Catches drift early, ensures consistency, simple to implement. Cons: Adds ~10-15s to CI per PR.
- Option B: Only check git diff without regenerating — Pros: Faster CI. Cons: Doesn't validate generation works, developer may commit broken generated files.
- Option C: Generate at build time only — Pros: Always in sync. Cons: Generated files not visible in repo, harder to review, can't commit changes to generated format without regenerating entire repo.

**Chosen:** Option A — Generate in CI and check git diff for staleness

**Rationale:** This provides the strongest guardrails. By regenerating in CI and checking git diff, we ensure that:
1. Registry changes always include updated bindings
2. Generator script continues to work as registries evolve
3. No stale generated files can be committed
4. Reviewers can see exactly what changed in generated bindings

The performance cost (~10s) is acceptable for this critical foundation layer, and CI will only run when contracts change (via `paths:` filter in workflow).

**Related ADRs:** None (new CI workflow)

### Decision 3: Testing Framework

**Options considered:**
- Option A: pytest — Pros: Matches provided test files in tooling bundle, mature framework, clean assertions, fixtures for registry loading. Cons: Python-specific, requires Python in dev environment.
- Option B: Vitest — Pros: Existing JS/TS test framework, familiar to team. Cons: Would need to rewrite provided Python tests, registry validation needs proper JSON parsing.
- Option C: Unittest — Pros: Built into Python stdlib. Cons: Less ergonomic than pytest, provided tests use pytest.

**Chosen:** Option A — pytest

**Rationale:** The provided tooling bundle includes pytest-based validation tests that validate registry structure, check for duplicates, and verify generated bindings contain all canonical values. These tests are comprehensive and ready to use. Migrating to another framework would be work with no benefit.

**Related ADRs:** None

### Decision 4: Schema Definition Format

**Options considered:**
- Option A: JSON Schema — Pros: Industry standard, validation libraries available in all stacks, can generate TypeScript types. Cons: Verbose for simple schemas.
- Option B: TypeScript interfaces only — Pros: Simple for web/desktop. Cons: No validation for Kotlin/Rust, not truly cross-platform.
- Option C: Protocol Buffers — Pros: Efficient serialization. Cons: Overkill for simple DTOs, tooling complexity.

**Chosen:** Option A — JSON Schema

**Rationale:** The provided tooling bundle includes JSON Schema definitions for `RelationRecord`, `AttachmentRecord`, and `EntityRef`. These schemas can be used for:
- Runtime validation in all platforms (python-jsonschema, TS validators, Kotlin JSON Schema libraries)
- TypeScript type generation if needed
- Documentation of expected payload shapes

JSON Schema is the most appropriate choice for defining shared data structures that aren't API responses per se but are used across multiple platforms.

**Related ADRs:** ADR-004 (Relationship Modeling Strategy — specifies `RelationRecord` structure)

### Decision 5: Registry Structure

**Options considered:**
- Option A: Single flat registry JSON — Pros: Simple to maintain. Cons: Mixed concerns (entity types vs relation types), harder to validate sections.
- Option B: Domain-grouped entity types, separate registries for relations/streams — Pros: Clear separation of concerns, matches domain organization (core/guidance/knowledge/tracking). Cons: More files to maintain.
- Option C: Nested registries with full versioning — Pros: Precise tracking of changes. Cons: Over-engineering for initial phase.

**Chosen:** Option B — Domain-grouped entity types with separate registries

**Rationale:** The provided spec and tooling bundle use this structure:
- `entity-types.json` with grouped sections (`core`, `guidance`, `knowledge`, `tracking`)
- `relation-types.json` for relation/source/status types
- `sync-streams.json` for sync stream names

This separation aligns with Altair's domain organization and makes it clear which registries can be extended independently.

**Related ADRs:** None (follows from shared contracts spec)

## Stack-Specific Details

### Python (Code Generation & Tests)

**Files to create/modify:**
- `packages/contracts/scripts/generate_contracts.py` — From tooling bundle
- `packages/contracts/tests/test_registry_shapes.py` — From tooling bundle
- `packages/contracts/tests/test_generated_*.py` — From tooling bundle
- `packages/contracts/pyproject.toml` — New: Python project config
- `packages/contracts/requirements.txt` — New: Python dependencies (pytest)
- `.github/workflows/contracts.yml` — Update: Replace placeholder with real implementation

**Patterns to follow:**
- Use standard library `json` module for registry parsing
- Use `pathlib.Path` for cross-platform file paths
- Follow Python PEP 8 naming conventions (snake_case)
- pytest fixtures for registry loading and generator execution

**Dependencies:**
- `pytest` — Test framework (already in tooling bundle tests)
- No additional runtime dependencies (uses stdlib only)

### TypeScript (Generated Bindings)

**Files to create/modify:**
- `packages/contracts/generated/typescript/contracts.ts` — Generated (commit to repo)
- No source files needed for generation (script emits directly)

**Patterns to follow:**
- See `.claude/rules/sveltekit-svelte5-tailwind.md` — TypeScript strict mode
- Generated exports follow: `export const NAME = { VALUES } as const;`
- Generated types: `export type Name = typeof NAME[keyof typeof NAME];`
- Interface exports for DTOs (RelationRecord, AttachmentRecord, EntityRef)

**Dependencies:**
- None (generated code has no runtime dependencies)

### Kotlin (Generated Bindings)

**Files to create/modify:**
- `packages/contracts/generated/kotlin/Contracts.kt` — Generated (commit to repo)

**Patterns to follow:**
- See `.claude/rules/android-kotlin.md` — Naming conventions
- Generated enums: `enum class EntityType(val: String)` with `fromWire()` companion
- Package declaration: `package com.altair.contracts`
- Serialization: `val` property stores canonical string value

**Dependencies:**
- None (generated code has no runtime dependencies)

### Rust (Generated Bindings)

**Files to create/modify:**
- `packages/contracts/generated/rust/contracts.rs` — Generated (commit to repo)

**Patterns to follow:**
- See `.claude/rules/rust.md` — Rust conventions
- Derive macros: `Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash`
- Enum variant naming: PascalCase with snake_case serde attribute
- Use `serde(rename_all = "snake_case") for enum values

**Dependencies:**
- `serde` — Already in server/worker Cargo.toml (derive macros in generated code)

### GitHub Actions (CI Enforcement)

**Files to create/modify:**
- `.github/workflows/contracts.yml` — Replace placeholder with full implementation

**Patterns to follow:**
- Use `actions/checkout@v4`
- Trigger on `pull_request` with `paths:` filter for contracts package
- Run `python3 packages/contracts/scripts/generate_contracts.py`
- Run `pytest packages/contracts/tests/`
- Check git diff: `git diff --exit-code packages/contracts/generated/`
- Fail if diff finds changes (exit code 1 means uncommitted changes)

**Dependencies:**
- Python 3 (pre-installed on GitHub Actions runners)
- No additional setup needed

## Integration Points

### TypeScript → Kotlin → Rust
All three languages import canonical identifiers from generated bindings:
- Entity types: `EntityType` enum/const
- Relation types: `RelationType` enum/const
- Source/status types: `RelationSourceType`/`RelationStatusType` enum/const
- Sync streams: `SyncStream` enum/const

Generator script reads same registries and emits language-appropriate constructs:
- **TypeScript:** Constants object with type export
- **Kotlin:** Enum class with String serialization
- **Rust:** Enum with serde derive macros

### Registry → Generated → CI
Flow:
1. Developer edits registry JSON in `packages/contracts/registry/`
2. Developer runs generator locally: `python3 scripts/generate_contracts.py --root .`
3. Generated files updated in `packages/contracts/generated/`
4. Developer commits both registry changes and generated bindings
5. PR opens, CI runs generator script
6. CI runs pytest tests
7. CI checks git diff on generated/
8. If diff is clean, PR passes. If diff has changes, PR fails (stale bindings)

### Generated Bindings → Platform Code
Each platform imports from generated directory:
- **Web/Desktop:** `import { ENTITY_TYPES, RelationType } from '$altair/contracts'`
- **Android:** `import com.altair.contracts.EntityType`
- **Server/Worker:** `use altair_contracts::{EntityType, RelationType}`

This ensures all platforms use the same canonical identifiers, validated by CI.

## Risks & Unknowns

- **Risk:** Generator script may fail on malformed registry JSON
  - **Mitigation:** Pytest tests validate registry structure before generation runs; CI catches failures early
- **Risk:** Python 3 not installed in local dev environment
  - **Mitigation:** Document Python 3 as prerequisite in contracts README; dev can run generator without installing globally (use system Python or pyenv)
- **Risk:** Generated bindings become stale if developer forgets to run generator
  - **Mitigation:** CI enforcement fails PR if bindings are stale; clear error message in workflow explains fix
- **Unknown:** Should Python dependencies be added to root `bun.lockb` or separate requirements?
  - **Resolution plan:** Use `requirements.txt` in contracts package for Python dependencies; no need to mix with JS lockfile
- **Unknown:** Should Kotlin package path `com.altair.contracts` be configurable?
  - **Resolution plan:** Hardcode for now; if Android team needs different package, make configurable via env var or CLI flag to generator

## Testing Strategy

### High-level approach

**Python validation tests (pytest):**
- **Registry shape tests:** Validate JSON structure, required fields, no missing keys
- **Duplicate detection:** Ensure no duplicate values within each registry file
- **Generation verification:** Run generator and check output files exist and contain expected values

**Language-specific binding tests:**
- **TypeScript tests:** Import generated contracts.ts and verify all constants are present and types are correctly exported
- **Kotlin tests:** Verify enum variants serialize to/from canonical string values
- **Rust tests:** Verify enums derive serde and Serialize/Deserialize work correctly

**CI integration:**
- All tests run as part of `.github/workflows/contracts.yml`
- Tests run on every PR that touches contracts package or workflow file
- Failure of any test blocks PR merge
- Git diff check ensures no uncommitted generated changes

### Test infrastructure

- `pytest` for Python tests (provided in tooling bundle)
- Test discovery: `pytest packages/contracts/tests/`
- Tests can be run locally: `pytest packages/contracts/tests/`
- CI runs same command with Python 3 from GitHub Actions

## Open Questions Resolution

From Spec.md Open Questions:

1. **Should generated bindings be re-generated in CI or only checked for staleness?**
   - **Resolved:** Re-generate in CI AND check git diff (Decision 2 above)
   - **Rationale:** Provides strongest guardrails, catches drift early

2. **Should contracts package have its own workspace configuration or rely on root workspace?**
   - **Resolved:** Use separate `pyproject.toml` / `requirements.txt` for Python portion, but generated bindings are consumed by existing workspaces (web/desktop use root bun workspace, Android uses Gradle, server/worker use Cargo)
   - **Rationale:** Python tooling is isolated; generated bindings integrate into each platform's existing workspace configuration

3. **What is the acceptable workflow for adding new entity types or relation types?**
   - **Status:** Requires ADR to formalize governance
   - **Recommendation:** Create ADR-005: "Shared Contracts Governance Process" before or during Phase 2/3 when first new identifier is needed
   - **Draft process:** ADR should define: when ADR is required, approval process, versioning strategy, deprecation policy

## Related Decisions

- **ADR-004 (Relationship Modeling Strategy):** Specifies `RelationRecord` structure with entity type/relation type/status fields — directly informed schema definitions
- **Shared conventions (all stacks):** Mandate using contracts package, never invent new shared identifiers inline — enforced by this feature
- **Stack-specific rules:** Android, Rust, SvelteKit rules all reference contracts package as source of canonical identifiers

## Implementation Dependencies

This feature (Phase 1) is a hard dependency for:
- **Phase 2 (Database & Schema):** Entity/relation types used in migrations and schema design
- **Phase 3 (Backend Core):** Relation APIs validate entity/relation types against contracts
- **Phase 4 (PowerSync):** Sync stream names from contracts must match PowerSync config

Cannot proceed to Phase 2+ without this foundation in place.

# Implementation Steps: Shared Contracts Foundation

**Spec:** Context/Features/001-Shared-Contracts-Foundation/Spec.md
**Tech:** Context/Features/001-Shared-Contracts-Foundation/Tech.md

## Progress
- **Status:** Not started
- **Current task:** —
- **Last milestone:** —

## Tasks

### Phase 1: Setup Registry & Schemas
- [ ] S001: Create packages/contracts directory structure
- [ ] S001-D: Create contracts package README with source-of-truth policy
- [ ] S002: Add entity-types.json registry with canonical values
- [ ] S003: Add relation-types.json registry with all relation/source/status types
- [ ] S004: Add sync-streams.json registry with auto-subscribed and on-demand streams
- [ ] S005: Add relation-record.schema.json JSON Schema
- [ ] S006: Add attachment-record.schema.json JSON Schema
- [ ] S007: Add entity-ref.schema.json JSON Schema (optional EntityRef DTO)

🏁 MILESTONE: Registry & Schemas Complete — verify against [A-001, A-002]

### Phase 2: Code Generation & Tests
- [ ] S008: Create scripts directory and add generate_contracts.py from tooling bundle [P]
- [ ] S009: Create tests directory and add pytest-based validation tests [P]
- [ ] S009-T: Test registry shape validation (required fields, no missing keys, proper types)
- [ ] S010: Run generator script to create TypeScript bindings [P]
- [ ] S011: Run generator script to create Kotlin bindings [P]
- [ ] S012: Run generator script to create Rust bindings [P]
- [ ] S012-T: Test generated TypeScript constants and types match registry values
- [ ] S013: Add pyproject.toml for Python project configuration
- [ ] S014: Add requirements.txt with pytest dependency

🏁 MILESTONE: Code Generation Working — verify against [A-003, A-004, A-005, A-006, A-007]

### Phase 3: CI Enforcement
- [ ] S015: Update .github/workflows/contracts.yml with real implementation
- [ ] S016: Configure CI to run generator script on PR
- [ ] S017: Configure CI to run pytest validation tests
- [ ] S018: Configure CI to fail if git diff finds uncommitted generated changes
- [ ] S018-T: Test CI enforcement workflow locally or via workflow_dispatch
- [ ] S019-D: Update root README with contracts setup instructions

🏁 MILESTONE: CI Enforcement Complete — verify against [A-008, A-009, A-010]

### Phase 4: Platform Integration
- [ ] S020: Add contracts import path to TypeScript project configuration
- [ ] S021: Add contracts import path to Android Gradle configuration
- [ ] S022: Add contracts module to Rust server Cargo.toml
- [ ] S023: Add contracts module to Rust worker Cargo.toml
- [ ] S024: Update backend docs to reference canonical entity types
- [ ] S025: Update PowerSync spec docs to reference canonical stream names
- [ ] S026-D: Update CLAUDE.md shared conventions to enforce contract usage

🏁 MILESTONE: Feature Complete — verify all assertions, full drift check

## Task Details

### S001: Create packages/contracts directory structure
Create the base directory structure for the contracts package:
```
packages/contracts/
├── registry/
├── schemas/
├── generated/
│   ├── typescript/
│   ├── kotlin/
│   └── rust/
├── scripts/
└── tests/
```

### S001-D: Create contracts package README
Document the source-of-truth policy, how to run the generator, and examples of importing generated bindings for each platform. Include warning that registry JSON is canonical and should not be edited elsewhere.

### S002: Add entity-types.json registry
Create `packages/contracts/registry/entity-types.json` with:
- Version field
- entityTypes object with grouped sections: core, guidance, knowledge, tracking
- All canonical entity types from shared contracts spec (user, household, initiative, tag, attachment, guidance_*, knowledge_*, tracking_*)

### S003: Add relation-types.json registry
Create `packages/contracts/registry/relation-types.json` with:
- relationTypes: references, supports, requires, related_to, depends_on, duplicates, similar_to, generated_from
- sourceTypes: user, ai, import, rule, migration, system
- statusTypes: accepted, suggested, dismissed, rejected, expired

### S004: Add sync-streams.json registry
Create `packages/contracts/registry/sync-streams.json` with:
- autoSubscribed: my_profile, my_memberships, my_personal_data, my_household_data, my_relations, my_attachment_metadata
- onDemand: initiative_detail, note_detail, item_history, quest_detail

### S005: Add relation-record.schema.json
Create JSON Schema matching the ADR-004 specification with required fields: id, from, to, relationType, sourceType, status, createdAt, updatedAt. Include optional fields: confidence, evidence. From/to use EntityRef schema structure.

### S006: Add attachment-record.schema.json
Create JSON Schema for AttachmentRecord with fields for attachment ID, kind, processing state, and metadata.

### S007: Add entity-ref.schema.json
Create JSON Schema for EntityRef with entityType and entityId fields.

### S008: Create scripts directory and add generate_contracts.py
Copy the Python generator script from tooling bundle to `packages/contracts/scripts/generate_contracts.py`. Script should:
- Accept --root parameter
- Load registry JSON files
- Emit TypeScript bindings to generated/typescript/contracts.ts
- Emit Kotlin bindings to generated/kotlin/Contracts.kt
- Emit Rust bindings to generated/rust/contracts.rs
- Create output directories if they don't exist

### S009: Create tests directory and add pytest-based validation tests
Copy pytest tests from tooling bundle to `packages/contracts/tests/`:
- test_registry_shapes.py — validates registry JSON structure and required fields
- test_generated_typescript.py — verifies TS bindings
- test_generated_kotlin.py — verifies Kotlin bindings
- test_generated_rust.py — verifies Rust bindings

### S009-T: Test registry shape validation
Run pytest on test_registry_shapes.py to verify:
- Required fields present in all registries
- entityTypes has all four domain groups (core, guidance, knowledge, tracking)
- No duplicate values within any single registry
- Proper JSON structure and types

### S010-S012: Generate language bindings
Run generator script for each language. These tasks are parallelizable as they emit to separate directories:
- S010: `python3 scripts/generate_contracts.py --root .` creates TypeScript bindings
- S011: Same command creates Kotlin bindings
- S012: Same command creates Rust bindings

### S012-T: Test generated bindings
Run pytest to verify:
- TypeScript exports contain all ENTITY_TYPES, RELATION_TYPES, SYNC_STREAMS values
- Kotlin enums serialize to/from canonical string values correctly
- Rust enums derive serde and Serialize/Deserialize work

### S013: Add pyproject.toml
Create Python project configuration for the contracts package with pytest dependency.

### S014: Add requirements.txt
List pytest as the only runtime dependency for the Python code generation and tests.

### S015: Update .github/workflows/contracts.yml
Replace placeholder workflow with full implementation:
- Checkout code
- Set up Python 3
- Run generator script
- Run pytest tests
- Check git diff on packages/contracts/generated/
- Fail if diff finds changes (exit code 1)

### S016-S017: Configure CI enforcement
Ensure workflow:
- Triggers on pull_request with paths filter for contracts package
- Runs generator script from tooling bundle
- Runs all pytest tests
- Checks git diff for staleness

### S018-T: Test CI enforcement workflow
Trigger workflow via workflow_dispatch or create a test PR to verify:
- Generator runs successfully
- Tests pass
- Git diff check works (try making a change to registry without regenerating to verify failure)

### S019-D: Update root README
Add section on shared contracts with:
- Location of contracts package
- How to run generator locally
- Where generated bindings are imported for each platform
- Link to contracts spec doc

### S020-S023: Add contracts to platform configurations
Each platform needs to import generated bindings:
- TypeScript: Add path to tsconfig.json or import alias
- Android: Add source directory to Gradle sourceSets
- Rust server: Add contracts module as workspace dependency or path dependency
- Rust worker: Same as server

### S024-S026: Update documentation and conventions
- Backend docs: Reference canonical entity types in API examples
- PowerSync spec docs: Reference canonical stream names
- CLAUDE.md: Reinforce that shared contracts must be used for all cross-platform identifiers

## Milestone Checkpoints

### Milestone 1: Registry & Schemas Complete
Verify against testable assertions:
- **A-001:** Registry files exist in packages/contracts/registry/ with required structure
- **A-002:** Registry files contain all canonical values from shared contracts spec

Drift check: Confirm registry JSON files match altair-shared-contracts-spec.md exactly.

### Milestone 2: Code Generation Working
Verify against testable assertions:
- **A-003:** No duplicate identifiers within any single registry
- **A-004:** Running generator script creates bindings without errors
- **A-005:** Generated TypeScript exports constants and types for all registry values
- **A-006:** Generated Kotlin enums serialize/deserialize correctly
- **A-007:** Generated Rust enums derive serde and work

Drift check: Regenerate bindings and verify they match expected structure from tech plan.

### Milestone 3: CI Enforcement Complete
Verify against testable assertions:
- **A-008:** CI workflow runs generator and fails if git diff finds uncommitted changes
- **A-009:** CI workflow runs validation tests and fails on any failure
- **A-011:** Generated bindings are committed and readable in repo

Drift check: Create test PR, trigger workflow, verify it enforces correctly.

### Milestone 4: Feature Complete
Verify against remaining testable assertions:
- **A-010:** All pytest tests run locally
- **A-012:** Entity types cover all domains (core, guidance, knowledge, tracking)

Full drift check: Compare implementation against Spec.md requirements and Tech.md decisions.

## Open Question Dependency

**Note:** S020-S026 (platform integration tasks) depend on resolving Open Question #3 from Spec.md: "What is the acceptable workflow for adding new entity types or relation types?" This requires ADR-005 (Shared Contracts Governance Process) before first new identifier is needed. These tasks can be completed for initial identifiers, but governance should be documented before extending registries.

## Parallelizable Tasks

**Phase 2 tasks S010, S011, S012 are marked [P]** and can be run concurrently as they emit to separate directories and don't depend on each other.

**Phase 4 tasks S020, S021, S022, S023 are marked [P]** as they configure independent platforms.

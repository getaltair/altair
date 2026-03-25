# Feature 001: Shared Contracts Foundation

## Overview

Establish a shared contracts layer that provides canonical identifiers and schemas across all Altair platforms (web, desktop, Android, backend, server). This prevents string drift and identifier chaos before backend and clients proliferate, ensuring that entity types, relation types, sync stream names, and cross-platform data structures remain consistent and maintainable.

## Problem Statement

As multi-platform development proceeds, each codebase inevitably invents its own naming scheme for shared concepts (entity types, relation types, sync streams). Without a source of truth, this leads to:

- String drift: "initiative" vs "Initiative" vs "INITIATIVE" across services
- Synchronization mismatches: sync stream names diverge between PowerSync config and client subscriptions
- Integration pain: AI pipelines and indexing services must hardcode mappings to multiple naming conventions
- Maintenance burden: Changing an identifier requires hunting through multiple codebases to find all occurrences

The foundation for shared contracts must be established early, before backend APIs and client logic proliferate, otherwise fixing identifier mismatches becomes prohibitively expensive.

## User Stories

- As a **platform developer**, I want canonical entity types and relation types so that I don't need to guess what string to use when creating relations between entities across different services.
- As a **backend developer**, I want validated sync stream identifiers so that PowerSync configuration references the same names used in client subscription code.
- As an **Android developer**, I want generated Kotlin constants for entity types so that I don't introduce a typo when creating relation records from native code.
- As a **frontend developer**, I want TypeScript type safety for relation records and sync streams so that refactoring doesn't silently break contract compatibility.
- As a **devops engineer**, I want CI enforcement that prevents developers from adding new magic strings outside the contracts package, ensuring long-term maintainability.

## Requirements

### Must Have

- **Registry JSON files** containing canonical identifiers for:
  - Entity types (core, guidance, knowledge, tracking domains)
  - Relation types (references, supports, requires, etc.)
  - Relation source types (user, AI, import, rule, migration, system)
  - Relation status types (accepted, suggested, dismissed, rejected, expired)
  - Sync stream names (auto-subscribed and on-demand streams)
  - Attachment processing states (pending, uploaded, processing, ready, failed, deleted)

- **Shared schema definitions** for cross-platform data structures:
  - `RelationRecord` schema with required fields (id, from, to, relationType, sourceType, status, timestamps)
  - `AttachmentRecord` schema for attachment metadata
  - `EntityRef` schema for referencing entities by type and ID

- **Generated language bindings** from registry JSON:
  - TypeScript constants and types (consts + value types)
  - Kotlin enums with serialization support
  - Rust enums with serde derive macros

- **Code generation script** that:
  - Reads registry JSON files
  - Emits language-specific bindings without manual edits
  - Creates output directories if needed
  - Provides clear error messages for missing or malformed registry files

- **Validation tests** that:
  - Validate registry JSON structure and required fields
  - Ensure no duplicate values within each registry
  - Verify generated bindings contain all canonical values from registries
  - Run successfully in both local development and CI

- **CI enforcement** that:
  - Runs code generation on every PR
  - Fails if generated files are not up-to-date (git diff detection)
  - Runs validation tests as part of CI pipeline
  - Prevents merging PRs that add new magic strings outside contracts

- **Documentation** that:
  - Explains source-of-truth policy (registry JSON is canonical)
  - Documents how to run the generator script
  - Provides examples of how to import and use generated bindings
  - Notes what belongs in contracts vs. what doesn't (stable identifiers only, no business logic)

### Should Have

- **Optional `SyncSubscriptionRequest` schema** for on-demand stream subscription requests
- **README** in contracts package with quick-start examples for each platform
- **Generated bindings checked into repo** (not generated at build time) to ensure visibility and stability
- **Clear separation** between `registry/` (source of truth) and `generated/` (derived artifacts)

### Won't Have (this iteration)

- Business logic or platform-specific behavior in contracts package
- Repository implementations or database queries in contracts
- Every API payload standardized as a mega-contract blob (only DTOs that are truly cross-platform and stable)
- Automatic schema migration or versioning tools (manual updates acceptable for initial phase)

## Testable Assertions

| ID    | Assertion                                                                 | Verification                                                        |
|-------|-----------------------------------------------------------------------------|-------------------------------------------------------------------|
| A-001 | Registry JSON files exist in `packages/contracts/registry/` with required structure | File existence check and JSON schema validation |
| A-002 | Registry files contain all canonical values approved in shared contracts spec | Values match altair-shared-contracts-spec.md exactly |
| A-003 | No duplicate identifiers exist within any single registry file | Duplicate detection test passes for all registries |
| A-004 | Running generator script creates TypeScript, Kotlin, and Rust bindings without errors | `python scripts/generate_contracts.py --root packages/contracts` completes successfully |
| A-005 | Generated TypeScript exports constants and types for all registry values | Import and type check against generated/contracts/typescript/contracts.ts |
| A-006 | Generated Kotlin enums serialize to/deserialize from canonical string values | Unit test for fromWire/serialization passes |
| A-007 | Generated Rust enums derive serde and can be serialized/deserialized | Unit test for serde Serialize/Deserialize passes |
| A-008 | Validation tests run locally with `pytest` or `vitest` | All tests in packages/contracts/tests/ pass |
| A-009 | CI workflow runs generator and fails if git diff finds uncommitted changes | CI job creates fresh generation, runs git diff, fails on non-zero exit |
| A-010 | CI workflow runs validation tests and fails on any failure | Validation tests run in CI and job fails if any test fails |
| A-011 | Generated bindings are committed and readable in repo | Generated files exist in packages/contracts/generated/ with proper permissions |
| A-012 | Entity types cover all domains: core, guidance, knowledge, tracking | Generated ENTITY_TYPES includes values from all four domain groups |

## Open Questions

- [ ] Should generated bindings be re-generated in CI or only checked for staleness? (Current plan: generate in CI to catch drift, but commit generated files to repo)
- [ ] Should the contracts package have its own workspace configuration or rely on root workspace? (Likely needs its own for clarity)
- [ ] What is the acceptable workflow for adding new entity types or relation types? (ADR may be needed to formalize this)

## Revision History

| Date       | Change                    | ADR  |
|------------|----------------------------|------|
| 2026-03-24 | Initial spec               | —    |

## Related Documents

- Altair Phase 0–4 Execution Checklist (Phase 1 tasks P1-001 through P1-007)
- Altair Shared Contracts Specification
- Altair Architecture Spec

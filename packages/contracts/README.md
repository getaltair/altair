# Altair Shared Contracts

**This package is the source of truth for cross-platform identifiers and schemas.**

## Purpose

Keep these systems aligned with consistent naming:

- Backend (Rust)
- Android app (Kotlin)
- Web app (TypeScript/Svelte)
- Desktop app (Tauri/TypeScript)
- Search/indexing pipelines
- AI enrichment jobs
- Sync configuration (PowerSync)

> Without a shared contracts layer, every codebase eventually invents its own slightly cursed naming scheme.

## What Belongs Here

### Include

- Entity type identifiers
- Relation type identifiers
- Relation source/status identifiers
- Sync stream names
- Attachment kinds / processing states
- Shared DTO schemas (truly cross-platform only)
- Validation-friendly schema definitions
- Generated language bindings

### Do Not Include

- UI logic
- Platform-specific behavior
- Repository implementations
- Database queries
- Backend-only service internals

## Structure

```text
contracts/
  registry/
    entity-types.json      # Canonical entity type identifiers
    relation-types.json    # Canonical relation type identifiers
    sync-streams.json      # Canonical PowerSync stream names
  schemas/                 # JSON Schema definitions (planned)
  generated/               # Generated bindings (planned)
    typescript/
    kotlin/
    rust/
```

## Registry-First Approach

Canonical registries live in machine-readable JSON files under `registry/`.

Language-specific constants are generated from these registries (or manually synchronized in early phases).

### Rule

> If a string is used in more than one codebase, it should not be invented inline.

## Current Registries

| Registry | File | Description |
|----------|------|-------------|
| Entity Types | `registry/entity-types.json` | Core, guidance, knowledge, tracking entities |
| Relation Types | `registry/relation-types.json` | Cross-entity relationship identifiers |
| Sync Streams | `registry/sync-streams.json` | PowerSync subscription stream names |

## Versioning

### Additive (allowed in minor versions)

- Add new entity/relation types
- Add new stream names
- Add optional fields to DTO schemas

### Breaking (requires migration)

- Rename/remove existing identifiers
- Change semantic meaning
- Repurpose stream names

## More Information

See [altair-shared-contracts-spec.md](../../docs/altair-shared-contracts-spec.md) for full specification.

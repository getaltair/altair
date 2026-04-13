# packages/contracts — Shared Contract Registry

This package is the single source of truth for cross-platform data contracts in Altair. It defines the canonical identifiers and schema definitions shared across all clients (web, Android, server), ensuring consistency in entity types, relation types, and sync configuration without duplication or drift.

## Registry Files

- **`entity-types.json`** — Canonical list of all 18 entity type identifiers. Source for all language bindings.
- **`relation-types.json`** — Canonical list of all 8 relation type identifiers used in the knowledge graph.
- **`sync-streams.json`** — PowerSync sync stream name constants (v1; marked provisional — Step 4 may revise).

Each registry follows this structure:

```json
{
  "contracts_version": "1.0.0",
  "values": [
    { "id": "identifier", "description": "Human-readable description" }
  ]
}
```

## Language Binding Locations

| Stack | File(s) |
|---|---|
| TypeScript (Web) | `apps/web/src/lib/contracts/entityTypes.ts`, `relationTypes.ts`, `syncStreams.ts`, `dtos.ts` |
| Kotlin (Android) | `apps/android/app/src/main/java/com/getaltair/altair/contracts/EntityType.kt`, `RelationType.kt`, `SyncStream.kt`, `Dtos.kt` |
| Rust (Server) | `apps/server/server/src/contracts.rs` |

Import path examples:
- TypeScript: `import { EntityType } from '$lib/contracts'`
- Kotlin: `import com.getaltair.altair.contracts.EntityType`
- Rust: `use crate::contracts::EntityType`

## Validation Script

The CI validation script checks that all three language bindings match their source registries.

**Run locally:**
```bash
bun run packages/contracts/scripts/validate.ts
```

Exit 0 = bindings are in sync. Exit 1 = drift detected with a human-readable diff.

## How to Add a New Entity Type (or Relation Type)

1. Add an entry to the relevant JSON registry (`entity-types.json` or `relation-types.json`)
2. Add the corresponding constant to all three language bindings:
   - TypeScript: add to `apps/web/src/lib/contracts/entityTypes.ts` (or `relationTypes.ts`)
   - Kotlin: add to `apps/android/app/src/main/java/com/getaltair/altair/contracts/EntityType.kt` (or `RelationType.kt`)
   - Rust: add a variant with `#[serde(rename = "new_id")]` to `contracts.rs`
3. Run `bun run packages/contracts/scripts/validate.ts` — must exit 0
4. Open a PR; CI will re-run the validation script automatically

**Missing a binding file?** The CI `validate-contracts` job will fail with a diff showing which values are missing from which stack.

## Versioning Rules

- **Additive changes** (new entity/relation type): increment minor version (`1.0.0` → `1.1.0`)
- **Renames or removals**: breaking change — increment major version and coordinate a cross-stack migration
- `contracts_version` field in each registry tracks the version for CI tooling and future code generation

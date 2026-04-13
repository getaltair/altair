# Tech Plan: Shared Contracts

**Spec:** `Context/Features/002-SharedContracts/Spec.md`
**Stacks involved:** TypeScript/SvelteKit, Kotlin/Android, Rust/Axum, GitHub Actions CI

---

## Architecture Overview

Shared Contracts is a pure infrastructure step. It produces no user-facing behaviour — it creates the compile-time safety net that all subsequent steps rely on. The output is three JSON registries in `packages/contracts/` and hand-written language bindings written into each app's source tree.

```
packages/contracts/
├── entity-types.json
├── relation-types.json
├── sync-streams.json
└── scripts/
    └── validate.ts          ← Bun script; run by CI

apps/web/src/lib/contracts/
├── entityTypes.ts
├── relationTypes.ts
├── syncStreams.ts
└── dtos.ts                  ← EntityRef, RelationRecord, AttachmentRecord, SyncSubscriptionRequest

apps/android/app/src/main/java/com/getaltair/altair/contracts/
├── EntityType.kt
├── RelationType.kt
├── SyncStream.kt
└── Dtos.kt

apps/server/server/src/
└── contracts.rs             ← All enums + DTOs in one module, re-exported from mod
```

The CI validation script is the enforcement mechanism. It reads each JSON registry and checks every value appears in the corresponding binding file (all three stacks). A missing or extra value exits 1.

---

## Key Decisions

### Decision 1: TypeScript binding shape — `as const` object vs. string enum

**Options considered:**
- **Option A: String enum** (`enum EntityType { User = 'user', ... }`) — native TypeScript construct, exhaustive switch possible.
  - Downside: TypeScript string enums are not structurally compatible with plain strings; passing `'user'` where `EntityType` is expected fails. Forces callers to import the enum rather than using string literals. Harder to iterate values at runtime.
- **Option B: `as const` object with derived union type** (`export const EntityType = { User: 'user', ... } as const; export type EntityTypeValue = typeof EntityType[keyof typeof EntityType];`) — values are plain strings at runtime; the type is a string literal union. IDE autocomplete works. Iteration via `Object.values(EntityType)` works directly.

**Chosen:** Option B — `as const` object

**Rationale:** PowerSync subscriptions and serialized JSON both deal in plain strings. The `as const` pattern gives the type-safety benefit without the string incompatibility problem. It also makes the CI validation script straightforward: `Object.values(EntityType)` returns the exact string set to compare against the registry.

---

### Decision 2: Kotlin binding shape — enum class vs. sealed class

**Options considered:**
- **Option A: `enum class EntityType(val value: String)`** — straightforward, iterable via `entries`, `when` exhaustiveness checking works natively. `fromValue()` companion lookup is trivial.
- **Option B: Sealed class with object subclasses** — more idiomatic for complex ADTs, but overkill for string constants with no associated data variants.

**Chosen:** Option A — `enum class` with a `value: String` constructor parameter

**Rationale:** These are string constants with no variant-specific data. Enums are iterable via `EntityType.entries`, exhaustive `when` works at compile time, and a companion `fromValue(s: String)` method provides safe lookup. Sealed classes add complexity with no benefit here.

---

### Decision 3: Rust binding shape — `rename_all` vs. per-variant `rename`

**Options considered:**
- **Option A: `#[serde(rename_all = "snake_case")]`** — works cleanly when all variant names map uniformly to snake_case strings. Fails here because entity types have domain prefixes: `GuidanceEpic` → `guidance_epic` works, but `User` → `user` and `TrackingItem` → `tracking_item` diverge in prefix structure. The Rust variant name would have to match the full registry string, e.g., `GuidanceEpic` for `"guidance_epic"` — which works with `rename_all = "snake_case"`. Actually usable.
- **Option B: Per-variant `#[serde(rename = "guidance_epic")]`** — explicit, no naming ambiguity, resistant to future rename-all policy changes. Slightly more verbose.

**Chosen:** Option B — per-variant `#[serde(rename = "...")]`

**Rationale:** Per-variant renames make the mapping from Rust variant to registry string explicit and readable in the source file. This also makes the CI validation script reliable: it can grep for `rename = "value"` strings without having to simulate Rust's naming transformation logic. The verbosity is acceptable for a ~20-entry enum.

**Serde is already a dependency** (`serde = { version = "1.0.228", features = ["derive"] }` in `server/Cargo.toml`). No new dependencies needed.

---

### Decision 4: UUID representation in DTOs across stacks

**Options considered:**
- **Option A: String everywhere** — UUIDs as `String` (TypeScript/Kotlin) and `String` (Rust). Simple cross-platform serialization; no special types needed.
- **Option B: Native UUID types** — `string` (TS), `String` (Kotlin with format doc), `uuid::Uuid` (Rust). Rust already has `uuid` crate; TypeScript and Kotlin treat UUIDs as strings naturally.

**Chosen:** Option B — native `uuid::Uuid` in Rust; `string` in TypeScript; `String` in Kotlin

**Rationale:** The `uuid` crate is already in `server/Cargo.toml`. Using `uuid::Uuid` in Rust DTOs provides parse-time validation and prevents raw string confusion. TypeScript and Kotlin use `string` since both serialize UUIDs as strings for JSON/PowerSync. A JSDoc/KDoc comment documents the UUID constraint.

---

### Decision 5: CI validation approach — dynamic import vs. text parsing

**Options considered:**
- **Option A: Parse binding files as text** — regex/grep for string literals in each file. Simple, no build step needed. Slightly fragile if formatting changes (e.g., multiline string).
- **Option B: Dynamic import of TypeScript bindings** — `import * as EntityTypes from '...'` in the validation script, then compare `Object.values()`. Reliable for TypeScript. Kotlin and Rust bindings still require text parsing.

**Chosen:** Hybrid — dynamic import for TypeScript; text parsing for Kotlin and Rust

**Rationale:** For TypeScript, dynamic import is exact and immune to formatting. For Kotlin and Rust, the binding files follow a fixed pattern: Kotlin enums list `EnumVariant("registry_value")` and Rust uses `#[serde(rename = "registry_value")]`. Extracting quoted strings from these patterns is reliable given the disciplined structure of hand-written binding files. The validation script is a Bun TypeScript script at `packages/contracts/scripts/validate.ts`.

---

## Stack-Specific Details

### Rust (`apps/server/server/src/`)

**Files to create:**
- `contracts.rs` — single module containing `EntityType`, `RelationType`, `SyncStream` enums and `EntityRef`, `RelationRecord`, `AttachmentRecord`, `SyncSubscriptionRequest` structs

**Add to `main.rs`:**
```rust
mod contracts;
pub use contracts::*;
```

**Enum shape:**
```rust
// Source of truth: packages/contracts/entity-types.json
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "user")]           User,
    #[serde(rename = "household")]      Household,
    #[serde(rename = "initiative")]     Initiative,
    #[serde(rename = "tag")]            Tag,
    #[serde(rename = "attachment")]     Attachment,
    #[serde(rename = "guidance_epic")]         GuidanceEpic,
    #[serde(rename = "guidance_quest")]        GuidanceQuest,
    // ... all 18 variants
}
```

**DTO shapes** (field names snake_case matching the ERD column names):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRef {
    pub entity_type: EntityType,
    pub entity_id: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationRecord {
    pub id: uuid::Uuid,
    pub from_entity_type: EntityType,
    pub from_entity_id: uuid::Uuid,
    pub to_entity_type: EntityType,
    pub to_entity_id: uuid::Uuid,
    pub relation_type: RelationType,
    pub source_type: String,
    pub status: String,
    pub confidence: Option<f64>,
    pub evidence: Option<String>,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

**Dependencies:** None new. `serde`, `uuid`, `chrono` already present in `server/Cargo.toml`.

**Patterns to follow:** `.claude/rules/rust-axum.md` — module structure, no `unwrap()`, `derive` macros.

---

### TypeScript (`apps/web/src/lib/contracts/`)

**Files to create:**
- `entityTypes.ts`
- `relationTypes.ts`
- `syncStreams.ts`
- `dtos.ts`
- `index.ts` — barrel re-export

**Binding shape:**
```typescript
// Source of truth: packages/contracts/entity-types.json
export const EntityType = {
  User:           'user',
  Household:      'household',
  Initiative:     'initiative',
  Tag:            'tag',
  Attachment:     'attachment',
  GuidanceEpic:         'guidance_epic',
  GuidanceQuest:        'guidance_quest',
  // ... all 18
} as const;

export type EntityTypeValue = typeof EntityType[keyof typeof EntityType];
```

**DTO shapes:**
```typescript
export interface EntityRef {
  entity_type: EntityTypeValue;
  entity_id: string; // UUID
}

export interface RelationRecord {
  id: string;
  from_entity_type: EntityTypeValue;
  from_entity_id: string;
  to_entity_type: EntityTypeValue;
  to_entity_id: string;
  relation_type: RelationTypeValue;
  source_type: string;
  status: string;
  confidence: number | null;
  evidence: string | null;
  user_id: string;
  created_at: string; // ISO 8601
  updated_at: string;
  deleted_at: string | null;
}
```

**Import path:** `import { EntityType } from '$lib/contracts'`

**Patterns to follow:** `.claude/rules/svelte.md` — `$lib/` alias, barrel exports via `index.ts`, strict TypeScript.

---

### Kotlin (`apps/android/app/src/main/java/com/getaltair/altair/contracts/`)

**Files to create:**
- `EntityType.kt`
- `RelationType.kt`
- `SyncStream.kt`
- `Dtos.kt`

**Binding shape:**
```kotlin
// Source of truth: packages/contracts/entity-types.json
enum class EntityType(val value: String) {
    USER("user"),
    HOUSEHOLD("household"),
    INITIATIVE("initiative"),
    TAG("tag"),
    ATTACHMENT("attachment"),
    GUIDANCE_EPIC("guidance_epic"),
    GUIDANCE_QUEST("guidance_quest"),
    // ... all 18

    companion object {
        fun fromValue(value: String): EntityType? =
            entries.find { it.value == value }
    }
}
```

**DTO shapes** (using `@SerializedName` for JSON field mapping with Gson, or `@Json` for Moshi):
```kotlin
data class EntityRef(
    val entityType: String,   // EntityType.value
    val entityId: String      // UUID string
)
```

**Note:** The Android project currently has no DI framework or JSON library set up beyond the scaffold. The DTO serialization annotations (`@SerializedName` vs `@Json`) depend on which JSON library is chosen during Step 3 (Server Core) when the API client is built. For Step 2, define DTOs as plain `data class` without annotations — annotations are added in Step 8 (Android Client).

**Patterns to follow:** `.claude/rules/kotlin-android.md` — `val` over `var`, no `!!`, packages lowercase.

---

## Registry Structure

Each registry is a JSON object with a `version` field and a `values` array. Each entry has an `id` (the canonical string) and a `description`.

```json
{
  "contracts_version": "1.0.0",
  "values": [
    { "id": "user",           "description": "Platform user account" },
    { "id": "household",      "description": "Shared household space" },
    { "id": "initiative",     "description": "High-level goal or project" },
    { "id": "tag",            "description": "User-defined label" },
    { "id": "attachment",     "description": "File attached to an entity" },
    { "id": "guidance_epic",  "description": "Epic: a grouped set of quests within an initiative" },
    { "id": "guidance_quest", "description": "Quest: a single actionable task" },
    ...
  ]
}
```

**Provisional sync stream names** for `sync-streams.json` (Step 4 may revise):

| Stream ID      | Scope                                          |
|----------------|------------------------------------------------|
| `user_data`    | User's own Core entities (initiatives, tags)   |
| `household`    | Household-scoped data (membership, locations)  |
| `guidance`     | Guidance domain tables                         |
| `knowledge`    | Knowledge domain tables                        |
| `tracking`     | Tracking domain tables                         |

---

## Integration Points

The contracts module has no runtime integration in Step 2 — the bindings are imported by other modules but no API calls or database writes happen here. The integration point is the CI validation script:

```
packages/contracts/scripts/validate.ts
  reads → entity-types.json, relation-types.json, sync-streams.json
  checks → apps/web/src/lib/contracts/{entityTypes,relationTypes,syncStreams}.ts
  checks → apps/android/.../contracts/{EntityType,RelationType,SyncStream}.kt
  checks → apps/server/server/src/contracts.rs
  exits 1 if any binding is missing a registry value or has an extra value
```

The CI workflow (`.github/workflows/ci.yml`) gets a new job `validate-contracts` that runs `bun run packages/contracts/scripts/validate.ts` and is declared as a dependency of the existing `smoke-test` job.

**Assertion A-013 dependency:** Step 3 (Server Core) adds the actual HTTP handler that calls `EntityType::from_str()` or match-validates the incoming entity type. That code can reference `contracts::EntityType` from the module added here.

---

## Risks & Unknowns

- **Risk:** Sync stream names defined here conflict with the bucket names chosen in Step 4.
  - **Mitigation:** Treat `sync-streams.json` as provisional (noted in a comment in the file). When Step 4 finalizes bucket names, update the registry and bindings together — the CI validation script catches any binding left behind.

- **Risk:** Kotlin DTO serialization annotations need to be chosen (Gson vs. Moshi vs. kotlinx.serialization) before Android client work begins in Step 8.
  - **Mitigation:** DTOs are defined as plain `data class` in Step 2. The JSON library decision is deferred to Step 8 (Android Client); annotations are added then. This avoids a premature dependency on a library that may be revisited.

- **Risk:** `RelationRecord` and `AttachmentRecord` contain `status` and `source_type` as raw `String` — not typed to their enum values — because those enums (`RelationStatus`, `AttachmentState`, etc.) are domain-specific and belong in Step 3/8 domain modules, not the cross-platform contracts layer.
  - **Mitigation:** Accepted tradeoff. The contracts layer owns the *identifier registries*; domain-specific status enums live in their respective domain modules. Document this boundary in `contracts.rs`.

---

## Testing Strategy

- **Unit tests in Rust** (`#[cfg(test)]` in `contracts.rs`): round-trip serde tests — serialize an `EntityType` variant to JSON string, deserialize back, assert equality. One test per enum is sufficient.
- **Unit tests in TypeScript** (co-located `contracts.spec.ts`): assert `Object.values(EntityType)` has the expected count and contains specific known values.
- **CI validation script** (`validate.ts`): this is the primary consistency test. It runs on every push; failure means a binding drifted from the registry.
- **No Kotlin unit tests in Step 2** — the enum is trivially correct; tested implicitly when the Android project compiles (`./gradlew assembleDebug`).

---

## Revision History

| Date       | Change       | ADR |
|------------|--------------|-----|
| 2026-04-12 | Initial tech plan | —   |

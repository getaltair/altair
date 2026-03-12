# Altair Shared Contracts Specification

| Field | Value |
|---|---|
| **Document** | Altair Shared Contracts Specification |
| **Version** | 1.0 |
| **Status** | Draft |
| **Last Updated** | 2026-03-11 |

---

# 1. Purpose

This document defines the **shared contracts layer** for Altair.

The goal is to keep these systems aligned:

- backend
- Android app
- web app
- desktop app
- search/indexing pipelines
- AI enrichment jobs
- sync configuration

Without a shared contracts layer, every codebase eventually invents its own slightly cursed naming scheme.

---

# 2. What belongs in shared contracts

The shared contracts package should contain **stable identifiers and schemas**, not business logic.

## Include

- entity type identifiers
- relation type identifiers
- relation source/status identifiers
- sync stream names
- attachment kinds / processing states
- shared DTO schemas where they are truly cross-platform
- validation-friendly schema definitions
- generated language bindings where practical

## Do not include

- UI logic
- platform-specific behavior
- repository implementations
- database queries
- backend-only service internals

---

# 3. Monorepo Layout

Recommended layout:

```text
altair/
  packages/
    contracts/
      registry/
        entity-types.json
        relation-types.json
        sync-streams.json
      schemas/
        relation-record.schema.json
        attachment-record.schema.json
      generated/
        typescript/
        kotlin/
        rust/
      src/
        typescript/
        kotlin/
        rust/
      README.md
```

---

# 4. Source of Truth Strategy

Use a **registry-first** approach.

Canonical registries should live in machine-readable files:

- `entity-types.json`
- `relation-types.json`
- `sync-streams.json`

Language-specific constants are generated from those registries or kept manually synchronized in the early phase.

## Rule

If a string is used in more than one codebase, it should not be invented inline.

---

# 5. Entity Types

These are used in:

- `entity_relations`
- search documents
- AI enrichment outputs
- analytics annotations
- automation rules

## Canonical entity types

### Core
- `user`
- `household`
- `initiative`
- `tag`
- `attachment`

### Guidance
- `guidance_epic`
- `guidance_quest`
- `guidance_routine`
- `guidance_focus_session`
- `guidance_daily_checkin`

### Knowledge
- `knowledge_note`
- `knowledge_note_snapshot`

### Tracking
- `tracking_location`
- `tracking_category`
- `tracking_item`
- `tracking_item_event`
- `tracking_shopping_list`
- `tracking_shopping_list_item`

---

# 6. Relation Types

Starter relation types:

- `references`
- `supports`
- `requires`
- `related_to`
- `depends_on`
- `duplicates`
- `similar_to`
- `generated_from`

These should remain stable and broadly meaningful across domains.

Do not create five different spellings of “related to” because one service was feeling artistic.

---

# 7. Relation Source Types

- `user`
- `ai`
- `import`
- `rule`
- `migration`
- `system`

---

# 8. Relation Status Types

- `accepted`
- `suggested`
- `dismissed`
- `rejected`
- `expired`

---

# 9. Sync Stream Names

Canonical stream identifiers:

## Auto-subscribed
- `my_profile`
- `my_memberships`
- `my_personal_data`
- `my_household_data`
- `my_relations`
- `my_attachment_metadata`

## On-demand
- `initiative_detail`
- `note_detail`
- `item_history`
- `quest_detail`

These names should be reused consistently in:

- PowerSync config
- client subscriptions
- sync debugging/logging
- metrics

---

# 10. Attachment Processing States

Starter states:

- `pending`
- `uploaded`
- `processing`
- `ready`
- `failed`
- `deleted`

---

# 11. Versioning Rules

## Additive change
Allowed in minor versions:
- add new entity types
- add new relation types
- add new stream names
- add optional fields to DTO schemas

## Breaking change
Requires deliberate migration:
- rename existing identifiers
- remove existing identifiers
- change semantic meaning of an existing identifier
- repurpose a stream name for a different scope

## Rule of sanity
Treat identifiers as API, not as implementation trivia.

---

# 12. Suggested Generated Artifacts

## TypeScript
- `entityTypes.ts`
- `relationTypes.ts`
- `syncStreams.ts`
- `contracts.ts`

## Kotlin
- `EntityType.kt`
- `RelationType.kt`
- `SyncStream.kt`
- `Contracts.kt`

## Rust
- `entity_type.rs`
- `relation_type.rs`
- `sync_stream.rs`
- `mod.rs`

---

# 13. DTOs worth standardizing early

Only standardize DTOs that are truly cross-platform and stable enough.

Good candidates:

- `RelationRecord`
- `AttachmentRecord`
- `SyncSubscriptionRequest`
- `EntityRef`

Avoid prematurely centralizing every API payload into one mega-contract blob. That way lies sadness.

---

# 14. Initial Contract Examples

## EntityRef
- `entityType`
- `entityId`

## RelationRecord
- `id`
- `from`
- `to`
- `relationType`
- `sourceType`
- `status`
- `confidence`
- `evidence`
- `createdAt`
- `updatedAt`

## SyncSubscriptionRequest
- `stream`
- `parameters`

---

# 15. Enforcement Strategy

Use these guardrails:

1. no inline magic strings for shared identifiers
2. lint/tests should validate registry values against language bindings
3. backend write paths should reject unknown entity or relation types
4. AI pipelines must map outputs to canonical registry values
5. PowerSync config changes should reference canonical stream names

---

# 16. Recommended Next Step

After introducing this contracts layer:

1. wire constants into backend and clients
2. add schema validation tests
3. add a tiny codegen script to emit TypeScript/Kotlin/Rust constants from the JSON registries
4. update PowerSync config to reference the canonical stream names in docs and app code

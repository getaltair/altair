# Altair Entity Type Registry

> **Source of truth:** `packages/contracts/registry/`
> **Generated bindings:** `packages/contracts/generated/`

Use these exact identifiers in:
- `entity_relations.from_entity_type`
- `entity_relations.to_entity_type`
- search documents
- AI enrichment outputs
- automation rules
- analytics/event annotations

**Import from contracts package, never hardcode:**

```typescript
// CORRECT
import { ENTITY_TYPES } from '@altair/contracts';

const type = ENTITY_TYPES.KNOWLEDGE_NOTE;

// WRONG
const entityType = 'knowledge_note';  // Hardcoded string
```

---

## Core
- `user`
- `household`
- `initiative`
- `tag`
- `attachment`

## Guidance
- `guidance_epic`
- `guidance_quest`
- `guidance_routine`
- `guidance_focus_session`
- `guidance_daily_checkin`

## Knowledge
- `knowledge_note`
- `knowledge_note_snapshot`

## Tracking
- `tracking_location`
- `tracking_category`
- `tracking_item`
- `tracking_item_event`
- `tracking_shopping_list`
- `tracking_shopping_list_item`

---

## Relationship types (starter set)
- `references`
- `supports`
- `requires`
- `related_to`
- `depends_on`
- `duplicates`
- `similar_to`
- `generated_from`

---

## Source types
- `user`
- `ai`
- `import`
- `rule`
- `migration`
- `system`

---

## Status values
- `accepted`
- `suggested`
- `dismissed`
- `rejected`
- `expired`

---

## Attachment processing states
- `pending`
- `uploaded`
- `processing`
- `ready`
- `failed`
- `deleted`

---

## Rules
1. Never invent ad hoc entity type strings in app code.
2. Never use UI labels as entity types.
3. Add new values only through `packages/contracts`.
4. Keep old values stable for backward compatibility.

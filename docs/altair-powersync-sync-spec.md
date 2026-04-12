# Altair PowerSync Sync Scope Specification

| Field | Value |
|---|---|
| **Document** | Altair PowerSync Sync Scope Specification |
| **Version** | 1.0 |
| **Status** | Draft |
| **Last Updated** | 2026-03-11 |

---

# 1. Purpose

This document defines the initial PowerSync sync design for Altair.

It covers:

- sync scope boundaries
- stream design
- table replication strategy
- entity type registry usage
- practical guidance for PowerSync-friendly data shaping
- a starter development seed dataset

This document assumes the selected architecture:

- PostgreSQL as source of truth
- SQLite on clients
- PowerSync as sync layer
- Android + desktop as strong offline clients
- web as first-class access surface

---

# 2. Why Sync Streams

Altair should use **PowerSync Sync Streams** rather than the older Sync Rules format.

Reasons:

- Sync Streams are the current format for new work
- They support `auto_subscribe: true` for always-on streams
- They support `subscription.parameter(...)` for on-demand streams
- They support grouping multiple compatible queries in one stream
- They simplify migration from older bucket-based rules

---

# 3. Core Design Rule

Sync data by **clear scope boundaries**, not by arbitrary graph traversal.

Good scopes:

- user
- household
- initiative
- selected detail views

Bad scopes:

- “everything two hops away from anything semantically related to this note”

If you make sync scope depend on graph spelunking, the goblin wins.

---

# 4. Scope Model

## 4.1 Auto-subscribed baseline streams

These sync automatically when the client connects.

### A. user_core
Personal records needed on almost every device session.

Includes:
- current user profile row
- household memberships for the user
- user-owned initiatives
- user-owned tags
- user-owned routines
- assigned quests

### B. household_core
Shared records for households the user belongs to.

Includes:
- household rows
- household memberships
- household initiatives
- household tags
- shared chores / quests
- shared routines
- tracking locations
- tracking categories
- tracking items
- active shopping lists
- shopping list items

### C. relation_core
Relationship records scoped to user-visible initiatives and households.

Includes:
- entity_relations for synced households
- entity_relations for synced initiatives
- optionally accepted/suggested statuses only

### D. attachment_metadata
Attachment metadata only.

Includes:
- attachment rows for synced user/household/initiative entities
- no binary blobs

## 4.2 On-demand streams

These are subscribed when the user navigates into a deeper context.

### A. initiative_detail
Includes:
- epics for initiative
- quests for initiative
- notes for initiative
- item rows directly linked to initiative

### B. note_detail
Includes:
- a specific note
- snapshots for a specific note
- note tags
- note attachments
- note-scoped relationship edges

### C. item_history
Includes:
- a specific item
- recent item events
- item attachments
- item tags
- item-related relationship edges

### D. quest_detail
Includes:
- a specific quest
- tags
- attachments
- related relationships
- optional focus sessions tied to that quest

---

# 5. Replication Strategy by Table

## 5.1 Auto-subscribed

Recommended for automatic sync:

- users (limited self row only)
- households
- household_memberships
- initiatives
- tags
- attachments
- entity_relations
- guidance_quests
- guidance_routines
- tracking_locations
- tracking_categories
- tracking_items
- tracking_shopping_lists
- tracking_shopping_list_items

## 5.2 Usually on-demand

Recommended for detail subscriptions:

- guidance_epics
- knowledge_notes
- note_tags
- item_tags
- initiative_tags
- quest_tags
- note_attachments
- item_attachments
- quest_attachments

## 5.3 Large / noisy / selective

Replicate cautiously:

- knowledge_note_snapshots
- tracking_item_events
- guidance_focus_sessions
- guidance_daily_checkins

These tables can grow quickly or be too context-specific to auto-sync broadly.

---

# 6. Entity Type Registry

`entity_relations` depends on a stable registry of entity type names.

Use canonical lowercase identifiers and never improvise them ad hoc in app code.

## 6.1 Initial entity type registry

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

## 6.2 Guardrails

- store registry constants in one shared contract package
- use the same values in:
  - backend
  - Android
  - web/desktop
  - search/index pipelines
  - AI enrichment jobs

- do not encode UI labels as entity types
- do not let AI invent new entity types at write time

---

# 7. Authorization and Shape Rules

Every stream query must be filtered by real access boundaries.

For Altair, acceptable access checks are based on:

- `auth.user_id()`
- household membership
- initiative ownership
- initiative household visibility
- direct record ownership

Do not rely only on subscription parameters. A client can ask for any parameter value; the stream query must still authorize the data.

---

# 8. Starter Stream Layout

## 8.1 Auto-subscribed
- `my_profile`
- `my_memberships`
- `my_personal_data`
- `my_household_data`
- `my_relations`
- `my_attachment_metadata`

## 8.2 On-demand
- `initiative_detail`
- `note_detail`
- `item_history`
- `quest_detail`

This balances:

- fast initial sync
- good offline baseline
- controlled data volume
- optional deep-context hydration

---

# 9. Practical Query Design Rules

1. Keep each stream aligned to one obvious scope.
2. Prefer repeated straightforward queries over one cursed mega-query.
3. Use initiative or household scope as the common join key whenever possible.
4. Avoid making relation traversal the main sync selection mechanism.
5. Sync the relation rows that belong to the already-authorized scopes.

---

# 10. Seed Dataset Goals

The initial dev seed dataset should prove these workflows:

## Personal
- one user
- one personal initiative
- one personal note
- one personal routine

## Shared household
- one household
- two members
- one shared shopping list
- one shared chore quest
- a few shared tracking items

## Cross-domain relationships
- note references item
- quest requires item
- note supports initiative
- item related_to note

## Event history
- a few item events
- a note snapshot
- one completed quest

If the app can survive this seed data without becoming weird, that is a decent omen.

---

# 11. Next Implementation Steps

1. wire the entity type registry into backend constants
2. implement the starter seed dataset
3. stand up PowerSync with the starter stream config
4. test:
   - personal offline sync
   - household shared updates
   - quest completion propagation
   - inventory count updates
   - note + relation hydration

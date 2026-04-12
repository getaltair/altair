# Domain Model

| Field | Value |
|---|---|
| **Document** | 02-domain-model |
| **Version** | 1.0 |
| **Status** | Draft |
| **Last Updated** | 2026-04-12 |
| **Source Docs** | `docs/altair-architecture-spec.md` (sections 8-9), `docs/altair-schema-design-spec.md`, `docs/altair-shared-contracts-spec.md` |

---

## Bounded Contexts

| Context | Responsibilities | Agent Type |
|---|---|---|
| **Identity** | Accounts, auth, password/OIDC login, roles, token/session lifecycle, per-user isolation | Backend |
| **Core** | Initiatives, inbox, tags, attachment metadata, notification preferences, shared settings | Full-stack |
| **Guidance** | Epics, quests, routines, checkpoints, focus sessions, energy state, daily check-ins | Full-stack |
| **Knowledge** | Notes, backlinks, graph relationships, import/export, semantic enrichment, OCR/transcription | Full-stack |
| **Tracking** | Items, categories, locations, stock levels, reservations, maintenance, shopping lists | Full-stack |
| **Search** | Keyword indexing, semantic embeddings, hybrid ranking, cross-app result shaping | Backend |
| **Sync** | Mutation ingestion, conflict detection, device checkpoints, downstream changes, reconciliation | Backend |

---

## Entity Hierarchy

```mermaid
classDiagram
    class User {
        +id: UUID
        +email: String
        +display_name: String
        +created_at: DateTime
        +updated_at: DateTime
    }
    class Household {
        +id: UUID
        +name: String
        +owner_id: UUID
    }
    class HouseholdMembership {
        +id: UUID
        +household_id: UUID
        +user_id: UUID
        +role: String
    }
    class Initiative {
        +id: UUID
        +title: String
        +description: String?
        +status: InitiativeStatus
        +user_id: UUID
        +household_id: UUID?
    }
    class Tag {
        +id: UUID
        +name: String
        +user_id: UUID
    }
    class Attachment {
        +id: UUID
        +entity_type: String
        +entity_id: UUID
        +file_name: String
        +content_type: String
        +state: AttachmentState
        +storage_path: String?
    }
    class EntityRelation {
        +id: UUID
        +from_entity_type: String
        +from_entity_id: UUID
        +to_entity_type: String
        +to_entity_id: UUID
        +relation_type: RelationType
        +source_type: SourceType
        +status: RelationStatus
        +confidence: Float?
    }

    User "1" --> "*" Household : owns
    User "1" --> "*" HouseholdMembership
    Household "1" --> "*" HouseholdMembership
    User "1" --> "*" Initiative
    User "1" --> "*" Tag
    User "1" --> "*" Attachment
```

---

## Aggregate Roots

| Aggregate Root | Context | Owned Entities |
|---|---|---|
| **User** | Identity | HouseholdMembership, personal settings |
| **Household** | Core | HouseholdMembership, shared Initiatives |
| **Initiative** | Core | Epics, Quests (scoped), Notes (scoped), Items (scoped) |
| **Quest** | Guidance | FocusSessions (child) |
| **Routine** | Guidance | Spawned Quests (via routine_id) |
| **Note** | Knowledge | NoteSnapshots (child) |
| **Item** | Tracking | ItemEvents (child) |
| **ShoppingList** | Tracking | ShoppingListItems (child) |

---

## Core Entities

### Guidance Domain

```mermaid
erDiagram
    INITIATIVE ||--o{ EPIC : contains
    INITIATIVE ||--o{ QUEST : groups
    EPIC ||--o{ QUEST : contains
    ROUTINE ||--o{ QUEST : spawns
    QUEST ||--o{ FOCUS_SESSION : tracks
    USER ||--o{ DAILY_CHECKIN : submits

    INITIATIVE {
        uuid id PK
        string title
        string description
        string status
        uuid user_id FK
        uuid household_id FK
    }
    EPIC {
        uuid id PK
        uuid initiative_id FK
        string title
        string status
        int sort_order
    }
    QUEST {
        uuid id PK
        string title
        string status
        string priority
        date due_date
        uuid epic_id FK
        uuid initiative_id FK
        uuid routine_id FK
        uuid user_id FK
    }
    ROUTINE {
        uuid id PK
        string title
        string frequency_type
        string frequency_config
        string status
        uuid user_id FK
    }
    FOCUS_SESSION {
        uuid id PK
        uuid quest_id FK
        timestamp started_at
        timestamp ended_at
        int duration_minutes
    }
    DAILY_CHECKIN {
        uuid id PK
        uuid user_id FK
        date checkin_date
        int energy_level
        string mood
        string notes
    }
```

### Knowledge Domain

```mermaid
erDiagram
    NOTE ||--o{ NOTE_SNAPSHOT : versions
    NOTE }o--o{ NOTE : "references (backlinks)"
    NOTE }o--|| INITIATIVE : grouped_under

    NOTE {
        uuid id PK
        string title
        text content
        uuid user_id FK
        uuid initiative_id FK
        timestamp created_at
        timestamp updated_at
    }
    NOTE_SNAPSHOT {
        uuid id PK
        uuid note_id FK
        text content
        timestamp captured_at
    }
```

### Tracking Domain

```mermaid
erDiagram
    ITEM ||--o{ ITEM_EVENT : records
    ITEM }o--|| LOCATION : stored_in
    ITEM }o--|| CATEGORY : categorized
    SHOPPING_LIST ||--o{ SHOPPING_LIST_ITEM : contains
    SHOPPING_LIST_ITEM }o--o| ITEM : references

    LOCATION {
        uuid id PK
        string name
        uuid household_id FK
    }
    CATEGORY {
        uuid id PK
        string name
        uuid household_id FK
    }
    ITEM {
        uuid id PK
        string name
        int quantity
        uuid location_id FK
        uuid category_id FK
        uuid user_id FK
        uuid household_id FK
        uuid initiative_id FK
    }
    ITEM_EVENT {
        uuid id PK
        uuid item_id FK
        string event_type
        int quantity_change
        timestamp occurred_at
    }
    SHOPPING_LIST {
        uuid id PK
        string name
        uuid household_id FK
    }
    SHOPPING_LIST_ITEM {
        uuid id PK
        uuid shopping_list_id FK
        uuid item_id FK
        string name
        string status
    }
```

---

## Value Objects

| Value Object | Used By | Fields |
|---|---|---|
| `EntityRef` | EntityRelation, Search | `entity_type: String`, `entity_id: UUID` |
| `Frequency` | Routine | `type: FrequencyType`, `config: JSON` (days, interval) |
| `Priority` | Quest | `level: low \| medium \| high \| urgent` |
| `MutationEnvelope` | Sync | `mutation_id`, `device_id`, `entity_type`, `entity_id`, `operation`, `base_version`, `payload`, `occurred_at` |

---

## Enumerations

### Entity Types (from canonical registry)

**Core:** `user`, `household`, `initiative`, `tag`, `attachment`

**Guidance:** `guidance_epic`, `guidance_quest`, `guidance_routine`, `guidance_focus_session`, `guidance_daily_checkin`

**Knowledge:** `knowledge_note`, `knowledge_note_snapshot`

**Tracking:** `tracking_location`, `tracking_category`, `tracking_item`, `tracking_item_event`, `tracking_shopping_list`, `tracking_shopping_list_item`

### Relation Types
`references`, `supports`, `requires`, `related_to`, `depends_on`, `duplicates`, `similar_to`, `generated_from`

### Relation Source Types
`user`, `ai`, `import`, `rule`, `migration`, `system`

### Relation Statuses
`accepted`, `suggested`, `dismissed`, `rejected`, `expired`

### Attachment States
`pending`, `uploaded`, `processing`, `ready`, `failed`, `deleted`

### Quest Statuses
`not_started`, `in_progress`, `completed`, `cancelled`, `deferred`

### Initiative Statuses
`draft`, `active`, `completed`, `paused`, `archived`

<!-- INFERRED: verify these status enums against actual implementation when it exists -->

---

## Domain Events

| Event | Emitted By | Consumers | Description |
|---|---|---|---|
| `QuestCompleted` | Guidance | Notifications, Analytics | A quest transitions to `completed` |
| `RoutineDue` | Guidance | Notifications | A routine's next occurrence is now |
| `ItemQuantityChanged` | Tracking | Notifications (low stock), Guidance (restock automation) | An item_event modifies quantity |
| `NoteLinked` | Knowledge | Search (re-index) | A note-to-entity relation is created |
| `AttachmentUploaded` | Core | AI (OCR/transcription), Search | Attachment binary is available server-side |
| `SyncConflictDetected` | Sync | Client notification | Server detected conflicting mutations |
| `MemberJoinedHousehold` | Core | Sync (expand scope), Notifications | A user joins a household |

---

## Relationships Summary

```mermaid
graph TD
    User --> Initiative
    User --> Tag
    User --> Attachment
    Initiative --> Epic
    Initiative --> Quest
    Initiative --> Note
    Initiative --> Item
    Epic --> Quest
    Routine --> Quest
    Quest --> FocusSession
    Note --> NoteSnapshot
    Note -.->|backlinks| Note
    Note -.->|entity_relations| Quest
    Note -.->|entity_relations| Item
    Item --> ItemEvent
    Item --> Location
    ShoppingList --> ShoppingListItem
    ShoppingListItem -.-> Item
    Tag -.-> Quest
    Tag -.-> Note
    Tag -.-> Item
    Attachment -.-> Note
    Attachment -.-> Quest
    Attachment -.-> Item
```

Solid arrows = direct foreign key. Dashed arrows = via `entity_relations` or junction table.

---

## Consistency Rules

1. All entity IDs are UUIDs generated client-side (for offline-first creation)
2. `updated_at` is maintained server-side via database trigger
3. Soft deletes (`deleted_at` timestamp) are required for all synced entities
4. Entity type identifiers must come from the canonical registry — no inline strings
5. Cross-domain references use `entity_relations`, not direct foreign keys
6. Attachment metadata syncs; binary blobs do not flow through the sync engine

---

## Query Patterns

| Pattern | Description | Index Hint |
|---|---|---|
| Today's quests | Quests where `due_date = today` and `status != completed` for user | `idx_quests_user_due_status` |
| Today's routines | Routines where `status = active` and frequency matches today | `idx_routines_user_status` |
| Initiative tree | Epic → Quest hierarchy for an initiative | `idx_epics_initiative`, `idx_quests_epic` |
| Note backlinks | entity_relations where `to_entity_type = knowledge_note` and `to_entity_id = X` | `idx_relations_to` |
| Item inventory | Items for a household, filtered by location and category | `idx_items_household_location_category` |
| Shopping list | Shopping list items for a list, with item reference details | `idx_sli_list` |
| Cross-app search | FTS across notes, quests, items | Search index (external) |

# Event Bus

## Purpose

This document describes Altair's internal event bus—the mechanism by which modules communicate without direct
dependencies. It covers event categories, delivery semantics, and patterns for cross-module features.

---

## Architecture

### Why an Event Bus

Modules (Guidance, Knowledge, Tracking) need to react to each other's actions:

- When a Note mentions an Item, Tracking might suggest linking them
- When a Quest is completed, Knowledge might prompt for a reflection note
- When an Item's quantity hits zero, Guidance might suggest a restock Quest

Direct function calls between modules create circular dependencies and tight coupling. The event bus enables loose
coupling through publish/subscribe.

See [ADR-003](../adr/003-event-bus-for-modules.md) for the full decision rationale.

### Delivery Semantics

| Property    | Behavior                            |
| ----------- | ----------------------------------- |
| Delivery    | At-most-once (no persistence)       |
| Ordering    | FIFO within a single publisher      |
| Blocking    | Non-blocking publish                |
| Subscribers | Multiple subscribers per event type |
| Scope       | In-process only (no network)        |

Events are fire-and-forget. If a subscriber is slow or fails, it doesn't block the publisher. Events are not persisted,
if the app crashes, in-flight events are lost.

### Channel Structure

```text
Event Bus
├── guidance:* — Quest, Epic, Checkpoint, Energy events
├── knowledge:* — Note, Folder, Tag, Link events
├── tracking:* — Item, Location, Container events
└── system:* — App lifecycle, settings, sync events
```

Subscribers can listen to:

- All events (`*`)
- A category (`guidance:*`)
- A specific event (`guidance:quest_completed`)

---

## Event Categories

### Guidance Events

| Event                            | Payload                      | Trigger                 |
| -------------------------------- | ---------------------------- | ----------------------- |
| `guidance:quest_created`         | quest_id, title, energy_cost | New Quest created       |
| `guidance:quest_started`         | quest_id                     | Quest moved to active   |
| `guidance:quest_completed`       | quest_id, energy_spent       | Quest marked complete   |
| `guidance:quest_abandoned`       | quest_id                     | Quest abandoned         |
| `guidance:checkpoint_completed`  | quest_id, checkpoint_id      | Checkpoint checked off  |
| `guidance:epic_created`          | epic_id, title               | New Epic created        |
| `guidance:epic_completed`        | epic_id                      | All Quests in Epic done |
| `guidance:energy_budget_changed` | date, new_budget             | Daily budget adjusted   |
| `guidance:energy_depleted`       | date, budget, spent          | Spent exceeds budget    |

### Knowledge Events

| Event                         | Payload                    | Trigger                          |
| ----------------------------- | -------------------------- | -------------------------------- |
| `knowledge:note_created`      | note_id, title, folder_id  | New Note created                 |
| `knowledge:note_updated`      | note_id, changed_fields    | Note content or metadata changed |
| `knowledge:note_deleted`      | note_id                    | Note soft-deleted                |
| `knowledge:note_restored`     | note_id                    | Note restored from trash         |
| `knowledge:link_created`      | source_id, target_id       | New link between Notes           |
| `knowledge:link_broken`       | source_id, target_title    | Link target deleted              |
| `knowledge:folder_created`    | folder_id, name, parent_id | New Folder created               |
| `knowledge:folder_deleted`    | folder_id                  | Folder deleted                   |
| `knowledge:tag_applied`       | note_id, tag_id            | Tag added to Note                |
| `knowledge:tag_removed`       | note_id, tag_id            | Tag removed from Note            |
| `knowledge:embedding_updated` | note_id                    | Note embedding regenerated       |

### Tracking Events

| Event                        | Payload                    | Trigger               |
| ---------------------------- | -------------------------- | --------------------- |
| `tracking:item_created`      | item_id, name, template_id | New Item created      |
| `tracking:item_updated`      | item_id, changed_fields    | Item metadata changed |
| `tracking:item_deleted`      | item_id                    | Item soft-deleted     |
| `tracking:item_moved`        | item_id, from, to          | Item location changed |
| `tracking:quantity_changed`  | item_id, old_qty, new_qty  | Quantity updated      |
| `tracking:quantity_zero`     | item_id, name              | Quantity reached zero |
| `tracking:location_created`  | location_id, name          | New Location created  |
| `tracking:container_created` | container_id, name         | New Container created |
| `tracking:container_moved`   | container_id, from, to     | Container relocated   |

### System Events

| Event                        | Payload                        | Trigger                   |
| ---------------------------- | ------------------------------ | ------------------------- |
| `system:app_started`         | version                        | Application launched      |
| `system:app_closing`         | —                              | Application shutting down |
| `system:settings_changed`    | changed_keys                   | User settings modified    |
| `system:sync_started`        | —                              | Sync process beginning    |
| `system:sync_completed`      | changes_pulled, changes_pushed | Sync finished             |
| `system:sync_conflict`       | entity_type, entity_id         | Conflict detected         |
| `system:backup_completed`    | backup_path                    | Backup finished           |
| `system:ai_provider_changed` | capability, provider           | AI config changed         |

---

## Event Flow

### Publishing

Modules publish events after completing their primary action:

```text
1. Module receives command (e.g., complete_quest)
2. Module executes business logic
3. Module persists changes to database
4. Module publishes event (quest_completed)
5. Module returns result to caller
```

Events are published after persistence succeeds. If the database write fails, no event is published.

### Subscribing

Modules subscribe to events they care about at startup:

```text
1. App initializes Event Bus
2. Each module registers its subscriptions
3. Event Bus delivers matching events to subscribers
4. Subscribers handle events asynchronously
```

Subscribers should handle events quickly. Long-running reactions should spawn background tasks.

### Handling

Event handlers must be:

- **Idempotent**: Safe to receive the same event twice
- **Failure-tolerant**: Log errors, don't crash
- **Non-blocking**: Don't hold up other handlers

---

## Cross-Module Patterns

### Discovery Pattern

One module detects something relevant to another module.

**Example**: Knowledge detects an Item mention in a Note.

```text
1. User edits Note content
2. Knowledge parses content for `[[Item:...]]` patterns
3. Knowledge publishes `knowledge:note_updated` with mention info
4. Tracking receives event
5. Tracking checks if mentioned Item exists
6. If new mention, Tracking prompts user to link or create Item
```

### Suggestion Pattern

One module suggests an action in another module.

**Example**: Tracking suggests a Quest when Item quantity is low.

```text
1. User decrements Item quantity
2. Tracking publishes `tracking:quantity_changed`
3. Tracking also publishes `tracking:quantity_zero` if applicable
4. Guidance receives quantity_zero event
5. Guidance creates a suggested Quest: "Restock {item_name}"
6. User sees suggestion in Guidance inbox
```

### Reflection Pattern

One module prompts documentation in another module.

**Example**: Guidance prompts for a reflection Note after Quest completion.

```text
1. User completes Quest
2. Guidance publishes `guidance:quest_completed`
3. Knowledge receives event
4. Knowledge prompts: "Add notes about completing {quest_title}?"
5. If accepted, Knowledge creates Note linked to Quest
```

---

## Frontend Integration

### Event Forwarding

Backend events can be forwarded to the frontend for reactive UI updates:

```text
Backend Event Bus → Tauri Event Emitter → Frontend Event Listener
```

Not all events are forwarded—only those needed for UI reactivity.

### Forwarded Events

| Backend Event              | Frontend Use                          |
| -------------------------- | ------------------------------------- |
| `guidance:quest_completed` | Update energy meter, show celebration |
| `guidance:energy_depleted` | Show warning indicator                |
| `knowledge:note_updated`   | Refresh note list if visible          |
| `tracking:quantity_zero`   | Show low-stock badge                  |
| `system:sync_completed`    | Clear sync indicator                  |

### Frontend-Only Events

Some UI events don't involve the backend:

| Event              | Purpose                |
| ------------------ | ---------------------- |
| `ui:modal_opened`  | Track modal state      |
| `ui:navigation`    | Page change            |
| `ui:theme_changed` | Dark/light mode toggle |

---

## Error Handling

### Publisher Errors

If publishing fails (channel full, bus unavailable):

1. Log warning with event details
2. Continue execution (don't fail the primary action)
3. Event is lost (at-most-once delivery)

### Subscriber Errors

If a handler throws an error:

1. Log error with event and handler details
2. Continue delivering to other subscribers
3. Don't retry (handler should be idempotent anyway)

### Circuit Breaker

If a subscriber fails repeatedly:

1. After N consecutive failures, mark subscriber as unhealthy
2. Stop delivering events to unhealthy subscriber
3. Periodically probe with single event to check recovery
4. Restore delivery when probe succeeds

---

## Debugging

### Event Logging

In development mode, all events are logged:

```text
[EVENT] guidance:quest_completed { quest_id: "01HXK3...", energy_spent: 3 }
```

### Event History

Recent events are kept in a ring buffer for debugging:

- Last 1000 events retained in memory
- Accessible via debug command or dev tools
- Cleared on app restart

### Tracing

Events can be correlated with request traces:

- Each event includes optional `trace_id`
- Trace spans the original user action through all resulting events
- Enables "what happened when I clicked X?" debugging

---

## Performance Considerations

### Channel Capacity

The broadcast channel has bounded capacity:

- Default: 256 events
- If full, oldest events dropped (with warning)
- Slow subscribers should process asynchronously

### Subscription Filtering

Subscribers filter at subscription time, not delivery time:

- `subscribe("guidance:*")` only receives Guidance events
- Reduces unnecessary deserialization and dispatch

### Batch Events

For bulk operations, consider batch events:

- `knowledge:notes_imported { count: 50, folder_id }` vs 50 individual events
- Reduces event bus load
- Subscribers can handle bulk updates efficiently

---

## References

- [ADR-003: Event Bus for Inter-Module Communication](../adr/003-event-bus-for-modules.md)
- [System Architecture](./system-architecture.md) — Component relationships
- [Domain Model](./domain-model.md) — Entity definitions

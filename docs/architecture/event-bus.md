# Event Bus

## Purpose

This document describes Altair's internal event bus—the mechanism by which modules communicate
without direct dependencies. It covers event categories, delivery semantics, and patterns for
cross-module features.

---

## Architecture

### Why an Event Bus

Modules (Guidance, Knowledge, Tracking) need to react to each other's actions:

- When a Note mentions an Item, Tracking might suggest linking them
- When a Quest is completed, Knowledge might prompt for a reflection note
- When an Item's quantity hits zero, Guidance might suggest a restock Quest

Direct function calls between modules create circular dependencies and tight coupling. The event bus
enables loose coupling through publish/subscribe.

See [ADR-003](../adr/003-event-bus-for-modules.md) for the full decision rationale.

### Delivery Semantics

| Property    | Behavior                            |
| ----------- | ----------------------------------- |
| Delivery    | At-most-once (no persistence)       |
| Ordering    | FIFO within a single publisher      |
| Blocking    | Non-blocking publish                |
| Subscribers | Multiple subscribers per event type |
| Scope       | In-process only (desktop)           |

Events are fire-and-forget. If a subscriber is slow or fails, it doesn't block the publisher. Events
are not persisted; if the app crashes, in-flight events are lost.

### Implementation

Desktop uses Kotlin coroutines `SharedFlow` for the event bus:

```kotlin
class EventBus {
    private val _events = MutableSharedFlow<AppEvent>(
        replay = 0,
        extraBufferCapacity = 100,
        onBufferOverflow = BufferOverflow.DROP_OLDEST
    )
    val events: SharedFlow<AppEvent> = _events.asSharedFlow()

    suspend fun publish(event: AppEvent) {
        _events.emit(event)
    }

    fun <T : AppEvent> subscribe(
        eventClass: KClass<T>,
        scope: CoroutineScope,
        handler: suspend (T) -> Unit
    ): Job = scope.launch {
        events
            .filterIsInstance(eventClass)
            .collect { handler(it) }
    }
}
```

### Mobile Considerations

Mobile clients have a simplified architecture focused on quick capture. The event bus is primarily a
desktop feature where modules interact in real-time. Mobile syncs to server; cross-module reactions
happen server-side or when data syncs to desktop.

---

## Event Categories

### Guidance Events

```kotlin
sealed class GuidanceEvent : AppEvent() {
    data class QuestCreated(val quest: Quest) : GuidanceEvent()
    data class QuestStarted(val quest: Quest) : GuidanceEvent()
    data class QuestCompleted(val quest: Quest) : GuidanceEvent()
    data class QuestAbandoned(val quest: Quest) : GuidanceEvent()
    data class CheckpointCompleted(val quest: Quest, val checkpoint: Checkpoint) : GuidanceEvent()
    data class EpicCompleted(val epic: Epic) : GuidanceEvent()
    data class EnergyBudgetExceeded(val budget: Int, val spent: Int) : GuidanceEvent()
}
```

### Knowledge Events

```kotlin
sealed class KnowledgeEvent : AppEvent() {
    data class NoteCreated(val note: Note) : KnowledgeEvent()
    data class NoteUpdated(val note: Note, val oldContent: String) : KnowledgeEvent()
    data class NoteDeleted(val note: Note) : KnowledgeEvent()
    data class LinkCreated(val source: Note, val target: Note) : KnowledgeEvent()
    data class TagAdded(val note: Note, val tag: Tag) : KnowledgeEvent()
    data class FolderCreated(val folder: Folder) : KnowledgeEvent()
    data class ItemMentioned(val note: Note, val itemName: String) : KnowledgeEvent()
}
```

### Tracking Events

```kotlin
sealed class TrackingEvent : AppEvent() {
    data class ItemCreated(val item: Item) : TrackingEvent()
    data class ItemUpdated(val item: Item) : TrackingEvent()
    data class ItemMoved(val item: Item, val from: String?, val to: String?) : TrackingEvent()
    data class QuantityChanged(val item: Item, val oldQty: Int, val newQty: Int) : TrackingEvent()
    data class QuantityZero(val item: Item) : TrackingEvent()
    data class LocationCreated(val location: Location) : TrackingEvent()
    data class ContainerCreated(val container: Container) : TrackingEvent()
}
```

### System Events

```kotlin
sealed class SystemEvent : AppEvent() {
    data object AppStarted : SystemEvent()
    data object AppBackgrounded : SystemEvent()
    data object AppForegrounded : SystemEvent()
    data class SettingsChanged(val key: String, val value: Any?) : SystemEvent()
    data class SyncStarted(val direction: SyncDirection) : SystemEvent()
    data class SyncCompleted(val direction: SyncDirection, val changes: Int) : SystemEvent()
    data class SyncFailed(val direction: SyncDirection, val error: Throwable) : SystemEvent()
}
```

---

## Event Flow

### Publishing Events

Modules publish events after successful database operations:

```kotlin
class QuestRepository(
    private val db: Database,
    private val eventBus: EventBus
) {
    suspend fun completeQuest(questId: String): Quest {
        val quest = db.updateQuestStatus(questId, QuestStatus.COMPLETED)
        eventBus.publish(GuidanceEvent.QuestCompleted(quest))
        return quest
    }
}
```

### Subscribing to Events

Modules subscribe to events they care about:

```kotlin
class ReflectionPromptService(
    private val eventBus: EventBus,
    private val scope: CoroutineScope
) {
    init {
        eventBus.subscribe(GuidanceEvent.QuestCompleted::class, scope) { event ->
            showReflectionPrompt(event.quest)
        }
    }

    private suspend fun showReflectionPrompt(quest: Quest) {
        // Show UI prompt to create reflection note
    }
}
```

---

## Cross-Module Patterns

### Pattern: Suggestion Cards

When one module detects something relevant to another, it publishes an event. A suggestion service
listens and shows dismissible cards in the relevant module.

```kotlin
// Knowledge detects item mention in note content
class NoteMentionScanner(
    private val eventBus: EventBus
) {
    suspend fun scanNote(note: Note) {
        val mentions = extractItemMentions(note.content)
        mentions.forEach { itemName ->
            eventBus.publish(KnowledgeEvent.ItemMentioned(note, itemName))
        }
    }
}

// Tracking listens and shows suggestion
class TrackingSuggestionService(
    private val eventBus: EventBus,
    private val suggestionStore: SuggestionStore
) {
    init {
        eventBus.subscribe(KnowledgeEvent.ItemMentioned::class, scope) { event ->
            val existingItem = findItemByName(event.itemName)
            if (existingItem != null) {
                suggestionStore.add(Suggestion(
                    type = SuggestionType.LINK_NOTE_TO_ITEM,
                    data = mapOf("note" to event.note, "item" to existingItem)
                ))
            }
        }
    }
}
```

### Pattern: Auto-Discovery

Semantic similarity triggers suggestions across modules:

```kotlin
class AutoDiscoveryService(
    private val eventBus: EventBus,
    private val aiService: AiService
) {
    init {
        eventBus.subscribe(KnowledgeEvent.NoteUpdated::class, scope) { event ->
            // Debounced, background embedding update
            updateEmbeddingAndFindRelated(event.note)
        }
    }

    private suspend fun updateEmbeddingAndFindRelated(note: Note) {
        val embedding = aiService.embed(listOf(note.content)).first()
        // Store embedding, find similar notes, quests, items
        // Publish discovery events for high-similarity matches
    }
}
```

### Pattern: Cascade Reactions

One event triggers another:

```kotlin
// Quest completion triggers reflection prompt
eventBus.subscribe(GuidanceEvent.QuestCompleted::class, scope) { event ->
    showReflectionPrompt(event.quest)
}

// Reflection note creation triggers link suggestion
eventBus.subscribe(KnowledgeEvent.NoteCreated::class, scope) { event ->
    if (event.note.title.startsWith("Reflection:")) {
        suggestLinkToCompletedQuest(event.note)
    }
}
```

---

## Error Handling

### Subscriber Failures

Subscribers should handle their own errors. Failures don't propagate to publishers:

```kotlin
eventBus.subscribe(SomeEvent::class, scope) { event ->
    try {
        processEvent(event)
    } catch (e: Exception) {
        logger.error("Failed to process event", e)
        // Optionally: publish error event, show user notification
    }
}
```

### Buffer Overflow

If events are published faster than consumed, oldest events are dropped (configured via
`BufferOverflow.DROP_OLDEST`). This prevents memory growth but means some events may be lost under
extreme load.

---

## Debugging

### Event Logging

In debug builds, all events are logged:

```kotlin
class EventLogger(
    private val eventBus: EventBus,
    private val scope: CoroutineScope
) {
    init {
        eventBus.subscribe(AppEvent::class, scope) { event ->
            logger.debug("Event: ${event::class.simpleName} - $event")
        }
    }
}
```

### Event History (Debug Only)

Debug builds maintain a bounded history of recent events:

```kotlin
class EventHistory(
    private val eventBus: EventBus,
    private val maxSize: Int = 1000
) {
    private val history = ArrayDeque<Pair<Instant, AppEvent>>()

    fun getRecent(count: Int): List<Pair<Instant, AppEvent>> =
        history.takeLast(count)

    fun getByType(type: KClass<out AppEvent>): List<Pair<Instant, AppEvent>> =
        history.filter { type.isInstance(it.second) }
}
```

---

## References

- [ADR-003: Event Bus for Modules](../adr/003-event-bus-for-modules.md)
- [Kotlin SharedFlow Documentation](https://kotlinlang.org/api/kotlinx.coroutines/kotlinx-coroutines-core/kotlinx.coroutines.flow/-shared-flow/)
- FR-G-031: Cross-app drag-drop
- FR-K-130: Cross-app discovery
- FR-T-038: Real-time text analysis

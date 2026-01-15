# Event Bus

## Purpose

This document describes Altair's internal event bus—the mechanism by which modules communicate
without direct dependencies. It covers event categories, delivery semantics, and patterns for
cross-module features.

---

## Architecture

### Why an Event Bus

Modules (Guidance, Knowledge, Tracking) and system-level features (Initiatives, Inbox, Routines)
need to react to each other's actions:

- When a Note mentions an Item, Tracking might suggest linking them
- When a Quest is completed, Knowledge might prompt for a reflection note
- When an Item's quantity hits zero, Guidance might suggest a restock Quest
- When an Initiative is focused, all modules update their context displays
- When an InboxItem is triaged, the destination module receives it
- When a Routine generates an instance, Guidance adds the Quest

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

### User Events

```kotlin
sealed class UserEvent : AppEvent() {
    data class UserLoggedIn(val userId: String) : UserEvent()
    data class UserLoggedOut(val userId: String) : UserEvent()
    data class UserSettingsChanged(val key: String, val value: Any?) : UserEvent()
}
```

### Initiative Events

```kotlin
sealed class InitiativeEvent : AppEvent() {
    data class InitiativeCreated(val initiative: Initiative) : InitiativeEvent()
    data class InitiativeUpdated(val initiative: Initiative) : InitiativeEvent()
    data class InitiativeFocused(val initiative: Initiative?) : InitiativeEvent()
    data class InitiativeCompleted(val initiative: Initiative) : InitiativeEvent()
    data class InitiativeArchived(val initiative: Initiative) : InitiativeEvent()
    data class InitiativePaused(val initiative: Initiative) : InitiativeEvent()
    data class InitiativeResumed(val initiative: Initiative) : InitiativeEvent()
}
```

### Inbox Events

```kotlin
sealed class InboxEvent : AppEvent() {
    data class InboxItemCaptured(val item: InboxItem) : InboxEvent()
    data class InboxItemTriagedToQuest(val item: InboxItem, val quest: Quest) : InboxEvent()
    data class InboxItemTriagedToNote(val item: InboxItem, val note: Note) : InboxEvent()
    data class InboxItemTriagedToItem(val item: InboxItem, val trackingItem: Item) : InboxEvent()
    data class InboxItemTriagedToSourceDocument(val item: InboxItem, val sourceDocument: SourceDocument) : InboxEvent()
    data class InboxItemArchived(val item: InboxItem) : InboxEvent()
    data class InboxCountChanged(val count: Int) : InboxEvent()
}
```

### Routine Events

```kotlin
sealed class RoutineEvent : AppEvent() {
    data class RoutineCreated(val routine: Routine) : RoutineEvent()
    data class RoutineUpdated(val routine: Routine) : RoutineEvent()
    data class RoutineActivated(val routine: Routine) : RoutineEvent()
    data class RoutineDeactivated(val routine: Routine) : RoutineEvent()
    data class RoutineInstanceSpawned(val routine: Routine, val quest: Quest) : RoutineEvent()
    data class RoutineInstanceSkipped(val routine: Routine, val dueDate: LocalDate) : RoutineEvent()
    data class RoutineDue(val routine: Routine) : RoutineEvent()
}
```

### Guidance Events

```kotlin
sealed class GuidanceEvent : AppEvent() {
    data class QuestCreated(val quest: Quest) : GuidanceEvent()
    data class QuestStarted(val quest: Quest) : GuidanceEvent()
    data class QuestCompleted(val quest: Quest) : GuidanceEvent()
    data class QuestAbandoned(val quest: Quest) : GuidanceEvent()
    data class CheckpointCompleted(val quest: Quest, val checkpoint: Checkpoint) : GuidanceEvent()
    data class EpicCreated(val epic: Epic) : GuidanceEvent()
    data class EpicCompleted(val epic: Epic) : GuidanceEvent()
    data class EpicLinkedToInitiative(val epic: Epic, val initiative: Initiative) : GuidanceEvent()
    data class EnergyBudgetExceeded(val budget: Int, val spent: Int) : GuidanceEvent()
    data class EnergyCheckedIn(val level: Int, val date: LocalDate) : GuidanceEvent()
}
```

### Knowledge Events

```kotlin
sealed class KnowledgeEvent : AppEvent() {
    data class NoteCreated(val note: Note) : KnowledgeEvent()
    data class NoteUpdated(val note: Note, val oldContent: String) : KnowledgeEvent()
    data class NoteDeleted(val note: Note) : KnowledgeEvent()
    data class NoteLinkCreated(val source: Note, val target: Note) : KnowledgeEvent()
    data class NoteLinkedToInitiative(val note: Note, val initiative: Initiative) : KnowledgeEvent()
    data class NoteLinkedToSourceDocument(val note: Note, val sourceDocument: SourceDocument) : KnowledgeEvent()
    data class TagAdded(val note: Note, val tag: Tag) : KnowledgeEvent()
    data class FolderCreated(val folder: Folder) : KnowledgeEvent()
    data class ItemMentioned(val note: Note, val itemName: String) : KnowledgeEvent()
}
```

### SourceDocument Events

```kotlin
sealed class SourceDocumentEvent : AppEvent() {
    data class SourceDocumentImported(val document: SourceDocument) : SourceDocumentEvent()
    data class SourceDocumentUpdated(val document: SourceDocument) : SourceDocumentEvent()
    data class SourceDocumentDeleted(val document: SourceDocument) : SourceDocumentEvent()
    data class SourceDocumentLinkedToInitiative(val document: SourceDocument, val initiative: Initiative) : SourceDocumentEvent()
    data class ExtractionStarted(val document: SourceDocument) : SourceDocumentEvent()
    data class ExtractionCompleted(val document: SourceDocument) : SourceDocumentEvent()
    data class ExtractionFailed(val document: SourceDocument, val error: String) : SourceDocumentEvent()
    data class SourceDocumentStale(val document: SourceDocument) : SourceDocumentEvent()
    data class AnnotationCreated(val annotation: SourceAnnotation) : SourceDocumentEvent()
    data class AnnotationUpdated(val annotation: SourceAnnotation) : SourceDocumentEvent()
    data class AnnotationDeleted(val annotation: SourceAnnotation) : SourceDocumentEvent()
    data class AnnotationPromotedToNote(val annotation: SourceAnnotation, val note: Note) : SourceDocumentEvent()
    data class WatchedFolderCreated(val folder: WatchedFolder) : SourceDocumentEvent()
    data class WatchedFolderScanned(val folder: WatchedFolder, val newDocs: Int, val staleDocs: Int) : SourceDocumentEvent()
    data class WatchedFolderError(val folder: WatchedFolder, val error: String) : SourceDocumentEvent()
}
```

### Tracking Events

```kotlin
sealed class TrackingEvent : AppEvent() {
    data class ItemCreated(val item: Item) : TrackingEvent()
    data class ItemUpdated(val item: Item) : TrackingEvent()
    data class ItemMoved(val item: Item, val from: String?, val to: String?) : TrackingEvent()
    data class ItemLinkedToInitiative(val item: Item, val initiative: Initiative) : TrackingEvent()
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
    data class SyncConflictDetected(val entityType: String, val entityId: String) : SystemEvent()
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

### Pattern: Initiative Context Updates

When Initiative focus changes, all modules update their displays:

```kotlin
class InitiativeContextService(
    private val eventBus: EventBus,
    private val scope: CoroutineScope
) {
    init {
        eventBus.subscribe(InitiativeEvent.InitiativeFocused::class, scope) { event ->
            // Broadcast to all UI components
            updateInitiativeCard(event.initiative)
            updateGuidanceFilter(event.initiative)
            updateKnowledgeFilter(event.initiative)
            updateTrackingFilter(event.initiative)
        }
    }
}
```

### Pattern: Inbox Triage Flow

When an inbox item is triaged, the destination module receives it:

```kotlin
class InboxTriageService(
    private val eventBus: EventBus,
    private val scope: CoroutineScope
) {
    suspend fun triageToQuest(inboxItem: InboxItem, initiativeId: String?) {
        val quest = questRepository.createFromInbox(inboxItem, initiativeId)
        inboxRepository.delete(inboxItem.id)
        eventBus.publish(InboxEvent.InboxItemTriagedToQuest(inboxItem, quest))
    }
}

// Guidance module listens
class GuidanceInboxHandler(
    private val eventBus: EventBus,
    private val scope: CoroutineScope
) {
    init {
        eventBus.subscribe(InboxEvent.InboxItemTriagedToQuest::class, scope) { event ->
            showNewQuestNotification(event.quest)
        }
    }
}
```

### Pattern: Routine Instance Generation

When routines generate Quest instances:

```kotlin
class RoutineScheduler(
    private val eventBus: EventBus,
    private val routineRepository: RoutineRepository,
    private val questRepository: QuestRepository
) {
    suspend fun generateDueInstances() {
        val dueRoutines = routineRepository.getDueRoutines()
        for (routine in dueRoutines) {
            val quest = questRepository.createFromRoutine(routine)
            routineRepository.updateNextDue(routine.id)
            eventBus.publish(RoutineEvent.RoutineInstanceSpawned(routine, quest))
        }
    }
}
```

### Pattern: SourceDocument Import and Extraction

When files are imported as SourceDocuments:

```kotlin
class SourceDocumentImportService(
    private val eventBus: EventBus,
    private val sourceDocumentRepository: SourceDocumentRepository,
    private val extractionJobRepository: ExtractionJobRepository
) {
    suspend fun importFromInbox(inboxItem: InboxItem, initiativeId: String?) {
        // Create SourceDocument from inbox item attachment
        val document = sourceDocumentRepository.createFromInbox(inboxItem, initiativeId)
        inboxRepository.delete(inboxItem.id)
        
        // Queue extraction job
        extractionJobRepository.create(document.id)
        
        eventBus.publish(InboxEvent.InboxItemTriagedToSourceDocument(inboxItem, document))
        eventBus.publish(SourceDocumentEvent.SourceDocumentImported(document))
    }
}

// Extraction service listens and processes
class ExtractionService(
    private val eventBus: EventBus,
    private val scope: CoroutineScope
) {
    init {
        eventBus.subscribe(SourceDocumentEvent.SourceDocumentImported::class, scope) { event ->
            processExtraction(event.document)
        }
    }
    
    private suspend fun processExtraction(document: SourceDocument) {
        eventBus.publish(SourceDocumentEvent.ExtractionStarted(document))
        try {
            val extracted = extractContent(document)
            sourceDocumentRepository.updateExtractedContent(document.id, extracted)
            eventBus.publish(SourceDocumentEvent.ExtractionCompleted(document))
        } catch (e: Exception) {
            sourceDocumentRepository.updateStatus(document.id, "failed", e.message)
            eventBus.publish(SourceDocumentEvent.ExtractionFailed(document, e.message ?: "Unknown error"))
        }
    }
}
```

### Pattern: WatchedFolder Scanning

When watched folders are scanned for changes:

```kotlin
class WatchedFolderScanner(
    private val eventBus: EventBus,
    private val watchedFolderRepository: WatchedFolderRepository,
    private val sourceDocumentRepository: SourceDocumentRepository
) {
    suspend fun scanFolder(folder: WatchedFolder) {
        val files = listFilesMatching(folder.path, folder.includePatterns, folder.excludePatterns)
        var newDocs = 0
        var staleDocs = 0
        
        for (file in files) {
            val hash = computeHash(file)
            val existing = sourceDocumentRepository.findByPath(file.path)
            
            when {
                existing == null -> {
                    // New file discovered
                    sourceDocumentRepository.create(
                        sourcePath = file.path,
                        contentHash = hash,
                        initiativeId = folder.initiativeId,
                        watchedFolderId = folder.id
                    )
                    newDocs++
                }
                existing.contentHash != hash -> {
                    // File modified
                    sourceDocumentRepository.markStale(existing.id)
                    staleDocs++
                    eventBus.publish(SourceDocumentEvent.SourceDocumentStale(existing))
                }
            }
        }
        
        watchedFolderRepository.updateLastScanned(folder.id)
        eventBus.publish(SourceDocumentEvent.WatchedFolderScanned(folder, newDocs, staleDocs))
    }
}
```

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
        // Suggest Initiative linking for high-similarity matches
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

// Routine instance completion triggers routine update
eventBus.subscribe(GuidanceEvent.QuestCompleted::class, scope) { event ->
    if (event.quest.routineId != null) {
        routineRepository.updateLastCompleted(event.quest.routineId)
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
- [Domain Model](./domain-model.md) — Entity definitions
- FR-IN-001 through FR-IN-014: Universal Inbox requirements
- FR-I-001 through FR-I-009: Initiative requirements
- FR-R-001 through FR-R-012: Routine requirements
- FR-G-031: Cross-app drag-drop
- FR-K-130: Cross-app discovery
- FR-T-038: Real-time text analysis

---

_Last updated: January 14, 2026_

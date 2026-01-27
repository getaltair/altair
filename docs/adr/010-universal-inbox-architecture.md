# ADR-010: Universal Inbox Architecture

| Field             | Value           |
| ----------------- | --------------- |
| **Status**        | Accepted        |
| **Date**          | 2026-01-14      |
| **Deciders**      | Robert Hamilton |
| **Supersedes**    | —               |
| **Superseded by** | —               |

## Context

Altair consists of three modules (Guidance, Knowledge, Tracking), each with its own data types
(Quest, Note, Item). When users want to capture something quickly, they face a cognitive burden:
"Is this a task? A note? An item to track?" This decision friction reduces capture velocity and
increases the likelihood that thoughts are lost.

The GTD methodology emphasizes "ubiquitous capture" — getting everything out of your head into a
trusted system. For users with ADHD, this is especially critical: the moment of inspiration is
often the only moment when the thought is accessible.

We needed to decide how to handle quick capture across the ecosystem:

1. **Module-specific capture**: Each module has its own capture mechanism
2. **Universal capture with immediate typing**: Single capture point, user classifies immediately
3. **Universal capture with deferred typing**: Single capture point, classification happens later

## Decision

We chose **Universal Inbox with deferred typing** — a single system-level capture point where items
remain untyped until the user explicitly triages them.

### Architecture

```
Universal Inbox (System-Level)
├── InboxItem entity (content, source, attachments)
├── Multiple capture methods (keyboard, voice, camera, share, widget, watch)
├── Triage actions → Quest | Note | Item | SourceDocument
└── Optional Initiative linking during triage
```

### Key Design Decisions

1. **Untyped by default**: InboxItems have content but no type until triaged
2. **Source tracking**: Record how each item was captured (keyboard, voice, camera, etc.)
3. **Mobile home screen**: Inbox is the default landing screen on mobile
4. **Desktop accessibility**: Global keyboard shortcut + sidebar navigation
5. **Triage workflow**: Convert → optionally link to Initiative → delete InboxItem

## Alternatives Considered

### Module-Specific Capture

Each module would have its own quick capture:
- Guidance: Quick add Quest
- Knowledge: Quick add Note
- Tracking: Quick add Item

**Rejected because:**
- Requires user to decide type at capture time (cognitive burden)
- Three different capture mechanisms to learn
- Doesn't match how thoughts actually arrive (often unclear what type they are)

### Universal Capture with Immediate Typing

Single capture point, but user must classify immediately:

```
[Quick Capture Field]
[Quest] [Note] [Item] ← Choose type
```

**Rejected because:**
- Still requires decision at capture time
- ADHD users often struggle with categorization during capture
- Interrupts the capture flow

### Smart Auto-Classification

AI automatically determines the type based on content:

**Rejected because:**
- Adds latency to capture
- AI can be wrong, requiring correction
- Removes user agency
- Offline capture would be degraded

## Consequences

### Positive

- **Reduced capture friction**: User just captures, decides later
- **Single capture point**: One mechanism to learn, one shortcut to remember
- **ADHD-friendly**: Defers decision-making to a dedicated triage session
- **Flexible triage**: User can batch-process inbox items when they have time
- **Multi-modal**: Supports text, voice, camera, share — all to same destination

### Negative

- **Requires triage**: Inbox can accumulate if user doesn't process regularly
- **Potential overwhelm**: Large inbox count could trigger anxiety
- **Two-step process**: Capture then triage vs. single-step module-specific capture

### Mitigations

- **Inbox count badge**: Visible reminder to triage, but not intrusive
- **Auto-archive suggestion**: Items older than 90 days prompt for archive
- **Bulk triage**: Select multiple items for batch operations
- **Today summary**: Mobile home screen shows inbox count alongside daily tasks

## Implementation Notes

### InboxItem Entity

```kotlin
data class InboxItem(
    val id: String,
    val userId: String,
    val content: String,
    val source: CaptureSource,
    val attachments: List<Attachment>,
    val createdAt: Instant,
    val deletedAt: Instant?
)

enum class CaptureSource {
    KEYBOARD, VOICE, CAMERA, SHARE, WIDGET, WATCH
}
```

### Triage Actions

```kotlin
sealed class TriageAction {
    data class ToQuest(val initiativeId: String?) : TriageAction()
    data class ToNote(val folderId: String?, val initiativeId: String?) : TriageAction()
    data class ToItem(val locationId: String?, val initiativeId: String?) : TriageAction()
    data class ToSourceDocument(val initiativeId: String?) : TriageAction()
    object Archive : TriageAction()
}
```

### Platform Integration

| Platform | Capture Methods |
|----------|-----------------|
| Desktop | Global keyboard shortcut, sidebar, menu bar |
| Mobile | Home screen field, voice button, camera button, share target, widget |
| Watch | Voice capture, complication |

## References

- [altair-prd-core.md §11](../requirements/altair-prd-core.md) — Universal Inbox requirements
- [domain-model.md](../architecture/domain-model.md) — InboxItem entity definition
- [event-bus.md](../architecture/event-bus.md) — InboxEvent types

---

_Decision made: 2026-01-14_

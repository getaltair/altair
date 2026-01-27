# ADR-011: Initiative System Design

| Field             | Value           |
| ----------------- | --------------- |
| **Status**        | Accepted        |
| **Date**          | 2026-01-14      |
| **Deciders**      | Robert Hamilton |
| **Supersedes**    | —               |
| **Superseded by** | —               |

## Context

Altair's three modules (Guidance, Knowledge, Tracking) each manage different data types, but users
often work on things that span all three. A "Bathroom Renovation" involves tasks (Quests), research
(Notes), and materials (Items). Without a unifying concept, users must mentally track these
connections or manually search across modules.

We evaluated several approaches to cross-cutting organization:

1. **Projects in Guidance only**: Traditional PM approach, other modules reference
2. **Tags as organizers**: Use shared tags to group related content
3. **Dedicated cross-cutting entity**: System-level organizer that all modules reference

Additionally, we needed to decide whether to support hierarchy (projects within areas) and how to
distinguish between "projects" (have an end) and "areas" (maintained indefinitely).

## Decision

We chose **Initiatives as a dedicated cross-cutting entity** with support for hierarchy and the
ability to toggle between project and area modes.

### Architecture

```
Initiative (System-Level)
├── Links to Epic (Guidance)
├── Links to Note (Knowledge)
├── Links to Item (Tracking)
├── Links to SourceDocument (Knowledge)
├── Optional parent_id (hierarchy)
├── Ongoing toggle (area vs. project)
└── Initiative Card (cross-app context display)
```

### Key Design Decisions

1. **Optional by default**: Users can use Altair without ever creating an Initiative
2. **Nesting support**: Up to 3 levels (Area → Project → Sub-project)
3. **Areas vs. Projects**: `ongoing` boolean distinguishes maintained areas from completable projects
4. **Focus mode**: One Initiative can be "focused" for prominent display
5. **Initiative Card**: Persistent UI element showing cross-app summary

### Nesting Rules

- Areas (ongoing=true) can contain projects or other areas
- Projects (ongoing=false) can contain sub-projects but not areas
- Maximum depth: 3 levels

Example: "Home Improvement" (area) → "Bathroom Renovation" (project) → "Plumbing Phase" (sub-project)

## Alternatives Considered

### Projects in Guidance Only

Guidance module owns "Projects" and other modules reference them:

**Rejected because:**
- Creates artificial hierarchy (Guidance becomes "primary" module)
- Some work doesn't have tasks (pure research projects)
- Some inventory exists without related tasks
- Doesn't match user mental model of cross-cutting work

### Tags as Cross-Module Organizers

Use a shared tag like `#bathroom-renovation` across all content:

**Rejected because:**
- Tags are flat (no hierarchy)
- No status tracking (active, paused, completed)
- No target dates
- No focused display mechanism
- Conflates categorization with organization

### Folders per Module

Each module has folders; user creates matching folders across modules:

**Rejected because:**
- Requires manual synchronization
- No unified view across modules
- No relationship enforcement
- Maintenance burden

## Consequences

### Positive

- **Unified context**: Single place to see all related work across modules
- **Natural hierarchy**: Areas contain projects, projects contain sub-projects
- **Status tracking**: Active, Paused, Completed, Archived transitions
- **Focus display**: Initiative Card provides cross-app context
- **Optional complexity**: Users who don't need organization can ignore it

### Negative

- **Additional concept**: One more thing for users to learn
- **Linking overhead**: User must choose to link content to Initiatives
- **Hierarchy complexity**: Nesting rules require understanding

### Mitigations

- **Optional**: All modules work without Initiatives
- **Quick linking**: One-click link during creation and triage
- **Auto-discovery**: AI can suggest Initiative links based on content similarity
- **Clear defaults**: New content has no Initiative (not required)

## Implementation Notes

### Initiative Entity

```kotlin
data class Initiative(
    val id: String,
    val userId: String,
    val name: String,
    val description: String?,
    val parentId: String?,
    val ongoing: Boolean,
    val targetDate: LocalDate?,
    val status: InitiativeStatus,
    val focused: Boolean,
    val createdAt: Instant,
    val updatedAt: Instant
)

enum class InitiativeStatus {
    ACTIVE, PAUSED, COMPLETED, ARCHIVED
}
```

### Status Transitions

```
Active ──→ Paused (not working on this now)
Active ──→ Completed (project finished, only if ongoing=false)
Active ←→ Ongoing toggle (convert between project and area)
Any ────→ Archived (hide from views, keep data)
```

### Initiative Card UI

```
┌─────────────────────────────────────────────┐
│ 🏠 Bathroom Renovation                      │
│                                             │
│ Guidance: 3 Quests (1 active)    →          │
│ Knowledge: 12 Notes              →          │
│ Tracking: 47 Items (8 needed)    →          │
│                                             │
│ [Pause]  [Complete]                         │
└─────────────────────────────────────────────┘
```

### Focus Behavior

- Only one Initiative can be focused at a time per user
- Focusing an Initiative:
  - Shows Initiative Card prominently in all modules
  - Optionally filters module views to Initiative content
  - Updates Initiative Card summary in real-time

## References

- [altair-prd-core.md §9](../requirements/altair-prd-core.md) — Initiative requirements
- [domain-model.md](../architecture/domain-model.md) — Initiative entity definition
- [event-bus.md](../architecture/event-bus.md) — InitiativeEvent types

---

_Decision made: 2026-01-14_

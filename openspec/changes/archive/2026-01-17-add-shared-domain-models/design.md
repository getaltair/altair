# Design: Shared Domain Models

## Context

Altair requires consistent domain models across all platforms (Android, iOS, Desktop, Server). The
shared KMP module serves as the single source of truth for entity definitions, ensuring type safety
in client-server communication and consistent business logic.

**Constraints:**

- Must work on all KMP targets (Android, iOS, JVM)
- Must be serializable for network transport and database storage
- Must follow Arrow conventions for typed error handling
- Must support soft delete for most entities

## Goals / Non-Goals

**Goals:**

- Define all core entities as `@Serializable` data classes
- Use value classes for type-safe identifiers (ULID)
- Provide enums for all fixed-value fields
- Support Arrow optics for immutable updates
- Include validation at construction time

**Non-Goals:**

- Repository implementations (Phase 3)
- Database-specific mappings (Phase 5)
- DTOs for API requests/responses (Phase 3)
- RPC service definitions (Phase 4)

## Decisions

### 1. Package Structure

```
shared/src/commonMain/kotlin/com/getaltair/altair/domain/
├── model/
│   ├── system/          # User, Initiative, InboxItem, Routine
│   ├── guidance/        # Epic, Quest, Checkpoint, EnergyBudget
│   ├── knowledge/       # Note, NoteLink, Folder, Tag, Attachment, SourceDocument, SourceAnnotation
│   └── tracking/        # Item, Location, Container, ItemTemplate, CustomField, FieldDefinition
├── types/
│   ├── Ulid.kt          # ULID value class and generator
│   ├── Schedule.kt      # Sealed class for routine patterns
│   └── enums/           # All enum definitions
└── common/
    ├── Timestamped.kt   # Interface for created_at, updated_at
    └── SoftDeletable.kt # Interface for deleted_at
```

**Rationale:** Organizing by module mirrors the PRD structure and makes it easy to find related
entities. The `types/` package separates reusable value objects from entity models.

### 2. Identifier Strategy: ULID Value Class

```kotlin
@JvmInline
@Serializable
value class Ulid(val value: String) {
    init {
        require(value.length == 26) { "ULID must be 26 characters" }
        require(value.all { it in ULID_CHARS }) { "Invalid ULID characters" }
    }

    companion object {
        fun generate(): Ulid = Ulid(UlidGenerator.generate())
    }
}
```

**Rationale:** Value classes provide type safety with zero runtime overhead. Using ULID over UUID
gives us sortable, URL-safe identifiers without coordination requirements.

**Alternatives considered:**

- Plain String: No type safety, easy to confuse different ID types
- UUID: Not sortable, longer string representation
- Long/Int: Requires coordination for generation

### 3. Entity Definition Pattern

All entities follow this pattern:

```kotlin
@Serializable
data class Quest(
    val id: Ulid,
    val userId: Ulid,
    val title: String,
    val description: String?,
    val energyCost: Int,
    val status: QuestStatus,
    val epicId: Ulid?,
    val routineId: Ulid?,
    val createdAt: Instant,
    val updatedAt: Instant,
    val startedAt: Instant?,
    val completedAt: Instant?,
    val deletedAt: Instant?,
) : SoftDeletable, Timestamped {
    init {
        require(title.isNotBlank()) { "Quest title must not be blank" }
        require(title.length <= 200) { "Quest title must be at most 200 characters" }
        require(energyCost in 1..5) { "Energy cost must be between 1 and 5" }
    }
}
```

**Rationale:**

- `@Serializable` for kotlinx.serialization compatibility
- Data class for value semantics and generated `copy()`
- `init` block for validation at construction
- Nullable fields for optional references
- Interfaces for common behaviors

### 4. Enum Definitions

```kotlin
@Serializable
enum class QuestStatus {
    @SerialName("backlog") BACKLOG,
    @SerialName("active") ACTIVE,
    @SerialName("completed") COMPLETED,
    @SerialName("abandoned") ABANDONED,
}
```

**Rationale:** Using `@SerialName` ensures consistent JSON representation regardless of Kotlin
naming conventions. Lowercase values match database conventions.

### 5. Schedule Value Object

```kotlin
@Serializable
sealed interface Schedule {
    @Serializable
    @SerialName("daily")
    data object Daily : Schedule

    @Serializable
    @SerialName("weekly")
    data class Weekly(val days: Set<DayOfWeek>) : Schedule {
        init { require(days.isNotEmpty()) { "Weekly schedule must have at least one day" } }
    }

    @Serializable
    @SerialName("monthly_date")
    data class MonthlyDate(val dayOfMonth: Int) : Schedule {
        init { require(dayOfMonth in 1..31) { "Day of month must be 1-31" } }
    }

    @Serializable
    @SerialName("monthly_relative")
    data class MonthlyRelative(val week: WeekOfMonth, val day: DayOfWeek) : Schedule

    @Serializable
    @SerialName("interval")
    data class Interval(val days: Int) : Schedule {
        init { require(days >= 1) { "Interval must be at least 1 day" } }
    }
}
```

**Rationale:** Sealed interface with polymorphic serialization provides type-safe schedule patterns.
Each variant carries only the data it needs.

### 6. Common Interfaces

```kotlin
interface Timestamped {
    val createdAt: Instant
    val updatedAt: Instant
}

interface SoftDeletable {
    val deletedAt: Instant?

    val isDeleted: Boolean get() = deletedAt != null
}
```

**Rationale:** Interfaces provide consistent behavior across entities that share these patterns.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Large number of files | Organized package structure, IDE navigation |
| Validation logic in constructors | Clear error messages, unit tests |
| Serialization compatibility across versions | Careful `@SerialName` usage, schema tests |
| ULID dependency | Use existing kotlin-ulid or simple implementation |

## Migration Plan

This is greenfield development—no migration needed. Future schema changes will require:

1. Adding new fields with defaults
2. Using `@Deprecated` for removed fields during transition
3. Version negotiation in RPC layer

## Open Questions

1. **ULID library choice**: Use `com.github.guepardoapps:kotlin-ulid` or implement minimal
   generator? → Recommend minimal implementation to avoid dependency.

2. **Arrow Optics usage**: Generate optics for all entities or start without? →
   Recommend deferring optics until needed; add `@optics` annotation when use case arises.

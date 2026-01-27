# ADR-013: Routine Scheduling Strategy

| Field             | Value           |
| ----------------- | --------------- |
| **Status**        | Accepted        |
| **Date**          | 2026-01-14      |
| **Deciders**      | Robert Hamilton |
| **Supersedes**    | —               |
| **Superseded by** | —               |

## Context

Users have recurring tasks that need to happen on a schedule — taking medication, paying bills,
weekly reviews, etc. Traditional task managers handle this in different ways:

1. **Repeating tasks**: Complete a task, it reappears with a new due date
2. **Recurring templates**: Define a template, system creates instances
3. **Calendar integration**: Sync with external calendar for scheduling

We needed to decide how Altair handles recurring work, balancing simplicity with flexibility.

Key questions:
- Should completed recurring tasks be their own entities (for history) or just timestamps?
- How do we handle "skip" vs "complete"?
- What schedule patterns should v1 support?
- How do notifications integrate?

## Decision

We chose **Routines as templates that spawn Quest instances** — separating the definition of what
repeats from the individual completable instances.

### Architecture

```
Routine (Template)
├── name, description
├── schedule (pattern definition)
├── time_of_day (optional notification time)
├── energy_cost (inherited by spawned Quests)
├── initiative_id (optional)
└── active (toggle generation on/off)

Quest (Instance)
├── Standard Quest entity
├── routine_id (links back to template)
└── Appears in Today view, Harvest, etc.
```

### Key Design Decisions

1. **Routines spawn Quests**: Each instance is a real Quest entity with full history
2. **Near-term generation only**: System doesn't create 52 weeks of future instances
3. **Skip without breaking**: Skipping an instance doesn't affect the routine
4. **Complete early**: Completing before due date advances to next occurrence
5. **Time of day for notifications**: Optional specific time triggers push/toast

### Schedule Patterns (v1)

| Pattern | Syntax | Example |
|---------|--------|---------|
| Daily | `daily` | Every day |
| Weekly | `weekly:1,3,5` | Mon, Wed, Fri |
| Monthly date | `monthly:15` | 15th of each month |
| Monthly relative | `monthly:first:1` | First Monday |
| Interval | `interval:3` | Every 3 days |

### v1.1 Extensions

- Weekdays only / weekends only
- Complex patterns ("every other Tuesday")
- Seasonal schedules
- Time range constraints

## Alternatives Considered

### Repeating Task Model

Task has a "repeat" setting; completing it creates a new copy:

```
Quest
├── repeat_pattern: "weekly"
├── on_complete: create new Quest with due_date + 7 days
```

**Rejected because:**
- Muddles Quest entity with scheduling logic
- No clear template to edit
- History tied to individual Quests, not the routine concept
- Harder to "pause" a routine

### Calendar-Based Scheduling

Sync with external calendar (Google Calendar, iCal):

**Rejected because:**
- Creates external dependency
- Offline complexity
- Privacy implications (calendar data leaves system)
- Different UX expectations

### Cron-Style Strings

Use cron syntax for schedule definition:

```
schedule: "0 8 * * 1-5"  // 8 AM weekdays
```

**Rejected because:**
- Unfamiliar to most users
- Error-prone to edit
- Overkill for personal routines
- Hard to display in UI

We use a structured format that's easier to validate and display.

## Consequences

### Positive

- **Clear separation**: Routine template vs. Quest instance
- **Full history**: Each completion is a real Quest in Harvest
- **Flexible control**: Skip, pause, complete early without breaking
- **Notification support**: Time of day enables push/toast reminders
- **Energy-aware**: Routine instances have energy cost like any Quest

### Negative

- **Instance generation complexity**: System must generate at the right time
- **Orphan handling**: What happens to instances if Routine is deleted?
- **Timezone complexity**: Schedules depend on user's timezone

### Mitigations

- **Server-side generation**: Scheduler runs on server for reliability
- **Soft delete**: Deleting Routine keeps orphan instances
- **User timezone**: Stored in user profile, used for all scheduling

## Implementation Notes

### Routine Entity

```kotlin
data class Routine(
    val id: String,
    val userId: String,
    val name: String,
    val description: String?,
    val schedule: Schedule,
    val timeOfDay: LocalTime?,
    val energyCost: Int,
    val initiativeId: String?,
    val active: Boolean,
    val nextDue: Instant?,
    val createdAt: Instant,
    val updatedAt: Instant
)

sealed class Schedule {
    object Daily : Schedule()
    data class Weekly(val days: Set<DayOfWeek>) : Schedule()
    data class MonthlyDate(val dayOfMonth: Int) : Schedule()
    data class MonthlyRelative(val week: WeekOfMonth, val day: DayOfWeek) : Schedule()
    data class Interval(val days: Int) : Schedule()
}
```

### Instance Generation

```kotlin
class RoutineScheduler(
    private val routineRepository: RoutineRepository,
    private val questRepository: QuestRepository,
    private val eventBus: EventBus
) {
    suspend fun generateDueInstances() {
        val now = Clock.System.now()
        val dueRoutines = routineRepository.findDue(now)
        
        for (routine in dueRoutines) {
            // Create Quest instance
            val quest = questRepository.create(
                QuestCreate(
                    title = routine.name,
                    description = routine.description,
                    energyCost = routine.energyCost,
                    routineId = routine.id
                )
            )
            
            // Advance routine to next occurrence
            val nextDue = calculateNextDue(routine.schedule, now)
            routineRepository.updateNextDue(routine.id, nextDue)
            
            eventBus.publish(RoutineEvent.RoutineInstanceSpawned(routine, quest))
        }
    }
}
```

### Notification Flow

```
1. Routine has time_of_day = 08:00
2. Scheduler generates instance when date matches schedule
3. Notification service schedules push/toast for 08:00
4. At 08:00: "Time for: Take morning meds"
5. User completes Quest → normal completion flow
```

### Skip vs. Complete

| Action | Result |
|--------|--------|
| Complete | Quest moves to Harvested; next instance on schedule |
| Skip | Quest deleted; next instance on schedule |
| Pause routine | No new instances until resumed |
| Complete early | Current instance done; next instance on schedule |

## References

- [altair-prd-core.md §10](../requirements/altair-prd-core.md) — Routine requirements
- [domain-model.md](../architecture/domain-model.md) — Routine entity definition
- [event-bus.md](../architecture/event-bus.md) — RoutineEvent types

---

_Decision made: 2026-01-14_

package com.getaltair.altair.shared.domain.guidance

import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Quest: An actionable task with energy cost tracking for ADHD-aware planning.
 *
 * Quests are the core unit of task execution in Altair's Guidance module. Each quest
 * has an energy cost (1-5) that represents cognitive/emotional load, enabling daily
 * capacity-based planning.
 *
 * ## Energy Cost Scale
 * - **1**: Trivial (< 5 min, minimal focus required)
 * - **2**: Quick (5-15 min, routine task)
 * - **3**: Moderate (30-60 min, requires focused attention)
 * - **4**: Heavy (2-3 hours, cognitively demanding)
 * - **5**: Intense (half-day or more, deep work required)
 *
 * ## WIP=1 Enforcement
 * Only ONE quest can be in ACTIVE status at a time per user. This constraint is enforced
 * at the business logic layer, not here in the domain entity.
 *
 * ## Routine-Spawned Quests
 * If a quest is spawned from a Routine template, [routineId] will reference the originating
 * routine. This enables tracking of recurring task instances.
 *
 * ## Lifecycle
 * - **BACKLOG**: Planned but not started
 * - **ACTIVE**: Currently being worked on (WIP=1)
 * - **COMPLETED**: Successfully finished
 * - **ABANDONED**: Intentionally cancelled or no longer relevant
 *
 * ## Soft Delete
 * Quests are soft-deleted via [deletedAt]. Deleted quests are excluded from queries
 * but preserved for audit purposes.
 *
 * @property id Unique identifier for this Quest
 * @property userId Owner of this Quest
 * @property title Short task description (max 200 characters)
 * @property description Optional detailed explanation or context
 * @property energyCost Cognitive/emotional load estimate (1-5 scale)
 * @property status Current execution status (BACKLOG, ACTIVE, COMPLETED, ABANDONED)
 * @property epicId Optional link to parent Epic for grouping
 * @property routineId Optional link to originating Routine if spawned from template
 * @property createdAt Timestamp when Quest was created
 * @property updatedAt Timestamp of last modification
 * @property startedAt Timestamp when status changed to ACTIVE (if applicable)
 * @property completedAt Timestamp when status changed to COMPLETED (if applicable)
 * @property deletedAt Timestamp of soft deletion (null if not deleted)
 *
 * @throws IllegalArgumentException if title is blank, exceeds 200 characters, or energyCost is outside 1-5 range
 */
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
    val deletedAt: Instant?
) {
    init {
        require(title.length <= 200) { "Quest title must not exceed 200 characters, got ${title.length}" }
        require(title.isNotBlank()) { "Quest title cannot be blank" }
        require(energyCost in 1..5) { "Energy cost must be in range 1-5, got $energyCost" }
    }

    /**
     * Returns true if this Quest is in ACTIVE status (currently being worked on).
     */
    val isActive: Boolean get() = status == QuestStatus.ACTIVE

    /**
     * Returns true if this Quest is in COMPLETED status.
     */
    val isCompleted: Boolean get() = status == QuestStatus.COMPLETED

    /**
     * Returns true if this Quest is in ABANDONED status.
     */
    val isAbandoned: Boolean get() = status == QuestStatus.ABANDONED

    /**
     * Returns true if this Quest is in BACKLOG status (not yet started).
     */
    val isInBacklog: Boolean get() = status == QuestStatus.BACKLOG

    /**
     * Returns true if this Quest was spawned from a Routine template.
     */
    val isFromRoutine: Boolean get() = routineId != null

    /**
     * Returns true if this Quest has been soft-deleted.
     */
    val isDeleted: Boolean get() = deletedAt != null
}

package com.getaltair.altair.domain.model.guidance

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.QuestStatus
import kotlinx.datetime.LocalDate
import kotlinx.serialization.Serializable
import kotlin.time.Instant

/**
 * The core unit of work in the Guidance module.
 *
 * Quests are actionable tasks with an associated energy cost (1-5).
 * They can be standalone, part of an Epic, or spawned from a Routine.
 * The energy cost helps with daily planning and workload management.
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
    val initiativeId: Ulid?,
    val dueDate: LocalDate?,
    val scheduledDate: LocalDate?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    val startedAt: Instant?,
    val completedAt: Instant?,
    override val deletedAt: Instant? = null,
) : Timestamped,
    SoftDeletable {
    init {
        require(title.isNotBlank()) { "Quest title must not be blank" }
        require(title.length <= 200) { "Quest title must be at most 200 characters" }
        require(energyCost in 1..5) { "Energy cost must be between 1 and 5" }

        // State consistency: startedAt required for active quests
        if (status == QuestStatus.ACTIVE) {
            require(startedAt != null) { "Active quests must have startedAt" }
        }

        // State consistency: completedAt required for terminal states
        if (status == QuestStatus.COMPLETED || status == QuestStatus.ABANDONED) {
            require(completedAt != null) { "Completed or abandoned quests must have completedAt" }
        }

        // State consistency: startedAt must be before completedAt
        if (startedAt != null && completedAt != null) {
            require(startedAt <= completedAt) { "startedAt must be before or equal to completedAt" }
        }
    }
}

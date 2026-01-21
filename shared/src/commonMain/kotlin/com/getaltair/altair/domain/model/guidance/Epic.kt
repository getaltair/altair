package com.getaltair.altair.domain.model.guidance

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.EpicStatus
import kotlinx.datetime.LocalDate
import kotlinx.serialization.Serializable
import kotlin.time.Instant

/**
 * A large goal that is broken down into smaller Quests.
 *
 * Epics represent significant accomplishments that require multiple steps.
 * They provide high-level organization for related Quests and can optionally
 * have target dates for planning purposes.
 */
@Serializable
data class Epic(
    val id: Ulid,
    val userId: Ulid,
    val title: String,
    val description: String?,
    val status: EpicStatus,
    val initiativeId: Ulid?,
    val targetDate: LocalDate?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    val completedAt: Instant?,
    override val deletedAt: Instant? = null,
) : Timestamped,
    SoftDeletable {
    init {
        require(title.isNotBlank()) { "Epic title must not be blank" }
        require(title.length <= 200) { "Epic title must be at most 200 characters" }

        // State consistency: completedAt required when status is COMPLETED
        if (status == EpicStatus.COMPLETED) {
            require(completedAt != null) { "Completed epics must have completedAt" }
        }
    }
}

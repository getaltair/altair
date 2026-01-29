package com.getaltair.altair.shared.domain.guidance

import com.getaltair.altair.shared.domain.common.EpicStatus
import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Epic: A grouping of related Quests that represents a larger body of work.
 *
 * Epics provide organizational hierarchy within the Guidance module, allowing users to
 * group thematically related tasks. An Epic can optionally be associated with an Initiative
 * (from the cross-module system) for broader project/area alignment.
 *
 * ## Lifecycle
 * - **ACTIVE**: Currently accepting work, quests can be added/completed
 * - **COMPLETED**: All associated quests are done, no further work expected
 * - **ARCHIVED**: Preserved for reference, no longer active
 *
 * ## Soft Delete
 * Epics are soft-deleted via the [deletedAt] timestamp. Deleted epics are hidden from
 * standard queries but preserved for audit/recovery purposes.
 *
 * @property id Unique identifier for this Epic
 * @property userId Owner of this Epic
 * @property title Short descriptive name (max 200 characters)
 * @property description Optional detailed explanation or context
 * @property status Current lifecycle status (ACTIVE, COMPLETED, ARCHIVED)
 * @property initiativeId Optional link to broader Initiative (cross-module)
 * @property createdAt Timestamp when Epic was created
 * @property updatedAt Timestamp of last modification
 * @property completedAt Timestamp when status changed to COMPLETED (if applicable)
 * @property deletedAt Timestamp of soft deletion (null if not deleted)
 *
 * @throws IllegalArgumentException if title is blank or exceeds 200 characters
 */
@Serializable
data class Epic(
    val id: Ulid,
    val userId: Ulid,
    val title: String,
    val description: String?,
    val status: EpicStatus,
    val initiativeId: Ulid?,
    val createdAt: Instant,
    val updatedAt: Instant,
    val completedAt: Instant?,
    val deletedAt: Instant?
) {
    init {
        require(title.length <= 200) { "Epic title must not exceed 200 characters, got ${title.length}" }
        require(title.isNotBlank()) { "Epic title cannot be blank" }
    }

    /**
     * Returns true if this Epic has been soft-deleted.
     */
    val isDeleted: Boolean get() = deletedAt != null

    /**
     * Returns true if this Epic is in COMPLETED status.
     */
    val isCompleted: Boolean get() = status == EpicStatus.COMPLETED

    /**
     * Returns true if this Epic is in ARCHIVED status.
     */
    val isArchived: Boolean get() = status == EpicStatus.ARCHIVED
}

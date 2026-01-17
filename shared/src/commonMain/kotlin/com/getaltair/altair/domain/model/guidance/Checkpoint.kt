package com.getaltair.altair.domain.model.guidance

import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * An optional sub-step within a Quest.
 *
 * Checkpoints break down a Quest into smaller, trackable steps.
 * They are simpler than Quests (no energy cost, no status beyond complete/incomplete)
 * and are ordered within their parent Quest.
 */
@Serializable
data class Checkpoint(
    val id: Ulid,
    val userId: Ulid,
    val questId: Ulid,
    val title: String,
    val sortOrder: Int,
    val isCompleted: Boolean,
    val completedAt: Instant?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
) : Timestamped {
    init {
        require(title.isNotBlank()) { "Checkpoint title must not be blank" }
        require(title.length <= 200) { "Checkpoint title must be at most 200 characters" }
        require(sortOrder >= 0) { "Sort order must be non-negative" }

        // State consistency: completedAt must align with isCompleted
        if (isCompleted) {
            require(completedAt != null) { "Completed checkpoints must have completedAt" }
        } else {
            require(completedAt == null) { "Incomplete checkpoints must not have completedAt" }
        }
    }
}

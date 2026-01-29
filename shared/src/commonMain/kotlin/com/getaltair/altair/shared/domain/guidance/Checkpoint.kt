package com.getaltair.altair.shared.domain.guidance

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Checkpoint: A discrete sub-step within a Quest, enabling progress tracking.
 *
 * Checkpoints break down larger quests into smaller, actionable steps. They provide
 * visual progress indicators and help users maintain momentum by showing incremental
 * completion. Checkpoints are ordered within their parent Quest.
 *
 * ## Use Cases
 * - Multi-step tasks that benefit from explicit sequencing
 * - Processes with clear milestones or stages
 * - Complex work requiring progress visibility beyond binary done/not-done
 *
 * ## Ordering
 * The [order] field determines display sequence within the Quest. Lower values appear
 * first. Order values should be unique per Quest but gaps are allowed (0, 10, 20, etc.)
 * to facilitate reordering without reassigning all checkpoints.
 *
 * ## Completion Tracking
 * The [completed] boolean tracks whether this checkpoint has been reached. The optional
 * [completedAt] timestamp records when completion occurred (null if not completed).
 *
 * @property id Unique identifier for this Checkpoint
 * @property questId Parent Quest this Checkpoint belongs to
 * @property title Description of this checkpoint step (max 200 characters)
 * @property completed Whether this checkpoint has been completed
 * @property order Display sequence within parent Quest (lower = earlier)
 * @property completedAt Timestamp when checkpoint was marked complete (null if incomplete)
 *
 * @throws IllegalArgumentException if title is blank, exceeds 200 characters, or order is negative
 */
@Serializable
data class Checkpoint(
    val id: Ulid,
    val questId: Ulid,
    val title: String,
    val completed: Boolean,
    val order: Int,
    val completedAt: Instant?
) {
    init {
        require(title.length <= 200) { "Checkpoint title must not exceed 200 characters, got ${title.length}" }
        require(title.isNotBlank()) { "Checkpoint title cannot be blank" }
        require(order >= 0) { "Checkpoint order must be non-negative, got $order" }
    }
}

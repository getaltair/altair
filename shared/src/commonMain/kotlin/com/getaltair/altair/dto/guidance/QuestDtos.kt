package com.getaltair.altair.dto.guidance

import kotlinx.serialization.Serializable

/**
 * Request to create a new quest.
 */
@Serializable
data class CreateQuestRequest(
    val title: String,
    val description: String? = null,
    val energyCost: Int,
    val epicId: String? = null,
    val initiativeId: String? = null,
    val dueDate: String? = null,
    val scheduledDate: String? = null,
)

/**
 * Request to update an existing quest.
 */
@Serializable
data class UpdateQuestRequest(
    val title: String? = null,
    val description: String? = null,
    val energyCost: Int? = null,
    val epicId: String? = null,
    val initiativeId: String? = null,
    val dueDate: String? = null,
    val scheduledDate: String? = null,
)

/**
 * Request to transition a quest to a new status.
 */
@Serializable
data class TransitionQuestStatusRequest(
    val newStatus: String,
)

/**
 * Response containing quest data.
 */
@Serializable
data class QuestResponse(
    val id: String,
    val title: String,
    val description: String?,
    val energyCost: Int,
    val status: String,
    val epicId: String?,
    val routineId: String?,
    val initiativeId: String?,
    val dueDate: String?,
    val scheduledDate: String?,
    val createdAt: String,
    val updatedAt: String,
    val startedAt: String?,
    val completedAt: String?,
)

/**
 * Request to create a new epic.
 */
@Serializable
data class CreateEpicRequest(
    val title: String,
    val description: String? = null,
    val initiativeId: String? = null,
    val targetDate: String? = null,
)

/**
 * Request to update an existing epic.
 */
@Serializable
data class UpdateEpicRequest(
    val title: String? = null,
    val description: String? = null,
    val initiativeId: String? = null,
    val targetDate: String? = null,
)

/**
 * Response containing epic data with progress.
 */
@Serializable
data class EpicResponse(
    val id: String,
    val title: String,
    val description: String?,
    val status: String,
    val initiativeId: String?,
    val targetDate: String?,
    val createdAt: String,
    val updatedAt: String,
    val completedAt: String?,
    val progress: EpicProgressResponse,
)

/**
 * Epic progress statistics.
 */
@Serializable
data class EpicProgressResponse(
    val totalQuests: Int,
    val completedQuests: Int,
    val totalEnergy: Int,
    val spentEnergy: Int,
    val completionPercent: Int,
)

/**
 * Request to create a new checkpoint.
 */
@Serializable
data class CreateCheckpointRequest(
    val questId: String,
    val title: String,
    val sortOrder: Int? = null,
)

/**
 * Request to update an existing checkpoint.
 */
@Serializable
data class UpdateCheckpointRequest(
    val title: String? = null,
)

/**
 * Response containing checkpoint data.
 */
@Serializable
data class CheckpointResponse(
    val id: String,
    val questId: String,
    val title: String,
    val sortOrder: Int,
    val isCompleted: Boolean,
    val completedAt: String?,
    val createdAt: String,
    val updatedAt: String,
)

/**
 * Request to reorder checkpoints.
 */
@Serializable
data class ReorderCheckpointsRequest(
    val questId: String,
    val orderedIds: List<String>,
)

package com.getaltair.altair.dto.guidance

import kotlinx.serialization.Serializable

/**
 * Request to create a new quest.
 *
 * @property title The quest title (1-200 characters)
 * @property description Optional detailed description
 * @property energyCost Energy cost from 1-5
 * @property epicId Optional epic ULID (26-character string)
 * @property initiativeId Optional initiative ULID (26-character string)
 * @property dueDate Optional due date in ISO-8601 format (YYYY-MM-DD)
 * @property scheduledDate Optional scheduled date in ISO-8601 format (YYYY-MM-DD)
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
 *
 * ## String Formats
 * - **IDs** (`id`, `epicId`, `routineId`, `initiativeId`): ULID format (26-character string)
 * - **Dates** (`dueDate`, `scheduledDate`): ISO-8601 date format (YYYY-MM-DD)
 * - **Timestamps** (`createdAt`, `updatedAt`, `startedAt`, `completedAt`): ISO-8601 datetime (YYYY-MM-DDTHH:MM:SSZ)
 * - **Status**: One of: backlog, active, completed, abandoned
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

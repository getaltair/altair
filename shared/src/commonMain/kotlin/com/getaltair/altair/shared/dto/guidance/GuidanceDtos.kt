package com.getaltair.altair.shared.dto.guidance

import com.getaltair.altair.shared.domain.common.EpicStatus
import com.getaltair.altair.shared.domain.common.QuestStatus
import kotlinx.serialization.Serializable

// === Request DTOs ===

/**
 * Request to create a new Quest.
 *
 * @property title The quest title (required)
 * @property description Optional detailed description
 * @property energyCost Energy units required (1-10 scale)
 * @property epicId Optional parent Epic ID for grouping
 */
@Serializable
data class CreateQuestRequest(
    val title: String,
    val description: String? = null,
    val energyCost: Int,
    val epicId: String? = null
)

/**
 * Request to update an existing Quest.
 * All fields are optional - only provided fields will be updated.
 *
 * @property title New title
 * @property description New description
 * @property energyCost New energy cost
 * @property epicId New parent Epic ID (use null to unlink)
 */
@Serializable
data class UpdateQuestRequest(
    val title: String? = null,
    val description: String? = null,
    val energyCost: Int? = null,
    val epicId: String? = null
)

/**
 * Request to create a new Epic.
 *
 * @property title The epic title (required)
 * @property description Optional detailed description
 * @property initiativeId Optional parent Initiative ID for cross-module organization
 */
@Serializable
data class CreateEpicRequest(
    val title: String,
    val description: String? = null,
    val initiativeId: String? = null
)

/**
 * Request to update an existing Epic.
 * All fields are optional - only provided fields will be updated.
 *
 * @property title New title
 * @property description New description
 * @property initiativeId New parent Initiative ID (use null to unlink)
 */
@Serializable
data class UpdateEpicRequest(
    val title: String? = null,
    val description: String? = null,
    val initiativeId: String? = null
)

/**
 * Request to create a new Checkpoint within a Quest.
 *
 * @property questId Parent Quest ID (required)
 * @property title Checkpoint description (required)
 * @property order Display order (optional, auto-assigned if not provided)
 */
@Serializable
data class CreateCheckpointRequest(
    val questId: String,
    val title: String,
    val order: Int? = null
)

/**
 * Request to update an existing Checkpoint.
 * All fields are optional - only provided fields will be updated.
 *
 * @property title New checkpoint description
 * @property completed Completion status
 * @property order New display order
 */
@Serializable
data class UpdateCheckpointRequest(
    val title: String? = null,
    val completed: Boolean? = null,
    val order: Int? = null
)

/**
 * Request to set the daily energy budget for a specific date.
 *
 * @property date Date in YYYY-MM-DD format
 * @property budget Available energy units for the day
 */
@Serializable
data class SetEnergyBudgetRequest(
    val date: String,
    val budget: Int
)

/**
 * Request to reorder checkpoints within a Quest.
 *
 * @property order Complete ordered list of checkpoint IDs
 */
@Serializable
data class ReorderCheckpointsRequest(
    val order: List<String>
)

// === Response DTOs ===

/**
 * Full Quest representation for API responses.
 *
 * @property id Unique Quest identifier
 * @property title Quest title
 * @property description Detailed description (nullable)
 * @property energyCost Energy units required (1-10 scale)
 * @property status Current lifecycle status
 * @property epicId Parent Epic ID (nullable)
 * @property routineId Parent Routine ID if this is a routine instance (nullable)
 * @property checkpoints List of all checkpoints in display order
 * @property createdAt ISO 8601 timestamp
 * @property updatedAt ISO 8601 timestamp
 * @property startedAt ISO 8601 timestamp when quest became Active (nullable)
 * @property completedAt ISO 8601 timestamp when quest completed (nullable)
 */
@Serializable
data class QuestResponse(
    val id: String,
    val title: String,
    val description: String?,
    val energyCost: Int,
    val status: QuestStatus,
    val epicId: String?,
    val routineId: String?,
    val checkpoints: List<CheckpointResponse>,
    val createdAt: String,
    val updatedAt: String,
    val startedAt: String?,
    val completedAt: String?
)

/**
 * Lightweight Quest summary for list views.
 *
 * @property id Unique Quest identifier
 * @property title Quest title
 * @property energyCost Energy units required
 * @property status Current lifecycle status
 * @property epicId Parent Epic ID (nullable)
 * @property checkpointCount Total number of checkpoints
 * @property completedCheckpointCount Number of completed checkpoints
 */
@Serializable
data class QuestSummaryResponse(
    val id: String,
    val title: String,
    val energyCost: Int,
    val status: QuestStatus,
    val epicId: String?,
    val checkpointCount: Int,
    val completedCheckpointCount: Int
)

/**
 * Checkpoint representation for API responses.
 *
 * @property id Unique Checkpoint identifier
 * @property title Checkpoint description
 * @property completed Completion status
 * @property order Display order within parent Quest
 * @property completedAt ISO 8601 timestamp when completed (nullable)
 */
@Serializable
data class CheckpointResponse(
    val id: String,
    val title: String,
    val completed: Boolean,
    val order: Int,
    val completedAt: String?
)

/**
 * Epic representation for API responses.
 *
 * @property id Unique Epic identifier
 * @property title Epic title
 * @property description Detailed description (nullable)
 * @property status Current lifecycle status
 * @property initiativeId Parent Initiative ID (nullable)
 * @property questCount Total number of child Quests
 * @property completedQuestCount Number of completed Quests
 * @property createdAt ISO 8601 timestamp
 * @property completedAt ISO 8601 timestamp when all Quests completed (nullable)
 */
@Serializable
data class EpicResponse(
    val id: String,
    val title: String,
    val description: String?,
    val status: EpicStatus,
    val initiativeId: String?,
    val questCount: Int,
    val completedQuestCount: Int,
    val createdAt: String,
    val completedAt: String?
)

/**
 * Energy budget information for a specific date.
 *
 * @property date Date in YYYY-MM-DD format
 * @property budget Total available energy units
 * @property spent Energy units consumed by completed/active Quests
 * @property remaining Remaining energy units
 * @property percentUsed Percentage of budget used (0.0-1.0)
 */
@Serializable
data class EnergyBudgetResponse(
    val date: String,
    val budget: Int,
    val spent: Int,
    val remaining: Int,
    val percentUsed: Float
)

/**
 * Complete view for "Today" screen showing current day's context.
 *
 * @property date Current date in YYYY-MM-DD format
 * @property energyBudget Energy budget details for today
 * @property activeQuest Currently in-progress Quest (nullable, WIP=1 enforcement)
 * @property readyQuests Available Quests that can be started
 * @property routineInstances Routine instances due today
 * @property completedToday Quests completed today
 */
@Serializable
data class TodayViewResponse(
    val date: String,
    val energyBudget: EnergyBudgetResponse,
    val activeQuest: QuestResponse?,
    val readyQuests: List<QuestSummaryResponse>,
    val routineInstances: List<QuestSummaryResponse>,
    val completedToday: List<QuestSummaryResponse>
)

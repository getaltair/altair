package com.getaltair.altair.domain

import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.QuestStatus
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Errors specific to Quest operations in the Guidance module.
 *
 * These errors extend [DomainError] to enable exhaustive when-matching
 * for Quest-specific error handling while maintaining compatibility
 * with generic error handlers.
 */
@Serializable
sealed interface QuestError : DomainError {
    /**
     * The requested quest could not be found.
     *
     * @property id The ULID of the quest that was not found
     */
    @Serializable
    @SerialName("quest_not_found")
    data class NotFound(val id: Ulid) : QuestError {
        override fun toUserMessage(): String = "The requested quest could not be found."
    }

    /**
     * The quest's energy cost would exceed the user's daily energy budget.
     *
     * @property required The energy cost of the quest
     * @property available The remaining energy in the user's budget
     */
    @Serializable
    @SerialName("quest_energy_budget_exceeded")
    data class EnergyBudgetExceeded(
        val required: Int,
        val available: Int,
    ) : QuestError {
        override fun toUserMessage(): String =
            "Not enough energy available. This quest requires $required energy, but only $available remains."
    }

    /**
     * The requested status transition is not valid for the quest's current state.
     *
     * @property questId The ULID of the quest
     * @property currentStatus The quest's current status
     * @property targetStatus The status being transitioned to
     */
    @Serializable
    @SerialName("quest_invalid_status_transition")
    data class InvalidStatusTransition(
        val questId: Ulid,
        val currentStatus: QuestStatus,
        val targetStatus: QuestStatus,
    ) : QuestError {
        override fun toUserMessage(): String =
            "Cannot change quest from ${currentStatus.name.lowercase()} to ${targetStatus.name.lowercase()}."
    }

    /**
     * The user has reached the maximum number of active (work-in-progress) quests.
     *
     * @property currentWip The current number of active quests
     * @property maxWip The maximum allowed number of active quests
     */
    @Serializable
    @SerialName("quest_wip_limit_exceeded")
    data class WipLimitExceeded(
        val currentWip: Int,
        val maxWip: Int,
    ) : QuestError {
        override fun toUserMessage(): String =
            "You have reached the maximum of $maxWip active quests. Complete or abandon a quest first."
    }
}

package com.getaltair.altair.domain

import com.getaltair.altair.domain.types.Ulid
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Errors specific to Epic operations in the Guidance module.
 *
 * These errors extend [DomainError] to enable exhaustive when-matching
 * for Epic-specific error handling while maintaining compatibility
 * with generic error handlers.
 */
@Serializable
sealed interface EpicError : DomainError {
    /**
     * The requested epic could not be found.
     *
     * @property id The ULID of the epic that was not found
     */
    @Serializable
    @SerialName("epic_not_found")
    data class NotFound(
        val id: Ulid,
    ) : EpicError {
        override fun toUserMessage(): String = "The requested epic could not be found."
    }

    /**
     * The epic cannot be modified because it has active quests.
     *
     * @property epicId The ULID of the epic
     * @property activeQuestCount The number of active quests preventing modification
     */
    @Serializable
    @SerialName("epic_has_active_quests")
    data class HasActiveQuests(
        val epicId: Ulid,
        val activeQuestCount: Int,
    ) : EpicError {
        override fun toUserMessage(): String {
            val suffix = if (activeQuestCount != 1) "s" else ""
            return "Cannot modify this epic because it has $activeQuestCount active quest$suffix."
        }
    }

    /**
     * The epic's status transition is not valid.
     *
     * @property epicId The ULID of the epic
     * @property currentStatus The epic's current status as a string
     * @property targetStatus The status being transitioned to as a string
     */
    @Serializable
    @SerialName("epic_invalid_status_transition")
    data class InvalidStatusTransition(
        val epicId: Ulid,
        val currentStatus: String,
        val targetStatus: String,
    ) : EpicError {
        override fun toUserMessage(): String = "Cannot change epic from $currentStatus to $targetStatus."
    }

    /**
     * The initiative associated with the epic could not be found.
     *
     * @property initiativeId The ULID of the initiative that was not found
     */
    @Serializable
    @SerialName("epic_initiative_not_found")
    data class InitiativeNotFound(
        val initiativeId: Ulid,
    ) : EpicError {
        override fun toUserMessage(): String = "The associated initiative could not be found."
    }
}

package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Lifecycle status of an Epic (large goal containing Quests).
 */
@Serializable
enum class EpicStatus {
    /** In progress with remaining Quests */
    @SerialName("active")
    ACTIVE,

    /** All Quests completed successfully */
    @SerialName("completed")
    COMPLETED,

    /** No longer active, kept for reference */
    @SerialName("archived")
    ARCHIVED,
}

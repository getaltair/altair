package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Lifecycle status of an Initiative (project or area).
 */
@Serializable
enum class InitiativeStatus {
    /** Currently being worked on */
    @SerialName("active")
    ACTIVE,

    /** Temporarily on hold */
    @SerialName("paused")
    PAUSED,

    /** All work completed successfully */
    @SerialName("completed")
    COMPLETED,

    /** No longer active, kept for reference */
    @SerialName("archived")
    ARCHIVED,
}

package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Lifecycle status of a Quest (unit of work).
 */
@Serializable
enum class QuestStatus {
    /** Not yet started, waiting in backlog */
    @SerialName("backlog")
    BACKLOG,

    /** Currently being worked on */
    @SerialName("active")
    ACTIVE,

    /** Successfully finished */
    @SerialName("completed")
    COMPLETED,

    /** Deliberately stopped without completion */
    @SerialName("abandoned")
    ABANDONED,
}

package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Which week of the month for relative monthly schedules.
 */
@Serializable
enum class WeekOfMonth {
    /** First week (days 1-7) */
    @SerialName("first")
    FIRST,

    /** Second week (days 8-14) */
    @SerialName("second")
    SECOND,

    /** Third week (days 15-21) */
    @SerialName("third")
    THIRD,

    /** Fourth week (days 22-28) */
    @SerialName("fourth")
    FOURTH,

    /** Last occurrence in the month */
    @SerialName("last")
    LAST,
}

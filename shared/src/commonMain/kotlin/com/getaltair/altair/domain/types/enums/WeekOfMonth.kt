package com.getaltair.altair.domain.types.enums

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Which week of the month for relative monthly schedules.
 */
@Serializable
enum class WeekOfMonth {
    /** First occurrence of a day in the month (e.g., first Tuesday) */
    @SerialName("first")
    FIRST,

    /** Second occurrence of a day in the month (e.g., second Tuesday) */
    @SerialName("second")
    SECOND,

    /** Third occurrence of a day in the month (e.g., third Tuesday) */
    @SerialName("third")
    THIRD,

    /** Fourth occurrence of a day in the month (e.g., fourth Tuesday) */
    @SerialName("fourth")
    FOURTH,

    /** Last occurrence of a day in the month (e.g., last Tuesday) */
    @SerialName("last")
    LAST,
}

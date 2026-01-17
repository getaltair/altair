package com.getaltair.altair.domain.types

import com.getaltair.altair.domain.types.enums.WeekOfMonth
import kotlinx.datetime.DayOfWeek
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Represents a recurring schedule pattern for routines.
 *
 * Sealed interface with polymorphic serialization to support various
 * recurrence patterns while maintaining type safety.
 */
@Serializable
sealed interface Schedule {
    /**
     * Runs every day.
     */
    @Serializable
    @SerialName("daily")
    data object Daily : Schedule

    /**
     * Runs on specific days of the week.
     *
     * Days are stored in a sorted order (Monday first) to ensure deterministic
     * serialization and consistent equality comparisons.
     *
     * @property days The days of the week to run on (must have at least one)
     */
    @Serializable
    @SerialName("weekly")
    data class Weekly private constructor(
        @Suppress("ConstructorParameterNaming")
        private val sortedDays: List<DayOfWeek>,
    ) : Schedule {
        /** The days of the week to run on, sorted Monday-first */
        val days: Set<DayOfWeek> get() = sortedDays.toSet()

        init {
            require(sortedDays.isNotEmpty()) { "Weekly schedule must have at least one day" }
        }

        companion object {
            /** Creates a Weekly schedule with days sorted for deterministic serialization */
            operator fun invoke(days: Set<DayOfWeek>): Weekly {
                require(days.isNotEmpty()) { "Weekly schedule must have at least one day" }
                return Weekly(days.sortedBy { it.ordinal })
            }
        }
    }

    /**
     * Runs on a specific day of the month (e.g., the 15th).
     *
     * @property dayOfMonth The day of month (1-31)
     */
    @Serializable
    @SerialName("monthly_date")
    data class MonthlyDate(val dayOfMonth: Int) : Schedule {
        init {
            require(dayOfMonth in 1..31) { "Day of month must be 1-31, got $dayOfMonth" }
        }
    }

    /**
     * Runs on a relative day of the month (e.g., second Tuesday).
     *
     * @property week Which week of the month
     * @property day Which day of that week
     */
    @Serializable
    @SerialName("monthly_relative")
    data class MonthlyRelative(
        val week: WeekOfMonth,
        val day: DayOfWeek,
    ) : Schedule

    /**
     * Runs every N days from the last occurrence.
     *
     * @property days The interval in days (must be at least 1)
     */
    @Serializable
    @SerialName("interval")
    data class Interval(val days: Int) : Schedule {
        init {
            require(days >= 1) { "Interval must be at least 1 day, got $days" }
        }
    }
}

package com.getaltair.altair.shared.domain.common

import kotlinx.datetime.DateTimeUnit
import kotlinx.datetime.DayOfWeek
import kotlinx.datetime.LocalDate
import kotlinx.datetime.isoDayNumber
import kotlinx.datetime.plus
import kotlinx.serialization.Serializable
import kotlin.math.ceil

/**
 * Sealed interface representing various recurrence patterns for routine scheduling.
 * Each implementation provides calculation for the next occurrence date after a given date.
 */
@Serializable
sealed interface Schedule {
    /**
     * Calculates the next occurrence AFTER the given date (not including from).
     * @param from The reference date
     * @return The next occurrence date after from
     */
    fun nextOccurrence(from: LocalDate): LocalDate

    /**
     * Serializes the schedule to a string representation for persistence.
     * @return String representation following the format specification
     */
    fun toSerializedString(): String

    /**
     * Daily recurrence - occurs every day.
     */
    @Serializable
    data object Daily : Schedule {
        override fun nextOccurrence(from: LocalDate): LocalDate =
            from.plus(1, DateTimeUnit.DAY)

        override fun toSerializedString(): String = "daily"
    }

    /**
     * Weekly recurrence on specific days of the week.
     * @property daysOfWeek Set of days of the week when this recurs (must be non-empty)
     */
    @Serializable
    data class Weekly(val daysOfWeek: Set<DayOfWeek>) : Schedule {
        init {
            require(daysOfWeek.isNotEmpty()) {
                "Weekly schedule must have at least one day"
            }
        }

        override fun nextOccurrence(from: LocalDate): LocalDate {
            val sortedDays = daysOfWeek.map { it.isoDayNumber }.sorted()
            val currentDay = from.dayOfWeek.isoDayNumber

            // Find next day in current week
            val nextDayThisWeek = sortedDays.firstOrNull { it > currentDay }

            return if (nextDayThisWeek != null) {
                // Next occurrence is later this week
                from.plus(nextDayThisWeek - currentDay, DateTimeUnit.DAY)
            } else {
                // Next occurrence is in next week, use first day
                val daysUntilNextWeek = 7 - currentDay + sortedDays.first()
                from.plus(daysUntilNextWeek, DateTimeUnit.DAY)
            }
        }

        override fun toSerializedString(): String =
            "weekly:${daysOfWeek.map { it.isoDayNumber }.sorted().joinToString(",")}"
    }

    /**
     * Monthly recurrence on a specific day of the month.
     * @property dayOfMonth Day of month (1-31)
     */
    @Serializable
    data class MonthlyDate(val dayOfMonth: Int) : Schedule {
        init {
            require(dayOfMonth in 1..31) {
                "Day must be 1-31"
            }
        }

        override fun nextOccurrence(from: LocalDate): LocalDate {
            val currentMonth = from.month
            val currentYear = from.year

            // Try current month first
            val candidateThisMonth = try {
                LocalDate(currentYear, currentMonth, dayOfMonth)
            } catch (e: IllegalArgumentException) {
                null // Day doesn't exist in this month
            }

            if (candidateThisMonth != null && candidateThisMonth > from) {
                return candidateThisMonth
            }

            // Try subsequent months until we find one where this day exists
            var testDate = from.plus(1, DateTimeUnit.MONTH)
            while (true) {
                try {
                    val candidate = LocalDate(
                        testDate.year,
                        testDate.month,
                        dayOfMonth
                    )
                    return candidate
                } catch (e: IllegalArgumentException) {
                    // Day doesn't exist in this month, try next
                    testDate = testDate.plus(1, DateTimeUnit.MONTH)
                }
            }
        }

        override fun toSerializedString(): String = "monthly:$dayOfMonth"
    }

    /**
     * Monthly recurrence on a relative weekday position (e.g., "first Monday", "last Friday").
     * @property week Which occurrence of the weekday (FIRST, SECOND, THIRD, FOURTH, LAST)
     * @property dayOfWeek Day of the week
     */
    @Serializable
    data class MonthlyRelative(
        val week: RelativeWeek,
        val dayOfWeek: DayOfWeek
    ) : Schedule {
        override fun nextOccurrence(from: LocalDate): LocalDate {
            // Try current month first
            val candidateThisMonth = findOccurrenceInMonth(from.year, from.monthNumber)
            if (candidateThisMonth > from) {
                return candidateThisMonth
            }

            // Move to next month
            val nextMonth = from.plus(1, DateTimeUnit.MONTH)
            return findOccurrenceInMonth(nextMonth.year, nextMonth.monthNumber)
        }

        private fun findOccurrenceInMonth(year: Int, monthValue: Int): LocalDate {
            val firstDayOfMonth = LocalDate(year, monthValue, 1)
            val lastDayOfMonth = firstDayOfMonth.plus(1, DateTimeUnit.MONTH)
                .plus(-1, DateTimeUnit.DAY)

            return when (week) {
                RelativeWeek.LAST -> {
                    // Start from last day and work backwards
                    var candidate = lastDayOfMonth
                    while (candidate.dayOfWeek != dayOfWeek) {
                        candidate = candidate.plus(-1, DateTimeUnit.DAY)
                    }
                    candidate
                }
                else -> {
                    // Find first occurrence of the day, then add weeks
                    var candidate = firstDayOfMonth
                    while (candidate.dayOfWeek != dayOfWeek) {
                        candidate = candidate.plus(1, DateTimeUnit.DAY)
                    }

                    val weeksToAdd = when (week) {
                        RelativeWeek.FIRST -> 0
                        RelativeWeek.SECOND -> 1
                        RelativeWeek.THIRD -> 2
                        RelativeWeek.FOURTH -> 3
                        RelativeWeek.LAST -> throw IllegalStateException()
                    }

                    candidate.plus(weeksToAdd * 7, DateTimeUnit.DAY)
                }
            }
        }

        override fun toSerializedString(): String {
            val weekStr = when (week) {
                RelativeWeek.FIRST -> "first"
                RelativeWeek.SECOND -> "second"
                RelativeWeek.THIRD -> "third"
                RelativeWeek.FOURTH -> "fourth"
                RelativeWeek.LAST -> "last"
            }
            return "monthly:$weekStr:${dayOfWeek.isoDayNumber}"
        }
    }

    /**
     * Interval-based recurrence - occurs every N days.
     * @property days Number of days between occurrences (minimum 1)
     */
    @Serializable
    data class Interval(val days: Int) : Schedule {
        init {
            require(days >= 1) {
                "Interval must be at least 1 day"
            }
        }

        override fun nextOccurrence(from: LocalDate): LocalDate =
            from.plus(days, DateTimeUnit.DAY)

        override fun toSerializedString(): String = "interval:$days"
    }

    companion object {
        /**
         * Parses a serialized schedule string into a Schedule instance.
         * @param serialized String representation of the schedule
         * @return Parsed Schedule, or null if invalid format
         */
        fun parse(serialized: String): Schedule? {
            return try {
                when {
                    serialized == "daily" -> Daily

                    serialized.startsWith("weekly:") -> {
                        val dayNumbers = serialized.substringAfter("weekly:")
                            .split(",")
                            .map { it.toInt() }
                        val days = dayNumbers.map { dayNum ->
                            DayOfWeek.entries.first { it.isoDayNumber == dayNum }
                        }.toSet()
                        Weekly(days)
                    }

                    serialized.startsWith("monthly:") -> {
                        val parts = serialized.substringAfter("monthly:").split(":")
                        when (parts.size) {
                            1 -> {
                                // MonthlyDate format: "monthly:15"
                                val day = parts[0].toInt()
                                MonthlyDate(day)
                            }
                            2 -> {
                                // MonthlyRelative format: "monthly:first:1"
                                val weekStr = parts[0]
                                val dayNum = parts[1].toInt()

                                val week = when (weekStr) {
                                    "first" -> RelativeWeek.FIRST
                                    "second" -> RelativeWeek.SECOND
                                    "third" -> RelativeWeek.THIRD
                                    "fourth" -> RelativeWeek.FOURTH
                                    "last" -> RelativeWeek.LAST
                                    else -> return null
                                }

                                val day = DayOfWeek.entries.first { it.isoDayNumber == dayNum }
                                MonthlyRelative(week, day)
                            }
                            else -> null
                        }
                    }

                    serialized.startsWith("interval:") -> {
                        val days = serialized.substringAfter("interval:").toInt()
                        Interval(days)
                    }

                    else -> null
                }
            } catch (e: Exception) {
                null
            }
        }
    }
}

package com.getaltair.altair.data.util

import kotlinx.datetime.Clock
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime

/**
 * Utility functions for timestamp generation and formatting.
 *
 * All timestamps follow ISO 8601 format for consistency and interoperability.
 */
object Timestamps {
    /**
     * Returns the current timestamp in ISO 8601 format.
     *
     * Example: "2024-01-15T10:30:00Z"
     *
     * @return Current UTC timestamp as ISO 8601 string
     */
    fun now(): String = Clock.System.now().toString()

    /**
     * Parses an ISO 8601 timestamp string to an Instant.
     *
     * @param timestamp ISO 8601 formatted timestamp string
     * @return Parsed Instant
     * @throws IllegalArgumentException if the timestamp format is invalid
     */
    fun parse(timestamp: String): Instant = Instant.parse(timestamp)

    /**
     * Formats an Instant to ISO 8601 string.
     *
     * @param instant The Instant to format
     * @return ISO 8601 formatted timestamp string
     */
    fun format(instant: Instant): String = instant.toString()

    /**
     * Checks if a timestamp string is valid ISO 8601 format.
     *
     * @param timestamp The timestamp string to validate
     * @return true if valid, false otherwise
     */
    fun isValid(timestamp: String): Boolean = try {
        Instant.parse(timestamp)
        true
    } catch (e: Exception) {
        false
    }
}

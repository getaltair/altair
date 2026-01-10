package com.getaltair.altair.data.util

import kotlinx.datetime.Clock
import kotlin.random.Random

/**
 * ULID (Universally Unique Lexicographically Sortable Identifier) generator.
 *
 * ULIDs are 26 character strings that are:
 * - Lexicographically sortable
 * - Canonically encoded as a 26 character string
 * - 128 bits of entropy (48-bit timestamp + 80-bit randomness)
 * - Case insensitive
 * - No special characters (URL safe)
 *
 * Format: TTTTTTTTTTRRRRRRRRRRRRRRRRR
 * Where T = timestamp (10 chars), R = randomness (16 chars)
 */
object UlidGenerator {
    private const val ENCODING = "0123456789ABCDEFGHJKMNPQRSTVWXYZ"
    private const val ENCODING_LENGTH = 32
    private const val TIMESTAMP_LENGTH = 10
    private const val RANDOMNESS_LENGTH = 16

    /**
     * Generates a new ULID string.
     *
     * @return A 26-character ULID string
     */
    fun generate(): String {
        val timestamp = Clock.System.now().toEpochMilliseconds()
        return encodeTimestamp(timestamp) + encodeRandomness()
    }

    /**
     * Generates a new ULID with a specific timestamp.
     *
     * Useful for testing or when a specific timestamp is needed.
     *
     * @param timestampMs Unix timestamp in milliseconds
     * @return A 26-character ULID string
     */
    fun generate(timestampMs: Long): String = encodeTimestamp(timestampMs) + encodeRandomness()

    /**
     * Extracts the timestamp from a ULID string.
     *
     * @param ulid The ULID string
     * @return Unix timestamp in milliseconds
     * @throws IllegalArgumentException if the ULID is invalid
     */
    fun extractTimestamp(ulid: String): Long {
        require(ulid.length == 26) { "Invalid ULID length: ${ulid.length}" }
        val timestampPart = ulid.substring(0, TIMESTAMP_LENGTH).uppercase()
        return decodeTimestamp(timestampPart)
    }

    /**
     * Validates a ULID string format.
     *
     * @param ulid The string to validate
     * @return true if valid ULID format, false otherwise
     */
    fun isValid(ulid: String): Boolean {
        if (ulid.length != 26) return false
        return ulid.uppercase().all { it in ENCODING }
    }

    private fun encodeTimestamp(timestamp: Long): String {
        var ts = timestamp
        val chars = CharArray(TIMESTAMP_LENGTH)
        for (i in TIMESTAMP_LENGTH - 1 downTo 0) {
            chars[i] = ENCODING[(ts % ENCODING_LENGTH).toInt()]
            ts /= ENCODING_LENGTH
        }
        return chars.concatToString()
    }

    private fun decodeTimestamp(encoded: String): Long {
        var timestamp = 0L
        for (char in encoded) {
            val index = ENCODING.indexOf(char)
            require(index >= 0) { "Invalid character in ULID: $char" }
            timestamp = timestamp * ENCODING_LENGTH + index
        }
        return timestamp
    }

    private fun encodeRandomness(): String {
        val chars = CharArray(RANDOMNESS_LENGTH)
        for (i in 0 until RANDOMNESS_LENGTH) {
            chars[i] = ENCODING[Random.nextInt(ENCODING_LENGTH)]
        }
        return chars.concatToString()
    }
}

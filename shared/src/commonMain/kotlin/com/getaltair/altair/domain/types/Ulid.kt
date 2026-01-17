package com.getaltair.altair.domain.types

import kotlin.time.Clock
import kotlinx.serialization.Serializable
import kotlin.jvm.JvmInline
import kotlin.random.Random

private val ulidChars = "0123456789ABCDEFGHJKMNPQRSTVWXYZ".toSet()
private const val ULID_LENGTH = 26
private val ulidEncoding = "0123456789ABCDEFGHJKMNPQRSTVWXYZ".toCharArray()

/**
 * A ULID (Universally Unique Lexicographically Sortable Identifier).
 *
 * ULIDs are 128-bit identifiers that are:
 * - Lexicographically sortable (timestamp-based ordering)
 * - URL-safe (uses Crockford's Base32)
 * - Case-insensitive (normalized to uppercase on construction)
 * - 26 characters long
 *
 * Format: TTTTTTTTTTRRRRRRRRRRRRRRRRR
 * - 10 characters for timestamp (48 bits, millisecond precision)
 * - 16 characters for randomness (80 bits)
 */
@JvmInline
@Serializable
value class Ulid private constructor(val value: String) {
    override fun toString(): String = value

    companion object {
        /**
         * Creates a ULID from a string, normalizing to uppercase.
         * @throws IllegalArgumentException if the string is not a valid ULID
         */
        operator fun invoke(value: String): Ulid {
            val normalized = value.uppercase()
            require(normalized.length == ULID_LENGTH) {
                "ULID must be $ULID_LENGTH characters, got ${value.length}"
            }
            require(normalized.all { it in ulidChars }) {
                "Invalid ULID characters in '$value'"
            }
            return Ulid(normalized)
        }

        /**
         * Generates a new ULID using the current timestamp and random data.
         */
        fun generate(): Ulid = Ulid(generateUlidString())

        /**
         * Generates a ULID with a specific timestamp (for testing).
         */
        fun generate(timestamp: Long): Ulid = Ulid(generateUlidString(timestamp))

        private fun generateUlidString(timestamp: Long = currentTimeMillis()): String {
            val chars = CharArray(ULID_LENGTH)

            // Encode timestamp (first 10 characters)
            var ts = timestamp
            for (i in 9 downTo 0) {
                chars[i] = ulidEncoding[(ts and 0x1F).toInt()]
                ts = ts shr 5
            }

            // Encode randomness (last 16 characters)
            for (i in 10 until ULID_LENGTH) {
                chars[i] = ulidEncoding[Random.nextInt(32)]
            }

            return chars.concatToString()
        }

        private fun currentTimeMillis(): Long = Clock.System.now().toEpochMilliseconds()
    }
}

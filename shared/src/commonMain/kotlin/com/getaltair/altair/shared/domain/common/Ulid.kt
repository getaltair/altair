package com.getaltair.altair.shared.domain.common

import kotlinx.datetime.Instant
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlin.jvm.JvmInline
import kotlin.random.Random

/**
 * Universally Unique Lexicographically Sortable Identifier (ULID).
 *
 * A ULID is a 26-character string encoding a 48-bit timestamp and 80-bit random component.
 * ULIDs are:
 * - Lexicographically sortable by timestamp
 * - Case-insensitive (uppercase canonical form)
 * - URL-safe (no special characters)
 * - Compatible with UUID (128-bit)
 *
 * Format: TTTTTTTTTTRRRRRRRRRRRRRRRR
 * - T (10 chars): 48-bit Unix timestamp in milliseconds
 * - R (16 chars): 80-bit random component
 *
 * Encoding: Crockford Base32 (0-9, A-H, J-K, M-N, P-T, V-Z)
 * - Excludes I, L, O, U to avoid confusion
 *
 * @property value The 26-character ULID string in uppercase
 * @throws IllegalArgumentException if value is not a valid ULID
 */
@Serializable(with = UlidSerializer::class)
@JvmInline
value class Ulid(val value: String) {
    init {
        require(value.length == LENGTH) {
            "ULID must be exactly $LENGTH characters, got ${value.length}"
        }
        require(value.all { it in ENCODING_CHARS }) {
            "ULID contains invalid characters. Valid characters: $ENCODING_CHARS"
        }
    }

    /**
     * Extracts the timestamp component from this ULID.
     *
     * @return The timestamp as an Instant, representing when this ULID was generated
     */
    fun timestamp(): Instant {
        val timestampPart = value.substring(0, TIMESTAMP_LENGTH)
        val millis = decodeBase32(timestampPart)
        return Instant.fromEpochMilliseconds(millis)
    }

    override fun toString(): String = value

    companion object {
        /** Length of a ULID string */
        private const val LENGTH = 26

        /** Length of the timestamp component */
        private const val TIMESTAMP_LENGTH = 10

        /** Length of the random component */
        private const val RANDOM_LENGTH = 16

        /** Crockford Base32 encoding characters */
        const val ENCODING_CHARS = "0123456789ABCDEFGHJKMNPQRSTVWXYZ"

        /** Base for encoding (32 for Base32) */
        private const val BASE = 32

        /**
         * Generates a new ULID with the current timestamp and random component.
         *
         * @param random Random instance for generating random component (defaults to default Random)
         * @return A newly generated ULID
         */
        fun generate(random: Random = Random.Default): Ulid {
            val timestamp = currentTimeMillis()
            val timestampPart = encodeBase32(timestamp, TIMESTAMP_LENGTH)

            // Generate 80-bit random component (10 bytes = 80 bits)
            val randomBytes = ByteArray(10)
            random.nextBytes(randomBytes)
            val randomValue = randomBytes.fold(0L) { acc, byte ->
                (acc shl 8) or (byte.toInt() and 0xFF).toLong()
            }
            val randomPart = encodeBase32(randomValue, RANDOM_LENGTH)

            return Ulid(timestampPart + randomPart)
        }

        /**
         * Parses a string into a ULID.
         *
         * Input is normalized to uppercase before validation.
         *
         * @param value The string to parse
         * @return A ULID if the string is valid, null otherwise
         */
        fun parse(value: String): Ulid? {
            val normalized = value.uppercase()
            return try {
                Ulid(normalized)
            } catch (e: IllegalArgumentException) {
                null
            }
        }

        /**
         * Encodes a long value into a Base32 string of specified length.
         *
         * @param value The value to encode
         * @param length The desired length of the output string
         * @return The Base32-encoded string, left-padded with '0' to specified length
         */
        private fun encodeBase32(value: Long, length: Int): String {
            var remaining = value
            val result = CharArray(length) { '0' }
            var index = length - 1

            while (remaining > 0 && index >= 0) {
                result[index] = ENCODING_CHARS[(remaining % BASE).toInt()]
                remaining /= BASE
                index--
            }

            return result.concatToString()
        }

        /**
         * Decodes a Base32 string into a long value.
         *
         * @param encoded The Base32-encoded string
         * @return The decoded long value
         */
        private fun decodeBase32(encoded: String): Long {
            return encoded.fold(0L) { acc, char ->
                val digit = ENCODING_CHARS.indexOf(char)
                require(digit >= 0) { "Invalid character in Base32 string: $char" }
                acc * BASE + digit
            }
        }
    }
}

/**
 * Custom serializer for ULID that serializes to/from JSON string.
 */
object UlidSerializer : KSerializer<Ulid> {
    override val descriptor: SerialDescriptor =
        PrimitiveSerialDescriptor("Ulid", PrimitiveKind.STRING)

    override fun serialize(encoder: Encoder, value: Ulid) {
        encoder.encodeString(value.value)
    }

    override fun deserialize(decoder: Decoder): Ulid {
        val string = decoder.decodeString()
        return Ulid.parse(string)
            ?: throw IllegalArgumentException("Invalid ULID string: $string")
    }
}

package com.getaltair.altair.shared.domain.common

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

class UlidTest {
    /**
     * Platform-independent busy-wait delay using the project's currentTimeMillis() helper.
     */
    private fun busyWaitMillis(millis: Long) {
        val start = currentTimeMillis()
        @Suppress("ControlFlowWithEmptyBody")
        while (currentTimeMillis() - start < millis) {
            // busy wait
        }
    }
    @Test
    fun testGenerate() {
        val ulid = Ulid.generate()
        assertEquals(26, ulid.value.length)
        assertTrue(ulid.value.all { it in Ulid.ENCODING_CHARS })
    }

    @Test
    fun testParseValid() {
        val ulid = Ulid.generate()
        val parsed = Ulid.parse(ulid.value)
        assertNotNull(parsed)
        assertEquals(ulid.value, parsed.value)
    }

    @Test
    fun testParseLowercase() {
        val ulid = Ulid.generate()
        val parsed = Ulid.parse(ulid.value.lowercase())
        assertNotNull(parsed)
        assertEquals(ulid.value, parsed.value)
    }

    @Test
    fun testParseInvalid() {
        assertNull(Ulid.parse(""))
        assertNull(Ulid.parse("TOO_SHORT"))
        assertNull(Ulid.parse("CONTAINS_INVALID_CHAR_I_L"))
    }

    @Test
    fun testTimestamp() {
        val ulid = Ulid.generate()
        val timestamp = ulid.timestamp()
        assertTrue(timestamp.toEpochMilliseconds() > 0)
    }

    @Test
    fun testLexicographicSorting() {
        busyWaitMillis(10) // Ensure different timestamps
        val ulid1 = Ulid.generate()
        busyWaitMillis(10)
        val ulid2 = Ulid.generate()

        assertTrue(ulid1.value < ulid2.value)
    }
}

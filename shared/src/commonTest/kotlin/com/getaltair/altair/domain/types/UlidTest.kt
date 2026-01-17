package com.getaltair.altair.domain.types

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertNotEquals
import kotlin.test.assertTrue

class UlidTest {
    @Test
    fun `generate creates valid 26-character ULID`() {
        val ulid = Ulid.generate()
        assertEquals(26, ulid.value.length)
    }

    @Test
    fun `generated ULIDs are unique`() {
        val ulids = (1..100).map { Ulid.generate() }
        val uniqueValues = ulids.map { it.value }.toSet()
        assertEquals(100, uniqueValues.size)
    }

    @Test
    fun `ULIDs generated later have greater values`() {
        val ulid1 = Ulid.generate(timestamp = 1000)
        val ulid2 = Ulid.generate(timestamp = 2000)
        assertTrue(ulid1.value < ulid2.value)
    }

    @Test
    fun `constructor accepts valid ULID string`() {
        val validUlid = "01ARZ3NDEKTSV4RRFFQ69G5FAV"
        val ulid = Ulid(validUlid)
        assertEquals(validUlid, ulid.value)
    }

    @Test
    fun `constructor rejects too short string`() {
        assertFailsWith<IllegalArgumentException> {
            Ulid("01ARZ3NDEKTSV4RRFFQ69G5FA")
        }
    }

    @Test
    fun `constructor rejects too long string`() {
        assertFailsWith<IllegalArgumentException> {
            Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAVX")
        }
    }

    @Test
    fun `constructor rejects invalid characters`() {
        assertFailsWith<IllegalArgumentException> {
            Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAI") // I is not valid
        }
        assertFailsWith<IllegalArgumentException> {
            Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAL") // L is not valid
        }
        assertFailsWith<IllegalArgumentException> {
            Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAO") // O is not valid
        }
        assertFailsWith<IllegalArgumentException> {
            Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAU") // U is not valid
        }
    }

    @Test
    fun `constructor normalizes lowercase to uppercase`() {
        val lowerUlid = "01arz3ndektsv4rrffq69g5fav"
        val ulid = Ulid(lowerUlid)
        // The value is normalized to uppercase
        assertEquals("01ARZ3NDEKTSV4RRFFQ69G5FAV", ulid.value)
    }

    @Test
    fun `lowercase and uppercase ULIDs are equal`() {
        val ulid1 = Ulid("01arz3ndektsv4rrffq69g5fav")
        val ulid2 = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV")
        assertEquals(ulid1, ulid2)
    }

    @Test
    fun `toString returns the value`() {
        val validUlid = "01ARZ3NDEKTSV4RRFFQ69G5FAV"
        val ulid = Ulid(validUlid)
        assertEquals(validUlid, ulid.toString())
    }

    @Test
    fun `equality works for same values`() {
        val ulid1 = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV")
        val ulid2 = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV")
        assertEquals(ulid1, ulid2)
    }

    @Test
    fun `inequality works for different values`() {
        val ulid1 = Ulid.generate()
        val ulid2 = Ulid.generate()
        assertNotEquals(ulid1, ulid2)
    }
}

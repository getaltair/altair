package com.getaltair.altair.repository

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

/**
 * Tests for [EpicProgress] validation and computed properties.
 */
class EpicProgressTest {
    // Valid construction tests

    @Test
    fun `EpicProgress with valid values succeeds`() {
        val progress =
            EpicProgress(
                totalQuests = 10,
                completedQuests = 5,
                totalEnergy = 100,
                spentEnergy = 50,
            )

        assertEquals(10, progress.totalQuests)
        assertEquals(5, progress.completedQuests)
        assertEquals(100, progress.totalEnergy)
        assertEquals(50, progress.spentEnergy)
    }

    @Test
    fun `EpicProgress with all zeros succeeds`() {
        val progress =
            EpicProgress(
                totalQuests = 0,
                completedQuests = 0,
                totalEnergy = 0,
                spentEnergy = 0,
            )

        assertEquals(0, progress.totalQuests)
        assertEquals(0, progress.completedQuests)
    }

    @Test
    fun `EpicProgress with completed equal to total succeeds`() {
        val progress =
            EpicProgress(
                totalQuests = 5,
                completedQuests = 5,
                totalEnergy = 20,
                spentEnergy = 20,
            )

        assertEquals(5, progress.completedQuests)
        assertEquals(20, progress.spentEnergy)
    }

    // Validation failure tests

    @Test
    fun `EpicProgress with negative totalQuests throws IllegalArgumentException`() {
        assertFailsWith<IllegalArgumentException> {
            EpicProgress(
                totalQuests = -1,
                completedQuests = 0,
                totalEnergy = 0,
                spentEnergy = 0,
            )
        }
    }

    @Test
    fun `EpicProgress with negative completedQuests throws IllegalArgumentException`() {
        assertFailsWith<IllegalArgumentException> {
            EpicProgress(
                totalQuests = 10,
                completedQuests = -1,
                totalEnergy = 0,
                spentEnergy = 0,
            )
        }
    }

    @Test
    fun `EpicProgress with completedQuests exceeding totalQuests throws IllegalArgumentException`() {
        assertFailsWith<IllegalArgumentException> {
            EpicProgress(
                totalQuests = 5,
                completedQuests = 6,
                totalEnergy = 0,
                spentEnergy = 0,
            )
        }
    }

    @Test
    fun `EpicProgress with negative totalEnergy throws IllegalArgumentException`() {
        assertFailsWith<IllegalArgumentException> {
            EpicProgress(
                totalQuests = 0,
                completedQuests = 0,
                totalEnergy = -1,
                spentEnergy = 0,
            )
        }
    }

    @Test
    fun `EpicProgress with negative spentEnergy throws IllegalArgumentException`() {
        assertFailsWith<IllegalArgumentException> {
            EpicProgress(
                totalQuests = 0,
                completedQuests = 0,
                totalEnergy = 10,
                spentEnergy = -1,
            )
        }
    }

    @Test
    fun `EpicProgress with spentEnergy exceeding totalEnergy throws IllegalArgumentException`() {
        assertFailsWith<IllegalArgumentException> {
            EpicProgress(
                totalQuests = 0,
                completedQuests = 0,
                totalEnergy = 10,
                spentEnergy = 11,
            )
        }
    }

    // Completion percent tests

    @Test
    fun `completionPercent is 0 when no quests`() {
        val progress =
            EpicProgress(
                totalQuests = 0,
                completedQuests = 0,
                totalEnergy = 0,
                spentEnergy = 0,
            )

        assertEquals(0, progress.completionPercent)
    }

    @Test
    fun `completionPercent is 0 when no quests completed`() {
        val progress =
            EpicProgress(
                totalQuests = 10,
                completedQuests = 0,
                totalEnergy = 100,
                spentEnergy = 0,
            )

        assertEquals(0, progress.completionPercent)
    }

    @Test
    fun `completionPercent is 100 when all quests completed`() {
        val progress =
            EpicProgress(
                totalQuests = 10,
                completedQuests = 10,
                totalEnergy = 100,
                spentEnergy = 100,
            )

        assertEquals(100, progress.completionPercent)
    }

    @Test
    fun `completionPercent is calculated correctly for partial completion`() {
        val progress =
            EpicProgress(
                totalQuests = 10,
                completedQuests = 5,
                totalEnergy = 100,
                spentEnergy = 50,
            )

        assertEquals(50, progress.completionPercent)
    }

    @Test
    fun `completionPercent truncates decimal values`() {
        val progress =
            EpicProgress(
                totalQuests = 3,
                completedQuests = 1,
                totalEnergy = 30,
                spentEnergy = 10,
            )

        // 1/3 = 0.333... * 100 = 33.33... -> truncated to 33
        assertEquals(33, progress.completionPercent)
    }

    // Data class equality tests

    @Test
    fun `EpicProgress equality works correctly`() {
        val progress1 =
            EpicProgress(
                totalQuests = 10,
                completedQuests = 5,
                totalEnergy = 100,
                spentEnergy = 50,
            )
        val progress2 =
            EpicProgress(
                totalQuests = 10,
                completedQuests = 5,
                totalEnergy = 100,
                spentEnergy = 50,
            )

        assertEquals(progress1, progress2)
    }

    @Test
    fun `EpicProgress copy works correctly`() {
        val original =
            EpicProgress(
                totalQuests = 10,
                completedQuests = 5,
                totalEnergy = 100,
                spentEnergy = 50,
            )
        val copied = original.copy(completedQuests = 6, spentEnergy = 60)

        assertEquals(10, copied.totalQuests)
        assertEquals(6, copied.completedQuests)
        assertEquals(100, copied.totalEnergy)
        assertEquals(60, copied.spentEnergy)
    }
}

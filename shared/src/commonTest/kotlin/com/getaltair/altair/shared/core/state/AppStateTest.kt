package com.getaltair.altair.shared.core.state

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull

/**
 * Tests for AppState with Arrow Optics.
 * Verifies optics-generated extensions work correctly.
 */
class AppStateTest {

    @Test
    fun appStateDefaultsAreCorrect() {
        val state = AppState()

        assertEquals(false, state.isLoading)
        assertNull(state.currentUser)
        assertEquals(GuidanceState(), state.guidance)
    }

    @Test
    fun guidanceStateDefaultsAreCorrect() {
        val state = GuidanceState()

        assertNull(state.activeQuest)
        assertEquals(0, state.wipCount)
        assertNull(state.todayBudget)
    }

    @Test
    fun nestedStateCanBeUpdatedWithOptics() {
        val initial = AppState(
            guidance = GuidanceState(wipCount = 0)
        )

        // Using optics-generated lens to modify nested state
        // Path: AppState -> guidance (GuidanceState) -> wipCount (Int)
        val updated = AppState.guidance.wipCount.modify(initial) { it + 1 }

        assertEquals(1, updated.guidance.wipCount)
        // Original unchanged (immutable)
        assertEquals(0, initial.guidance.wipCount)
    }
}

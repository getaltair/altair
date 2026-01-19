package com.getaltair.altair.ui.util

import androidx.compose.ui.unit.dp
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

/**
 * Tests for WindowSizeClass breakpoint calculations.
 */
class WindowSizeClassTest {
    @Test
    fun `calculateWindowSizeClass returns Compact for width below 600dp`() {
        assertEquals(WindowSizeClass.Compact, calculateWindowSizeClass(0.dp))
        assertEquals(WindowSizeClass.Compact, calculateWindowSizeClass(320.dp))
        assertEquals(WindowSizeClass.Compact, calculateWindowSizeClass(599.dp))
    }

    @Test
    fun `calculateWindowSizeClass returns Medium at exactly 600dp`() {
        assertEquals(WindowSizeClass.Medium, calculateWindowSizeClass(600.dp))
    }

    @Test
    fun `calculateWindowSizeClass returns Medium for width between 600dp and 840dp`() {
        assertEquals(WindowSizeClass.Medium, calculateWindowSizeClass(601.dp))
        assertEquals(WindowSizeClass.Medium, calculateWindowSizeClass(700.dp))
        assertEquals(WindowSizeClass.Medium, calculateWindowSizeClass(839.dp))
    }

    @Test
    fun `calculateWindowSizeClass returns Expanded at exactly 840dp`() {
        assertEquals(WindowSizeClass.Expanded, calculateWindowSizeClass(840.dp))
    }

    @Test
    fun `calculateWindowSizeClass returns Expanded for width above 840dp`() {
        assertEquals(WindowSizeClass.Expanded, calculateWindowSizeClass(841.dp))
        assertEquals(WindowSizeClass.Expanded, calculateWindowSizeClass(1024.dp))
        assertEquals(WindowSizeClass.Expanded, calculateWindowSizeClass(1920.dp))
    }

    @Test
    fun `useBottomNavigation returns true only for Compact`() {
        assertTrue(WindowSizeClass.Compact.useBottomNavigation())
        assertFalse(WindowSizeClass.Medium.useBottomNavigation())
        assertFalse(WindowSizeClass.Expanded.useBottomNavigation())
    }

    @Test
    fun `WindowSizeBreakpoints has correct values`() {
        assertEquals(600.dp, WindowSizeBreakpoints.CompactMaxWidth)
        assertEquals(840.dp, WindowSizeBreakpoints.MediumMaxWidth)
    }
}

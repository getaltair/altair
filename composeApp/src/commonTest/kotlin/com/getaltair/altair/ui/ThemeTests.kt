package com.getaltair.altair.ui

import com.getaltair.altair.ui.theme.AltairColors
import com.getaltair.altair.ui.theme.AltairShapes
import com.getaltair.altair.ui.theme.AltairSpacing
import com.getaltair.altair.ui.theme.AltairTypography
import com.getaltair.altair.ui.theme.darkColors
import kotlin.test.Test
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

/**
 * Test Scenario TS8: Theme Provider Tests
 * Verifies theme infrastructure and token accessibility.
 */
class ThemeTests {

    // TS8.1: Theme token creation
    @Test
    fun `darkColors creates valid AltairColors instance`() {
        val colors = darkColors()
        assertNotNull(colors)
        assertTrue(colors is AltairColors)
    }

    @Test
    fun `AltairTypography creates valid instance with defaults`() {
        val typography = AltairTypography()
        assertNotNull(typography)
        assertNotNull(typography.displayLarge)
        assertNotNull(typography.headlineMedium)
        assertNotNull(typography.bodyLarge)
        assertNotNull(typography.bodyMedium)
        assertNotNull(typography.labelSmall)
    }

    @Test
    fun `AltairSpacing creates valid instance with defaults`() {
        val spacing = AltairSpacing()
        assertNotNull(spacing)
        assertNotNull(spacing.xs)
        assertNotNull(spacing.sm)
        assertNotNull(spacing.md)
        assertNotNull(spacing.lg)
        assertNotNull(spacing.xl)
    }

    @Test
    fun `AltairShapes creates valid instance with defaults`() {
        val shapes = AltairShapes()
        assertNotNull(shapes)
        assertNotNull(shapes.sm)
        assertNotNull(shapes.md)
        assertNotNull(shapes.lg)
        assertNotNull(shapes.full)
    }

    // Color categories verification
    @Test
    fun `darkColors contains all background colors`() {
        val colors = darkColors()
        assertNotNull(colors.background)
        assertNotNull(colors.surface)
        assertNotNull(colors.surfaceElevated)
        assertNotNull(colors.surfaceHover)
    }

    @Test
    fun `darkColors contains all border colors`() {
        val colors = darkColors()
        assertNotNull(colors.border)
        assertNotNull(colors.borderFocused)
    }

    @Test
    fun `darkColors contains all text colors`() {
        val colors = darkColors()
        assertNotNull(colors.textPrimary)
        assertNotNull(colors.textSecondary)
        assertNotNull(colors.textTertiary)
    }

    @Test
    fun `darkColors contains all accent colors`() {
        val colors = darkColors()
        assertNotNull(colors.accent)
        assertNotNull(colors.accentHover)
    }

    @Test
    fun `darkColors contains all status colors`() {
        val colors = darkColors()
        assertNotNull(colors.success)
        assertNotNull(colors.warning)
        assertNotNull(colors.error)
    }

    @Test
    fun `darkColors contains all energy colors`() {
        val colors = darkColors()
        assertNotNull(colors.energy1)
        assertNotNull(colors.energy5)
    }
}

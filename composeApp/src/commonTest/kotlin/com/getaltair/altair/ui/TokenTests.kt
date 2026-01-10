package com.getaltair.altair.ui

import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.text.font.FontWeight
import com.getaltair.altair.ui.theme.AltairTypography
import com.getaltair.altair.ui.theme.AltairSpacing
import com.getaltair.altair.ui.theme.AltairShapes
import com.getaltair.altair.ui.theme.darkColors
import kotlin.test.Test
import kotlin.test.assertEquals

/**
 * Test Scenario TS1-TS4: Design Token Verification
 * Verifies all design token values match ADR-008 specifications.
 */
class TokenTests {

    // TS1.1: Background Colors Available
    @Test
    fun `background color equals 0xFF0A0A0B`() {
        val colors = darkColors()
        assertEquals(Color(0xFF0A0A0B), colors.background)
    }

    @Test
    fun `surface color equals 0xFF141415`() {
        val colors = darkColors()
        assertEquals(Color(0xFF141415), colors.surface)
    }

    @Test
    fun `surfaceElevated color equals 0xFF1C1C1E`() {
        val colors = darkColors()
        assertEquals(Color(0xFF1C1C1E), colors.surfaceElevated)
    }

    @Test
    fun `surfaceHover color equals 0xFF232326`() {
        val colors = darkColors()
        assertEquals(Color(0xFF232326), colors.surfaceHover)
    }

    // TS1.2: Text Colors Available
    @Test
    fun `textPrimary color equals 0xFFEEEEEF`() {
        val colors = darkColors()
        assertEquals(Color(0xFFEEEEEF), colors.textPrimary)
    }

    @Test
    fun `textSecondary color equals 0xFF8E8E93`() {
        val colors = darkColors()
        assertEquals(Color(0xFF8E8E93), colors.textSecondary)
    }

    @Test
    fun `textTertiary color equals 0xFF636366`() {
        val colors = darkColors()
        assertEquals(Color(0xFF636366), colors.textTertiary)
    }

    // Border Colors
    @Test
    fun `border color equals 0xFF2E2E32`() {
        val colors = darkColors()
        assertEquals(Color(0xFF2E2E32), colors.border)
    }

    @Test
    fun `borderFocused color equals 0xFF6366F1`() {
        val colors = darkColors()
        assertEquals(Color(0xFF6366F1), colors.borderFocused)
    }

    // TS1.3: Accent and Status Colors Available
    @Test
    fun `accent color equals 0xFF6366F1`() {
        val colors = darkColors()
        assertEquals(Color(0xFF6366F1), colors.accent)
    }

    @Test
    fun `accentHover color equals 0xFF818CF8`() {
        val colors = darkColors()
        assertEquals(Color(0xFF818CF8), colors.accentHover)
    }

    @Test
    fun `success color equals 0xFF22C55E`() {
        val colors = darkColors()
        assertEquals(Color(0xFF22C55E), colors.success)
    }

    @Test
    fun `warning color equals 0xFFF59E0B`() {
        val colors = darkColors()
        assertEquals(Color(0xFFF59E0B), colors.warning)
    }

    @Test
    fun `error color equals 0xFFEF4444`() {
        val colors = darkColors()
        assertEquals(Color(0xFFEF4444), colors.error)
    }

    // Energy Level Colors
    @Test
    fun `energy1 color equals 0xFF22C55E`() {
        val colors = darkColors()
        assertEquals(Color(0xFF22C55E), colors.energy1)
    }

    @Test
    fun `energy5 color equals 0xFFEF4444`() {
        val colors = darkColors()
        assertEquals(Color(0xFFEF4444), colors.energy5)
    }

    // TS2.1: Typography Scale Available
    @Test
    fun `displayLarge fontSize equals 32sp`() {
        val typography = AltairTypography()
        assertEquals(32.sp, typography.displayLarge.fontSize)
    }

    @Test
    fun `displayLarge fontWeight equals SemiBold`() {
        val typography = AltairTypography()
        assertEquals(FontWeight.SemiBold, typography.displayLarge.fontWeight)
    }

    @Test
    fun `headlineMedium fontSize equals 20sp`() {
        val typography = AltairTypography()
        assertEquals(20.sp, typography.headlineMedium.fontSize)
    }

    @Test
    fun `headlineMedium fontWeight equals Medium`() {
        val typography = AltairTypography()
        assertEquals(FontWeight.Medium, typography.headlineMedium.fontWeight)
    }

    @Test
    fun `bodyLarge fontSize equals 16sp`() {
        val typography = AltairTypography()
        assertEquals(16.sp, typography.bodyLarge.fontSize)
    }

    @Test
    fun `bodyLarge fontWeight equals Normal`() {
        val typography = AltairTypography()
        assertEquals(FontWeight.Normal, typography.bodyLarge.fontWeight)
    }

    @Test
    fun `bodyMedium fontSize equals 14sp`() {
        val typography = AltairTypography()
        assertEquals(14.sp, typography.bodyMedium.fontSize)
    }

    @Test
    fun `bodyMedium fontWeight equals Normal`() {
        val typography = AltairTypography()
        assertEquals(FontWeight.Normal, typography.bodyMedium.fontWeight)
    }

    @Test
    fun `labelSmall fontSize equals 12sp`() {
        val typography = AltairTypography()
        assertEquals(12.sp, typography.labelSmall.fontSize)
    }

    @Test
    fun `labelSmall fontWeight equals Medium`() {
        val typography = AltairTypography()
        assertEquals(FontWeight.Medium, typography.labelSmall.fontWeight)
    }

    // TS3.1: Spacing Scale Available
    @Test
    fun `spacing xs equals 4dp`() {
        val spacing = AltairSpacing()
        assertEquals(4.dp, spacing.xs)
    }

    @Test
    fun `spacing sm equals 8dp`() {
        val spacing = AltairSpacing()
        assertEquals(8.dp, spacing.sm)
    }

    @Test
    fun `spacing md equals 16dp`() {
        val spacing = AltairSpacing()
        assertEquals(16.dp, spacing.md)
    }

    @Test
    fun `spacing lg equals 24dp`() {
        val spacing = AltairSpacing()
        assertEquals(24.dp, spacing.lg)
    }

    @Test
    fun `spacing xl equals 32dp`() {
        val spacing = AltairSpacing()
        assertEquals(32.dp, spacing.xl)
    }

    // TS4.1: Border Radius Scale Available
    @Test
    fun `shape sm equals RoundedCornerShape 4dp`() {
        val shapes = AltairShapes()
        assertEquals(RoundedCornerShape(4.dp), shapes.sm)
    }

    @Test
    fun `shape md equals RoundedCornerShape 6dp`() {
        val shapes = AltairShapes()
        assertEquals(RoundedCornerShape(6.dp), shapes.md)
    }

    @Test
    fun `shape lg equals RoundedCornerShape 8dp`() {
        val shapes = AltairShapes()
        assertEquals(RoundedCornerShape(8.dp), shapes.lg)
    }

    @Test
    fun `shape full equals RoundedCornerShape 9999dp`() {
        val shapes = AltairShapes()
        assertEquals(RoundedCornerShape(9999.dp), shapes.full)
    }
}

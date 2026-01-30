package com.getaltair.altair.ui.theme

import androidx.compose.ui.graphics.Color

/**
 * Altair color palette.
 *
 * Dark-first, Linear-inspired theme with high contrast and subtle borders.
 * Follows ADR-008 specifications.
 */
object AltairColors {
    // Backgrounds (dark-first, Linear-inspired)
    /** Near-black primary background */
    val background = Color(0xFF0A0A0B)

    /** Slightly lighter surface for panels */
    val surface = Color(0xFF141415)

    /** Elevated surfaces for cards and dialogs */
    val surfaceElevated = Color(0xFF1C1C1E)

    /** Hover state background */
    val surfaceHover = Color(0xFF232326)

    // Borders
    /** Subtle border color for dividers and outlines */
    val border = Color(0xFF2E2E32)

    /** Focus ring and active border color */
    val borderFocused = Color(0xFF6366F1)

    // Text
    /** High contrast primary text */
    val textPrimary = Color(0xFFEEEEEF)

    /** Muted secondary text */
    val textSecondary = Color(0xFF8E8E93)

    /** Disabled/hint tertiary text */
    val textTertiary = Color(0xFF636366)

    // Accent (indigo)
    /** Primary accent color */
    val accent = Color(0xFF6366F1)

    /** Hover state for accent elements */
    val accentHover = Color(0xFF818CF8)

    // Status
    /** Success state indicator */
    val success = Color(0xFF22C55E)

    /** Warning state indicator */
    val warning = Color(0xFFF59E0B)

    /** Error state indicator */
    val error = Color(0xFFEF4444)

    // Energy levels (Guidance module)
    /** Low effort (energy level 1) */
    val energy1 = Color(0xFF22C55E)

    /** High effort (energy level 5) */
    val energy5 = Color(0xFFEF4444)
}

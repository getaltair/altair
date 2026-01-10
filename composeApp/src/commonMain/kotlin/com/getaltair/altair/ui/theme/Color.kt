package com.getaltair.altair.ui.theme

import androidx.compose.runtime.Immutable
import androidx.compose.ui.graphics.Color

/**
 * Altair Design System color tokens.
 *
 * Defines the complete dark-first color palette as specified in ADR-008.
 * All colors are optimized for high contrast and accessibility.
 *
 * @property background Near-black base background (#0A0A0B)
 * @property surface Slightly elevated surface (#141415)
 * @property surfaceElevated Cards, dialogs elevation (#1C1C1E)
 * @property surfaceHover Interactive hover states (#232326)
 * @property border Subtle structural borders (#2E2E32)
 * @property borderFocused Focus ring indicator (#6366F1)
 * @property textPrimary High contrast primary text (#EEEEEF)
 * @property textSecondary Muted secondary text (#8E8E93)
 * @property textTertiary Disabled/hint text (#636366)
 * @property accent Indigo primary action (#6366F1)
 * @property accentHover Accent interactive state (#818CF8)
 * @property success Positive states (#22C55E)
 * @property warning Caution states (#F59E0B)
 * @property error Error states (#EF4444)
 * @property energy1 Low effort energy level (#22C55E)
 * @property energy5 High effort energy level (#EF4444)
 */
@Immutable
data class AltairColors(
    // Background colors
    val background: Color,
    val surface: Color,
    val surfaceElevated: Color,
    val surfaceHover: Color,

    // Border colors
    val border: Color,
    val borderFocused: Color,

    // Text colors
    val textPrimary: Color,
    val textSecondary: Color,
    val textTertiary: Color,

    // Accent colors
    val accent: Color,
    val accentHover: Color,

    // Status colors
    val success: Color,
    val warning: Color,
    val error: Color,

    // Energy level colors (Guidance Module)
    val energy1: Color,
    val energy5: Color
)

/**
 * Creates the default dark color scheme for Altair.
 *
 * All color values match the ADR-008 specification for a Linear.app-inspired
 * professional productivity design with high contrast dark theme.
 *
 * @return [AltairColors] configured with the dark theme palette
 */
fun darkColors(): AltairColors = AltairColors(
    // Background colors
    background = Color(0xFF0A0A0B),
    surface = Color(0xFF141415),
    surfaceElevated = Color(0xFF1C1C1E),
    surfaceHover = Color(0xFF232326),

    // Border colors
    border = Color(0xFF2E2E32),
    borderFocused = Color(0xFF6366F1),

    // Text colors
    textPrimary = Color(0xFFEEEEEF),
    textSecondary = Color(0xFF8E8E93),
    textTertiary = Color(0xFF636366),

    // Accent colors
    accent = Color(0xFF6366F1),
    accentHover = Color(0xFF818CF8),

    // Status colors
    success = Color(0xFF22C55E),
    warning = Color(0xFFF59E0B),
    error = Color(0xFFEF4444),

    // Energy level colors
    energy1 = Color(0xFF22C55E),
    energy5 = Color(0xFFEF4444)
)

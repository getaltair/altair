package com.getaltair.altair.ui.theme

import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp

/**
 * Altair Design System tokens.
 *
 * A Linear-inspired dark theme with high contrast, minimal chrome, and
 * ADHD-friendly visual clarity. The theme is dark-first with support
 * for future light mode via [AltairThemeProvider].
 */
object AltairTheme {
    /**
     * Color tokens for the Altair design system.
     *
     * Colors are organized by semantic purpose:
     * - Backgrounds: Surface hierarchy
     * - Text: Content hierarchy
     * - Borders: Separation and focus states
     * - Accent: Primary action color
     * - Status: Semantic feedback colors
     */
    object Colors {
        // Backgrounds - Dark surface hierarchy
        val background: Color = Color(0xFF0D0D0D)
        val backgroundElevated: Color = Color(0xFF171717)
        val backgroundSubtle: Color = Color(0xFF1F1F1F)
        val backgroundHover: Color = Color(0xFF2A2A2A)
        val backgroundPressed: Color = Color(0xFF333333)

        // Text - Content hierarchy
        val textPrimary: Color = Color(0xFFF5F5F5)
        val textSecondary: Color = Color(0xFFA3A3A3)
        val textTertiary: Color = Color(0xFF737373)
        val textDisabled: Color = Color(0xFF525252)
        val textInverse: Color = Color(0xFF0D0D0D)

        // Borders - Separation and focus
        val border: Color = Color(0xFF2A2A2A)
        val borderSubtle: Color = Color(0xFF1F1F1F)
        val borderFocused: Color = Color(0xFF5C6AC4)

        // Accent - Primary action color (Linear purple-blue)
        val accent: Color = Color(0xFF5C6AC4)
        val accentHover: Color = Color(0xFF6B7AD4)
        val accentPressed: Color = Color(0xFF4B5AB4)
        val accentSubtle: Color = Color(0xFF1A1D33)

        // Status - Semantic feedback
        val success: Color = Color(0xFF22C55E)
        val successSubtle: Color = Color(0xFF14532D)
        val warning: Color = Color(0xFFF59E0B)
        val warningSubtle: Color = Color(0xFF78350F)
        val error: Color = Color(0xFFEF4444)
        val errorHover: Color = Color(0xFFF87171)
        val errorPressed: Color = Color(0xFFDC2626)
        val errorSubtle: Color = Color(0xFF7F1D1D)
        val info: Color = Color(0xFF3B82F6)
        val infoSubtle: Color = Color(0xFF1E3A5F)
    }

    /**
     * Typography tokens for the Altair design system.
     *
     * Provides a harmonized type ramp with display, headline, body, and label
     * styles for consistent text hierarchy across all platforms.
     */
    object Typography {
        // Display - Large headings for splash/hero
        val displayLarge: TextStyle =
            TextStyle(
                fontSize = 48.sp,
                fontWeight = FontWeight.Bold,
                lineHeight = 56.sp,
                letterSpacing = (-0.5).sp,
                color = Colors.textPrimary,
            )
        val displayMedium: TextStyle =
            TextStyle(
                fontSize = 36.sp,
                fontWeight = FontWeight.Bold,
                lineHeight = 44.sp,
                letterSpacing = (-0.25).sp,
                color = Colors.textPrimary,
            )
        val displaySmall: TextStyle =
            TextStyle(
                fontSize = 28.sp,
                fontWeight = FontWeight.SemiBold,
                lineHeight = 36.sp,
                letterSpacing = 0.sp,
                color = Colors.textPrimary,
            )

        // Headline - Section headings
        val headlineLarge: TextStyle =
            TextStyle(
                fontSize = 24.sp,
                fontWeight = FontWeight.SemiBold,
                lineHeight = 32.sp,
                letterSpacing = 0.sp,
                color = Colors.textPrimary,
            )
        val headlineMedium: TextStyle =
            TextStyle(
                fontSize = 20.sp,
                fontWeight = FontWeight.SemiBold,
                lineHeight = 28.sp,
                letterSpacing = 0.sp,
                color = Colors.textPrimary,
            )
        val headlineSmall: TextStyle =
            TextStyle(
                fontSize = 18.sp,
                fontWeight = FontWeight.Medium,
                lineHeight = 24.sp,
                letterSpacing = 0.sp,
                color = Colors.textPrimary,
            )

        // Body - Primary content
        val bodyLarge: TextStyle =
            TextStyle(
                fontSize = 16.sp,
                fontWeight = FontWeight.Normal,
                lineHeight = 24.sp,
                letterSpacing = 0.sp,
                color = Colors.textPrimary,
            )
        val bodyMedium: TextStyle =
            TextStyle(
                fontSize = 14.sp,
                fontWeight = FontWeight.Normal,
                lineHeight = 20.sp,
                letterSpacing = 0.sp,
                color = Colors.textPrimary,
            )
        val bodySmall: TextStyle =
            TextStyle(
                fontSize = 12.sp,
                fontWeight = FontWeight.Normal,
                lineHeight = 16.sp,
                letterSpacing = 0.sp,
                color = Colors.textSecondary,
            )

        // Label - UI chrome, buttons, inputs
        val labelLarge: TextStyle =
            TextStyle(
                fontSize = 14.sp,
                fontWeight = FontWeight.Medium,
                lineHeight = 20.sp,
                letterSpacing = 0.sp,
                color = Colors.textPrimary,
            )
        val labelMedium: TextStyle =
            TextStyle(
                fontSize = 12.sp,
                fontWeight = FontWeight.Medium,
                lineHeight = 16.sp,
                letterSpacing = 0.sp,
                color = Colors.textPrimary,
            )
        val labelSmall: TextStyle =
            TextStyle(
                fontSize = 11.sp,
                fontWeight = FontWeight.Medium,
                lineHeight = 14.sp,
                letterSpacing = 0.5.sp,
                color = Colors.textSecondary,
            )
    }

    /**
     * Spacing tokens for consistent layout rhythm.
     */
    object Spacing {
        val xs: Dp = 4.dp
        val sm: Dp = 8.dp
        val md: Dp = 16.dp
        val lg: Dp = 24.dp
        val xl: Dp = 32.dp
        val xxl: Dp = 48.dp
    }

    /**
     * Border radius tokens for component shapes.
     */
    object Radii {
        val sm: Dp = 4.dp
        val md: Dp = 8.dp
        val lg: Dp = 12.dp
        val xl: Dp = 16.dp
        val full: Dp = 9999.dp
    }
}

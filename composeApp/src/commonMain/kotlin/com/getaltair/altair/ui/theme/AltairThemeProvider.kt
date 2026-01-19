package com.getaltair.altair.ui.theme

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color

/**
 * Color configuration for the Altair theme.
 *
 * This class holds the full set of semantic colors used throughout the app.
 * Use [LocalAltairColors] to access the current colors from composables.
 */
data class AltairColors(
    // Backgrounds
    val background: Color = AltairTheme.Colors.background,
    val backgroundElevated: Color = AltairTheme.Colors.backgroundElevated,
    val backgroundSubtle: Color = AltairTheme.Colors.backgroundSubtle,
    val backgroundHover: Color = AltairTheme.Colors.backgroundHover,
    val backgroundPressed: Color = AltairTheme.Colors.backgroundPressed,
    // Text
    val textPrimary: Color = AltairTheme.Colors.textPrimary,
    val textSecondary: Color = AltairTheme.Colors.textSecondary,
    val textTertiary: Color = AltairTheme.Colors.textTertiary,
    val textDisabled: Color = AltairTheme.Colors.textDisabled,
    val textInverse: Color = AltairTheme.Colors.textInverse,
    // Borders
    val border: Color = AltairTheme.Colors.border,
    val borderSubtle: Color = AltairTheme.Colors.borderSubtle,
    val borderFocused: Color = AltairTheme.Colors.borderFocused,
    // Accent
    val accent: Color = AltairTheme.Colors.accent,
    val accentHover: Color = AltairTheme.Colors.accentHover,
    val accentPressed: Color = AltairTheme.Colors.accentPressed,
    val accentSubtle: Color = AltairTheme.Colors.accentSubtle,
    // Status
    val success: Color = AltairTheme.Colors.success,
    val successSubtle: Color = AltairTheme.Colors.successSubtle,
    val warning: Color = AltairTheme.Colors.warning,
    val warningSubtle: Color = AltairTheme.Colors.warningSubtle,
    val error: Color = AltairTheme.Colors.error,
    val errorSubtle: Color = AltairTheme.Colors.errorSubtle,
    val info: Color = AltairTheme.Colors.info,
    val infoSubtle: Color = AltairTheme.Colors.infoSubtle,
)

/**
 * CompositionLocal for accessing Altair colors.
 *
 * Usage: `val colors = LocalAltairColors.current`
 */
val LocalAltairColors = staticCompositionLocalOf { AltairColors() }

/**
 * Dark theme color configuration (default).
 */
val DarkAltairColors = AltairColors()

/**
 * Light theme color configuration for future light mode support.
 */
val LightAltairColors = AltairColors(
    // Backgrounds - Light surface hierarchy
    background = Color(0xFFFAFAFA),
    backgroundElevated = Color(0xFFFFFFFF),
    backgroundSubtle = Color(0xFFF5F5F5),
    backgroundHover = Color(0xFFEEEEEE),
    backgroundPressed = Color(0xFFE0E0E0),
    // Text - Content hierarchy
    textPrimary = Color(0xFF171717),
    textSecondary = Color(0xFF525252),
    textTertiary = Color(0xFF737373),
    textDisabled = Color(0xFFA3A3A3),
    textInverse = Color(0xFFF5F5F5),
    // Borders
    border = Color(0xFFE5E5E5),
    borderSubtle = Color(0xFFF0F0F0),
    borderFocused = Color(0xFF5C6AC4),
    // Accent (same as dark theme)
    accent = Color(0xFF5C6AC4),
    accentHover = Color(0xFF4B5AB4),
    accentPressed = Color(0xFF3A4AA4),
    accentSubtle = Color(0xFFEEF0FA),
    // Status (slightly adjusted for light background)
    success = Color(0xFF16A34A),
    successSubtle = Color(0xFFDCFCE7),
    warning = Color(0xFFD97706),
    warningSubtle = Color(0xFFFEF3C7),
    error = Color(0xFFDC2626),
    errorSubtle = Color(0xFFFEE2E2),
    info = Color(0xFF2563EB),
    infoSubtle = Color(0xFFDBEAFE),
)

/**
 * Provides the Altair theme to the composable hierarchy.
 *
 * Wraps content with the appropriate color scheme and sets a background
 * matching the theme.
 *
 * @param colors The color configuration to use. Defaults to [DarkAltairColors].
 * @param content The composable content to wrap with the theme.
 */
@Composable
fun AltairThemeProvider(
    colors: AltairColors = DarkAltairColors,
    content: @Composable () -> Unit,
) {
    CompositionLocalProvider(LocalAltairColors provides colors) {
        Box(modifier = Modifier.background(colors.background)) {
            content()
        }
    }
}

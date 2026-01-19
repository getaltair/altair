package com.getaltair.altair.ui.theme

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color

/**
 * Background color tokens.
 */
data class BackgroundColors(
    val background: Color = AltairTheme.Colors.background,
    val elevated: Color = AltairTheme.Colors.backgroundElevated,
    val subtle: Color = AltairTheme.Colors.backgroundSubtle,
    val hover: Color = AltairTheme.Colors.backgroundHover,
    val pressed: Color = AltairTheme.Colors.backgroundPressed,
)

/**
 * Text color tokens.
 */
data class TextColors(
    val primary: Color = AltairTheme.Colors.textPrimary,
    val secondary: Color = AltairTheme.Colors.textSecondary,
    val tertiary: Color = AltairTheme.Colors.textTertiary,
    val disabled: Color = AltairTheme.Colors.textDisabled,
    val inverse: Color = AltairTheme.Colors.textInverse,
)

/**
 * Border color tokens.
 */
data class BorderColors(
    val default: Color = AltairTheme.Colors.border,
    val subtle: Color = AltairTheme.Colors.borderSubtle,
    val focused: Color = AltairTheme.Colors.borderFocused,
)

/**
 * Accent color tokens with state variants.
 */
data class AccentColors(
    val default: Color = AltairTheme.Colors.accent,
    val hover: Color = AltairTheme.Colors.accentHover,
    val pressed: Color = AltairTheme.Colors.accentPressed,
    val subtle: Color = AltairTheme.Colors.accentSubtle,
)

/**
 * Status color tokens for semantic feedback.
 */
data class StatusColors(
    val success: Color = AltairTheme.Colors.success,
    val successSubtle: Color = AltairTheme.Colors.successSubtle,
    val warning: Color = AltairTheme.Colors.warning,
    val warningSubtle: Color = AltairTheme.Colors.warningSubtle,
    val error: Color = AltairTheme.Colors.error,
    val errorHover: Color = AltairTheme.Colors.errorHover,
    val errorPressed: Color = AltairTheme.Colors.errorPressed,
    val errorSubtle: Color = AltairTheme.Colors.errorSubtle,
    val info: Color = AltairTheme.Colors.info,
    val infoSubtle: Color = AltairTheme.Colors.infoSubtle,
)

/**
 * Color configuration for the Altair theme.
 *
 * This class holds the full set of semantic colors used throughout the app,
 * organized into logical groups: backgrounds, text, borders, accent, and status.
 * Use [LocalAltairColors] to access the current colors from composables.
 */
data class AltairColors(
    val backgrounds: BackgroundColors = BackgroundColors(),
    val textColors: TextColors = TextColors(),
    val borders: BorderColors = BorderColors(),
    val accentColors: AccentColors = AccentColors(),
    val statusColors: StatusColors = StatusColors(),
) {
    // Convenience accessors for backward compatibility and easier access
    val background: Color get() = backgrounds.background
    val backgroundElevated: Color get() = backgrounds.elevated
    val backgroundSubtle: Color get() = backgrounds.subtle
    val backgroundHover: Color get() = backgrounds.hover
    val backgroundPressed: Color get() = backgrounds.pressed

    val textPrimary: Color get() = textColors.primary
    val textSecondary: Color get() = textColors.secondary
    val textTertiary: Color get() = textColors.tertiary
    val textDisabled: Color get() = textColors.disabled
    val textInverse: Color get() = textColors.inverse

    val border: Color get() = borders.default
    val borderSubtle: Color get() = borders.subtle
    val borderFocused: Color get() = borders.focused

    val accent: Color get() = accentColors.default
    val accentHover: Color get() = accentColors.hover
    val accentPressed: Color get() = accentColors.pressed
    val accentSubtle: Color get() = accentColors.subtle

    val success: Color get() = statusColors.success
    val successSubtle: Color get() = statusColors.successSubtle
    val warning: Color get() = statusColors.warning
    val warningSubtle: Color get() = statusColors.warningSubtle
    val error: Color get() = statusColors.error
    val errorHover: Color get() = statusColors.errorHover
    val errorPressed: Color get() = statusColors.errorPressed
    val errorSubtle: Color get() = statusColors.errorSubtle
    val info: Color get() = statusColors.info
    val infoSubtle: Color get() = statusColors.infoSubtle
}

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
val LightAltairColors =
    AltairColors(
        backgrounds =
            BackgroundColors(
                background = Color(0xFFFAFAFA),
                elevated = Color(0xFFFFFFFF),
                subtle = Color(0xFFF5F5F5),
                hover = Color(0xFFEEEEEE),
                pressed = Color(0xFFE0E0E0),
            ),
        textColors =
            TextColors(
                primary = Color(0xFF171717),
                secondary = Color(0xFF525252),
                tertiary = Color(0xFF737373),
                disabled = Color(0xFFA3A3A3),
                inverse = Color(0xFFF5F5F5),
            ),
        borders =
            BorderColors(
                default = Color(0xFFE5E5E5),
                subtle = Color(0xFFF0F0F0),
                focused = Color(0xFF5C6AC4),
            ),
        accentColors =
            AccentColors(
                default = Color(0xFF5C6AC4),
                hover = Color(0xFF4B5AB4),
                pressed = Color(0xFF3A4AA4),
                subtle = Color(0xFFEEF0FA),
            ),
        statusColors =
            StatusColors(
                success = Color(0xFF16A34A),
                successSubtle = Color(0xFFDCFCE7),
                warning = Color(0xFFD97706),
                warningSubtle = Color(0xFFFEF3C7),
                error = Color(0xFFDC2626),
                errorHover = Color(0xFFEF4444),
                errorPressed = Color(0xFFB91C1C),
                errorSubtle = Color(0xFFFEE2E2),
                info = Color(0xFF2563EB),
                infoSubtle = Color(0xFFDBEAFE),
            ),
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
@Suppress("ktlint:compose:modifier-missing-check")
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

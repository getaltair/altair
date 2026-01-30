package com.getaltair.altair.ui.theme

import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.staticCompositionLocalOf

/**
 * CompositionLocal for accessing Altair colors.
 */
val LocalAltairColors = staticCompositionLocalOf { AltairColors }

/**
 * CompositionLocal for accessing Altair typography.
 */
val LocalAltairTypography = staticCompositionLocalOf { AltairTypography }

/**
 * CompositionLocal for accessing Altair spacing.
 */
val LocalAltairSpacing = staticCompositionLocalOf { AltairSpacing }

/**
 * Altair theme composable.
 *
 * Provides the Altair design system (colors, typography, spacing) to all child components
 * via CompositionLocal. This is similar to MaterialTheme but without Material dependencies.
 *
 * Follows ADR-008 specifications for a Linear-inspired dark-first theme.
 *
 * @param content The composable content to theme
 *
 * Example usage:
 * ```
 * AltairTheme {
 *     Surface(color = AltairTheme.colors.background) {
 *         Text(
 *             text = "Hello, Altair!",
 *             style = AltairTheme.typography.bodyLarge,
 *             color = AltairTheme.colors.textPrimary
 *         )
 *     }
 * }
 * ```
 */
@Composable
fun AltairTheme(
    content: @Composable () -> Unit
) {
    CompositionLocalProvider(
        LocalAltairColors provides AltairColors,
        LocalAltairTypography provides AltairTypography,
        LocalAltairSpacing provides AltairSpacing
    ) {
        content()
    }
}

/**
 * Singleton object for accessing Altair theme values.
 *
 * Provides composable accessors for colors, typography, and spacing.
 * Must be used within [AltairTheme].
 */
object AltairTheme {
    /** Access Altair color palette */
    val colors: AltairColors
        @Composable
        get() = LocalAltairColors.current

    /** Access Altair typography scale */
    val typography: AltairTypography
        @Composable
        get() = LocalAltairTypography.current

    /** Access Altair spacing scale */
    val spacing: AltairSpacing
        @Composable
        get() = LocalAltairSpacing.current
}

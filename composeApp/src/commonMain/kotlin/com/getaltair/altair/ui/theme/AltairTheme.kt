package com.getaltair.altair.ui.theme

import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.ReadOnlyComposable
import androidx.compose.runtime.staticCompositionLocalOf

/**
 * CompositionLocal for providing [AltairColors] throughout the composition tree.
 */
val LocalAltairColors = staticCompositionLocalOf {
    darkColors()
}

/**
 * CompositionLocal for providing [AltairTypography] throughout the composition tree.
 */
val LocalAltairTypography = staticCompositionLocalOf {
    AltairTypography()
}

/**
 * CompositionLocal for providing [AltairSpacing] throughout the composition tree.
 */
val LocalAltairSpacing = staticCompositionLocalOf {
    AltairSpacing()
}

/**
 * CompositionLocal for providing [AltairShapes] throughout the composition tree.
 */
val LocalAltairShapes = staticCompositionLocalOf {
    AltairShapes()
}

/**
 * Altair Design System theme provider.
 *
 * Provides design tokens to all descendant composables via CompositionLocal.
 * Implements a dark-first, Linear.app-inspired professional design system.
 *
 * @param colors Color tokens to provide (defaults to dark colors)
 * @param typography Typography tokens to provide
 * @param spacing Spacing tokens to provide
 * @param shapes Shape tokens to provide
 * @param content The composable content to wrap with theme
 */
@Composable
fun AltairTheme(
    colors: AltairColors = darkColors(),
    typography: AltairTypography = AltairTypography(),
    spacing: AltairSpacing = AltairSpacing(),
    shapes: AltairShapes = AltairShapes(),
    content: @Composable () -> Unit,
) {
    CompositionLocalProvider(
        LocalAltairColors provides colors,
        LocalAltairTypography provides typography,
        LocalAltairSpacing provides spacing,
        LocalAltairShapes provides shapes,
        content = content,
    )
}

/**
 * Companion object providing convenient accessors for theme tokens.
 *
 * Usage:
 * ```kotlin
 * AltairTheme {
 *     val background = AltairTheme.colors.background
 *     val bodyStyle = AltairTheme.typography.bodyLarge
 * }
 * ```
 */
object AltairTheme {
    /**
     * Retrieves the current [AltairColors] from the composition.
     */
    val colors: AltairColors
        @Composable
        @ReadOnlyComposable
        get() = LocalAltairColors.current

    /**
     * Retrieves the current [AltairTypography] from the composition.
     */
    val typography: AltairTypography
        @Composable
        @ReadOnlyComposable
        get() = LocalAltairTypography.current

    /**
     * Retrieves the current [AltairSpacing] from the composition.
     */
    val spacing: AltairSpacing
        @Composable
        @ReadOnlyComposable
        get() = LocalAltairSpacing.current

    /**
     * Retrieves the current [AltairShapes] from the composition.
     */
    val shapes: AltairShapes
        @Composable
        @ReadOnlyComposable
        get() = LocalAltairShapes.current
}

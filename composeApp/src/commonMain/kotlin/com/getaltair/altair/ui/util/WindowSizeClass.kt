package com.getaltair.altair.ui.util

import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.BoxWithConstraintsScope
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

/**
 * Window size classification following Material Design 3 guidelines.
 *
 * Used to determine the appropriate navigation pattern:
 * - [Compact]: Bottom navigation bar (phones, narrow windows)
 * - [Medium]: Navigation rail (tablets, medium windows)
 * - [Expanded]: Navigation rail (desktops, wide windows)
 */
enum class WindowSizeClass {
    /** Width < 600dp - phones and narrow windows. */
    Compact,

    /** 600dp <= width < 840dp - tablets and medium windows. */
    Medium,

    /** Width >= 840dp - desktops and wide windows. */
    Expanded,
}

/**
 * Material Design 3 breakpoints for window size classification.
 */
object WindowSizeBreakpoints {
    val CompactMaxWidth: Dp = 600.dp
    val MediumMaxWidth: Dp = 840.dp
}

/**
 * Calculates the [WindowSizeClass] based on the given width.
 *
 * @param width The window width in dp.
 * @return The corresponding [WindowSizeClass].
 */
fun calculateWindowSizeClass(width: Dp): WindowSizeClass =
    when {
        width < WindowSizeBreakpoints.CompactMaxWidth -> WindowSizeClass.Compact
        width < WindowSizeBreakpoints.MediumMaxWidth -> WindowSizeClass.Medium
        else -> WindowSizeClass.Expanded
    }

/**
 * Returns whether the current window size class should use a bottom navigation bar.
 *
 * Bottom navigation is appropriate for compact screens (phones).
 * Medium and expanded screens use navigation rail instead.
 */
fun WindowSizeClass.useBottomNavigation(): Boolean = this == WindowSizeClass.Compact

/**
 * A composable that provides [WindowSizeClass] based on the available width.
 *
 * This uses [BoxWithConstraints] internally to measure the available space
 * and determine the appropriate size class.
 *
 * @param modifier Modifier to apply to the container.
 * @param content Content that receives the current [WindowSizeClass].
 */
@Composable
fun WithWindowSizeClass(
    modifier: Modifier = Modifier,
    content: @Composable BoxWithConstraintsScope.(WindowSizeClass) -> Unit,
) {
    BoxWithConstraints(modifier = modifier) {
        val windowSizeClass = remember(maxWidth) { calculateWindowSizeClass(maxWidth) }
        content(windowSizeClass)
    }
}

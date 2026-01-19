package com.getaltair.altair.ui.theme

import androidx.compose.foundation.border
import androidx.compose.foundation.focusable
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.composed
import androidx.compose.ui.focus.onFocusChanged
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

/**
 * Adds a focus ring indicator to a composable for keyboard accessibility.
 *
 * The focus ring appears when the component receives keyboard focus,
 * providing a clear visual indicator for keyboard navigation.
 *
 * @param color The color of the focus ring. Defaults to the theme's focused border color.
 * @param width The width of the focus ring border.
 * @param shape The shape of the focus ring.
 * @param offset Additional offset around the component (not implemented - consider adding padding).
 * @return A modifier that displays a focus ring when focused.
 */
fun Modifier.focusRing(
    color: Color = AltairTheme.Colors.borderFocused,
    width: Dp = 2.dp,
    shape: Shape = RoundedCornerShape(AltairTheme.Radii.md),
    offset: Dp = 2.dp,
): Modifier = composed {
    var isFocused by remember { mutableStateOf(false) }

    this
        .focusable()
        .onFocusChanged { focusState ->
            isFocused = focusState.isFocused
        }
        .then(
            if (isFocused) {
                Modifier.border(width = width, color = color, shape = shape)
            } else {
                Modifier
            }
        )
}

/**
 * Adds a focus ring indicator with theme-aware colors.
 *
 * Uses [LocalAltairColors] to determine the focus ring color,
 * making it compatible with light/dark theme switching.
 *
 * @param width The width of the focus ring border.
 * @param shape The shape of the focus ring.
 * @return A modifier that displays a theme-aware focus ring when focused.
 */
@Composable
fun Modifier.themedFocusRing(
    width: Dp = 2.dp,
    shape: Shape = RoundedCornerShape(AltairTheme.Radii.md),
): Modifier {
    val colors = LocalAltairColors.current
    return this.focusRing(
        color = colors.borderFocused,
        width = width,
        shape = shape,
    )
}

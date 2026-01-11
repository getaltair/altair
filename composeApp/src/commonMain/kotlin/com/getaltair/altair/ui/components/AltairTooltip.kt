package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.hoverable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsHoveredAsState
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.window.Popup
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Altair Design System tooltip component.
 *
 * A tooltip that appears on hover to provide additional information.
 *
 * @param tooltip Tooltip text to display
 * @param modifier Modifier to be applied to the tooltip container
 * @param enabled Whether the tooltip is enabled
 * @param content Anchor content that triggers the tooltip
 */
@Composable
fun AltairTooltip(
    tooltip: String,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    content: @Composable () -> Unit,
) {
    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes
    val spacing = AltairTheme.spacing
    val typography = AltairTheme.typography

    val interactionSource = remember { MutableInteractionSource() }
    val isHovered by interactionSource.collectIsHoveredAsState()

    Box(
        modifier = modifier.hoverable(interactionSource = interactionSource),
    ) {
        content()

        if (enabled && isHovered) {
            Popup {
                Text(
                    text = tooltip,
                    style = typography.labelSmall,
                    color = colors.textPrimary,
                    modifier = Modifier
                        .clip(shapes.sm)
                        .background(colors.surfaceElevated)
                        .padding(horizontal = spacing.sm, vertical = spacing.xs),
                )
            }
        }
    }
}

/**
 * Altair Design System tooltip with custom content.
 *
 * A tooltip that can display any composable content.
 *
 * @param modifier Modifier to be applied to the tooltip container
 * @param enabled Whether the tooltip is enabled
 * @param tooltipContent Custom tooltip content
 * @param content Anchor content that triggers the tooltip
 */
@Composable
fun AltairTooltip(
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    tooltipContent: @Composable () -> Unit,
    content: @Composable () -> Unit,
) {
    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes
    val spacing = AltairTheme.spacing

    val interactionSource = remember { MutableInteractionSource() }
    val isHovered by interactionSource.collectIsHoveredAsState()

    Box(
        modifier = modifier.hoverable(interactionSource = interactionSource),
    ) {
        content()

        if (enabled && isHovered) {
            Popup {
                Box(
                    modifier = Modifier
                        .clip(shapes.sm)
                        .background(colors.surfaceElevated)
                        .padding(spacing.sm),
                ) {
                    tooltipContent()
                }
            }
        }
    }
}

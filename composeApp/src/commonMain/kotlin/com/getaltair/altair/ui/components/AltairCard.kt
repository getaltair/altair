package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.hoverable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsHoveredAsState
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxScope
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Altair Design System card component.
 *
 * A styled container with elevated surface background,
 * border styling, and hover state (changes to surfaceHover on desktop).
 *
 * @param modifier Modifier to be applied to the card
 * @param content Content to display inside the card
 */
@Composable
fun AltairCard(modifier: Modifier = Modifier, content: @Composable BoxScope.() -> Unit) {
    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes
    val spacing = AltairTheme.spacing

    val interactionSource = remember { MutableInteractionSource() }
    val isHovered by interactionSource.collectIsHoveredAsState()

    val backgroundColor = if (isHovered) colors.surfaceHover else colors.surfaceElevated

    Box(
        modifier = modifier
            .clip(shapes.md)
            .hoverable(interactionSource = interactionSource)
            .background(backgroundColor, shapes.md)
            .border(1.dp, colors.border, shapes.md)
            .padding(spacing.md),
        content = content,
    )
}

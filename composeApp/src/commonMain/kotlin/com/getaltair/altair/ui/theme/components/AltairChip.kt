package com.getaltair.altair.ui.theme.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsHoveredAsState
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicText
import androidx.compose.ui.text.TextStyle
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.getaltair.altair.ui.theme.AltairColors

/**
 * Altair-styled chip component.
 *
 * Provides a small, rounded badge for tags, filters, or status indicators.
 * Supports selected state and optional click interaction.
 *
 * @param label Text displayed in the chip
 * @param modifier Modifier to be applied to the chip
 * @param selected Whether the chip is in selected state (changes to accent background)
 * @param onClick Optional click handler - makes the chip clickable
 */
@Composable
fun AltairChip(
    label: String,
    modifier: Modifier = Modifier,
    selected: Boolean = false,
    onClick: (() -> Unit)? = null
) {
    val interactionSource = remember { MutableInteractionSource() }
    val isHovered by interactionSource.collectIsHoveredAsState()

    val backgroundColor = when {
        selected -> AltairColors.accent
        isHovered && onClick != null -> AltairColors.surfaceHover
        else -> AltairColors.surface
    }

    val textColor = if (selected) {
        androidx.compose.ui.graphics.Color.White
    } else {
        AltairColors.textPrimary
    }

    val borderColor = if (selected) {
        AltairColors.accent
    } else {
        AltairColors.border
    }

    val shape = RoundedCornerShape(16.dp)

    Box(
        modifier = modifier
            .clip(shape)
            .background(backgroundColor)
            .border(
                width = 1.dp,
                color = borderColor,
                shape = shape
            )
            .then(
                if (onClick != null) {
                    Modifier.clickable(
                        interactionSource = interactionSource,
                        indication = null,
                        onClick = onClick
                    )
                } else {
                    Modifier
                }
            )
            .padding(horizontal = 12.dp, vertical = 6.dp)
    ) {
        BasicText(
            text = label,
            style = TextStyle(
                color = textColor,
                fontSize = 12.sp
            )
        )
    }
}

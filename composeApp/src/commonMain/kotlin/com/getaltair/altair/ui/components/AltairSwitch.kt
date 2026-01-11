package com.getaltair.altair.ui.components

import androidx.compose.animation.core.animateDpAsState
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.draw.clip
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.role
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Altair Design System switch/toggle component.
 *
 * A toggle switch for on/off states with smooth animation.
 *
 * @param checked Whether the switch is in the on state
 * @param onCheckedChange Callback when the switch state changes
 * @param modifier Modifier to be applied to the switch
 * @param enabled Whether the switch is enabled
 */
@Composable
fun AltairSwitch(
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
) {
    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes

    val trackWidth = 44.dp
    val trackHeight = 24.dp
    val thumbSize = 20.dp
    val thumbPadding = 2.dp

    val thumbOffset by animateDpAsState(
        targetValue = if (checked) trackWidth - thumbSize - thumbPadding * 2 else 0.dp,
        animationSpec = tween(durationMillis = 150),
        label = "thumb_offset",
    )

    val trackColor = when {
        !enabled -> colors.surfaceHover
        checked -> colors.accent
        else -> colors.surface
    }

    val thumbColor = when {
        !enabled -> colors.textTertiary
        else -> colors.textPrimary
    }

    val interactionSource = remember { MutableInteractionSource() }

    Box(
        modifier = modifier
            .semantics { role = Role.Switch }
            .size(width = trackWidth, height = trackHeight)
            .clip(shapes.full)
            .background(trackColor)
            .clickable(
                interactionSource = interactionSource,
                indication = null,
                enabled = enabled,
                onClick = { onCheckedChange(!checked) },
            )
            .alpha(if (enabled) 1f else 0.5f),
        contentAlignment = Alignment.CenterStart,
    ) {
        Box(
            modifier = Modifier
                .padding(thumbPadding)
                .offset(x = thumbOffset)
                .size(thumbSize)
                .clip(CircleShape)
                .background(thumbColor),
        )
    }
}

/**
 * Altair Design System switch with label.
 *
 * A switch with accompanying text label for settings.
 *
 * @param checked Whether the switch is in the on state
 * @param onCheckedChange Callback when the switch state changes
 * @param modifier Modifier to be applied to the row
 * @param enabled Whether the switch is enabled
 * @param label Content to display as the label (typically Text)
 */
@Composable
fun AltairSwitchWithLabel(
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    label: @Composable () -> Unit,
) {
    val spacing = AltairTheme.spacing

    Row(
        modifier = modifier,
        horizontalArrangement = Arrangement.spacedBy(spacing.sm),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        label()
        AltairSwitch(
            checked = checked,
            onCheckedChange = onCheckedChange,
            enabled = enabled,
        )
    }
}

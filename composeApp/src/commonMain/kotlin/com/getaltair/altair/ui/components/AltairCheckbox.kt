package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material3.Icon
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.role
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Altair Design System checkbox component.
 *
 * A styled checkbox with proper accessibility semantics.
 *
 * @param checked Whether the checkbox is checked
 * @param onCheckedChange Callback when the checked state changes
 * @param modifier Modifier to be applied to the checkbox
 * @param enabled Whether the checkbox is enabled
 */
@Composable
fun AltairCheckbox(
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
) {
    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes

    val backgroundColor = when {
        !enabled -> colors.surfaceHover
        checked -> colors.accent
        else -> colors.surface
    }

    val borderColor = when {
        !enabled -> colors.border
        checked -> Color.Transparent
        else -> colors.border
    }

    val contentColor = when {
        !enabled -> colors.textTertiary
        else -> colors.textPrimary
    }

    val interactionSource = remember { MutableInteractionSource() }

    Box(
        modifier = modifier
            .semantics { role = Role.Checkbox }
            .size(20.dp)
            .clip(shapes.sm)
            .background(backgroundColor)
            .border(1.dp, borderColor, shapes.sm)
            .clickable(
                interactionSource = interactionSource,
                indication = null,
                enabled = enabled,
                onClick = { onCheckedChange(!checked) },
            )
            .alpha(if (enabled) 1f else 0.5f),
        contentAlignment = Alignment.Center,
    ) {
        if (checked) {
            Icon(
                imageVector = Icons.Default.Check,
                contentDescription = null,
                modifier = Modifier.size(14.dp),
                tint = contentColor,
            )
        }
    }
}

/**
 * Altair Design System checkbox with label.
 *
 * A checkbox with accompanying text label for forms and settings.
 *
 * @param checked Whether the checkbox is checked
 * @param onCheckedChange Callback when the checked state changes
 * @param modifier Modifier to be applied to the row
 * @param enabled Whether the checkbox is enabled
 * @param label Content to display as the label (typically Text)
 */
@Composable
fun AltairCheckboxWithLabel(
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    label: @Composable () -> Unit,
) {
    val spacing = AltairTheme.spacing
    val interactionSource = remember { MutableInteractionSource() }

    Row(
        modifier = modifier
            .clickable(
                interactionSource = interactionSource,
                indication = null,
                enabled = enabled,
                onClick = { onCheckedChange(!checked) },
            ),
        horizontalArrangement = Arrangement.spacedBy(spacing.sm),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        AltairCheckbox(
            checked = checked,
            onCheckedChange = onCheckedChange,
            enabled = enabled,
        )
        label()
    }
}

package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.LocalAltairColors

/**
 * Altair-styled checkbox component.
 *
 * A toggleable checkbox with custom styling. Can be used standalone
 * or combined with a label for form fields and settings.
 *
 * @param checked Whether the checkbox is checked.
 * @param onCheckedChange Callback invoked when the checked state changes.
 * @param modifier Modifier to be applied to the checkbox.
 * @param enabled Whether the checkbox is interactive.
 */
@Composable
fun AltairCheckbox(
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
) {
    val colors = LocalAltairColors.current

    val backgroundColor =
        when {
            !enabled -> colors.backgroundSubtle
            checked -> colors.accent
            else -> Color.Transparent
        }

    val borderColor =
        when {
            !enabled -> colors.borderSubtle
            checked -> Color.Transparent
            else -> colors.border
        }

    val checkColor =
        if (!enabled) {
            colors.textDisabled
        } else {
            colors.textPrimary
        }

    val interactionSource = remember { MutableInteractionSource() }

    Box(
        modifier =
            modifier
                .size(20.dp)
                .clip(RoundedCornerShape(AltairTheme.Radii.sm))
                .background(backgroundColor)
                .border(
                    width = if (borderColor != Color.Transparent) 2.dp else 0.dp,
                    color = borderColor,
                    shape = RoundedCornerShape(AltairTheme.Radii.sm),
                ).clickable(
                    interactionSource = interactionSource,
                    indication = null,
                    enabled = enabled,
                    onClick = { onCheckedChange(!checked) },
                ),
        contentAlignment = Alignment.Center,
    ) {
        if (checked) {
            // Simple checkmark using unicode character
            AltairText(
                text = "\u2713", // Unicode checkmark
                style = AltairTheme.Typography.labelMedium,
                color = checkColor,
            )
        }
    }
}

/**
 * Altair-styled checkbox with label.
 *
 * Combines a checkbox with a text label for form fields and settings.
 * The entire row is clickable to toggle the checkbox.
 *
 * @param checked Whether the checkbox is checked.
 * @param onCheckedChange Callback invoked when the checked state changes.
 * @param label Text label displayed next to the checkbox.
 * @param modifier Modifier to be applied to the row.
 * @param enabled Whether the checkbox is interactive.
 */
@Composable
fun AltairCheckboxRow(
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit,
    label: String,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
) {
    val colors = LocalAltairColors.current
    val interactionSource = remember { MutableInteractionSource() }

    val textColor =
        if (enabled) {
            colors.textPrimary
        } else {
            colors.textDisabled
        }

    Row(
        modifier =
            modifier
                .clickable(
                    interactionSource = interactionSource,
                    indication = null,
                    enabled = enabled,
                    onClick = { onCheckedChange(!checked) },
                ),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.Start,
    ) {
        AltairCheckbox(
            checked = checked,
            onCheckedChange = onCheckedChange,
            enabled = enabled,
        )
        Spacer(modifier = Modifier.width(AltairTheme.Spacing.sm))
        AltairText(
            text = label,
            style = AltairTheme.Typography.bodyMedium,
            color = textColor,
        )
    }
}

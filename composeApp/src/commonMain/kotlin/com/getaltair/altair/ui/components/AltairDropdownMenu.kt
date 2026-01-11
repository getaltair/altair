package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.hoverable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsHoveredAsState
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Altair Design System dropdown menu component.
 *
 * A dropdown menu with anchor trigger and styled menu panel.
 *
 * @param expanded Whether the menu is expanded
 * @param onDismissRequest Callback when the menu is dismissed
 * @param modifier Modifier to be applied to the dropdown
 * @param trigger Content for the menu trigger
 * @param content Content to display inside the menu panel
 */
@Composable
fun AltairDropdownMenu(
    expanded: Boolean,
    onDismissRequest: () -> Unit,
    modifier: Modifier = Modifier,
    trigger: @Composable () -> Unit,
    content: @Composable () -> Unit,
) {
    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes
    val spacing = AltairTheme.spacing

    Box(modifier = modifier) {
        trigger()

        DropdownMenu(
            expanded = expanded,
            onDismissRequest = onDismissRequest,
            modifier = Modifier
                .background(colors.surfaceElevated)
                .border(1.dp, colors.border, shapes.md),
        ) {
            content()
        }
    }
}

/**
 * Altair Design System dropdown menu item.
 *
 * A selectable item within an AltairDropdownMenu.
 *
 * @param onClick Callback when the item is clicked
 * @param modifier Modifier to be applied to the item
 * @param enabled Whether the item is enabled
 * @param content Content to display inside the item
 */
@Composable
fun AltairDropdownMenuItem(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    content: @Composable () -> Unit,
) {
    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes
    val spacing = AltairTheme.spacing

    val interactionSource = remember { MutableInteractionSource() }
    val isHovered by interactionSource.collectIsHoveredAsState()

    val backgroundColor = if (isHovered) colors.surfaceHover else colors.surfaceElevated

    Box(
        modifier = modifier
            .fillMaxWidth()
            .clip(shapes.sm)
            .background(backgroundColor)
            .hoverable(interactionSource = interactionSource)
            .clickable(
                interactionSource = interactionSource,
                indication = null,
                enabled = enabled,
                onClick = onClick,
            )
            .padding(horizontal = spacing.md, vertical = spacing.sm),
    ) {
        content()
    }
}

/**
 * Altair Design System select dropdown component.
 *
 * A dropdown menu for selecting from a list of options.
 *
 * @param selectedOption The currently selected option
 * @param options List of available options
 * @param onOptionSelected Callback when an option is selected
 * @param modifier Modifier to be applied to the dropdown
 * @param placeholder Placeholder text when no option is selected
 * @param optionLabel Function to get the display label for an option
 */
@Composable
fun <T> AltairSelect(
    selectedOption: T?,
    options: List<T>,
    onOptionSelected: (T) -> Unit,
    modifier: Modifier = Modifier,
    placeholder: String = "Select...",
    optionLabel: (T) -> String = { it.toString() },
) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography

    var expanded by remember { mutableStateOf(false) }

    AltairDropdownMenu(
        expanded = expanded,
        onDismissRequest = { expanded = false },
        modifier = modifier,
        trigger = {
            AltairButton(
                onClick = { expanded = !expanded },
                variant = ButtonVariant.Secondary,
            ) {
                Text(
                    text = selectedOption?.let { optionLabel(it) } ?: placeholder,
                    style = typography.bodyMedium,
                    color = if (selectedOption != null) colors.textPrimary else colors.textTertiary,
                )
            }
        },
    ) {
        Column {
            options.forEach { option ->
                AltairDropdownMenuItem(
                    onClick = {
                        onOptionSelected(option)
                        expanded = false
                    },
                ) {
                    Text(
                        text = optionLabel(option),
                        style = typography.bodyMedium,
                        color = colors.textPrimary,
                    )
                }
            }
        }
    }
}

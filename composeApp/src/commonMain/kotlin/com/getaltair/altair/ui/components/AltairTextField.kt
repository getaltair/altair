package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsFocusedAsState
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Altair Design System text field component.
 *
 * A styled text input that displays with surface background,
 * border color that changes to borderFocused on focus, and
 * theme-consistent text colors.
 *
 * @param value Current text value
 * @param onValueChange Callback when the text value changes
 * @param modifier Modifier to be applied to the text field
 * @param placeholder Placeholder composable shown when text is empty
 * @param singleLine Whether the text field should be single line
 * @param enabled Whether the text field is enabled for input
 * @param textStyle Text style to apply to input text
 */
@Composable
fun AltairTextField(
    value: String,
    onValueChange: (String) -> Unit,
    modifier: Modifier = Modifier,
    placeholder: @Composable (() -> Unit)? = null,
    singleLine: Boolean = true,
    enabled: Boolean = true,
    textStyle: TextStyle = AltairTheme.typography.bodyMedium,
) {
    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes
    val spacing = AltairTheme.spacing

    val interactionSource = remember { MutableInteractionSource() }
    val isFocused by interactionSource.collectIsFocusedAsState()

    val borderColor = if (isFocused) colors.borderFocused else colors.border

    BasicTextField(
        value = value,
        onValueChange = onValueChange,
        modifier = modifier
            .clip(shapes.md)
            .background(colors.surface, shapes.md)
            .border(1.dp, borderColor, shapes.md)
            .padding(spacing.sm),
        enabled = enabled,
        singleLine = singleLine,
        textStyle = textStyle.copy(color = colors.textPrimary),
        cursorBrush = SolidColor(colors.accent),
        interactionSource = interactionSource,
        decorationBox = { innerTextField ->
            Box {
                if (value.isEmpty() && placeholder != null) {
                    placeholder()
                }
                innerTextField()
            }
        },
    )
}

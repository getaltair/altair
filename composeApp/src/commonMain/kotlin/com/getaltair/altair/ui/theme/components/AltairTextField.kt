package com.getaltair.altair.ui.theme.components

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsFocusedAsState
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicText
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.getaltair.altair.ui.theme.AltairColors

/**
 * Altair-styled text field component.
 *
 * Provides a dark-themed input field with label support, focus states,
 * and error indication.
 *
 * @param value Current text value
 * @param onValueChange Callback invoked when text changes
 * @param modifier Modifier to be applied to the text field
 * @param label Optional label displayed above the field
 * @param placeholder Optional placeholder text shown when empty
 * @param enabled Whether the field accepts input
 * @param singleLine Whether the field should be single-line
 * @param isError Whether the field is in error state
 */
@Composable
fun AltairTextField(
    value: String,
    onValueChange: (String) -> Unit,
    modifier: Modifier = Modifier,
    label: String? = null,
    placeholder: String? = null,
    enabled: Boolean = true,
    singleLine: Boolean = true,
    isError: Boolean = false
) {
    val interactionSource = remember { MutableInteractionSource() }
    val isFocused by interactionSource.collectIsFocusedAsState()

    val borderColor = when {
        isError -> AltairColors.error
        isFocused -> AltairColors.borderFocused
        else -> AltairColors.border
    }

    val textColor = if (enabled) AltairColors.textPrimary else AltairColors.textTertiary

    Column(modifier = modifier) {
        // Label
        if (label != null) {
            BasicText(
                text = label,
                style = TextStyle(
                    color = AltairColors.textSecondary,
                    fontSize = 12.sp
                ),
                modifier = Modifier.padding(bottom = 6.dp)
            )
        }

        // Text Field
        BasicTextField(
            value = value,
            onValueChange = onValueChange,
            modifier = Modifier
                .fillMaxWidth()
                .clip(RoundedCornerShape(8.dp))
                .background(AltairColors.surface)
                .border(
                    width = 1.dp,
                    color = borderColor,
                    shape = RoundedCornerShape(8.dp)
                )
                .padding(horizontal = 12.dp, vertical = 10.dp),
            enabled = enabled,
            textStyle = TextStyle(
                color = textColor,
                fontSize = 14.sp
            ),
            singleLine = singleLine,
            interactionSource = interactionSource,
            cursorBrush = SolidColor(AltairColors.accent),
            decorationBox = { innerTextField ->
                Box {
                    if (value.isEmpty() && placeholder != null) {
                        BasicText(
                            text = placeholder,
                            style = TextStyle(
                                color = AltairColors.textTertiary,
                                fontSize = 14.sp
                            )
                        )
                    }
                    innerTextField()
                }
            }
        )
    }
}

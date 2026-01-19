package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Altair-styled text field component.
 *
 * Provides consistent styling with support for labels, error states,
 * placeholder text, and visual transformation (for passwords).
 *
 * @param value The current text value.
 * @param onValueChange Callback invoked when the text value changes.
 * @param modifier Modifier to be applied to the text field container.
 * @param label Optional label text displayed above the input.
 * @param placeholder Optional placeholder text shown when input is empty.
 * @param isError Whether the text field is in an error state.
 * @param errorMessage Error message to display below the input when [isError] is true.
 * @param enabled Whether the text field accepts input.
 * @param singleLine Whether the input is limited to a single line.
 * @param visualTransformation Visual transformation applied to the input (e.g., password masking).
 * @param leading Optional composable displayed at the start of the input.
 * @param trailing Optional composable displayed at the end of the input.
 */
@Composable
fun AltairTextField(
    value: String,
    onValueChange: (String) -> Unit,
    modifier: Modifier = Modifier,
    label: String? = null,
    placeholder: String? = null,
    isError: Boolean = false,
    errorMessage: String? = null,
    enabled: Boolean = true,
    singleLine: Boolean = true,
    visualTransformation: VisualTransformation = VisualTransformation.None,
    leading: @Composable (() -> Unit)? = null,
    trailing: @Composable (() -> Unit)? = null,
) {
    val borderColor = when {
        isError -> AltairTheme.Colors.error
        !enabled -> AltairTheme.Colors.borderSubtle
        else -> AltairTheme.Colors.border
    }

    val backgroundColor = if (enabled) {
        AltairTheme.Colors.backgroundElevated
    } else {
        AltairTheme.Colors.backgroundSubtle
    }

    val textColor = if (enabled) {
        AltairTheme.Colors.textPrimary
    } else {
        AltairTheme.Colors.textDisabled
    }

    Column(modifier = modifier) {
        if (label != null) {
            AltairText(
                text = label,
                style = AltairTheme.Typography.labelMedium,
                color = if (isError) AltairTheme.Colors.error else AltairTheme.Colors.textSecondary,
            )
            Spacer(modifier = Modifier.height(AltairTheme.Spacing.xs))
        }

        BasicTextField(
            value = value,
            onValueChange = onValueChange,
            textStyle = AltairTheme.Typography.bodyMedium.copy(color = textColor),
            modifier = Modifier.fillMaxWidth(),
            enabled = enabled,
            singleLine = singleLine,
            visualTransformation = visualTransformation,
            cursorBrush = SolidColor(AltairTheme.Colors.accent),
            decorationBox = { innerTextField ->
                TextFieldDecoration(
                    value = value,
                    placeholder = placeholder,
                    leading = leading,
                    trailing = trailing,
                    backgroundColor = backgroundColor,
                    borderColor = borderColor,
                    innerTextField = innerTextField,
                )
            },
        )

        if (isError && errorMessage != null) {
            Spacer(modifier = Modifier.height(AltairTheme.Spacing.xs))
            AltairText(
                text = errorMessage,
                style = AltairTheme.Typography.labelSmall,
                color = AltairTheme.Colors.error,
            )
        }
    }
}

@Composable
private fun TextFieldDecoration(
    value: String,
    placeholder: String?,
    leading: @Composable (() -> Unit)?,
    trailing: @Composable (() -> Unit)?,
    backgroundColor: Color,
    borderColor: Color,
    innerTextField: @Composable () -> Unit,
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .background(
                color = backgroundColor,
                shape = RoundedCornerShape(AltairTheme.Radii.md),
            )
            .border(
                width = 1.dp,
                color = borderColor,
                shape = RoundedCornerShape(AltairTheme.Radii.md),
            )
            .padding(
                horizontal = AltairTheme.Spacing.md,
                vertical = AltairTheme.Spacing.sm + 4.dp,
            ),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        if (leading != null) {
            leading()
            Spacer(modifier = Modifier.width(AltairTheme.Spacing.sm))
        }

        Box(modifier = Modifier.weight(1f)) {
            if (value.isEmpty() && placeholder != null) {
                AltairText(
                    text = placeholder,
                    style = AltairTheme.Typography.bodyMedium,
                    color = AltairTheme.Colors.textTertiary,
                )
            }
            innerTextField()
        }

        if (trailing != null) {
            Spacer(modifier = Modifier.width(AltairTheme.Spacing.sm))
            trailing()
        }
    }
}

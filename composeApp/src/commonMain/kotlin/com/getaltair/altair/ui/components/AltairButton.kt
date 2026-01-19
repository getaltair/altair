package com.getaltair.altair.ui.components

import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.composeunstyled.UnstyledButton
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Altair-styled button component wrapping Compose Unstyled Button.
 *
 * Provides consistent styling across all platforms with support for
 * multiple visual variants: Primary, Secondary, Ghost, and Danger.
 *
 * @param onClick Callback invoked when the button is clicked.
 * @param modifier Modifier to be applied to the button.
 * @param variant Visual style variant. Defaults to [ButtonVariant.Primary].
 * @param enabled Whether the button is enabled. Defaults to true.
 * @param content The button content (text, icons, etc.).
 */
@Composable
fun AltairButton(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    variant: ButtonVariant = ButtonVariant.Primary,
    enabled: Boolean = true,
    content: @Composable RowScope.() -> Unit,
) {
    val backgroundColor = if (enabled) {
        variant.backgroundColor()
    } else {
        AltairTheme.Colors.backgroundSubtle
    }

    val contentColor = if (enabled) {
        variant.contentColor()
    } else {
        AltairTheme.Colors.textDisabled
    }

    val borderColor = if (enabled) {
        variant.borderColor()
    } else {
        Color.Transparent
    }

    UnstyledButton(
        onClick = { if (enabled) onClick() },
        modifier = modifier,
        backgroundColor = backgroundColor,
        contentColor = contentColor,
        contentPadding = PaddingValues(
            horizontal = AltairTheme.Spacing.md,
            vertical = AltairTheme.Spacing.sm + 4.dp,
        ),
        shape = RoundedCornerShape(AltairTheme.Radii.md),
        borderColor = borderColor,
        borderWidth = if (borderColor != Color.Transparent) 1.dp else 0.dp,
        enabled = enabled,
        content = content,
    )
}

/**
 * Altair-styled text button (Ghost variant shorthand).
 *
 * A convenience composable for creating minimal text-style buttons
 * commonly used for secondary or tertiary actions.
 *
 * @param onClick Callback invoked when the button is clicked.
 * @param modifier Modifier to be applied to the button.
 * @param enabled Whether the button is enabled. Defaults to true.
 * @param content The button content.
 */
@Composable
fun AltairTextButton(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    content: @Composable RowScope.() -> Unit,
) {
    AltairButton(
        onClick = onClick,
        modifier = modifier,
        variant = ButtonVariant.Ghost,
        enabled = enabled,
        content = content,
    )
}

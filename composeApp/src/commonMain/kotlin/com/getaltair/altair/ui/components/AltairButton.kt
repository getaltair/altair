package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.focusable
import androidx.compose.foundation.hoverable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsFocusedAsState
import androidx.compose.foundation.interaction.collectIsHoveredAsState
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
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
 * Button variants for AltairButton styling.
 */
enum class ButtonVariant {
    /** Primary action button with accent background */
    Primary,
    /** Secondary button with surface background and border */
    Secondary,
    /** Ghost button with transparent background */
    Ghost
}

/**
 * Altair Design System button component.
 *
 * A styled button that supports three variants (Primary, Secondary, Ghost),
 * focus ring on keyboard navigation, and hover states.
 *
 * @param onClick Callback when the button is clicked
 * @param modifier Modifier to be applied to the button
 * @param variant Button styling variant (Primary, Secondary, Ghost)
 * @param enabled Whether the button is enabled and interactive
 * @param content Content to display inside the button (typically Text)
 */
@Composable
fun AltairButton(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    variant: ButtonVariant = ButtonVariant.Primary,
    enabled: Boolean = true,
    content: @Composable RowScope.() -> Unit
) {
    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes
    val spacing = AltairTheme.spacing

    val interactionSource = remember { MutableInteractionSource() }
    val isFocused by interactionSource.collectIsFocusedAsState()
    val isHovered by interactionSource.collectIsHoveredAsState()

    // Determine background color based on variant and hover state
    val backgroundColor = when (variant) {
        ButtonVariant.Primary -> if (isHovered) colors.accentHover else colors.accent
        ButtonVariant.Secondary -> if (isHovered) colors.surfaceHover else colors.surface
        ButtonVariant.Ghost -> if (isHovered) colors.surfaceHover else Color.Transparent
    }

    // Determine border based on variant and focus state
    val borderModifier = when {
        isFocused -> Modifier.border(2.dp, colors.borderFocused, shapes.md)
        variant == ButtonVariant.Secondary -> Modifier.border(1.dp, colors.border, shapes.md)
        else -> Modifier
    }

    Row(
        modifier = modifier
            .semantics { role = Role.Button }
            .clip(shapes.md)
            .then(borderModifier)
            .background(backgroundColor, shapes.md)
            .clickable(
                interactionSource = interactionSource,
                indication = null,
                enabled = enabled,
                onClick = onClick
            )
            .focusable(interactionSource = interactionSource)
            .hoverable(interactionSource = interactionSource)
            .padding(horizontal = spacing.md, vertical = spacing.sm)
            .alpha(if (enabled) 1f else 0.5f),
        horizontalArrangement = Arrangement.Center,
        verticalAlignment = Alignment.CenterVertically,
        content = content
    )
}

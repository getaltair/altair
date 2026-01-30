package com.getaltair.altair.ui.theme.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsFocusedAsState
import androidx.compose.foundation.interaction.collectIsHoveredAsState
import androidx.compose.foundation.interaction.collectIsPressedAsState
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.unit.dp
import androidx.compose.runtime.staticCompositionLocalOf
import com.getaltair.altair.ui.theme.AltairColors

/**
 * Local content color provider for AltairButton content.
 * Allows button content to read the appropriate text color based on variant.
 */
val LocalAltairContentColor = staticCompositionLocalOf { AltairColors.textPrimary }

/**
 * Button variants for different visual styles.
 */
enum class ButtonVariant {
    /** Accent background with white text - primary action */
    Primary,
    /** Surface background with border and primary text - secondary action */
    Secondary,
    /** Transparent background with accent text - tertiary action */
    Ghost
}

/**
 * Altair-styled button component using Compose foundation primitives.
 *
 * Provides three visual variants (Primary, Secondary, Ghost) with consistent
 * styling following the Altair design system.
 *
 * @param onClick Callback invoked when the button is clicked
 * @param modifier Modifier to be applied to the button
 * @param variant Visual style variant (Primary, Secondary, or Ghost)
 * @param enabled Whether the button is enabled for interaction
 * @param content Button content (typically text or icon+text)
 */
@Composable
fun AltairButton(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    variant: ButtonVariant = ButtonVariant.Primary,
    enabled: Boolean = true,
    content: @Composable RowScope.() -> Unit
) {
    val interactionSource = remember { MutableInteractionSource() }
    val isHovered by interactionSource.collectIsHoveredAsState()
    val isFocused by interactionSource.collectIsFocusedAsState()
    val isPressed by interactionSource.collectIsPressedAsState()

    val backgroundColor = when {
        !enabled -> AltairColors.surface.copy(alpha = 0.5f)
        isPressed -> when (variant) {
            ButtonVariant.Primary -> AltairColors.accent.copy(alpha = 0.8f)
            ButtonVariant.Secondary -> AltairColors.surfaceHover
            ButtonVariant.Ghost -> Color.Transparent
        }
        isHovered -> when (variant) {
            ButtonVariant.Primary -> AltairColors.accentHover
            ButtonVariant.Secondary -> AltairColors.surfaceHover
            ButtonVariant.Ghost -> AltairColors.surface
        }
        else -> when (variant) {
            ButtonVariant.Primary -> AltairColors.accent
            ButtonVariant.Secondary -> AltairColors.surface
            ButtonVariant.Ghost -> Color.Transparent
        }
    }

    val contentColor = when {
        !enabled -> AltairColors.textTertiary
        else -> when (variant) {
            ButtonVariant.Primary -> Color.White
            ButtonVariant.Secondary -> AltairColors.textPrimary
            ButtonVariant.Ghost -> AltairColors.accent
        }
    }

    val borderColor = when (variant) {
        ButtonVariant.Secondary -> if (isFocused) AltairColors.borderFocused else AltairColors.border
        else -> if (isFocused) AltairColors.borderFocused else Color.Transparent
    }

    val borderWidth = when {
        isFocused && variant != ButtonVariant.Secondary -> 2.dp
        variant == ButtonVariant.Secondary -> 1.dp
        else -> 0.dp
    }

    val shape = RoundedCornerShape(8.dp)

    CompositionLocalProvider(LocalAltairContentColor provides contentColor) {
        Row(
            modifier = modifier
                .clip(shape)
                .background(backgroundColor)
                .border(
                    width = borderWidth,
                    color = borderColor,
                    shape = shape
                )
                .clickable(
                    onClick = onClick,
                    enabled = enabled,
                    role = Role.Button,
                    interactionSource = interactionSource,
                    indication = null
                )
                .padding(horizontal = 16.dp, vertical = 12.dp),
            verticalAlignment = Alignment.CenterVertically,
            content = content
        )
    }
}

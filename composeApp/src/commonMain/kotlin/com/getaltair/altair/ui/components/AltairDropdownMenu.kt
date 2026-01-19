package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Popup
import androidx.compose.ui.window.PopupProperties
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.LocalAltairColors

/** Default vertical offset for dropdown menu positioning. */
private const val DROPDOWN_MENU_OFFSET_Y = 4

/**
 * Altair-styled dropdown menu component.
 *
 * A popup menu that appears below an anchor element. Use for context menus,
 * action menus, and select dropdowns.
 *
 * @param expanded Whether the menu is currently visible.
 * @param onDismissRequest Callback invoked when the menu should be dismissed.
 * @param modifier Modifier to be applied to the menu panel.
 * @param offset Offset from the anchor position.
 * @param content The menu items.
 */
@Composable
fun AltairDropdownMenu(
    expanded: Boolean,
    onDismissRequest: () -> Unit,
    modifier: Modifier = Modifier,
    offset: IntOffset = IntOffset(0, DROPDOWN_MENU_OFFSET_Y),
    content: @Composable ColumnScope.() -> Unit,
) {
    val colors = LocalAltairColors.current

    if (expanded) {
        Popup(
            onDismissRequest = onDismissRequest,
            offset = offset,
            properties = PopupProperties(focusable = true),
        ) {
            Column(
                modifier =
                    modifier
                        .widthIn(min = 160.dp, max = 280.dp)
                        .shadow(
                            elevation = 8.dp,
                            shape = RoundedCornerShape(AltairTheme.Radii.md),
                            ambientColor = Color.Black.copy(alpha = 0.3f),
                            spotColor = Color.Black.copy(alpha = 0.3f),
                        ).clip(RoundedCornerShape(AltairTheme.Radii.md))
                        .background(colors.backgroundElevated)
                        .border(
                            width = 1.dp,
                            color = colors.border,
                            shape = RoundedCornerShape(AltairTheme.Radii.md),
                        ).padding(vertical = AltairTheme.Spacing.xs),
                content = content,
            )
        }
    }
}

/**
 * A clickable item within [AltairDropdownMenu].
 *
 * @param onClick Callback invoked when the item is clicked.
 * @param modifier Modifier to be applied to the item.
 * @param enabled Whether the item is clickable.
 * @param content The item content (typically text and optional icon).
 */
@Composable
fun AltairDropdownMenuItem(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    content: @Composable () -> Unit,
) {
    val interactionSource = remember { MutableInteractionSource() }

    Box(
        modifier =
            modifier
                .fillMaxWidth()
                .clickable(
                    interactionSource = interactionSource,
                    indication = null,
                    enabled = enabled,
                    onClick = onClick,
                ).background(Color.Transparent)
                .padding(
                    horizontal = AltairTheme.Spacing.md,
                    vertical = AltairTheme.Spacing.sm,
                ),
        contentAlignment = Alignment.CenterStart,
    ) {
        content()
    }
}

/**
 * Text-only dropdown menu item.
 *
 * A convenience composable for simple text menu items.
 *
 * @param text The menu item text.
 * @param onClick Callback invoked when the item is clicked.
 * @param modifier Modifier to be applied to the item.
 * @param enabled Whether the item is clickable.
 */
@Composable
fun AltairDropdownMenuItem(
    text: String,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
) {
    val colors = LocalAltairColors.current

    AltairDropdownMenuItem(
        onClick = onClick,
        modifier = modifier,
        enabled = enabled,
    ) {
        AltairText(
            text = text,
            style = AltairTheme.Typography.bodyMedium,
            color =
                if (enabled) {
                    colors.textPrimary
                } else {
                    colors.textDisabled
                },
        )
    }
}

/**
 * Divider for separating groups of menu items.
 *
 * @param modifier Modifier to be applied to the divider.
 */
@Composable
fun AltairDropdownMenuDivider(
    modifier: Modifier = Modifier,
) {
    val colors = LocalAltairColors.current

    Spacer(
        modifier =
            modifier
                .fillMaxWidth()
                .padding(vertical = AltairTheme.Spacing.xs)
                .height(1.dp)
                .background(colors.borderSubtle),
    )
}

/**
 * Section header within a dropdown menu.
 *
 * Use to label groups of related menu items.
 *
 * @param text The section header text.
 * @param modifier Modifier to be applied to the header.
 */
@Composable
fun AltairDropdownMenuHeader(
    text: String,
    modifier: Modifier = Modifier,
) {
    val colors = LocalAltairColors.current

    AltairText(
        text = text,
        style = AltairTheme.Typography.labelSmall,
        color = colors.textTertiary,
        modifier =
            modifier
                .fillMaxWidth()
                .padding(
                    horizontal = AltairTheme.Spacing.md,
                    vertical = AltairTheme.Spacing.xs,
                ),
    )
}

package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.MenuBook
import androidx.compose.material.icons.automirrored.outlined.MenuBook
import androidx.compose.material.icons.filled.BarChart
import androidx.compose.material.icons.filled.Explore
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material.icons.outlined.BarChart
import androidx.compose.material.icons.outlined.Explore
import androidx.compose.material.icons.outlined.Home
import androidx.compose.material.icons.outlined.Settings
import androidx.compose.material3.Icon
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.selected
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import com.getaltair.altair.navigation.MainDestination
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.LocalAltairColors
import com.getaltair.altair.ui.theme.themedFocusRing

/**
 * Icon configuration for a navigation destination.
 */
private data class NavigationIcon(
    val filled: ImageVector,
    val outlined: ImageVector,
)

/**
 * Returns the icon configuration for a [MainDestination].
 */
private fun MainDestination.navigationIcon(): NavigationIcon =
    when (this) {
        MainDestination.Home -> NavigationIcon(Icons.Filled.Home, Icons.Outlined.Home)
        MainDestination.Guidance -> NavigationIcon(Icons.Filled.Explore, Icons.Outlined.Explore)
        MainDestination.Knowledge ->
            NavigationIcon(Icons.AutoMirrored.Filled.MenuBook, Icons.AutoMirrored.Outlined.MenuBook)
        MainDestination.Tracking -> NavigationIcon(Icons.Filled.BarChart, Icons.Outlined.BarChart)
        MainDestination.Settings -> NavigationIcon(Icons.Filled.Settings, Icons.Outlined.Settings)
    }

/**
 * Returns the display label for a [MainDestination].
 * Used by navigation UI components and accessibility descriptions.
 */
private fun MainDestination.label(): String =
    when (this) {
        MainDestination.Home -> "Home"
        MainDestination.Guidance -> "Guidance"
        MainDestination.Knowledge -> "Knowledge"
        MainDestination.Tracking -> "Tracking"
        MainDestination.Settings -> "Settings"
    }

/**
 * A navigation item for use in bottom bar or rail.
 *
 * @param destination The destination this item represents.
 * @param selected Whether this item is currently selected.
 * @param onClick Called when the item is clicked.
 * @param modifier Modifier to apply.
 * @param showLabel Whether to show the text label below the icon.
 */
@Composable
fun AltairNavigationItem(
    destination: MainDestination,
    selected: Boolean,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    showLabel: Boolean = true,
) {
    val colors = LocalAltairColors.current
    val icon = destination.navigationIcon()
    val interactionSource = remember { MutableInteractionSource() }

    val backgroundColor = if (selected) colors.accentSubtle else colors.background
    val iconColor = if (selected) colors.accent else colors.textSecondary
    val labelColor = if (selected) colors.textPrimary else colors.textSecondary

    Column(
        modifier =
            modifier
                .semantics { this.selected = selected }
                .themedFocusRing(shape = RoundedCornerShape(AltairTheme.Radii.md))
                .clip(RoundedCornerShape(AltairTheme.Radii.md))
                .clickable(
                    interactionSource = interactionSource,
                    indication = null,
                    onClick = onClick,
                    role = Role.Tab,
                ).padding(horizontal = AltairTheme.Spacing.sm, vertical = AltairTheme.Spacing.xs),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center,
    ) {
        Box(
            modifier =
                Modifier
                    .clip(RoundedCornerShape(AltairTheme.Radii.md))
                    .background(backgroundColor)
                    .padding(horizontal = AltairTheme.Spacing.md, vertical = AltairTheme.Spacing.xs),
            contentAlignment = Alignment.Center,
        ) {
            Icon(
                imageVector = if (selected) icon.filled else icon.outlined,
                contentDescription = destination.label(),
                tint = iconColor,
                modifier = Modifier.size(24.dp),
            )
        }
        if (showLabel) {
            AltairText(
                text = destination.label(),
                style = AltairTheme.Typography.labelSmall,
                color = labelColor,
                modifier = Modifier.padding(top = AltairTheme.Spacing.xs),
            )
        }
    }
}

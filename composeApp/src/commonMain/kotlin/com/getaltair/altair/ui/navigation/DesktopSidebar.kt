package com.getaltair.altair.ui.navigation

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.getaltair.altair.navigation.RootComponent
import com.getaltair.altair.ui.theme.AltairColors
import com.getaltair.altair.ui.theme.AltairSpacing
import com.getaltair.altair.ui.theme.AltairTypography

/**
 * Sidebar navigation for desktop platforms.
 * Vertical list of navigation items with icons/labels.
 */
@Composable
fun DesktopSidebar(
    currentConfig: RootComponent.Config,
    onNavigate: (RootComponent.Config) -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier
            .fillMaxHeight()
            .width(200.dp)
            .background(AltairColors.surfaceElevated)
            .padding(vertical = AltairSpacing.md),
        horizontalAlignment = Alignment.Start
    ) {
        SidebarItem("Home", currentConfig is RootComponent.Config.Home) {
            onNavigate(RootComponent.Config.Home)
        }
        SidebarItem("Guidance", currentConfig is RootComponent.Config.Guidance) {
            onNavigate(RootComponent.Config.Guidance)
        }
        SidebarItem("Knowledge", currentConfig is RootComponent.Config.Knowledge) {
            onNavigate(RootComponent.Config.Knowledge)
        }
        SidebarItem("Tracking", currentConfig is RootComponent.Config.Tracking) {
            onNavigate(RootComponent.Config.Tracking)
        }
        SidebarItem("Settings", currentConfig is RootComponent.Config.Settings) {
            onNavigate(RootComponent.Config.Settings)
        }
    }
}

@Composable
private fun SidebarItem(
    label: String,
    selected: Boolean,
    onClick: () -> Unit
) {
    BasicText(
        text = label,
        style = AltairTypography.bodyMedium.copy(
            color = if (selected) AltairColors.accent else AltairColors.textSecondary
        ),
        modifier = Modifier
            .clickable(onClick = onClick)
            .padding(horizontal = AltairSpacing.md, vertical = AltairSpacing.sm)
    )
}

package com.getaltair.altair.ui.navigation

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.getaltair.altair.navigation.RootComponent
import com.getaltair.altair.ui.theme.AltairColors
import com.getaltair.altair.ui.theme.AltairSpacing
import com.getaltair.altair.ui.theme.AltairTypography

/**
 * Bottom navigation bar for mobile platforms.
 * Shows icons/labels for main modules: Home, Guidance, Knowledge, Tracking, Settings.
 */
@Composable
fun BottomNavBar(
    currentConfig: RootComponent.Config,
    onNavigate: (RootComponent.Config) -> Unit,
    modifier: Modifier = Modifier
) {
    Row(
        modifier = modifier
            .fillMaxWidth()
            .background(AltairColors.surface)
            .padding(vertical = AltairSpacing.sm),
        horizontalArrangement = Arrangement.SpaceEvenly,
        verticalAlignment = Alignment.CenterVertically
    ) {
        NavItem("Home", currentConfig is RootComponent.Config.Home) {
            onNavigate(RootComponent.Config.Home)
        }
        NavItem("Guidance", currentConfig is RootComponent.Config.Guidance) {
            onNavigate(RootComponent.Config.Guidance)
        }
        NavItem("Knowledge", currentConfig is RootComponent.Config.Knowledge) {
            onNavigate(RootComponent.Config.Knowledge)
        }
        NavItem("Tracking", currentConfig is RootComponent.Config.Tracking) {
            onNavigate(RootComponent.Config.Tracking)
        }
        NavItem("Settings", currentConfig is RootComponent.Config.Settings) {
            onNavigate(RootComponent.Config.Settings)
        }
    }
}

@Composable
private fun NavItem(
    label: String,
    selected: Boolean,
    onClick: () -> Unit
) {
    Column(
        modifier = Modifier
            .clickable(onClick = onClick)
            .padding(AltairSpacing.sm),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        BasicText(
            text = label,
            style = AltairTypography.labelSmall.copy(
                color = if (selected) AltairColors.accent else AltairColors.textSecondary
            )
        )
    }
}

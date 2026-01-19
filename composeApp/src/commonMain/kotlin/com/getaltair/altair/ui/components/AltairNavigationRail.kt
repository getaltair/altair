package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.getaltair.altair.navigation.MainDestination
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.LocalAltairColors

/**
 * Navigation rail for desktop/tablet platforms.
 *
 * Displays the main destinations (Home, Guidance, Knowledge, Tracking) at the top,
 * with Settings pinned at the bottom.
 *
 * @param currentDestination The currently selected destination.
 * @param onSelectDestination Called when a destination is selected.
 * @param modifier Modifier to apply.
 */
@Composable
fun AltairNavigationRail(
    currentDestination: MainDestination,
    onSelectDestination: (MainDestination) -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalAltairColors.current

    // Split destinations: main ones at top, settings at bottom
    val topDestinations = MainDestination.entries.filter { it != MainDestination.Settings }

    Column(
        modifier =
            modifier
                .fillMaxHeight()
                .width(80.dp)
                .background(colors.backgroundElevated)
                .padding(vertical = AltairTheme.Spacing.md),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Top,
    ) {
        // Main destinations at top
        topDestinations.forEach { destination ->
            AltairNavigationItem(
                destination = destination,
                selected = destination == currentDestination,
                onClick = { onSelectDestination(destination) },
                showLabel = true,
                modifier = Modifier.padding(vertical = AltairTheme.Spacing.xs),
            )
        }

        // Spacer pushes settings to bottom
        Spacer(modifier = Modifier.weight(1f))

        // Settings at bottom
        AltairNavigationItem(
            destination = MainDestination.Settings,
            selected = currentDestination == MainDestination.Settings,
            onClick = { onSelectDestination(MainDestination.Settings) },
            showLabel = true,
            modifier = Modifier.padding(vertical = AltairTheme.Spacing.xs),
        )
    }
}

package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.getaltair.altair.navigation.MainDestination
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.LocalAltairColors

/**
 * Bottom navigation bar for mobile platforms.
 *
 * Displays all 5 main destinations (Home, Guidance, Knowledge, Tracking, Settings)
 * in a horizontal row at the bottom of the screen.
 *
 * @param currentDestination The currently selected destination.
 * @param onSelectDestination Called when a destination is selected.
 * @param modifier Modifier to apply.
 */
@Composable
fun AltairBottomNavigationBar(
    currentDestination: MainDestination,
    onSelectDestination: (MainDestination) -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalAltairColors.current

    Row(
        modifier =
            modifier
                .fillMaxWidth()
                .background(colors.backgroundElevated)
                .padding(vertical = AltairTheme.Spacing.xs),
        horizontalArrangement = Arrangement.SpaceEvenly,
        verticalAlignment = Alignment.CenterVertically,
    ) {
        MainDestination.entries.forEach { destination ->
            AltairNavigationItem(
                destination = destination,
                selected = destination == currentDestination,
                onClick = { onSelectDestination(destination) },
                showLabel = true,
            )
        }
    }
}

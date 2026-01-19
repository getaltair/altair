package com.getaltair.altair.ui.components

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.movableContentOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import com.getaltair.altair.navigation.MainDestination
import com.getaltair.altair.ui.util.WithWindowSizeClass
import com.getaltair.altair.ui.util.useBottomNavigation

/**
 * Platform-adaptive navigation shell.
 *
 * Switches between:
 * - Bottom navigation bar for compact screens (phones)
 * - Side navigation rail for medium and expanded screens (tablets, desktops)
 *
 * Uses [movableContentOf] to preserve content state when switching layouts
 * during window resize.
 *
 * @param currentDestination The currently selected destination.
 * @param onSelectDestination Called when a destination is selected.
 * @param modifier Modifier to apply to the shell container.
 * @param content The main content area, displayed alongside the navigation.
 */
@Composable
fun NavigationShell(
    currentDestination: MainDestination,
    onSelectDestination: (MainDestination) -> Unit,
    modifier: Modifier = Modifier,
    content: @Composable () -> Unit,
) {
    // Use movableContentOf to preserve state when content moves between layouts
    val movableContent = remember(content) { movableContentOf(content) }

    WithWindowSizeClass(modifier = modifier.fillMaxSize()) { windowSizeClass ->
        if (windowSizeClass.useBottomNavigation()) {
            // Mobile: Content above, bottom navigation bar below
            Column(modifier = Modifier.fillMaxSize()) {
                Box(modifier = Modifier.weight(1f)) {
                    movableContent()
                }
                AltairBottomNavigationBar(
                    currentDestination = currentDestination,
                    onSelectDestination = onSelectDestination,
                )
            }
        } else {
            // Desktop/Tablet: Navigation rail on left, content on right
            Row(modifier = Modifier.fillMaxSize()) {
                AltairNavigationRail(
                    currentDestination = currentDestination,
                    onSelectDestination = onSelectDestination,
                )
                Box(modifier = Modifier.weight(1f)) {
                    movableContent()
                }
            }
        }
    }
}

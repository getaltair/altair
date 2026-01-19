package com.getaltair.altair.navigation

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import com.getaltair.altair.ui.components.NavigationShell
import com.getaltair.altair.ui.guidance.GuidanceScreen
import com.getaltair.altair.ui.home.HomeScreen
import com.getaltair.altair.ui.knowledge.KnowledgeScreen
import com.getaltair.altair.ui.settings.SettingsScreen
import com.getaltair.altair.ui.tracking.TrackingScreen

/**
 * Main content composable that renders the navigation shell with child screens.
 *
 * This is the primary authenticated user interface, containing:
 * - Platform-adaptive navigation (bottom bar on mobile, side rail on desktop)
 * - Tab-based navigation between Home, Guidance, Knowledge, Tracking, and Settings
 *
 * @param component The MainComponent that holds navigation state.
 * @param modifier Modifier to apply.
 */
@Composable
fun MainContent(
    component: MainComponent,
    modifier: Modifier = Modifier,
) {
    val currentDestination by component.currentDestination.collectAsState()

    NavigationShell(
        currentDestination = currentDestination,
        onSelectDestination = { component.navigateTo(it) },
        modifier = modifier.fillMaxSize(),
    ) {
        when (currentDestination) {
            MainDestination.Home -> HomeScreen(modifier = Modifier.fillMaxSize())
            MainDestination.Guidance -> GuidanceScreen(modifier = Modifier.fillMaxSize())
            MainDestination.Knowledge -> KnowledgeScreen(modifier = Modifier.fillMaxSize())
            MainDestination.Tracking -> TrackingScreen(modifier = Modifier.fillMaxSize())
            MainDestination.Settings -> SettingsScreen(modifier = Modifier.fillMaxSize())
        }
    }
}

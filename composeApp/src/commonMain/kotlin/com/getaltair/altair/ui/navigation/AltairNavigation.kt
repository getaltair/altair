package com.getaltair.altair.ui.navigation

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.getaltair.altair.navigation.RootComponent

/**
 * Platform-adaptive navigation that shows:
 * - BottomNavBar on mobile (Android, iOS)
 * - DesktopSidebar on desktop (JVM)
 *
 * Uses runtime platform detection.
 */
@Composable
fun AltairNavigation(
    currentConfig: RootComponent.Config,
    onNavigate: (RootComponent.Config) -> Unit,
    modifier: Modifier = Modifier,
    isDesktop: Boolean = false
) {
    if (isDesktop) {
        DesktopSidebar(currentConfig, onNavigate, modifier)
    } else {
        BottomNavBar(currentConfig, onNavigate, modifier)
    }
}

package com.getaltair.altair

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import com.arkivanov.decompose.extensions.compose.stack.Children
import com.arkivanov.decompose.extensions.compose.stack.animation.fade
import com.arkivanov.decompose.extensions.compose.stack.animation.stackAnimation
import com.arkivanov.decompose.extensions.compose.subscribeAsState
import com.getaltair.altair.navigation.RootComponent
import com.getaltair.altair.ui.HomeScreen
import com.getaltair.altair.ui.SettingsScreen
import com.getaltair.altair.ui.navigation.BottomNavBar
import com.getaltair.altair.ui.screens.GuidanceScreen
import com.getaltair.altair.ui.screens.KnowledgeScreen
import com.getaltair.altair.ui.screens.TrackingScreen
import com.getaltair.altair.ui.theme.AltairColors
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Main application composable.
 * Receives RootComponent for navigation and renders current screen.
 */
@Composable
fun App(rootComponent: RootComponent) {
    AltairTheme {
        val stack by rootComponent.stack.subscribeAsState()
        val currentConfig = stack.active.configuration

        Column(
            modifier = Modifier
                .fillMaxSize()
                .background(AltairColors.background)
        ) {
            // Main content area
            Box(modifier = Modifier.weight(1f)) {
                Children(
                    stack = rootComponent.stack,
                    animation = stackAnimation(fade())
                ) { child ->
                    when (val instance = child.instance) {
                        is RootComponent.Child.Home -> HomeScreen(instance.component)
                        is RootComponent.Child.Guidance -> GuidanceScreen(instance.component)
                        is RootComponent.Child.Knowledge -> KnowledgeScreen(instance.component)
                        is RootComponent.Child.Tracking -> TrackingScreen(instance.component)
                        is RootComponent.Child.Settings -> SettingsScreen(instance.component)
                    }
                }
            }

            // Bottom navigation
            BottomNavBar(
                currentConfig = currentConfig,
                onNavigate = { rootComponent.navigateTo(it) }
            )
        }
    }
}
package com.getaltair.altair.navigation

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.arkivanov.decompose.extensions.compose.stack.Children
import com.arkivanov.decompose.extensions.compose.stack.animation.fade
import com.arkivanov.decompose.extensions.compose.stack.animation.stackAnimation
import com.getaltair.altair.ui.auth.LoginScreen
import com.getaltair.altair.ui.auth.RegisterScreen
import com.getaltair.altair.ui.components.AltairSurface
import com.getaltair.altair.ui.theme.AltairThemeProvider

/**
 * Root content composable that renders the navigation stack.
 *
 * This is the main entry point for the UI, handling navigation between:
 * - Authentication screens (Login/Register) for unauthenticated users
 * - Main application shell with tab navigation for authenticated users
 */
@Composable
fun RootContent(
    component: RootComponent,
    modifier: Modifier = Modifier,
) {
    AltairThemeProvider {
        AltairSurface(modifier = modifier.fillMaxSize()) {
            Children(
                stack = component.stack,
                modifier = Modifier.fillMaxSize(),
                animation = stackAnimation(fade()),
            ) { child ->
                when (val instance = child.instance) {
                    is RootComponent.Child.Main -> MainContent(instance.component)
                    is RootComponent.Child.Login -> LoginScreen(instance.component)
                    is RootComponent.Child.Register -> RegisterScreen(instance.component)
                }
            }
        }
    }
}

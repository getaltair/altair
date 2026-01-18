package com.getaltair.altair.navigation

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.arkivanov.decompose.extensions.compose.stack.Children
import com.arkivanov.decompose.extensions.compose.stack.animation.fade
import com.arkivanov.decompose.extensions.compose.stack.animation.stackAnimation
import com.getaltair.altair.ui.auth.LoginScreen
import com.getaltair.altair.ui.auth.RegisterScreen
import com.getaltair.altair.ui.home.HomeScreen

/**
 * Root content composable that renders the navigation stack.
 *
 * This is the main entry point for the UI, handling navigation between
 * authentication screens and the main application content.
 */
@Composable
fun RootContent(
    component: RootComponent,
    modifier: Modifier = Modifier,
) {
    MaterialTheme {
        Surface(
            modifier = modifier.fillMaxSize(),
            color = MaterialTheme.colorScheme.background,
        ) {
            Children(
                stack = component.stack,
                modifier = Modifier.fillMaxSize(),
                animation = stackAnimation(fade()),
            ) { child ->
                when (val instance = child.instance) {
                    is RootComponent.Child.Home -> HomeScreen()
                    is RootComponent.Child.Login -> LoginScreen(instance.component)
                    is RootComponent.Child.Register -> RegisterScreen(instance.component)
                }
            }
        }
    }
}

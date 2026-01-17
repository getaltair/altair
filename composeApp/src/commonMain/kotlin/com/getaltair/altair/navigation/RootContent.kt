package com.getaltair.altair.navigation

import androidx.compose.runtime.Composable
import com.arkivanov.decompose.extensions.compose.stack.Children
import com.getaltair.altair.App

/**
 * Renders the current navigation stack.
 * As features are added, this will route to different screens.
 */
@Composable
fun RootContent(component: RootComponent) {
    Children(
        stack = component.stack,
    ) { child ->
        when (val instance = child.instance) {
            is RootComponent.Child.Home -> App()
        }
    }
}

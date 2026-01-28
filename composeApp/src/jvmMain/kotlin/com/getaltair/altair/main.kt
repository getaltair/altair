package com.getaltair.altair

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.getaltair.altair.di.initKoinDesktop
import com.getaltair.altair.navigation.RootComponent
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.swing.Swing

fun main() = runBlocking(Dispatchers.Swing) {
    // Initialize Koin for desktop
    initKoinDesktop()

    // Create lifecycle for Decompose
    val lifecycle = LifecycleRegistry()

    // Create root component on Swing/main thread
    val rootComponent = RootComponent(
        componentContext = DefaultComponentContext(lifecycle = lifecycle)
    )

    application {
        Window(
            onCloseRequest = ::exitApplication,
            title = "Altair",
        ) {
            App(rootComponent)
        }
    }
}
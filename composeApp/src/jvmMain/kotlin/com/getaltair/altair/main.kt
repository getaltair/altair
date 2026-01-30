package com.getaltair.altair

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.getaltair.altair.di.initKoinDesktop
import com.getaltair.altair.navigation.RootComponent
import com.getaltair.altair.persistence.DesktopSurrealDbClient
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.swing.Swing
import org.koin.java.KoinJavaComponent.getKoin

fun main() = runBlocking(Dispatchers.Swing) {
    // Initialize Koin for desktop
    initKoinDesktop()

    // Initialize embedded database connection
    val dbClient = getKoin().get<DesktopSurrealDbClient>()
    dbClient.connect()

    // Create lifecycle for Decompose
    val lifecycle = LifecycleRegistry()

    // Create root component on Swing/main thread
    val rootComponent = RootComponent(
        componentContext = DefaultComponentContext(lifecycle = lifecycle)
    )

    application {
        Window(
            onCloseRequest = {
                // Cleanup database connection on exit
                runBlocking { dbClient.disconnect() }
                exitApplication()
            },
            title = "Altair",
        ) {
            App(rootComponent)
        }
    }
}
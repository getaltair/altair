package com.getaltair.altair

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import com.getaltair.altair.di.initKoin
import com.getaltair.altair.ui.ErrorScreen
import org.koin.core.error.KoinApplicationAlreadyStartedException

/**
 * Desktop application entry point.
 * Initializes Koin and displays the main application window with error handling.
 */
fun main() {
    var startupError: Throwable? = null

    try {
        initKoin()
    } catch (e: KoinApplicationAlreadyStartedException) {
        // Koin already started - this is recoverable, log and continue
        System.err.println("Koin already initialized: ${e.message}")
        // Don't set startupError - continue with existing Koin instance
    } catch (e: IllegalStateException) {
        System.err.println("Failed to initialize Koin: ${e.message}")
        startupError = e
    }

    application {
        Window(
            onCloseRequest = ::exitApplication,
            title = if (startupError != null) "Altair - Error" else "Altair",
        ) {
            if (startupError != null) {
                ErrorScreen(startupError)
            } else {
                App()
            }
        }
    }
}

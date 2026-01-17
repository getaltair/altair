package com.getaltair.altair

import androidx.compose.ui.window.ComposeUIViewController
import com.getaltair.altair.di.initKoin
import com.getaltair.altair.ui.ErrorScreen
import org.koin.core.error.KoinApplicationAlreadyStartedException

/**
 * iOS main view controller factory.
 * Initializes Koin and creates the Compose UI view controller.
 */
fun MainViewController() =
    ComposeUIViewController {
        App()
    }

/**
 * iOS Koin initialization helper.
 * Call this from Swift before creating the view controller.
 *
 * Example Swift usage:
 * ```swift
 * MainViewControllerKt.doInitKoin()
 * let controller = MainViewControllerKt.MainViewController()
 * ```
 *
 * @return null on success, or an error message on failure
 */
fun doInitKoin(): String? =
    try {
        initKoin()
        null
    } catch (e: KoinApplicationAlreadyStartedException) {
        // Koin already started - this is not an error, return null (success)
        println("iOS: Koin already initialized: ${e.message}")
        null
    } catch (e: IllegalStateException) {
        println("iOS: Failed to initialize Koin: ${e.message}")
        e.message ?: "Unknown initialization error"
    }

/**
 * Creates a view controller that displays an error screen.
 * Use this when [doInitKoin] returns an error.
 */
fun createErrorViewController(errorMessage: String) =
    ComposeUIViewController {
        ErrorScreen(RuntimeException(errorMessage))
    }

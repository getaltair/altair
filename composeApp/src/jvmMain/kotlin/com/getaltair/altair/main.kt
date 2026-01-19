package com.getaltair.altair

import androidx.compose.runtime.remember
import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import com.arkivanov.essenty.lifecycle.destroy
import com.getaltair.altair.db.DesktopMigrationRunner
import com.getaltair.altair.db.EmbeddedSurrealClient
import com.getaltair.altair.db.desktopDatabaseModule
import com.getaltair.altair.di.desktopAuthModule
import com.getaltair.altair.di.initKoin
import com.getaltair.altair.navigation.RootComponent
import com.getaltair.altair.navigation.RootContent
import com.getaltair.altair.navigation.createRootComponentContext
import com.getaltair.altair.navigation.getLifecycle
import com.getaltair.altair.rpc.httpClientModule
import com.getaltair.altair.service.auth.AuthManager
import com.getaltair.altair.ui.ErrorScreen
import kotlinx.coroutines.runBlocking
import org.koin.core.context.GlobalContext
import org.koin.core.error.KoinApplicationAlreadyStartedException

/**
 * Desktop application entry point.
 * Initializes Koin, database, and displays the main application window with error handling.
 */
fun main() {
    var startupError: Throwable? = null

    try {
        initKoin {
            modules(desktopDatabaseModule, desktopAuthModule, httpClientModule)
        }
    } catch (e: KoinApplicationAlreadyStartedException) {
        // Koin already started - this is recoverable, log and continue
        System.err.println("Koin already initialized: ${e.message}")
        // Don't set startupError - continue with existing Koin instance
    } catch (e: IllegalStateException) {
        System.err.println("Failed to initialize Koin: ${e.message}")
        startupError = e
    }

    // Initialize database connection and run migrations
    if (startupError == null) {
        try {
            val koin = GlobalContext.get()
            val dbClient = koin.get<EmbeddedSurrealClient>()
            val migrationRunner = koin.get<DesktopMigrationRunner>()

            runBlocking {
                val connectResult = dbClient.connect()
                connectResult.fold(
                    ifLeft = { error ->
                        throw IllegalStateException("Failed to connect to database: ${error.toUserMessage()}")
                    },
                    ifRight = {
                        migrationRunner.runMigrations().fold(
                            ifLeft = { error ->
                                throw IllegalStateException(
                                    "Failed to run database migrations: ${error.toUserMessage()}",
                                )
                            },
                            ifRight = { /* success */ },
                        )
                    },
                )
            }
        } catch (e: Exception) {
            System.err.println("Database initialization failed: ${e.message}")
            startupError = e
        }
    }

    application {
        // Create navigation component context on the Compose main thread
        val componentContext = remember { createRootComponentContext() }

        // Create the root component if startup was successful
        val rootComponent = remember(startupError) {
            if (startupError == null) {
                try {
                    val koin = GlobalContext.get()
                    val authManager = koin.get<AuthManager>()
                    RootComponent(
                        componentContext = componentContext,
                        authManager = authManager,
                    )
                } catch (e: Exception) {
                    System.err.println("Failed to create RootComponent: ${e.message}")
                    null
                }
            } else {
                null
            }
        }

        Window(
            onCloseRequest = {
                // Destroy the component lifecycle
                try {
                    componentContext.getLifecycle().destroy()
                } catch (e: Exception) {
                    System.err.println("Error destroying lifecycle: ${e.message}")
                }

                // Close database connection on shutdown
                try {
                    val koin = GlobalContext.getOrNull()
                    koin?.getOrNull<EmbeddedSurrealClient>()?.let { dbClient ->
                        runBlocking { dbClient.close() }
                    }
                } catch (e: Exception) {
                    System.err.println("Error closing database: ${e.message}")
                }
                exitApplication()
            },
            title = if (startupError != null) "Altair - Error" else "Altair",
        ) {
            if (startupError != null || rootComponent == null) {
                ErrorScreen(startupError ?: IllegalStateException("Failed to initialize app"))
            } else {
                RootContent(component = rootComponent)
            }
        }
    }
}

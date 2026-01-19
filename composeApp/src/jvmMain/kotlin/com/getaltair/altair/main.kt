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
 * Initializes Koin dependency injection.
 *
 * @return An error if initialization failed, or null on success.
 */
private fun initializeKoin(): Throwable? =
    try {
        initKoin {
            modules(desktopDatabaseModule, desktopAuthModule, httpClientModule)
        }
        null
    } catch (e: KoinApplicationAlreadyStartedException) {
        // Koin already started - this is recoverable, log and continue
        System.err.println("Koin already initialized: ${e.message}")
        null
    } catch (e: IllegalStateException) {
        System.err.println("Failed to initialize Koin: ${e.message}")
        e
    }

/**
 * Initializes the database connection and runs migrations.
 *
 * @return An error if initialization failed, or null on success.
 */
private fun initializeDatabase(): Throwable? =
    try {
        val koin = GlobalContext.get()
        val dbClient = koin.get<EmbeddedSurrealClient>()
        val migrationRunner = koin.get<DesktopMigrationRunner>()

        runBlocking {
            dbClient.connect().fold(
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
        null
    } catch (e: Exception) {
        System.err.println("Database initialization failed: ${e.message}")
        e
    }

/**
 * Performs cleanup operations when the application is closing.
 *
 * @param componentContext The component context to destroy.
 */
private fun performCleanup(componentContext: com.arkivanov.decompose.ComponentContext) {
    try {
        componentContext.getLifecycle().destroy()
    } catch (e: Exception) {
        System.err.println("Error destroying lifecycle: ${e.message}")
    }

    try {
        val koin = GlobalContext.getOrNull()
        koin?.getOrNull<EmbeddedSurrealClient>()?.let { dbClient ->
            runBlocking { dbClient.close() }
        }
    } catch (e: Exception) {
        System.err.println("Error closing database: ${e.message}")
    }
}

/**
 * Desktop application entry point.
 * Initializes Koin, database, and displays the main application window with error handling.
 */
fun main() {
    val startupError = initializeKoin() ?: initializeDatabase()

    application {
        val componentContext = remember { createRootComponentContext() }
        var componentError: Throwable? = null

        val rootComponent =
            remember(startupError) {
                if (startupError != null) return@remember null
                try {
                    val koin = GlobalContext.get()
                    val authManager = koin.get<AuthManager>()
                    RootComponent(
                        componentContext = componentContext,
                        authManager = authManager,
                    )
                } catch (e: Exception) {
                    System.err.println("Failed to create RootComponent: ${e.message}")
                    e.printStackTrace()
                    componentError = e
                    null
                }
            }

        val displayError = startupError ?: componentError

        Window(
            onCloseRequest = {
                performCleanup(componentContext)
                exitApplication()
            },
            title = if (displayError != null) "Altair - Error" else "Altair",
        ) {
            if (displayError != null || rootComponent == null) {
                ErrorScreen(displayError ?: IllegalStateException("Failed to initialize app"))
            } else {
                RootContent(component = rootComponent)
            }
        }
    }
}

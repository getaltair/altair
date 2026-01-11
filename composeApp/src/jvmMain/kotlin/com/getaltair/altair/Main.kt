package com.getaltair.altair

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.getaltair.altair.data.db.DatabaseVerification
import com.getaltair.altair.data.db.SurrealDbConfig
import com.getaltair.altair.data.db.SurrealDbConnection
import com.getaltair.altair.data.getAppDataDirectory
import com.getaltair.altair.di.sharedKoinModules
import com.getaltair.altair.navigation.DefaultRootComponent
import com.getaltair.altair.ui.RootContent
import kotlinx.coroutines.runBlocking
import org.koin.core.context.startKoin
import org.koin.core.context.stopKoin

fun main() {
    // Initialize Koin DI
    startKoin {
        modules(sharedKoinModules())
    }

    // Initialize database before starting the application
    val dbInitialized = initializeDatabase()

    if (!dbInitialized) {
        System.err.println("[Altair] Failed to initialize database. Exiting.")
        stopKoin()
        return
    }

    // Create Decompose lifecycle and root component
    val lifecycle = LifecycleRegistry()
    val rootComponent = DefaultRootComponent(
        componentContext = DefaultComponentContext(lifecycle = lifecycle),
    )

    application {
        Window(
            onCloseRequest = {
                // Gracefully close database connection and stop Koin on exit
                runBlocking {
                    SurrealDbConnection.disconnect()
                }
                stopKoin()
                exitApplication()
            },
            title = "Altair",
        ) {
            // Use RootContent with NavigationRail for desktop
            RootContent(
                component = rootComponent,
                useRail = true,
            )
        }
    }
}

/**
 * Initializes the SurrealDB database connection and performs startup verification.
 *
 * @return true if initialization succeeded, false otherwise
 */
private fun initializeDatabase(): Boolean = try {
    runBlocking {
        // Get platform-specific data directory
        val dataPath = getAppDataDirectory()
        println("[Altair] Data directory: $dataPath")

        // Create database configuration for embedded storage
        val config = SurrealDbConfig.embedded(dataPath)

        // Connect to database
        SurrealDbConnection.connect(config)

        // Perform startup verification
        val verificationResult = DatabaseVerification.performStartupVerification()
        println("[Altair] $verificationResult")

        verificationResult.isSuccess
    }
} catch (e: Exception) {
    System.err.println("[Altair] Database initialization failed: ${e.message}")
    e.printStackTrace()
    false
}

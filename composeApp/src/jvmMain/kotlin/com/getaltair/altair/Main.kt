package com.getaltair.altair

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import com.getaltair.altair.data.db.DatabaseVerification
import com.getaltair.altair.data.db.SurrealDbConfig
import com.getaltair.altair.data.db.SurrealDbConnection
import com.getaltair.altair.data.getAppDataDirectory
import kotlinx.coroutines.runBlocking

fun main() {
    // Initialize database before starting the application
    val dbInitialized = initializeDatabase()

    if (!dbInitialized) {
        System.err.println("[Altair] Failed to initialize database. Exiting.")
        return
    }

    application {
        Window(
            onCloseRequest = {
                // Gracefully close database connection on exit
                runBlocking {
                    SurrealDbConnection.disconnect()
                }
                exitApplication()
            },
            title = "Altair",
        ) {
            App()
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

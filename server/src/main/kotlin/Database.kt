package com.getaltair

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.databaseModule
import io.ktor.server.application.Application
import io.ktor.server.application.ApplicationStopped
import io.ktor.server.application.install
import kotlinx.coroutines.runBlocking
import org.koin.ktor.ext.inject
import org.koin.ktor.plugin.Koin
import org.koin.logger.slf4jLogger

/**
 * Configure database connectivity and dependency injection.
 *
 * This sets up:
 * - Koin dependency injection with the database module
 * - SurrealDB client connection
 * - Database migrations
 */
fun Application.configureDatabase() {
    // Install Koin for dependency injection
    install(Koin) {
        slf4jLogger()
        modules(databaseModule)
    }

    // Get database client from Koin and initialize
    val dbClient: SurrealDbClient by inject()
    val migrationRunner: MigrationRunner by inject()

    // Connect and run migrations on startup
    runBlocking {
        dbClient
            .connect()
            .onRight {
                // Run migrations
                migrationRunner
                    .runMigrations()
                    .onLeft { error ->
                        throw IllegalStateException("Failed to run database migrations: $error")
                    }
            }.onLeft { error ->
                throw IllegalStateException("Failed to connect to database: $error")
            }
    }

    // Register shutdown hook to close database connection
    monitor.subscribe(ApplicationStopped) {
        runBlocking {
            dbClient.close()
        }
    }
}

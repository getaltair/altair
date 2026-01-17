package com.getaltair.altair.db

import org.koin.dsl.module

/**
 * Koin module for desktop database dependencies.
 *
 * Provides:
 * - DesktopDatabaseConfig (from environment)
 * - EmbeddedSurrealClient (singleton)
 * - DesktopMigrationRunner (singleton)
 *
 * Note: Repository implementations for desktop are not yet available.
 * The desktop version will use the same repository interfaces as mobile/server
 * but with desktop-specific implementations that need to be created.
 * For now, the desktop will connect to the embedded database and run migrations,
 * but actual data operations will require the repository implementations.
 */
val desktopDatabaseModule =
    module {
        // Desktop database configuration from environment
        single<DesktopDatabaseConfig> { DesktopDatabaseConfig.fromEnvironment() }

        // Embedded SurrealDB client (singleton)
        single { EmbeddedSurrealClient(get()) }

        // Desktop migration runner
        single { DesktopMigrationRunner(get()) }

        // TODO: Add repository implementations for desktop
        // These should be created in composeApp/src/jvmMain and use EmbeddedSurrealClient directly.
        // For now, the application will start but data operations won't work until
        // repository implementations are added.
    }

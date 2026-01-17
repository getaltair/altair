package com.getaltair.altair.db

import java.nio.file.Path
import java.nio.file.Paths

/**
 * Configuration for the desktop embedded SurrealDB database.
 *
 * The desktop application uses SurrealDB in embedded mode with SurrealKV
 * as the storage engine, providing graph query capabilities and vector
 * search without requiring a separate server process.
 */
data class DesktopDatabaseConfig(
    /** Directory for database files */
    val dataDirectory: Path,
    /** Namespace for multi-tenancy */
    val namespace: String = "altair",
    /** Database name */
    val database: String = "desktop",
) {
    init {
        require(namespace.isNotBlank()) { "Namespace must not be blank" }
        require(database.isNotBlank()) { "Database must not be blank" }
    }
    companion object {
        /**
         * Creates a default desktop database configuration.
         *
         * Uses platform-specific data directories:
         * - Windows: %APPDATA%/Altair
         * - macOS: ~/Library/Application Support/Altair
         * - Linux: ~/.local/share/altair
         */
        fun default(): DesktopDatabaseConfig {
            val dataDir = getDefaultDataDirectory()
            return DesktopDatabaseConfig(dataDirectory = dataDir)
        }

        /**
         * Creates a configuration from environment variables.
         *
         * Supports the following environment variables:
         * - ALTAIR_DATA_DIR: Override the data directory
         * - ALTAIR_DB_NAMESPACE: Override the namespace (default: altair)
         * - ALTAIR_DB_DATABASE: Override the database name (default: desktop)
         */
        fun fromEnvironment(): DesktopDatabaseConfig {
            val dataDir =
                System.getenv("ALTAIR_DATA_DIR")?.let { Paths.get(it) }
                    ?: getDefaultDataDirectory()

            return DesktopDatabaseConfig(
                dataDirectory = dataDir,
                namespace = System.getenv("ALTAIR_DB_NAMESPACE") ?: "altair",
                database = System.getenv("ALTAIR_DB_DATABASE") ?: "desktop",
            )
        }

        private fun getDefaultDataDirectory(): Path {
            val os = System.getProperty("os.name").lowercase()
            val home = System.getProperty("user.home")

            return when {
                os.contains("win") -> {
                    val appData = System.getenv("APPDATA") ?: "$home/AppData/Roaming"
                    Paths.get(appData, "Altair")
                }
                os.contains("mac") -> {
                    Paths.get(home, "Library", "Application Support", "Altair")
                }
                else -> {
                    // Linux and others
                    val xdgData = System.getenv("XDG_DATA_HOME") ?: "$home/.local/share"
                    Paths.get(xdgData, "altair")
                }
            }
        }
    }

    /**
     * Gets the full path to the database files.
     */
    val databasePath: Path
        get() = dataDirectory.resolve("db")

    /**
     * Gets the connection URL for embedded SurrealDB.
     */
    val connectionUrl: String
        get() = "surrealkv://${databasePath.toAbsolutePath()}"
}

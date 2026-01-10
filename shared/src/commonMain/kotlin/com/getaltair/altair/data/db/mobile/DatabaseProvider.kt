package com.getaltair.altair.data.db.mobile

import app.cash.sqldelight.db.SqlDriver
import com.getaltair.altair.database.AltairDatabase

/**
 * Singleton provider for the AltairDatabase instance.
 *
 * This class manages the lifecycle of the database connection and ensures
 * only one instance of the database is created per application lifecycle.
 *
 * Usage:
 * ```kotlin
 * // Initialize once at app startup
 * DatabaseProvider.initialize(driverFactory)
 *
 * // Access database anywhere
 * val database = DatabaseProvider.database
 * ```
 */
object DatabaseProvider {
    private var driver: SqlDriver? = null
    private var _database: AltairDatabase? = null

    /**
     * The shared database instance.
     *
     * @throws IllegalStateException if database has not been initialized
     */
    val database: AltairDatabase
        get() = _database ?: throw IllegalStateException(
            "Database has not been initialized. Call initialize() first.",
        )

    /**
     * Checks if the database has been initialized.
     */
    val isInitialized: Boolean
        get() = _database != null

    /**
     * Initializes the database with the provided driver factory.
     *
     * This should be called once at application startup before any database operations.
     *
     * @param driverFactory Platform-specific driver factory
     * @return The initialized AltairDatabase instance
     */
    fun initialize(driverFactory: DriverFactory): AltairDatabase {
        if (_database != null) {
            return _database!!
        }

        driver = driverFactory.createDriver()
        _database = AltairDatabase(driver!!)
        return _database!!
    }

    /**
     * Initializes the database with a pre-created driver.
     *
     * Useful for testing scenarios where a custom driver is needed.
     *
     * @param sqlDriver Pre-configured SqlDriver instance
     * @return The initialized AltairDatabase instance
     */
    fun initialize(sqlDriver: SqlDriver): AltairDatabase {
        if (_database != null) {
            return _database!!
        }

        driver = sqlDriver
        _database = AltairDatabase(sqlDriver)
        return _database!!
    }

    /**
     * Closes the database connection and resets the singleton.
     *
     * Call this when the application is being destroyed or for testing cleanup.
     */
    fun close() {
        driver?.close()
        driver = null
        _database = null
    }

    /**
     * Resets the database provider for testing purposes.
     *
     * This clears the current instance without closing the driver,
     * allowing tests to reinitialize with different configurations.
     */
    internal fun reset() {
        driver = null
        _database = null
    }
}

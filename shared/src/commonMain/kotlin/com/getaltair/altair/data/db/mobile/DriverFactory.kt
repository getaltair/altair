package com.getaltair.altair.data.db.mobile

import app.cash.sqldelight.db.SqlDriver

/**
 * Platform-specific SQLite driver factory for mobile platforms (Android/iOS).
 *
 * This expect class must be implemented in androidMain and iosMain source sets
 * to provide platform-specific SQLite driver creation.
 *
 * Usage:
 * ```kotlin
 * val driverFactory = DriverFactory(context) // Android
 * val driverFactory = DriverFactory()        // iOS
 * val driver = driverFactory.createDriver()
 * ```
 */
expect class DriverFactory {
    /**
     * Creates a platform-specific SQLite driver.
     *
     * @return SqlDriver instance configured for the platform
     */
    fun createDriver(): SqlDriver
}

/**
 * Database name constant used across platforms.
 */
const val DATABASE_NAME = "altair.db"

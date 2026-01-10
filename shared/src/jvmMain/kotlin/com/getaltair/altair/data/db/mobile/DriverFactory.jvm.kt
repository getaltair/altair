package com.getaltair.altair.data.db.mobile

import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.getaltair.altair.database.AltairDatabase
import java.io.File

/**
 * JVM implementation of DriverFactory using JdbcSqliteDriver.
 *
 * This implementation is used for desktop/server JVM environments and testing.
 * For mobile platforms, see the Android and iOS implementations.
 *
 * @property dbPath Optional path to the database file. If null, creates in-memory database.
 */
actual class DriverFactory(private val dbPath: String? = null) {
    /**
     * Creates a JVM SQLite driver using JdbcSqliteDriver.
     *
     * If dbPath is provided, creates a file-based database.
     * If dbPath is null, creates an in-memory database (useful for testing).
     *
     * @return SqlDriver configured for JVM platform
     */
    actual fun createDriver(): SqlDriver {
        val url = if (dbPath != null) {
            // Ensure parent directory exists
            File(dbPath).parentFile?.mkdirs()
            "jdbc:sqlite:$dbPath"
        } else {
            JdbcSqliteDriver.IN_MEMORY
        }

        val driver = JdbcSqliteDriver(url)

        // Create schema if needed (for new databases)
        AltairDatabase.Schema.create(driver)

        return driver
    }
}

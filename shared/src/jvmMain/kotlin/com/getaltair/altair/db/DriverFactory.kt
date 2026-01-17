package com.getaltair.altair.db

import app.cash.sqldelight.async.coroutines.synchronous
import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver

/**
 * Factory for creating SQLDelight database drivers on JVM.
 *
 * Uses JdbcSqliteDriver for SQLite access on JVM platforms (testing, desktop).
 */
class DriverFactory(
    private val databasePath: String? = null,
) {
    /**
     * Creates a new SQLDriver for the Altair database.
     *
     * @return A configured SqlDriver for JVM
     */
    fun createDriver(): SqlDriver {
        val url =
            if (databasePath != null) {
                "jdbc:sqlite:$databasePath"
            } else {
                JdbcSqliteDriver.IN_MEMORY
            }
        val driver = JdbcSqliteDriver(url)
        AltairDatabase.Schema.synchronous().create(driver)
        return driver
    }

    companion object {
        private const val DATABASE_NAME = "altair.db"
    }
}

/**
 * Creates a new AltairDatabase instance for JVM.
 *
 * @param databasePath Optional path to the database file. If null, uses in-memory database.
 * @return A configured AltairDatabase instance
 */
fun createDatabase(databasePath: String? = null): AltairDatabase {
    val driver = DriverFactory(databasePath).createDriver()
    return AltairDatabase(driver)
}

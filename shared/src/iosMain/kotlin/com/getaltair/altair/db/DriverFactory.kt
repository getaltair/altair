package com.getaltair.altair.db

import app.cash.sqldelight.async.coroutines.synchronous
import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.native.NativeSqliteDriver

/**
 * Factory for creating SQLDelight database drivers on iOS.
 *
 * Uses NativeSqliteDriver for native SQLite access on iOS devices.
 */
class DriverFactory {
    /**
     * Creates a new SQLDriver for the Altair database.
     *
     * @return A configured SqlDriver for iOS
     */
    fun createDriver(): SqlDriver =
        NativeSqliteDriver(
            schema = AltairDatabase.Schema.synchronous(),
            name = DATABASE_NAME,
        )

    companion object {
        private const val DATABASE_NAME = "altair.db"
    }
}

/**
 * Creates a new AltairDatabase instance for iOS.
 *
 * @return A configured AltairDatabase instance
 */
fun createDatabase(): AltairDatabase {
    val driver = DriverFactory().createDriver()
    return AltairDatabase(driver)
}

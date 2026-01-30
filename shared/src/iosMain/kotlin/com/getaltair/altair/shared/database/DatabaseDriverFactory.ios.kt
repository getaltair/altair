package com.getaltair.altair.shared.database

import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.native.NativeSqliteDriver

/**
 * iOS implementation of DatabaseDriverFactory.
 *
 * Uses NativeSqliteDriver which works with iOS native SQLite.
 */
actual class DatabaseDriverFactory {
    actual fun createDriver(): SqlDriver {
        return NativeSqliteDriver(
            schema = AltairDatabase.Schema,
            name = "altair.db"
        )
    }
}

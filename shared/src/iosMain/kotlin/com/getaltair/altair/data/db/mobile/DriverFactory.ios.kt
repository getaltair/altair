package com.getaltair.altair.data.db.mobile

import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.native.NativeSqliteDriver
import com.getaltair.altair.database.AltairDatabase

/**
 * iOS implementation of DriverFactory using NativeSqliteDriver.
 */
actual class DriverFactory {
    /**
     * Creates an iOS SQLite driver using NativeSqliteDriver.
     *
     * @return SqlDriver configured for iOS platform
     */
    actual fun createDriver(): SqlDriver = NativeSqliteDriver(
        schema = AltairDatabase.Schema,
        name = DATABASE_NAME,
    )
}

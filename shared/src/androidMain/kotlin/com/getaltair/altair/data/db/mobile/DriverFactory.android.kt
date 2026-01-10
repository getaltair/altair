package com.getaltair.altair.data.db.mobile

import android.content.Context
import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.android.AndroidSqliteDriver
import com.getaltair.altair.database.AltairDatabase

/**
 * Android implementation of DriverFactory using AndroidSqliteDriver.
 *
 * @property context Android application context required for database creation
 */
actual class DriverFactory(private val context: Context) {
    /**
     * Creates an Android SQLite driver using AndroidSqliteDriver.
     *
     * @return SqlDriver configured for Android platform
     */
    actual fun createDriver(): SqlDriver = AndroidSqliteDriver(
        schema = AltairDatabase.Schema,
        context = context,
        name = DATABASE_NAME,
    )
}

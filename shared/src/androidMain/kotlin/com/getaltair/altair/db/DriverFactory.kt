package com.getaltair.altair.db

import android.content.Context
import app.cash.sqldelight.async.coroutines.synchronous
import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.android.AndroidSqliteDriver

/**
 * Factory for creating SQLDelight database drivers on Android.
 *
 * Uses AndroidSqliteDriver for native SQLite access on Android devices.
 */
class DriverFactory(
    private val context: Context,
) {
    /**
     * Creates a new SQLDriver for the Altair database.
     *
     * @return A configured SqlDriver for Android
     */
    fun createDriver(): SqlDriver =
        AndroidSqliteDriver(
            schema = AltairDatabase.Schema.synchronous(),
            context = context,
            name = DATABASE_NAME,
        )

    companion object {
        private const val DATABASE_NAME = "altair.db"
    }
}

/**
 * Creates a new AltairDatabase instance for Android.
 *
 * @param context The Android Context
 * @return A configured AltairDatabase instance
 */
fun createDatabase(context: Context): AltairDatabase {
    val driver = DriverFactory(context).createDriver()
    return AltairDatabase(driver)
}

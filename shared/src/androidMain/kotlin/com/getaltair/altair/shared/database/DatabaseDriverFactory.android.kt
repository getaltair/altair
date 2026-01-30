package com.getaltair.altair.shared.database

import android.content.Context
import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.android.AndroidSqliteDriver

/**
 * Android implementation of DatabaseDriverFactory.
 *
 * Uses AndroidSqliteDriver which requires an Android Context.
 *
 * @param context Android application or activity context
 */
actual class DatabaseDriverFactory(private val context: Context) {
    actual fun createDriver(): SqlDriver {
        return AndroidSqliteDriver(
            schema = AltairDatabase.Schema,
            context = context,
            name = "altair.db"
        )
    }
}

package com.getaltair.altair.shared.database

import app.cash.sqldelight.db.SqlDriver

/**
 * Platform-specific factory for creating SQLDelight database drivers.
 *
 * Implementations:
 * - Android: Uses AndroidSqliteDriver with Context
 * - iOS: Uses NativeSqliteDriver
 */
expect class DatabaseDriverFactory {
    fun createDriver(): SqlDriver
}

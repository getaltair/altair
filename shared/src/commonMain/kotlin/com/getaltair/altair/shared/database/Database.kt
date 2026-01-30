package com.getaltair.altair.shared.database

/**
 * Factory function to create an AltairDatabase instance.
 *
 * @param driverFactory Platform-specific driver factory
 * @return Configured AltairDatabase instance
 */
fun createDatabase(driverFactory: DatabaseDriverFactory): AltairDatabase {
    val driver = driverFactory.createDriver()
    return AltairDatabase(driver)
}

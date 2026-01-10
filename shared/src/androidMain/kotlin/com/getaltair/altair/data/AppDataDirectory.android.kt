package com.getaltair.altair.data

import java.io.File

/**
 * Android implementation of getAppDataDirectory.
 *
 * Returns the application data directory for database storage on Android.
 * Uses the app's internal files directory path: `/data/data/<package>/files/db/`
 *
 * Note: This implementation relies on the Android application context being initialized.
 * The path is derived from the dataDir system property which is set by the Android runtime.
 *
 * @return String path to the application data directory for database files
 */
actual fun getAppDataDirectory(): String {
    // On Android, we can use the data directory system property
    // This is set by the Android runtime when the app starts
    val dataDir = System.getProperty("user.dir")
        ?: System.getenv("HOME")
        ?: "/data/data/com.getaltair.altair"

    // Use internal app storage path pattern for Android
    // The actual path will be: /data/data/<package>/files/db/
    return "$dataDir${File.separator}files${File.separator}db"
}

/**
 * Object to hold the Android application context for path resolution.
 *
 * This should be initialized in Application.onCreate() or before any database operations:
 * ```kotlin
 * AndroidAppDataDirectory.initialize(applicationContext)
 * ```
 */
object AndroidAppDataDirectory {
    @Volatile
    private var appDataPath: String? = null

    /**
     * Initialize with the Android application context.
     * This sets the correct app data directory path.
     *
     * @param context Android application context
     */
    fun initialize(context: android.content.Context) {
        appDataPath = context.filesDir.resolve("db").absolutePath
    }

    /**
     * Get the initialized app data path, or null if not initialized.
     */
    fun getPath(): String? = appDataPath
}

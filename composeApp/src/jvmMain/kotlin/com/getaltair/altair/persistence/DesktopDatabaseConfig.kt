package com.getaltair.altair.persistence

/**
 * Configuration for embedded SurrealDB on desktop.
 * Uses file-based storage with platform-specific default paths.
 */
data class DesktopDatabaseConfig(
    val storagePath: String = getDefaultStoragePath(),
    val namespace: String = "altair",
    val database: String = "main"
) {
    /**
     * Connection URL for embedded SurrealDB.
     * Uses file:// protocol for persistent storage.
     */
    val connectionUrl: String get() = "file://$storagePath/altair.db"

    companion object {
        private fun getDefaultStoragePath(): String {
            val userHome = System.getProperty("user.home")
            val osName = System.getProperty("os.name").lowercase()
            return when {
                osName.contains("mac") || osName.contains("darwin") ->
                    "$userHome/Library/Application Support/Altair/data"
                osName.contains("win") ->
                    System.getenv("APPDATA")?.let { "$it/Altair/data" }
                        ?: "$userHome/AppData/Roaming/Altair/data"
                else -> // Linux and others
                    System.getenv("XDG_DATA_HOME")?.let { "$it/altair/data" }
                        ?: "$userHome/.local/share/altair/data"
            }
        }
    }
}

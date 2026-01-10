package com.getaltair.altair.data

import java.io.File

/**
 * JVM implementation of getAppDataDirectory.
 *
 * Returns the platform-appropriate application data directory for database storage:
 * - Linux: `$HOME/.local/share/altair/db/`
 * - macOS: `$HOME/Library/Application Support/altair/db/`
 * - Windows: `%APPDATA%/altair/db/`
 *
 * @return String path to the application data directory for database files
 */
actual fun getAppDataDirectory(): String {
    val osName = System.getProperty("os.name").lowercase()
    val userHome = System.getProperty("user.home")

    val basePath = when {
        osName.contains("linux") -> {
            "$userHome/.local/share"
        }

        osName.contains("mac") || osName.contains("darwin") -> {
            "$userHome/Library/Application Support"
        }

        osName.contains("windows") -> {
            System.getenv("APPDATA") ?: "$userHome${File.separator}AppData${File.separator}Roaming"
        }

        else -> {
            // Fallback to XDG Base Directory Specification
            System.getenv("XDG_DATA_HOME") ?: "$userHome/.local/share"
        }
    }

    return "$basePath${File.separator}altair${File.separator}db".replace("/", File.separator)
}

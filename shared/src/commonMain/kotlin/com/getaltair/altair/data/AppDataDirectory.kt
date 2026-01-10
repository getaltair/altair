package com.getaltair.altair.data

/**
 * Returns the platform-appropriate application data directory path for database storage.
 *
 * Platform-specific paths:
 * - Linux: `$HOME/.local/share/altair/db/`
 * - macOS: `$HOME/Library/Application Support/altair/db/`
 * - Windows: `%APPDATA%/altair/db/`
 *
 * @return String path to the application data directory for database files
 */
expect fun getAppDataDirectory(): String

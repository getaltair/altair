package com.getaltair.altair.data

import java.io.File
import java.nio.file.Paths
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Unit tests for AppDataDirectory platform-specific implementation.
 * Tests verify correct path resolution for each supported platform.
 */
class AppDataDirectoryTest {

    @Test
    fun `getAppDataDirectory returns non-empty path`() {
        val path = getAppDataDirectory()
        assertTrue(path.isNotEmpty(), "App data directory path should not be empty")
    }

    @Test
    fun `getAppDataDirectory returns path ending with altair db`() {
        val path = getAppDataDirectory()
        assertTrue(
            path.endsWith("altair${File.separator}db") || path.endsWith("altair/db"),
            "Path should end with 'altair/db' or 'altair${File.separator}db', got: $path"
        )
    }

    @Test
    fun `getAppDataDirectory returns absolute path`() {
        val path = getAppDataDirectory()
        val nioPath = Paths.get(path)
        assertTrue(nioPath.isAbsolute, "Path should be absolute: $path")
    }

    @Test
    fun `getAppDataDirectory returns platform-appropriate path`() {
        val path = getAppDataDirectory()
        val osName = System.getProperty("os.name").lowercase()

        when {
            osName.contains("linux") -> {
                val expectedPath = "${System.getProperty("user.home")}/.local/share/altair/db"
                assertEquals(expectedPath, path, "Linux path should be ~/.local/share/altair/db")
            }
            osName.contains("mac") || osName.contains("darwin") -> {
                val expectedPath = "${System.getProperty("user.home")}/Library/Application Support/altair/db"
                assertEquals(expectedPath, path, "macOS path should be ~/Library/Application Support/altair/db")
            }
            osName.contains("windows") -> {
                val appData = System.getenv("APPDATA")
                val expectedPath = "$appData${File.separator}altair${File.separator}db"
                assertEquals(expectedPath, path, "Windows path should be %APPDATA%/altair/db")
            }
            else -> {
                // For unknown OS, just verify the path structure
                assertTrue(
                    path.contains("altair") && path.contains("db"),
                    "Path should contain 'altair' and 'db'"
                )
            }
        }
    }

    @Test
    fun `getAppDataDirectory is consistent across multiple calls`() {
        val path1 = getAppDataDirectory()
        val path2 = getAppDataDirectory()
        assertEquals(path1, path2, "Multiple calls should return the same path")
    }
}

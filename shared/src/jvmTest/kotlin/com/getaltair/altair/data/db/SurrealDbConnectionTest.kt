package com.getaltair.altair.data.db

import kotlinx.coroutines.test.runTest
import java.io.File
import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Unit tests for SurrealDbConnection singleton.
 * Tests verify connection lifecycle management and thread-safety.
 */
class SurrealDbConnectionTest {

    private val testDataPath = "/tmp/altair-test-db-${System.currentTimeMillis()}"

    @BeforeTest
    fun setUp() {
        // Ensure clean state before each test
        File(testDataPath).deleteRecursively()
    }

    @AfterTest
    fun tearDown() = runTest {
        // Ensure connection is closed after each test
        SurrealDbConnection.disconnect()
        // Clean up test directory
        File(testDataPath).deleteRecursively()
    }

    @Test
    fun `isConnected returns false before connection`() {
        assertFalse(SurrealDbConnection.isConnected())
    }

    @Test
    fun `getCurrentConfig returns null before connection`() {
        assertNull(SurrealDbConnection.getCurrentConfig())
    }

    @Test
    fun `getDriver throws exception when not connected`() {
        assertFalse(SurrealDbConnection.isConnected())

        val exception = assertFailsWith<SurrealDbConnectionException> {
            SurrealDbConnection.getDriver()
        }

        assertTrue(exception.message!!.contains("Not connected"))
    }

    @Test
    fun `connect with memory config establishes connection`() = runTest {
        val config = SurrealDbConfig.memory()

        SurrealDbConnection.connect(config)

        assertTrue(SurrealDbConnection.isConnected())
        assertEquals(config, SurrealDbConnection.getCurrentConfig())
        assertNotNull(SurrealDbConnection.getDriver())
    }

    @Test
    fun `connect with embedded config establishes connection`() = runTest {
        val config = SurrealDbConfig.embedded(testDataPath)

        SurrealDbConnection.connect(config)

        assertTrue(SurrealDbConnection.isConnected())
        assertEquals(config, SurrealDbConnection.getCurrentConfig())
        assertNotNull(SurrealDbConnection.getDriver())
    }

    @Test
    fun `connect creates data directory if not exists`() = runTest {
        val config = SurrealDbConfig.embedded(testDataPath)
        assertFalse(File(testDataPath).exists())

        SurrealDbConnection.connect(config)

        assertTrue(File(testDataPath).exists())
    }

    @Test
    fun `connect is idempotent with same config`() = runTest {
        val config = SurrealDbConfig.memory()

        SurrealDbConnection.connect(config)
        val driver1 = SurrealDbConnection.getDriver()

        SurrealDbConnection.connect(config)
        val driver2 = SurrealDbConnection.getDriver()

        // Should be the same driver instance
        assertEquals(driver1, driver2)
        assertTrue(SurrealDbConnection.isConnected())
    }

    @Test
    fun `connect with different config reconnects`() = runTest {
        val config1 = SurrealDbConfig.memory()
        val config2 = SurrealDbConfig(
            namespace = "test",
            database = "test",
            dataPath = null,
        )

        SurrealDbConnection.connect(config1)
        assertEquals(config1, SurrealDbConnection.getCurrentConfig())

        SurrealDbConnection.connect(config2)
        assertEquals(config2, SurrealDbConnection.getCurrentConfig())
    }

    @Test
    fun `disconnect closes connection`() = runTest {
        val config = SurrealDbConfig.memory()
        SurrealDbConnection.connect(config)
        assertTrue(SurrealDbConnection.isConnected())

        SurrealDbConnection.disconnect()

        assertFalse(SurrealDbConnection.isConnected())
        assertNull(SurrealDbConnection.getCurrentConfig())
    }

    @Test
    fun `disconnect is safe to call multiple times`() = runTest {
        SurrealDbConnection.disconnect()
        SurrealDbConnection.disconnect()

        assertFalse(SurrealDbConnection.isConnected())
    }

    @Test
    fun `disconnect is safe to call when not connected`() = runTest {
        assertFalse(SurrealDbConnection.isConnected())

        SurrealDbConnection.disconnect()

        assertFalse(SurrealDbConnection.isConnected())
    }

    @Test
    fun `can reconnect after disconnect`() = runTest {
        val config = SurrealDbConfig.memory()

        SurrealDbConnection.connect(config)
        assertTrue(SurrealDbConnection.isConnected())

        SurrealDbConnection.disconnect()
        assertFalse(SurrealDbConnection.isConnected())

        SurrealDbConnection.connect(config)
        assertTrue(SurrealDbConnection.isConnected())
    }

    @Test
    fun `driver can execute simple query after connection`() = runTest {
        val config = SurrealDbConfig.memory()
        SurrealDbConnection.connect(config)

        val driver = SurrealDbConnection.getDriver()

        // Execute a simple info query to verify the driver works
        val result = driver.query("INFO FOR DB")
        assertNotNull(result)
    }
}

package com.getaltair.altair.data.db

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Unit tests for SurrealDbConfig.
 * Tests verify correct configuration creation and connection string generation.
 */
class SurrealDbConfigTest {

    @Test
    fun `default config uses default namespace and database`() {
        val config = SurrealDbConfig()

        assertEquals(SurrealDbConfig.DEFAULT_NAMESPACE, config.namespace)
        assertEquals(SurrealDbConfig.DEFAULT_DATABASE, config.database)
        assertNull(config.dataPath)
    }

    @Test
    fun `embedded factory creates config with data path`() {
        val path = "/tmp/test-db"
        val config = SurrealDbConfig.embedded(path)

        assertEquals(path, config.dataPath)
        assertEquals(SurrealDbConfig.DEFAULT_NAMESPACE, config.namespace)
        assertEquals(SurrealDbConfig.DEFAULT_DATABASE, config.database)
    }

    @Test
    fun `memory factory creates config without data path`() {
        val config = SurrealDbConfig.memory()

        assertNull(config.dataPath)
        assertEquals(SurrealDbConfig.DEFAULT_NAMESPACE, config.namespace)
        assertEquals(SurrealDbConfig.DEFAULT_DATABASE, config.database)
    }

    @Test
    fun `connectionString returns surrealkv protocol for embedded config`() {
        val path = "/data/altair/db"
        val config = SurrealDbConfig.embedded(path)

        assertEquals("surrealkv://$path", config.connectionString())
    }

    @Test
    fun `connectionString returns memory for in-memory config`() {
        val config = SurrealDbConfig.memory()

        assertEquals("memory", config.connectionString())
    }

    @Test
    fun `isPersistent returns true for embedded config`() {
        val config = SurrealDbConfig.embedded("/tmp/db")

        assertTrue(config.isPersistent)
    }

    @Test
    fun `isPersistent returns false for memory config`() {
        val config = SurrealDbConfig.memory()

        assertFalse(config.isPersistent)
    }

    @Test
    fun `custom namespace and database can be specified`() {
        val config = SurrealDbConfig(
            namespace = "custom_ns",
            database = "custom_db",
            dataPath = "/tmp/custom"
        )

        assertEquals("custom_ns", config.namespace)
        assertEquals("custom_db", config.database)
        assertEquals("/tmp/custom", config.dataPath)
    }

    @Test
    fun `default namespace constant is altair`() {
        assertEquals("altair", SurrealDbConfig.DEFAULT_NAMESPACE)
    }

    @Test
    fun `default database constant is main`() {
        assertEquals("main", SurrealDbConfig.DEFAULT_DATABASE)
    }
}

package com.getaltair.altair.db

import com.getaltair.altair.domain.DomainError
import kotlinx.coroutines.runBlocking
import java.nio.file.Files
import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertIs
import kotlin.test.assertTrue

/**
 * Tests for EmbeddedSurrealClient.
 *
 * These tests verify the basic lifecycle and error handling of the embedded database client.
 */
class EmbeddedSurrealClientTest {
    private lateinit var tempDir: java.nio.file.Path
    private lateinit var config: DesktopDatabaseConfig
    private lateinit var client: EmbeddedSurrealClient

    @BeforeTest
    fun setup() {
        tempDir = Files.createTempDirectory("embedded-surreal-test")
        config =
            DesktopDatabaseConfig(
                dataDirectory = tempDir,
                namespace = "test",
                database = "test_db",
            )
        client = EmbeddedSurrealClient(config)
    }

    @AfterTest
    fun tearDown() {
        runBlocking {
            client.close()
        }
        // Clean up temp directory
        tempDir.toFile().deleteRecursively()
    }

    @Test
    fun `client starts in disconnected state`() {
        runBlocking {
            assertFalse(client.isConnected())
        }
    }

    @Test
    fun `close on disconnected client does not throw`() {
        runBlocking {
            // Should not throw even when called on disconnected client
            client.close()
            assertFalse(client.isConnected())
        }
    }

    @Test
    fun `query returns error when not connected`() {
        runBlocking {
            val result = client.query<Any>("SELECT * FROM test")

            assertTrue(result.isLeft())
            result.onLeft { error ->
                val unexpectedError = assertIs<DomainError.UnexpectedError>(error)
                assertTrue(
                    unexpectedError.message.contains("Not connected"),
                    "Expected error message to contain 'Not connected', but was: ${unexpectedError.message}",
                )
            }
        }
    }

    @Test
    fun `execute returns error when not connected`() {
        runBlocking {
            val result = client.execute("CREATE test SET name = 'value'")

            assertTrue(result.isLeft())
            result.onLeft { error ->
                val unexpectedError = assertIs<DomainError.UnexpectedError>(error)
                assertTrue(
                    unexpectedError.message.contains("Not connected"),
                    "Expected error message to contain 'Not connected', but was: ${unexpectedError.message}",
                )
            }
        }
    }

    @Test
    fun `config creates proper connection URL`() {
        // databasePath is dataDirectory/db
        val expectedPath = tempDir.resolve("db").toAbsolutePath()
        val expectedUrl = "surrealkv://$expectedPath"
        assertEquals(expectedUrl, config.connectionUrl)
    }

    @Test
    fun `config creates proper paths`() {
        assertEquals(tempDir, config.dataDirectory)
        assertEquals(tempDir.resolve("db"), config.databasePath)
    }
}

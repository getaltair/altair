package com.getaltair.altair.db

import com.getaltair.altair.domain.DomainError
import kotlinx.coroutines.runBlocking
import java.nio.file.Files
import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertIs
import kotlin.test.assertTrue

/**
 * Tests for DesktopMigrationRunner.
 *
 * These tests verify the migration runner behavior, particularly error handling
 * when the database is not connected.
 */
class DesktopMigrationRunnerTest {
    private lateinit var tempDir: java.nio.file.Path
    private lateinit var config: DesktopDatabaseConfig
    private lateinit var client: EmbeddedSurrealClient
    private lateinit var migrationRunner: DesktopMigrationRunner

    @BeforeTest
    fun setup() {
        tempDir = Files.createTempDirectory("migration-runner-test")
        config =
            DesktopDatabaseConfig(
                dataDirectory = tempDir,
                namespace = "test",
                database = "test_db",
            )
        client = EmbeddedSurrealClient(config)
        migrationRunner = DesktopMigrationRunner(client)
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
    fun `runMigrations returns error when database is not connected`() {
        runBlocking {
            val result = migrationRunner.runMigrations()

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
}

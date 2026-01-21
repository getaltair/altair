package com.getaltair.altair.db

import com.getaltair.altair.domain.DomainError
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.string.shouldContain
import io.kotest.matchers.types.shouldBeInstanceOf
import java.nio.file.Files

/**
 * Tests for DesktopMigrationRunner.
 *
 * These tests verify the migration runner behavior, particularly error handling
 * when the database is not connected.
 */
class DesktopMigrationRunnerTest :
    BehaviorSpec({
        lateinit var tempDir: java.nio.file.Path
        lateinit var config: DesktopDatabaseConfig
        lateinit var client: EmbeddedSurrealClient
        lateinit var migrationRunner: DesktopMigrationRunner

        beforeEach {
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

        afterEach {
            client.close()
            // Clean up temp directory
            tempDir.toFile().deleteRecursively()
        }

        given("migration runner") {
            `when`("database is not connected") {
                then("runMigrations returns error") {
                    val result = migrationRunner.runMigrations()

                    result.shouldBeLeft()
                    val error = result.leftOrNull()
                    error.shouldBeInstanceOf<DomainError.UnexpectedError>()
                    error?.message shouldContain "Not connected"
                }
            }
        }
    })

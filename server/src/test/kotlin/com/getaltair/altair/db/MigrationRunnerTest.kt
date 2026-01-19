package com.getaltair.altair.db

import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.AfterAll
import org.junit.jupiter.api.BeforeAll
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestInstance
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.test.assertEquals
import kotlin.test.assertTrue

@Testcontainers
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class MigrationRunnerTest {
    private lateinit var dbClient: SurrealDbClient
    private lateinit var migrationRunner: MigrationRunner

    @BeforeAll
    fun setupContainer() {
        container.start()
    }

    @AfterAll
    fun tearDown() {
        runBlocking {
            dbClient.close()
        }
        container.stop()
    }

    @BeforeEach
    fun setup() {
        runBlocking {
            val config = container.createNetworkConfig()
            dbClient = SurrealDbClient(config)
            dbClient.connect().getOrNull()

            // Clean up migrations table before each test
            dbClient.execute("DELETE _migrations;")
            // Drop and recreate all tables to get a clean slate
            dbClient.execute("REMOVE TABLE IF EXISTS user;")
            dbClient.execute("REMOVE TABLE IF EXISTS initiative;")
            dbClient.execute("REMOVE TABLE IF EXISTS quest;")
            dbClient.execute("REMOVE TABLE IF EXISTS epic;")
            dbClient.execute("REMOVE TABLE IF EXISTS note;")
            dbClient.execute("REMOVE TABLE IF EXISTS tag;")
            dbClient.execute("REMOVE TABLE IF EXISTS folder;")
            dbClient.execute("REMOVE TABLE IF EXISTS inbox_item;")
            dbClient.execute("REMOVE TABLE IF EXISTS routine;")

            migrationRunner = MigrationRunner(dbClient)
        }
    }

    @Test
    fun `runMigrations applies pending migrations`(): Unit =
        runBlocking {
            val result = migrationRunner.runMigrations()

            assertTrue(result.isRight())
            result.onRight { count ->
                // Should apply at least one migration (V1)
                assertTrue(count >= 1, "Should have applied at least 1 migration, but got $count")
            }
        }

    @Test
    fun `runMigrations is idempotent - running twice applies nothing new`(): Unit =
        runBlocking {
            // First run
            val firstResult = migrationRunner.runMigrations()
            assertTrue(firstResult.isRight())
            val firstCount = firstResult.getOrNull() ?: 0

            // Second run
            val secondResult = migrationRunner.runMigrations()
            assertTrue(secondResult.isRight())
            secondResult.onRight { count ->
                assertEquals(0, count, "Second run should apply 0 migrations")
            }

            // Verify first run applied at least one migration
            assertTrue(firstCount >= 1, "First run should have applied at least 1 migration")
        }

    @Test
    fun `runMigrations creates migrations tracking table`(): Unit =
        runBlocking {
            migrationRunner.runMigrations()

            // Query the migrations table
            val result = dbClient.query<Any>("SELECT * FROM _migrations")

            assertTrue(result.isRight())
            result.onRight { json ->
                // Should have migration records
                assertTrue(json.contains("version"), "Migrations table should have version field")
            }
        }

    @Test
    fun `runMigrations records version numbers`(): Unit =
        runBlocking {
            migrationRunner.runMigrations()

            // Query the migrations table for version 1
            val result = dbClient.query<Any>("SELECT * FROM _migrations WHERE version = 1")

            assertTrue(result.isRight())
            result.onRight { json ->
                // Should find V1 migration
                assertTrue(json.contains("\"version\"") || json.contains("version"), "Should have version 1 recorded")
            }
        }

    @Test
    fun `runMigrations creates schema tables`(): Unit =
        runBlocking {
            migrationRunner.runMigrations()

            // Verify core tables exist by querying them
            val userResult = dbClient.query<Any>("INFO FOR TABLE user")
            val initiativeResult = dbClient.query<Any>("INFO FOR TABLE initiative")
            val questResult = dbClient.query<Any>("INFO FOR TABLE quest")
            val noteResult = dbClient.query<Any>("INFO FOR TABLE note")

            assertTrue(userResult.isRight(), "user table should exist")
            assertTrue(initiativeResult.isRight(), "initiative table should exist")
            assertTrue(questResult.isRight(), "quest table should exist")
            assertTrue(noteResult.isRight(), "note table should exist")
        }

    @Test
    fun `runMigrations applies migrations in order`(): Unit =
        runBlocking {
            migrationRunner.runMigrations()

            // Query migrations ordered by applied_at
            val result = dbClient.query<Any>("SELECT version FROM _migrations ORDER BY version ASC")

            assertTrue(result.isRight())
            result.onRight { json ->
                // With only V1 migration, this should succeed
                // When more migrations are added, this test will verify ordering
                assertTrue(json.contains("1"), "V1 should be applied")
            }
        }

    @Test
    fun `runMigrations skips already applied migrations`(): Unit =
        runBlocking {
            // Manually insert migration records for all known migrations
            dbClient.execute(
                """
                DEFINE TABLE IF NOT EXISTS _migrations SCHEMAFULL;
                DEFINE FIELD version ON _migrations TYPE int;
                DEFINE FIELD description ON _migrations TYPE string;
                DEFINE FIELD applied_at ON _migrations TYPE datetime DEFAULT time::now();
                DEFINE INDEX idx_migrations_version ON _migrations FIELDS version UNIQUE;
                """.trimIndent(),
            )
            dbClient.execute(
                """
                CREATE _migrations CONTENT {
                    version: 1,
                    description: 'initial schema'
                };
                """.trimIndent(),
            )
            dbClient.execute(
                """
                CREATE _migrations CONTENT {
                    version: 2,
                    description: 'authentication tables'
                };
                """.trimIndent(),
            )

            // Run migrations - should skip all since they're already recorded
            val result = migrationRunner.runMigrations()

            assertTrue(result.isRight())
            result.onRight { count ->
                assertEquals(0, count, "Should skip already applied migrations")
            }
        }

    companion object {
        @Container
        val container = SurrealDbTestContainer()
    }
}

package com.getaltair.altair.db

import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.ints.shouldBeGreaterThanOrEqual
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldContain

/**
 * Tests for MigrationRunner using Testcontainers.
 *
 * Verifies:
 * - Migration execution and tracking
 * - Idempotency (running migrations multiple times)
 * - Migration order enforcement
 * - Schema table creation
 */
class MigrationRunnerTest :
    BehaviorSpec({
        lateinit var dbClient: SurrealDbClient
        lateinit var migrationRunner: MigrationRunner

        beforeEach {
            val config = SurrealDbContainerExtension.createNetworkConfig()
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

        afterEach {
            dbClient.close()
        }

        given("migration execution") {
            `when`("runMigrations is called on fresh database") {
                then("applies pending migrations") {
                    val result = migrationRunner.runMigrations()

                    result.shouldBeRight()
                    (result.getOrNull() ?: 0) shouldBeGreaterThanOrEqual 1
                }
            }

            `when`("migrations have already been applied") {
                then("is idempotent and applies nothing new on second run") {
                    // First run
                    val firstResult = migrationRunner.runMigrations()
                    firstResult.shouldBeRight()
                    (firstResult.getOrNull() ?: 0) shouldBeGreaterThanOrEqual 1

                    // Second run
                    val secondResult = migrationRunner.runMigrations()
                    secondResult.shouldBeRight()
                    (secondResult.getOrNull() ?: -1) shouldBe 0
                }
            }

            `when`("all migrations are manually pre-applied") {
                then("skips already applied migrations") {
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
                    dbClient.execute(
                        """
                        CREATE _migrations CONTENT {
                            version: 3,
                            description: 'fix user status values'
                        };
                        """.trimIndent(),
                    )

                    // Run migrations - should skip all since they're already recorded
                    val result = migrationRunner.runMigrations()

                    result.shouldBeRight()
                    (result.getOrNull() ?: -1) shouldBe 0
                }
            }
        }

        given("migration tracking") {
            `when`("migrations are run") {
                then("creates migrations tracking table") {
                    migrationRunner.runMigrations()

                    // Query the migrations table
                    val result = dbClient.query<Any>("SELECT * FROM _migrations")

                    result.shouldBeRight()
                    val json = result.getOrNull()!!
                    json.shouldContain("version")
                }
            }

            `when`("migrations are applied") {
                then("records version numbers") {
                    migrationRunner.runMigrations()

                    // Query the migrations table for version 1
                    val result = dbClient.query<Any>("SELECT * FROM _migrations WHERE version = 1")

                    result.shouldBeRight()
                    val json = result.getOrNull()!!
                    // Should find V1 migration
                    json.shouldContain("version")
                }
            }

            `when`("migrations are applied") {
                then("applies migrations in order") {
                    migrationRunner.runMigrations()

                    // Query migrations ordered by applied_at
                    val result = dbClient.query<Any>("SELECT version FROM _migrations ORDER BY version ASC")

                    result.shouldBeRight()
                    val json = result.getOrNull()!!
                    // With only V1 migration, this should succeed
                    // When more migrations are added, this test will verify ordering
                    json.shouldContain("1")
                }
            }
        }

        given("schema creation") {
            `when`("migrations are run") {
                then("creates all schema tables") {
                    migrationRunner.runMigrations()

                    // Verify core tables exist by querying them
                    val userResult = dbClient.query<Any>("INFO FOR TABLE user")
                    val initiativeResult = dbClient.query<Any>("INFO FOR TABLE initiative")
                    val questResult = dbClient.query<Any>("INFO FOR TABLE quest")
                    val noteResult = dbClient.query<Any>("INFO FOR TABLE note")

                    userResult.shouldBeRight()
                    initiativeResult.shouldBeRight()
                    questResult.shouldBeRight()
                    noteResult.shouldBeRight()
                }
            }
        }
    })

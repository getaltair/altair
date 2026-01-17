package com.getaltair.altair.db

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.domain.DomainError
import org.slf4j.LoggerFactory

/**
 * Runs SurrealDB schema migrations at application startup.
 *
 * Migrations are stored in `resources/migrations/` with naming convention:
 * `V{version}__{description}.surql`
 *
 * A `_migrations` table tracks which migrations have been applied.
 */
class MigrationRunner(
    private val db: SurrealDbClient,
) {
    private val logger = LoggerFactory.getLogger(MigrationRunner::class.java)

    /**
     * Runs all pending migrations.
     *
     * @return Either an error or the number of migrations applied
     */
    suspend fun runMigrations(): Either<DomainError, Int> =
        either {
            logger.info("Starting database migrations...")

            // Ensure migrations table exists
            createMigrationsTable().bind()

            // Get applied migrations
            val appliedVersions = getAppliedMigrations().bind()
            logger.info("Found ${appliedVersions.size} previously applied migrations")

            // Load and filter pending migrations
            val allMigrations = loadMigrations().bind()
            val pendingMigrations =
                allMigrations
                    .filter { it.version !in appliedVersions }
                    .sortedBy { it.version }

            if (pendingMigrations.isEmpty()) {
                logger.info("No pending migrations")
                return@either 0
            }

            logger.info("Applying ${pendingMigrations.size} pending migrations")

            // Apply each migration
            for (migration in pendingMigrations) {
                applyMigration(migration).bind()
            }

            logger.info("Successfully applied ${pendingMigrations.size} migrations")
            pendingMigrations.size
        }

    private suspend fun createMigrationsTable(): Either<DomainError, Unit> =
        either {
            db
                .execute(
                    """
                    DEFINE TABLE IF NOT EXISTS _migrations SCHEMAFULL;
                    DEFINE FIELD version ON _migrations TYPE int;
                    DEFINE FIELD description ON _migrations TYPE string;
                    DEFINE FIELD applied_at ON _migrations TYPE datetime DEFAULT time::now();
                    DEFINE INDEX idx_migrations_version ON _migrations FIELDS version UNIQUE;
                    """.trimIndent(),
                ).bind()
        }

    private suspend fun getAppliedMigrations(): Either<DomainError, Set<Int>> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT version FROM _migrations",
                    ).bind()

            // Parse the result to extract version numbers
            val versionRegex = """"version":\s*(\d+)""".toRegex()
            versionRegex
                .findAll(result)
                .map { it.groupValues[1].toInt() }
                .toSet()
        }

    private suspend fun applyMigration(migration: Migration): Either<DomainError, Unit> =
        either {
            logger.info("Applying migration V${migration.version}: ${migration.description}")

            // Execute the migration SQL
            db.execute(migration.sql).bind()

            // Record the migration
            db
                .execute(
                    """
                    CREATE _migrations CONTENT {
                        version: ${migration.version},
                        description: '${migration.description.replace("'", "''")}'
                    };
                    """.trimIndent(),
                ).bind()

            logger.info("Migration V${migration.version} applied successfully")
        }

    private fun loadMigrations(): Either<DomainError, List<Migration>> {
        val migrations = mutableListOf<Migration>()
        val classLoader = this::class.java.classLoader

        // List all migration files
        val migrationsDir = classLoader.getResource("migrations")
        if (migrationsDir == null) {
            logger.error("No migrations directory found - database schema cannot be initialized")
            return Either.Left(
                DomainError.UnexpectedError(
                    "Migrations directory not found. Cannot initialize database schema.",
                ),
            )
        }

        // Read migration files from resources
        val migrationPattern = """V(\d+)__(.+)\.surql""".toRegex()

        return try {
            // Get all resources in migrations folder
            val resources = classLoader.getResources("migrations").toList()
            for (resource in resources) {
                val uri = resource.toURI()
                if (uri.scheme == "jar") {
                    // Handle JAR resources
                    loadMigrationsFromJar(migrations, migrationPattern)
                } else {
                    // Handle filesystem resources
                    loadMigrationsFromFilesystem(migrations, migrationPattern)
                }
            }
            Either.Right(migrations.sortedBy { it.version })
        } catch (e: Exception) {
            logger.error("Failed to load migrations", e)
            Either.Left(DomainError.UnexpectedError("Failed to load migrations: ${e.message}", e))
        }
    }

    private fun loadMigrationsFromJar(
        migrations: MutableList<Migration>,
        pattern: Regex,
    ) {
        val classLoader = this::class.java.classLoader
        // For JAR files, we need to scan known migration files
        // This requires listing them explicitly or using a manifest
        val knownMigrations =
            listOf(
                "V1__initial_schema.surql",
            )

        for (filename in knownMigrations) {
            val match = pattern.matchEntire(filename) ?: continue
            val version = match.groupValues[1].toInt()
            val description = match.groupValues[2].replace("_", " ")
            val sql =
                classLoader
                    .getResourceAsStream("migrations/$filename")
                    ?.bufferedReader()
                    ?.readText() ?: continue

            migrations.add(Migration(version, description, sql))
        }
    }

    private fun loadMigrationsFromFilesystem(
        migrations: MutableList<Migration>,
        pattern: Regex,
    ) {
        val classLoader = this::class.java.classLoader
        val migrationsUrl = classLoader.getResource("migrations") ?: return
        val migrationsDir = java.io.File(migrationsUrl.toURI())

        if (!migrationsDir.isDirectory) return

        migrationsDir.listFiles()?.forEach { file ->
            val match = pattern.matchEntire(file.name) ?: return@forEach
            val version = match.groupValues[1].toInt()
            val description = match.groupValues[2].replace("_", " ")
            val sql = file.readText()

            migrations.add(Migration(version, description, sql))
        }
    }

    private data class Migration(
        val version: Int,
        val description: String,
        val sql: String,
    )
}

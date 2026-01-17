package com.getaltair.altair.db

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.domain.DomainError
import org.slf4j.LoggerFactory

/**
 * Runs SurrealDB schema migrations for desktop embedded database.
 *
 * Migrations are stored in `resources/migrations/` with naming convention:
 * `V{version}__{description}.surql`
 *
 * A `_migrations` table tracks which migrations have been applied.
 */
class DesktopMigrationRunner(
    private val db: EmbeddedSurrealClient,
) {
    private val logger = LoggerFactory.getLogger(DesktopMigrationRunner::class.java)

    /**
     * Runs all pending migrations.
     *
     * @return Either an error or the number of migrations applied
     */
    suspend fun runMigrations(): Either<DomainError, Int> =
        either {
            logger.info("Starting desktop database migrations...")

            createMigrationsTable().bind()
            val appliedVersions = getAppliedMigrations().bind()

            logger.info("Found ${appliedVersions.size} previously applied migrations")
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
            applyMigrations(pendingMigrations).bind()
        }

    private suspend fun applyMigrations(migrations: List<Migration>): Either<DomainError, Int> =
        either {
            for (migration in migrations) {
                applyMigration(migration).bind()
            }
            logger.info("Successfully applied ${migrations.size} migrations")
            migrations.size
        }

    private suspend fun createMigrationsTable(): Either<DomainError, Unit> =
        db.execute(
            """
            DEFINE TABLE IF NOT EXISTS _migrations SCHEMAFULL;
            DEFINE FIELD version ON _migrations TYPE int;
            DEFINE FIELD description ON _migrations TYPE string;
            DEFINE FIELD applied_at ON _migrations TYPE datetime DEFAULT time::now();
            DEFINE INDEX idx_migrations_version ON _migrations FIELDS version UNIQUE;
            """.trimIndent(),
        )

    private suspend fun getAppliedMigrations(): Either<DomainError, Set<Int>> =
        db
            .query<Any>(
                "SELECT version FROM _migrations",
            ).map { result ->
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
            db.execute(migration.sql).bind()
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

        val migrationsDir = classLoader.getResource("migrations")
        if (migrationsDir == null) {
            logger.error("No migrations directory found - database schema cannot be initialized")
            return Either.Left(
                DomainError.UnexpectedError(
                    "Migrations directory not found. Cannot initialize database schema.",
                ),
            )
        }

        val migrationPattern = """V(\d+)__(.+)\.surql""".toRegex()

        return try {
            val resources = classLoader.getResources("migrations").toList()
            for (resource in resources) {
                val uri = resource.toURI()
                if (uri.scheme == "jar") {
                    loadMigrationsFromJar(migrations, migrationPattern)
                } else {
                    loadMigrationsFromFilesystem(migrations, migrationPattern)
                }
            }
            Either.Right(migrations.sortedBy { it.version })
        } catch (e: Exception) {
            logger.error("Failed to load migrations from resources", e)
            Either.Left(DomainError.UnexpectedError("Failed to load migrations: ${e.message}", e))
        }
    }

    private fun loadMigrationsFromJar(
        migrations: MutableList<Migration>,
        pattern: Regex,
    ) {
        val classLoader = this::class.java.classLoader
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

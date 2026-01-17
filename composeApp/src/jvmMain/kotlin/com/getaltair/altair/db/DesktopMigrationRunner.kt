package com.getaltair.altair.db

import arrow.core.Either
import arrow.core.flatMap
import arrow.core.right

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
    /**
     * Runs all pending migrations.
     *
     * @return Either an error or the number of migrations applied
     */
    suspend fun runMigrations(): Either<Throwable, Int> {
        // Ensure migrations table exists
        return createMigrationsTable()
            .flatMap {
                // Get applied migrations
                getAppliedMigrations()
            }.flatMap { appliedVersions ->
                // Load and filter pending migrations
                val allMigrations = loadMigrations()
                val pendingMigrations =
                    allMigrations
                        .filter { it.version !in appliedVersions }
                        .sortedBy { it.version }

                if (pendingMigrations.isEmpty()) {
                    return@flatMap 0.right()
                }

                // Apply each migration
                applyMigrations(pendingMigrations)
            }
    }

    private suspend fun applyMigrations(migrations: List<Migration>): Either<Throwable, Int> {
        var result: Either<Throwable, Unit> = Unit.right()
        for (migration in migrations) {
            result = result.flatMap { applyMigration(migration) }
            if (result.isLeft()) break
        }
        return result.map { migrations.size }
    }

    private suspend fun createMigrationsTable(): Either<Throwable, Unit> =
        db
            .execute(
                """
                DEFINE TABLE IF NOT EXISTS _migrations SCHEMAFULL;
                DEFINE FIELD version ON _migrations TYPE int;
                DEFINE FIELD description ON _migrations TYPE string;
                DEFINE FIELD applied_at ON _migrations TYPE datetime DEFAULT time::now();
                DEFINE INDEX idx_migrations_version ON _migrations FIELDS version UNIQUE;
                """.trimIndent(),
            ).mapLeft { RuntimeException(it.toUserMessage()) }

    private suspend fun getAppliedMigrations(): Either<Throwable, Set<Int>> =
        db
            .query<Any>(
                "SELECT version FROM _migrations",
            ).mapLeft { RuntimeException(it.toUserMessage()) }
            .map { result ->
                // Parse the result to extract version numbers
                val versionRegex = """"version":\s*(\d+)""".toRegex()
                versionRegex
                    .findAll(result)
                    .map { it.groupValues[1].toInt() }
                    .toSet()
            }

    private suspend fun applyMigration(migration: Migration): Either<Throwable, Unit> {
        // Execute the migration SQL
        return db
            .execute(migration.sql)
            .mapLeft { RuntimeException(it.toUserMessage()) }
            .flatMap {
                // Record the migration
                db
                    .execute(
                        """
                        CREATE _migrations CONTENT {
                            version: ${migration.version},
                            description: '${migration.description.replace("'", "''")}'
                        };
                        """.trimIndent(),
                    ).mapLeft { RuntimeException(it.toUserMessage()) }
            }
    }

    private fun loadMigrations(): List<Migration> {
        val migrations = mutableListOf<Migration>()
        val classLoader = this::class.java.classLoader

        // List all migration files
        val migrationsDir = classLoader.getResource("migrations")
        if (migrationsDir == null) {
            return emptyList()
        }

        // Read migration files from resources
        val migrationPattern = """V(\d+)__(.+)\.surql""".toRegex()

        try {
            // Get all resources in migrations folder
            val resources = classLoader.getResources("migrations").toList()
            for (resource in resources) {
                val uri = resource.toURI()
                if (uri.scheme == "jar") {
                    loadMigrationsFromJar(migrations, migrationPattern)
                } else {
                    loadMigrationsFromFilesystem(migrations, migrationPattern)
                }
            }
        } catch (e: Exception) {
            // Silently ignore - no migrations to run
        }

        return migrations.sortedBy { it.version }
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

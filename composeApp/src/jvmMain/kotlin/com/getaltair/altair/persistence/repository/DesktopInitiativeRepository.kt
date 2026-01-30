package com.getaltair.altair.persistence.repository

import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.persistence.DesktopSurrealDbClient
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.system.Initiative
import com.getaltair.altair.shared.repository.InitiativeRepository
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate

/**
 * Desktop implementation of the Initiative repository for single-user local database.
 *
 * Initiatives are cross-module organizational containers (Projects and Areas).
 * This repository enforces focus uniqueness (only one focused initiative) and
 * provides hierarchical query capabilities.
 *
 * Unlike the server version, this does not require AuthContext as it operates
 * in a single-user environment.
 *
 * @param db The desktop SurrealDB client for database operations
 */
class DesktopInitiativeRepository(
    private val db: DesktopSurrealDbClient
) : InitiativeRepository {

    private fun now(): Instant = Instant.fromEpochMilliseconds(System.currentTimeMillis())

    private fun notFoundError(id: Ulid): AltairError =
        AltairError.NotFoundError.InitiativeNotFound(id.toString())

    override suspend fun getById(id: Ulid): AltairResult<Initiative> {
        val sql = """
            SELECT * FROM initiative:${'$'}id WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(sql, mapOf("id" to id.toString()), Initiative::class)
            .flatMap { result ->
                result?.right() ?: notFoundError(id).left()
            }
    }

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Initiative>> {
        val sql = """
            SELECT * FROM initiative WHERE deleted_at IS NONE ORDER BY created_at DESC
        """.trimIndent()

        return db.query(sql, emptyMap(), Initiative::class)
    }

    override suspend fun getChildren(parentId: Ulid): AltairResult<List<Initiative>> {
        val sql = """
            SELECT * FROM initiative WHERE parent_id = ${'$'}parentId AND deleted_at IS NONE
        """.trimIndent()

        return db.query(sql, mapOf("parentId" to "initiative:${parentId}"), Initiative::class)
    }

    override suspend fun getFocused(userId: Ulid): AltairResult<Initiative?> {
        val sql = """
            SELECT * FROM initiative WHERE focused = true AND deleted_at IS NONE LIMIT 1
        """.trimIndent()

        return db.queryOne(sql, emptyMap(), Initiative::class)
            .map { it } // Returns nullable Initiative
    }

    override suspend fun create(initiative: Initiative): AltairResult<Initiative> {
        val sql = """
            CREATE initiative:${'$'}id CONTENT {
                name: ${'$'}name,
                description: ${'$'}description,
                parent_id: ${'$'}parentId,
                ongoing: ${'$'}ongoing,
                target_date: ${'$'}targetDate,
                status: ${'$'}status,
                focused: ${'$'}focused,
                created_at: ${'$'}now,
                updated_at: ${'$'}now,
                deleted_at: NONE
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to initiative.id.toString(),
                "name" to initiative.name,
                "description" to initiative.description,
                "parentId" to initiative.parentId?.toString(),
                "ongoing" to initiative.ongoing,
                "targetDate" to initiative.targetDate?.toString(),
                "status" to initiative.status.name,
                "focused" to initiative.focused,
                "now" to now().toString()
            ),
            Initiative::class
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create initiative").left()
        }
    }

    override suspend fun update(initiative: Initiative): AltairResult<Initiative> {
        val sql = """
            UPDATE initiative:${'$'}id SET
                name = ${'$'}name,
                description = ${'$'}description,
                parent_id = ${'$'}parentId,
                ongoing = ${'$'}ongoing,
                target_date = ${'$'}targetDate,
                status = ${'$'}status,
                focused = ${'$'}focused,
                updated_at = ${'$'}now
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to initiative.id.toString(),
                "name" to initiative.name,
                "description" to initiative.description,
                "parentId" to initiative.parentId?.toString(),
                "ongoing" to initiative.ongoing,
                "targetDate" to initiative.targetDate?.toString(),
                "status" to initiative.status.name,
                "focused" to initiative.focused,
                "now" to now().toString()
            ),
            Initiative::class
        ).flatMap { result ->
            result?.right() ?: notFoundError(initiative.id).left()
        }
    }

    override suspend fun setFocused(userId: Ulid, initiativeId: Ulid?): AltairResult<Unit> {
        val now = now()

        // First, unfocus any currently focused initiative
        val unfocusSql = """
            UPDATE initiative SET focused = false, updated_at = ${'$'}now
            WHERE focused = true AND deleted_at IS NONE
        """.trimIndent()

        return db.query(unfocusSql, mapOf("now" to now.toString()), Initiative::class)
            .flatMap {
                // If initiativeId is provided, focus that initiative
                if (initiativeId != null) {
                    val focusSql = """
                        UPDATE initiative:${'$'}id SET focused = true, updated_at = ${'$'}now
                        WHERE deleted_at IS NONE
                    """.trimIndent()

                    db.query(focusSql, mapOf("id" to initiativeId.toString(), "now" to now.toString()), Initiative::class)
                        .flatMap { results ->
                            if (results.isNotEmpty()) {
                                Unit.right()
                            } else {
                                notFoundError(initiativeId).left()
                            }
                        }
                } else {
                    // Just clearing focus, no specific initiative to focus
                    Unit.right()
                }
            }
    }

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> {
        val sql = """
            UPDATE initiative:${'$'}id SET deleted_at = ${'$'}now WHERE deleted_at IS NONE
        """.trimIndent()

        return db.query(sql, mapOf("id" to id.toString(), "now" to now().toString()), Initiative::class)
            .flatMap { results ->
                if (results.isNotEmpty()) Unit.right()
                else notFoundError(id).left()
            }
    }

    override suspend fun restore(id: Ulid): AltairResult<Unit> {
        val sql = """
            UPDATE initiative:${'$'}id SET deleted_at = NONE WHERE deleted_at IS NOT NONE
        """.trimIndent()

        return db.query(sql, mapOf("id" to id.toString()), Initiative::class)
            .flatMap { results ->
                if (results.isNotEmpty()) Unit.right()
                else notFoundError(id).left()
            }
    }
}

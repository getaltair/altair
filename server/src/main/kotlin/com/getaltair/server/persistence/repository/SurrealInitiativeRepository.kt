package com.getaltair.server.persistence.repository

import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.system.Initiative
import com.getaltair.altair.shared.repository.InitiativeRepository
import com.getaltair.server.auth.AuthContext
import com.getaltair.server.persistence.SurrealDbClient

/**
 * SurrealDB implementation of the Initiative repository.
 *
 * Initiatives are cross-module organizational containers (Projects and Areas).
 * This repository enforces focus uniqueness (only one focused initiative per user)
 * and provides hierarchical query capabilities.
 */
class SurrealInitiativeRepository(
    db: SurrealDbClient,
    auth: AuthContext
) : BaseSurrealRepository<Initiative>(db, auth, "initiative", Initiative::class), InitiativeRepository {

    override fun notFoundError(id: Ulid): AltairError =
        AltairError.NotFoundError.InitiativeNotFound(id.toString())

    override suspend fun getById(id: Ulid): AltairResult<Initiative> = findById(id)

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Initiative>> = findAllForUser()

    override suspend fun getChildren(parentId: Ulid): AltairResult<List<Initiative>> =
        findWhere("parent_id = \$parentId", mapOf("parentId" to "initiative:${parentId}"))

    override suspend fun getFocused(userId: Ulid): AltairResult<Initiative?> =
        findOneWhere("focused = true")

    override suspend fun create(initiative: Initiative): AltairResult<Initiative> {
        val sql = """
            CREATE initiative:${'$'}id CONTENT {
                user_id: ${'$'}userId,
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

        return executeQueryOne(
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
            )
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
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
        """.trimIndent()

        return executeQueryOne(
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
            )
        ).flatMap { result ->
            result?.right() ?: notFoundError(initiative.id).left()
        }
    }

    override suspend fun setFocused(userId: Ulid, initiativeId: Ulid?): AltairResult<Unit> {
        val now = now()

        // First, unfocus any currently focused initiative for this user
        val unfocusSql = """
            UPDATE initiative SET focused = false, updated_at = ${'$'}now
            WHERE user_id = ${'$'}userId AND focused = true AND deleted_at IS NONE
        """.trimIndent()

        return executeQuery(unfocusSql, mapOf("now" to now.toString()))
            .flatMap {
                // If initiativeId is provided, focus that initiative
                if (initiativeId != null) {
                    val focusSql = """
                        UPDATE initiative:${'$'}id SET focused = true, updated_at = ${'$'}now
                        WHERE user_id = ${'$'}userId AND deleted_at IS NONE
                    """.trimIndent()

                    executeQuery(focusSql, mapOf("id" to initiativeId.toString(), "now" to now.toString()))
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

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> = softDeleteEntity(id)

    override suspend fun restore(id: Ulid): AltairResult<Unit> = restoreEntity(id)
}

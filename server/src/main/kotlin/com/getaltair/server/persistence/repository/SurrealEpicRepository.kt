package com.getaltair.server.persistence.repository

import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.domain.common.EpicStatus
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.guidance.Epic
import com.getaltair.altair.shared.repository.EpicRepository
import com.getaltair.server.auth.AuthContext
import com.getaltair.server.persistence.SurrealDbClient

/**
 * SurrealDB implementation of [EpicRepository].
 *
 * Provides CRUD operations for Epics with automatic user scoping.
 * Epics serve as organizational containers for related Quests and
 * can optionally be linked to Initiatives for cross-module alignment.
 *
 * ## Lifecycle
 *
 * Epics progress through three states:
 * - **ACTIVE**: Accepting new quests, work in progress
 * - **COMPLETED**: All quests finished, sets completedAt timestamp
 * - **ARCHIVED**: Preserved for reference, no longer active
 *
 * ## Soft Delete
 *
 * Epics are soft-deleted via [softDelete]. Unlike Quests, there is no
 * restore operation for Epics - this is by design per the interface contract.
 *
 * @param db The SurrealDB client for database operations
 * @param auth The authentication context providing current user ID
 */
class SurrealEpicRepository(
    db: SurrealDbClient,
    auth: AuthContext
) : BaseSurrealRepository<Epic>(db, auth, "epic", Epic::class), EpicRepository {

    override fun notFoundError(id: Ulid): AltairError =
        AltairError.NotFoundError.EpicNotFound(id.toString())

    // ========== Core CRUD ==========

    override suspend fun getById(id: Ulid): AltairResult<Epic> =
        findById(id)

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Epic>> =
        findAllForUser()

    override suspend fun getByInitiative(initiativeId: Ulid): AltairResult<List<Epic>> =
        findWhere("initiative_id = \$initiativeId", mapOf("initiativeId" to "initiative:${initiativeId}"))

    override suspend fun getByStatus(userId: Ulid, status: EpicStatus): AltairResult<List<Epic>> =
        findWhere("status = \$status", mapOf("status" to status.name))

    override suspend fun create(epic: Epic): AltairResult<Epic> {
        val sql = """
            CREATE epic:${'$'}id CONTENT {
                user_id: ${'$'}userId,
                title: ${'$'}title,
                description: ${'$'}description,
                status: ${'$'}status,
                initiative_id: ${'$'}initiativeId,
                created_at: ${'$'}now,
                updated_at: ${'$'}now,
                completed_at: ${'$'}completedAt,
                deleted_at: NONE
            }
        """.trimIndent()

        return executeQueryOne(
            sql,
            mapOf(
                "id" to epic.id.toString(),
                "title" to epic.title,
                "description" to epic.description,
                "status" to epic.status.name,
                "initiativeId" to epic.initiativeId?.let { "initiative:$it" },
                "now" to now().toString(),
                "completedAt" to epic.completedAt?.toString()
            )
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create epic").left()
        }
    }

    override suspend fun update(epic: Epic): AltairResult<Epic> {
        val sql = """
            UPDATE epic:${'$'}id SET
                title = ${'$'}title,
                description = ${'$'}description,
                status = ${'$'}status,
                initiative_id = ${'$'}initiativeId,
                updated_at = ${'$'}now,
                completed_at = ${'$'}completedAt
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
        """.trimIndent()

        return executeQueryOne(
            sql,
            mapOf(
                "id" to epic.id.toString(),
                "title" to epic.title,
                "description" to epic.description,
                "status" to epic.status.name,
                "initiativeId" to epic.initiativeId?.let { "initiative:$it" },
                "now" to now().toString(),
                "completedAt" to epic.completedAt?.toString()
            )
        ).flatMap { result ->
            result?.right() ?: notFoundError(epic.id).left()
        }
    }

    // ========== Status Transitions ==========

    override suspend fun complete(id: Ulid): AltairResult<Epic> {
        val sql = """
            UPDATE epic:${'$'}id SET
                status = 'COMPLETED',
                completed_at = time::now(),
                updated_at = time::now()
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
        """.trimIndent()

        return executeQueryOne(sql, mapOf("id" to id.toString()))
            .flatMap { result ->
                result?.right() ?: notFoundError(id).left()
            }
    }

    override suspend fun archive(id: Ulid): AltairResult<Epic> {
        val sql = """
            UPDATE epic:${'$'}id SET
                status = 'ARCHIVED',
                updated_at = time::now()
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
        """.trimIndent()

        return executeQueryOne(sql, mapOf("id" to id.toString()))
            .flatMap { result ->
                result?.right() ?: notFoundError(id).left()
            }
    }

    // ========== Soft Delete ==========

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> =
        softDeleteEntity(id)
}

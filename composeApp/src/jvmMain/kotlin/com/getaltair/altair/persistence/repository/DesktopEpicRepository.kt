package com.getaltair.altair.persistence.repository

import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.persistence.DesktopSurrealDbClient
import com.getaltair.altair.shared.domain.common.EpicStatus
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.guidance.Epic
import com.getaltair.altair.shared.repository.EpicRepository

/**
 * Desktop implementation of EpicRepository using embedded SurrealDB.
 */
class DesktopEpicRepository(
    db: DesktopSurrealDbClient
) : BaseDesktopRepository<Epic>(db, "epic", Epic::class), EpicRepository {

    override suspend fun getById(id: Ulid): AltairResult<Epic> = findById(id)

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Epic>> {
        return findWhere("user_id = \$userId", mapOf("userId" to "user:$userId"))
    }

    override suspend fun getByInitiative(initiativeId: Ulid): AltairResult<List<Epic>> {
        return findWhere(
            "initiative_id = \$initiativeId",
            mapOf("initiativeId" to "initiative:$initiativeId")
        )
    }

    override suspend fun getByStatus(userId: Ulid, status: EpicStatus): AltairResult<List<Epic>> {
        return findWhere(
            "user_id = \$userId AND status = \$status",
            mapOf(
                "userId" to "user:$userId",
                "status" to status.name
            )
        )
    }

    override suspend fun create(epic: Epic): AltairResult<Epic> {
        val sql = """
            CREATE $tableName CONTENT {
                id: ${'$'}id,
                user_id: ${'$'}userId,
                title: ${'$'}title,
                description: ${'$'}description,
                status: ${'$'}status,
                initiative_id: ${'$'}initiativeId,
                created_at: ${'$'}createdAt,
                updated_at: ${'$'}updatedAt,
                completed_at: ${'$'}completedAt,
                deleted_at: ${'$'}deletedAt
            }
        """.trimIndent()

        val params = mapOf(
            "id" to "$tableName:${epic.id}",
            "userId" to "user:${epic.userId}",
            "title" to epic.title,
            "description" to epic.description,
            "status" to epic.status.name,
            "initiativeId" to epic.initiativeId?.let { "initiative:$it" },
            "createdAt" to now().toString(),
            "updatedAt" to now().toString(),
            "completedAt" to epic.completedAt?.toString(),
            "deletedAt" to epic.deletedAt?.toString()
        )

        return db.queryOne(sql, params, entityClass)
            .flatMap { result ->
                result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create epic").left()
            }
    }

    override suspend fun update(epic: Epic): AltairResult<Epic> {
        val sql = """
            UPDATE ${'$'}id MERGE {
                title: ${'$'}title,
                description: ${'$'}description,
                status: ${'$'}status,
                initiative_id: ${'$'}initiativeId,
                updated_at: ${'$'}updatedAt,
                completed_at: ${'$'}completedAt
            }
        """.trimIndent()

        val params = mapOf(
            "id" to "$tableName:${epic.id}",
            "title" to epic.title,
            "description" to epic.description,
            "status" to epic.status.name,
            "initiativeId" to epic.initiativeId?.let { "initiative:$it" },
            "updatedAt" to now().toString(),
            "completedAt" to epic.completedAt?.toString()
        )

        return db.queryOne(sql, params, entityClass)
            .flatMap { result ->
                result?.right() ?: notFoundError(epic.id).left()
            }
    }

    override suspend fun complete(id: Ulid): AltairResult<Epic> {
        val sql = """
            UPDATE ${'$'}id MERGE {
                status: 'COMPLETED',
                completed_at: ${'$'}completedAt,
                updated_at: ${'$'}updatedAt
            }
        """.trimIndent()

        val params = mapOf(
            "id" to "$tableName:$id",
            "completedAt" to now().toString(),
            "updatedAt" to now().toString()
        )

        return db.queryOne(sql, params, entityClass)
            .flatMap { result ->
                result?.right() ?: notFoundError(id).left()
            }
    }

    override suspend fun archive(id: Ulid): AltairResult<Epic> {
        val sql = """
            UPDATE ${'$'}id MERGE {
                status: 'ARCHIVED',
                updated_at: ${'$'}updatedAt
            }
        """.trimIndent()

        val params = mapOf(
            "id" to "$tableName:$id",
            "updatedAt" to now().toString()
        )

        return db.queryOne(sql, params, entityClass)
            .flatMap { result ->
                result?.right() ?: notFoundError(id).left()
            }
    }

    override fun notFoundError(id: Ulid): AltairError =
        AltairError.NotFoundError.EpicNotFound(id.toString())
}

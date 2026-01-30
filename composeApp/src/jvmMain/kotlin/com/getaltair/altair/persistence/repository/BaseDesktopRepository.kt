package com.getaltair.altair.persistence.repository

import arrow.core.Either
import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.persistence.DesktopSurrealDbClient
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import kotlinx.datetime.Instant
import kotlin.reflect.KClass

/**
 * Base repository for desktop with common CRUD patterns.
 * Unlike server version, no user scoping needed (single-user local DB).
 */
abstract class BaseDesktopRepository<T : Any>(
    protected val db: DesktopSurrealDbClient,
    protected val tableName: String,
    protected val entityClass: KClass<T>
) {
    protected fun now(): Instant = Instant.fromEpochMilliseconds(System.currentTimeMillis())

    protected fun whereNotDeleted(additionalConditions: String = ""): String {
        val baseCondition = "deleted_at IS NONE"
        return if (additionalConditions.isBlank()) {
            "WHERE $baseCondition"
        } else {
            "WHERE $baseCondition AND ($additionalConditions)"
        }
    }

    protected suspend fun findById(id: Ulid): AltairResult<T> {
        val sql = """
            SELECT * FROM $tableName
            WHERE id = ${'$'}id AND deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(sql, mapOf("id" to "$tableName:$id"), entityClass)
            .flatMap { entity ->
                entity?.right() ?: notFoundError(id).left()
            }
    }

    protected suspend fun findAll(): AltairResult<List<T>> {
        val sql = """
            SELECT * FROM $tableName
            ${whereNotDeleted()}
            ORDER BY created_at DESC
        """.trimIndent()

        return db.query(sql, emptyMap(), entityClass)
    }

    protected suspend fun findWhere(
        conditions: String,
        params: Map<String, Any?> = emptyMap()
    ): AltairResult<List<T>> {
        val sql = """
            SELECT * FROM $tableName
            ${whereNotDeleted(conditions)}
            ORDER BY created_at DESC
        """.trimIndent()

        return db.query(sql, params, entityClass)
    }

    protected suspend fun findOneWhere(
        conditions: String,
        params: Map<String, Any?> = emptyMap()
    ): AltairResult<T?> {
        val sql = """
            SELECT * FROM $tableName
            ${whereNotDeleted(conditions)}
            LIMIT 1
        """.trimIndent()

        return db.queryOne(sql, params, entityClass)
    }

    suspend fun softDelete(id: Ulid): AltairResult<Unit> {
        val sql = """
            UPDATE $tableName
            SET deleted_at = ${'$'}now, updated_at = ${'$'}now
            WHERE id = ${'$'}id AND deleted_at IS NONE
        """.trimIndent()

        return db.query(sql, mapOf("id" to "$tableName:$id", "now" to now().toString()), entityClass)
            .flatMap { results ->
                if (results.isNotEmpty()) Unit.right()
                else notFoundError(id).left()
            }
    }

    protected abstract fun notFoundError(id: Ulid): AltairError
}

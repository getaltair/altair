package com.getaltair.server.persistence.repository

import arrow.core.Either
import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.server.auth.AuthContext
import com.getaltair.server.persistence.SurrealDbClient
import kotlinx.datetime.Instant
import kotlin.reflect.KClass

/**
 * Base repository providing common CRUD patterns with user scoping.
 *
 * All queries automatically include user_id filtering to enforce
 * multi-tenant data isolation. Subclasses should use the protected
 * helper methods rather than calling SurrealDbClient directly.
 *
 * @param T The entity type this repository manages
 * @param db The SurrealDB client for database operations
 * @param auth The authentication context providing current user ID
 * @param tableName The database table name for this entity
 * @param entityClass The Kotlin class for deserialization
 */
abstract class BaseSurrealRepository<T : Any>(
    protected val db: SurrealDbClient,
    protected val auth: AuthContext,
    protected val tableName: String,
    protected val entityClass: KClass<T>
) {
    /**
     * The current user's ID for scoping all queries.
     */
    protected val currentUserId: Ulid
        get() = auth.currentUserId

    /**
     * Gets the current timestamp for audit fields.
     */
    protected fun now(): Instant = Instant.fromEpochMilliseconds(System.currentTimeMillis())

    /**
     * Builds a user-scoped WHERE clause.
     *
     * @param additionalConditions Optional additional conditions to AND with user scope
     * @return WHERE clause string including user_id filter
     */
    protected fun whereUserScoped(additionalConditions: String = ""): String {
        val userCondition = "user_id = \$userId AND deleted_at IS NONE"
        return if (additionalConditions.isBlank()) {
            "WHERE $userCondition"
        } else {
            "WHERE $userCondition AND ($additionalConditions)"
        }
    }

    /**
     * Base parameters including user ID for query binding.
     */
    protected fun baseParams(): Map<String, Any?> = mapOf(
        "userId" to currentUserId.toString()
    )

    /**
     * Merges additional parameters with base parameters.
     */
    protected fun params(vararg pairs: Pair<String, Any?>): Map<String, Any?> =
        baseParams() + pairs.toMap()

    /**
     * Finds an entity by ID with user scoping.
     *
     * @param id The entity's ULID
     * @return Either an error or the entity if found
     */
    protected suspend fun findById(id: Ulid): AltairResult<T> {
        val sql = """
            SELECT * FROM $tableName
            WHERE id = ${'$'}id AND user_id = ${'$'}userId AND deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(sql, params("id" to "$tableName:${id}"), entityClass)
            .flatMap { entity ->
                entity?.right() ?: notFoundError(id).left()
            }
    }

    /**
     * Finds all entities for the current user.
     *
     * @return Either an error or list of all user's entities
     */
    protected suspend fun findAllForUser(): AltairResult<List<T>> {
        val sql = """
            SELECT * FROM $tableName
            ${whereUserScoped()}
            ORDER BY created_at DESC
        """.trimIndent()

        return db.query(sql, baseParams(), entityClass)
    }

    /**
     * Finds entities matching additional conditions.
     *
     * @param conditions Additional WHERE conditions (will be ANDed with user scope)
     * @param additionalParams Parameters for the conditions
     * @return Either an error or matching entities
     */
    protected suspend fun findWhere(
        conditions: String,
        additionalParams: Map<String, Any?> = emptyMap()
    ): AltairResult<List<T>> {
        val sql = """
            SELECT * FROM $tableName
            ${whereUserScoped(conditions)}
            ORDER BY created_at DESC
        """.trimIndent()

        return db.query(sql, baseParams() + additionalParams, entityClass)
    }

    /**
     * Finds a single entity matching conditions.
     *
     * @param conditions Additional WHERE conditions
     * @param additionalParams Parameters for the conditions
     * @return Either an error or the entity (null if not found)
     */
    protected suspend fun findOneWhere(
        conditions: String,
        additionalParams: Map<String, Any?> = emptyMap()
    ): AltairResult<T?> {
        val sql = """
            SELECT * FROM $tableName
            ${whereUserScoped(conditions)}
            LIMIT 1
        """.trimIndent()

        return db.queryOne(sql, baseParams() + additionalParams, entityClass)
    }

    /**
     * Soft-deletes an entity by setting deleted_at timestamp.
     *
     * @param id The entity's ULID
     * @return Either an error or Unit on success
     */
    protected suspend fun softDeleteEntity(id: Ulid): AltairResult<Unit> {
        val sql = """
            UPDATE $tableName
            SET deleted_at = ${'$'}now, updated_at = ${'$'}now
            WHERE id = ${'$'}id AND user_id = ${'$'}userId AND deleted_at IS NONE
        """.trimIndent()

        return db.query(sql, params("id" to "$tableName:${id}", "now" to now().toString()), entityClass)
            .flatMap { results ->
                if (results.isNotEmpty()) {
                    Unit.right()
                } else {
                    notFoundError(id).left()
                }
            }
    }

    /**
     * Restores a soft-deleted entity by clearing deleted_at.
     *
     * @param id The entity's ULID
     * @return Either an error or Unit on success
     */
    protected suspend fun restoreEntity(id: Ulid): AltairResult<Unit> {
        val sql = """
            UPDATE $tableName
            SET deleted_at = NONE, updated_at = ${'$'}now
            WHERE id = ${'$'}id AND user_id = ${'$'}userId AND deleted_at IS NOT NONE
        """.trimIndent()

        return db.query(sql, params("id" to "$tableName:${id}", "now" to now().toString()), entityClass)
            .flatMap { results ->
                if (results.isNotEmpty()) {
                    Unit.right()
                } else {
                    notFoundError(id).left()
                }
            }
    }

    /**
     * Executes raw SQL with user parameters.
     * Useful for complex queries that don't fit standard patterns.
     */
    protected suspend fun executeQuery(
        sql: String,
        additionalParams: Map<String, Any?> = emptyMap()
    ): AltairResult<List<T>> {
        return db.query(sql, baseParams() + additionalParams, entityClass)
    }

    /**
     * Executes raw SQL expecting a single result.
     */
    protected suspend fun executeQueryOne(
        sql: String,
        additionalParams: Map<String, Any?> = emptyMap()
    ): AltairResult<T?> {
        return db.queryOne(sql, baseParams() + additionalParams, entityClass)
    }

    /**
     * Creates the appropriate NotFoundError for this entity type.
     * Subclasses should override to provide entity-specific errors.
     */
    protected abstract fun notFoundError(id: Ulid): AltairError
}

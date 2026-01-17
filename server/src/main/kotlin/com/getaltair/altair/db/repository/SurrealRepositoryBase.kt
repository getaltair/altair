package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

/**
 * Base class for SurrealDB repository implementations.
 *
 * Provides common functionality for querying and deserializing entities.
 */
abstract class SurrealRepositoryBase(
    protected val db: SurrealDbClient,
    protected val userId: String,
) {
    protected val json =
        Json {
            ignoreUnknownKeys = true
            encodeDefaults = true
            isLenient = true
        }

    /**
     * Extracts the string ID from a SurrealDB record ID.
     * SurrealDB returns IDs as "table:ulid", we want just "ulid".
     */
    protected fun extractId(recordId: String): String = recordId.substringAfter(":")

    /**
     * Builds a SurrealDB record ID from a table name and ULID.
     */
    protected fun recordId(
        table: String,
        ulid: String,
    ): String = "$table:$ulid"

    /**
     * Creates a Flow that emits query results.
     *
     * Note: SurrealDB doesn't have native reactive queries,
     * so this is a snapshot query wrapped in a Flow.
     * Real-time updates would require SurrealDB's LIVE queries.
     */
    protected fun <T> queryAsFlow(query: suspend () -> Either<DomainError, List<T>>): Flow<List<T>> =
        flow {
            val result = query()
            result.fold(
                ifLeft = { emit(emptyList()) },
                ifRight = { emit(it) },
            )
        }

    /**
     * Parses a JSON array result from SurrealDB.
     */
    protected fun parseJsonArray(result: String): List<JsonElement> =
        try {
            json.parseToJsonElement(result).jsonArray.toList()
        } catch (e: Exception) {
            emptyList()
        }

    /**
     * Parses a single JSON object result from SurrealDB.
     */
    protected fun parseJsonObject(result: String): JsonElement? =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            array.firstOrNull()
        } catch (e: Exception) {
            null
        }

    /**
     * Gets a string field from a JsonElement.
     */
    protected fun JsonElement.getString(key: String): String? =
        try {
            jsonObject[key]?.jsonPrimitive?.content
        } catch (e: Exception) {
            null
        }

    /**
     * Gets an int field from a JsonElement.
     */
    protected fun JsonElement.getInt(key: String): Int? =
        try {
            jsonObject[key]?.jsonPrimitive?.content?.toIntOrNull()
        } catch (e: Exception) {
            null
        }

    /**
     * Gets a long field from a JsonElement.
     */
    protected fun JsonElement.getLong(key: String): Long? =
        try {
            jsonObject[key]?.jsonPrimitive?.content?.toLongOrNull()
        } catch (e: Exception) {
            null
        }

    /**
     * Gets a boolean field from a JsonElement.
     */
    protected fun JsonElement.getBoolean(key: String): Boolean? =
        try {
            jsonObject[key]?.jsonPrimitive?.content?.toBooleanStrictOrNull()
        } catch (e: Exception) {
            null
        }

    /**
     * Common delete operation that sets deleted_at timestamp.
     */
    protected suspend fun softDelete(
        table: String,
        id: String,
    ): Either<DomainError, Unit> =
        either {
            db
                .execute(
                    """
                    UPDATE $table:$id SET
                        deleted_at = time::now(),
                        updated_at = time::now()
                    WHERE user_id = user:$userId;
                    """.trimIndent(),
                ).bind()
        }

    /**
     * Common hard delete operation.
     */
    protected suspend fun hardDelete(
        table: String,
        id: String,
    ): Either<DomainError, Unit> =
        either {
            db
                .execute(
                    """
                    DELETE $table:$id WHERE user_id = user:$userId;
                    """.trimIndent(),
                ).bind()
        }
}

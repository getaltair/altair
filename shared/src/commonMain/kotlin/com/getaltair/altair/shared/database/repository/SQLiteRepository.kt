package com.getaltair.altair.shared.database.repository

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.database.AltairDatabase
import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import kotlinx.datetime.Instant

/**
 * Base class for SQLite repository implementations.
 *
 * Provides common helper methods for mapping between SQLDelight generated types
 * and domain entities, as well as error handling utilities.
 *
 * @param database The AltairDatabase instance for executing queries
 */
abstract class SQLiteRepository(protected val database: AltairDatabase) {

    /**
     * Executes a database operation and wraps exceptions in AltairError.StorageError.
     *
     * @param T The return type of the operation
     * @param operation The database operation to execute
     * @return Either containing the result or a storage error
     */
    protected suspend fun <T> dbOperation(operation: suspend () -> T): Either<AltairError, T> =
        try {
            operation().right()
        } catch (e: Exception) {
            AltairError.StorageError.DatabaseError(
                e.message ?: "Unknown database error"
            ).left()
        }

    /**
     * Converts a String to Ulid.
     */
    protected fun String.toUlid(): Ulid = Ulid(this)

    /**
     * Converts a Long (epoch milliseconds) to Instant.
     */
    protected fun Long.toInstant(): Instant = Instant.fromEpochMilliseconds(this)

    /**
     * Converts an Instant to Long (epoch milliseconds).
     */
    protected fun Instant.toLong(): Long = this.toEpochMilliseconds()

    /**
     * Converts a String to QuestStatus enum.
     */
    protected fun String.toQuestStatus(): QuestStatus = QuestStatus.valueOf(this)

    /**
     * Converts a nullable Long to nullable Instant.
     */
    protected fun Long?.toInstantOrNull(): Instant? = this?.toInstant()

    /**
     * Converts a nullable Instant to nullable Long.
     */
    protected fun Instant?.toLongOrNull(): Long? = this?.toLong()

    /**
     * Converts an integer boolean (0/1) to Boolean.
     */
    protected fun Long.toBoolean(): Boolean = this != 0L

    /**
     * Converts a Boolean to integer (0/1).
     */
    protected fun Boolean.toLong(): Long = if (this) 1L else 0L

    /**
     * Serializes a list of floats to a string for storage.
     */
    protected fun List<Float>.serialize(): String = this.joinToString(",")

    /**
     * Deserializes a string to a list of floats.
     */
    protected fun String.deserializeFloatList(): List<Float> =
        if (this.isBlank()) emptyList()
        else this.split(",").map { it.toFloat() }

    /**
     * Serializes a list of Ulids to a string for storage.
     */
    protected fun List<Ulid>.serializeUlids(): String =
        this.joinToString(",") { it.value }

    /**
     * Deserializes a string to a list of Ulids.
     */
    protected fun String.deserializeUlidList(): List<Ulid> =
        if (this.isBlank()) emptyList()
        else this.split(",").map { Ulid(it) }
}

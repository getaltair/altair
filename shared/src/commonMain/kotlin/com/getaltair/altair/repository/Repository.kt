package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow

/**
 * Base repository interface defining common CRUD operations.
 *
 * All repository methods use Arrow's [Either] for explicit error handling
 * instead of throwing exceptions. This follows the architecture pattern
 * outlined in CLAUDE.md:
 * - All operations that can fail return `Either<Error, T>`
 * - Never throw exceptions for expected failures
 *
 * Note: The `userId` is NOT included in method signatures. Repository
 * implementations receive the authenticated user context during construction.
 * This eliminates redundant parameters and prevents potential misuse.
 *
 * @param T The domain entity type managed by this repository
 * @param E The domain error type for this repository's operations
 */
interface Repository<T, E : DomainError> {
    /**
     * Finds an entity by its unique identifier.
     *
     * @param id The ULID of the entity to find
     * @return Either an error if not found, or the entity
     */
    suspend fun findById(id: Ulid): Either<E, T>

    /**
     * Saves an entity (insert or update).
     *
     * If the entity's ID exists, it will be updated; otherwise, it will be inserted.
     *
     * @param entity The entity to save
     * @return Either an error on failure, or the saved entity (may have updated timestamps)
     */
    suspend fun save(entity: T): Either<E, T>

    /**
     * Deletes an entity by its unique identifier.
     *
     * For soft-deletable entities, this sets the `deletedAt` timestamp.
     * For other entities, this performs a hard delete.
     *
     * @param id The ULID of the entity to delete
     * @return Either an error on failure, or Unit on success
     */
    suspend fun delete(id: Ulid): Either<E, Unit>

    /**
     * Returns a reactive stream of all non-deleted entities.
     *
     * The Flow emits a new list whenever the underlying data changes.
     * This is useful for reactive UIs that need to stay in sync with data changes.
     *
     * @return A Flow emitting the current list of entities
     */
    fun findAll(): Flow<List<T>>
}

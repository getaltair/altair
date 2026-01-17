package com.getaltair.altair.repository

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.ensure
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow

/**
 * Pagination parameters for repository queries.
 *
 * Use the [invoke] factory method to create instances with validation,
 * or [unsafeCreate] when you are certain the values are valid.
 *
 * @property limit Maximum number of items to return (default: 50, max: 100)
 * @property offset Number of items to skip (default: 0)
 */
data class PageRequest private constructor(
    val limit: Int,
    val offset: Int,
) {
    companion object {
        const val DEFAULT_PAGE_SIZE = 50
        const val MAX_PAGE_SIZE = 100

        /**
         * Creates a PageRequest with validation.
         *
         * @param limit Maximum number of items to return (1 to 100)
         * @param offset Number of items to skip (must be non-negative)
         * @return Either a [DomainError.ValidationError] if parameters are invalid,
         *         or a valid [PageRequest]
         */
        operator fun invoke(
            limit: Int = DEFAULT_PAGE_SIZE,
            offset: Int = 0,
        ): Either<DomainError.ValidationError, PageRequest> =
            either {
                ensure(limit in 1..MAX_PAGE_SIZE) {
                    DomainError.ValidationError("limit", "Limit must be between 1 and $MAX_PAGE_SIZE")
                }
                ensure(offset >= 0) {
                    DomainError.ValidationError("offset", "Offset must be non-negative")
                }
                PageRequest(limit, offset)
            }

        /**
         * Creates a PageRequest without validation.
         *
         * Use this only when you are certain the values are valid (e.g., from
         * trusted internal sources or when values have already been validated).
         *
         * @throws IllegalArgumentException if limit or offset are invalid
         */
        fun unsafeCreate(
            limit: Int = DEFAULT_PAGE_SIZE,
            offset: Int = 0,
        ): PageRequest {
            require(limit in 1..MAX_PAGE_SIZE) { "Limit must be between 1 and $MAX_PAGE_SIZE" }
            require(offset >= 0) { "Offset must be non-negative" }
            return PageRequest(limit, offset)
        }
    }
}

/**
 * Paginated result containing items and pagination metadata.
 *
 * @property items The items in this page
 * @property totalCount Total number of items matching the query (must be >= items.size)
 * @property hasMore Whether there are more items after this page
 */
data class PageResult<T>(
    val items: List<T>,
    val totalCount: Int,
    val hasMore: Boolean,
) {
    init {
        require(totalCount >= 0) { "Total count must be non-negative" }
        require(totalCount >= items.size) { "Total count must be >= items size" }
    }
}

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

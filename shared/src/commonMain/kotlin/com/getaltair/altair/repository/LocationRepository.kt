package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.tracking.Location
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow

/**
 * Repository for Location entities.
 *
 * Locations are fixed physical spaces (rooms, buildings) that can contain
 * Items directly or Containers. They can be hierarchical via parentId.
 */
interface LocationRepository : Repository<Location, DomainError> {
    /**
     * Finds all root-level locations (locations without a parent).
     *
     * @return A Flow emitting root locations
     */
    fun findRoots(): Flow<List<Location>>

    /**
     * Finds all child locations of a specific parent.
     *
     * @param parentId The ULID of the parent location
     * @return A Flow emitting child locations
     */
    fun findByParent(parentId: Ulid): Flow<List<Location>>

    /**
     * Returns the full location hierarchy as a tree structure.
     *
     * @return A Flow emitting the location tree (roots with nested children)
     */
    fun findTree(): Flow<List<LocationNode>>

    /**
     * Gets the path from root to a specific location.
     *
     * @param id The ULID of the location
     * @return Either an error on failure, or the path (list of locations from root)
     */
    suspend fun getPath(id: Ulid): Either<DomainError, List<Location>>

    /**
     * Moves a location to a new parent.
     *
     * @param id The ULID of the location to move
     * @param newParentId The ULID of the new parent (null for root)
     * @return Either an error (e.g., circular reference), or the updated location
     */
    suspend fun move(
        id: Ulid,
        newParentId: Ulid?,
    ): Either<DomainError, Location>

    /**
     * Searches locations by name (case-insensitive partial match).
     *
     * @param query The search query
     * @return Either an error on failure, or matching locations
     */
    suspend fun searchByName(query: String): Either<DomainError, List<Location>>

    /**
     * Counts items and containers at a location (including descendants).
     *
     * @param id The ULID of the location
     * @return Either an error on failure, or the counts
     */
    suspend fun countContents(id: Ulid): Either<DomainError, LocationContents>
}

/**
 * A location with its nested children for tree representation.
 */
data class LocationNode(
    val location: Location,
    val children: List<LocationNode>,
)

/**
 * Contents count for a location.
 */
data class LocationContents(
    val itemCount: Int,
    val containerCount: Int,
)

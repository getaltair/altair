package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.ItemError
import com.getaltair.altair.domain.model.tracking.Container
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow

/**
 * Repository for Container entities.
 *
 * Containers are movable storage units (boxes, bags, drawers) that hold Items
 * and can be moved between Locations. They can also be nested inside other containers.
 */
interface ContainerRepository : Repository<Container, ItemError> {
    /**
     * Finds all containers at a specific location.
     *
     * @param locationId The ULID of the location
     * @return A Flow emitting containers at the location
     */
    fun findByLocation(locationId: Ulid): Flow<List<Container>>

    /**
     * Finds all containers nested inside a specific parent container.
     *
     * @param parentContainerId The ULID of the parent container
     * @return A Flow emitting nested containers
     */
    fun findByParentContainer(parentContainerId: Ulid): Flow<List<Container>>

    /**
     * Finds all root containers (not nested inside another container).
     *
     * @return A Flow emitting root containers
     */
    fun findRoots(): Flow<List<Container>>

    /**
     * Moves a container to a new location.
     *
     * Clears any parent container association.
     *
     * @param id The ULID of the container
     * @param locationId The ULID of the new location (null to clear location)
     * @return Either an error on failure, or the updated container
     */
    suspend fun moveToLocation(
        id: Ulid,
        locationId: Ulid?,
    ): Either<ItemError, Container>

    /**
     * Nests a container inside another container.
     *
     * Validates that this doesn't create a cycle.
     *
     * @param id The ULID of the container to nest
     * @param parentContainerId The ULID of the parent container
     * @return Either an error (e.g., cycle detected), or the updated container
     */
    suspend fun nestInContainer(
        id: Ulid,
        parentContainerId: Ulid,
    ): Either<ItemError, Container>

    /**
     * Removes a container from its parent container (un-nests it).
     *
     * @param id The ULID of the container
     * @return Either an error on failure, or the updated container
     */
    suspend fun unnest(id: Ulid): Either<ItemError, Container>

    /**
     * Searches containers by name or label (case-insensitive partial match).
     *
     * @param query The search query
     * @return Either an error on failure, or matching containers
     */
    suspend fun searchByNameOrLabel(query: String): Either<ItemError, List<Container>>

    /**
     * Counts items inside a container (direct contents only, not nested containers).
     *
     * @param id The ULID of the container
     * @return Either an error on failure, or the item count
     */
    suspend fun countItems(id: Ulid): Either<ItemError, Int>

    /**
     * Returns the container hierarchy path from root to the specified container.
     *
     * @param id The ULID of the container
     * @return Either an error on failure, or the path (list of containers from outermost)
     */
    suspend fun getPath(id: Ulid): Either<ItemError, List<Container>>
}

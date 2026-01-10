package com.getaltair.altair.data.repository

/**
 * Base repository interface for CRUD operations.
 *
 * Defines the standard data access pattern for entities.
 * All operations are suspend functions for non-blocking execution.
 *
 * @param T The entity type
 * @param ID The identifier type (typically String for ULID)
 */
interface Repository<T, ID> {

    /**
     * Creates a new entity in the database.
     *
     * @param entity The entity to create (ID may be generated if not provided)
     * @return The created entity with generated fields populated
     */
    suspend fun create(entity: T): T

    /**
     * Finds an entity by its identifier.
     *
     * @param id The entity identifier
     * @return The entity if found, null otherwise
     */
    suspend fun findById(id: ID): T?

    /**
     * Updates an existing entity.
     *
     * @param entity The entity with updated values
     * @return The updated entity
     */
    suspend fun update(entity: T): T

    /**
     * Deletes an entity by its identifier.
     *
     * Implementation should perform soft-delete by setting deleted_at field.
     *
     * @param id The entity identifier
     * @return true if the entity was deleted, false if not found
     */
    suspend fun delete(id: ID): Boolean

    /**
     * Finds all entities that are not soft-deleted.
     *
     * @return List of active entities
     */
    suspend fun findAll(): List<T>
}

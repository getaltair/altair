package com.getaltair.altair.data.repository.mobile

import com.getaltair.altair.data.entity.mobile.Quest
import com.getaltair.altair.data.entity.mobile.toDomain
import com.getaltair.altair.data.repository.Repository
import com.getaltair.altair.data.util.Timestamps
import com.getaltair.altair.data.util.UlidGenerator
import com.getaltair.altair.database.AltairDatabase

/**
 * SQLDelight-based repository implementation for Quest entities.
 *
 * Provides CRUD operations for Quest entities on mobile platforms (Android/iOS).
 * Uses soft-delete pattern for data retention and sync support.
 *
 * @property database The SQLDelight database instance
 */
class QuestRepositoryImpl(private val database: AltairDatabase) : Repository<Quest, String> {

    private val queries get() = database.questQueries

    /**
     * Creates a new quest in the database.
     *
     * Generates a ULID if the entity ID is empty and sets timestamps.
     *
     * @param entity The quest to create
     * @return The created quest with generated fields populated
     */
    override suspend fun create(entity: Quest): Quest {
        val now = Timestamps.now()
        val id = if (entity.id.isBlank()) UlidGenerator.generate() else entity.id

        queries.insert(
            id = id,
            title = entity.title,
            description = entity.description,
            status = entity.status,
            epic_id = entity.epicId,
            created_at = now,
            updated_at = now,
            deleted_at = null,
            sync_version = 0,
        )

        return entity.copy(
            id = id,
            createdAt = now,
            updatedAt = now,
            syncVersion = 0,
        )
    }

    /**
     * Finds a quest by its identifier.
     *
     * @param id The quest identifier (ULID)
     * @return The quest if found, null otherwise
     */
    override suspend fun findById(id: String): Quest? = queries.selectById(id).executeAsOneOrNull()?.toDomain()

    /**
     * Updates an existing quest.
     *
     * Updates the updatedAt timestamp and increments syncVersion.
     *
     * @param entity The quest with updated values
     * @return The updated quest
     */
    override suspend fun update(entity: Quest): Quest {
        val now = Timestamps.now()

        queries.updateById(
            title = entity.title,
            description = entity.description,
            status = entity.status,
            epic_id = entity.epicId,
            updated_at = now,
            id = entity.id,
        )

        return entity.copy(
            updatedAt = now,
            syncVersion = entity.syncVersion + 1,
        )
    }

    /**
     * Soft-deletes a quest by its identifier.
     *
     * Sets the deletedAt timestamp rather than removing the record.
     *
     * @param id The quest identifier
     * @return true if the quest was deleted, false if not found
     */
    override suspend fun delete(id: String): Boolean {
        val existing = findById(id) ?: return false
        if (existing.isDeleted) return false

        val now = Timestamps.now()
        queries.softDeleteById(
            deleted_at = now,
            updated_at = now,
            id = id,
        )
        return true
    }

    /**
     * Finds all quests that are not soft-deleted.
     *
     * @return List of active quests ordered by creation date (newest first)
     */
    override suspend fun findAll(): List<Quest> = queries.selectAll().executeAsList().map { it.toDomain() }
}

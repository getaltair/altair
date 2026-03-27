package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Update
import com.getaltair.altair.data.local.entity.EntityRelationEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface EntityRelationDao {

    @Query(
        """
        SELECT * FROM entity_relations
        WHERE (from_entity_type = :entityType AND from_entity_id = :entityId)
           OR (to_entity_type = :entityType AND to_entity_id = :entityId)
        ORDER BY created_at DESC
        """
    )
    fun getByEntity(entityType: String, entityId: UUID): Flow<List<EntityRelationEntity>>

    @Query("SELECT * FROM entity_relations WHERE from_entity_type = :entityType AND from_entity_id = :entityId ORDER BY created_at DESC")
    fun getByFromEntity(entityType: String, entityId: UUID): Flow<List<EntityRelationEntity>>

    @Query("SELECT * FROM entity_relations WHERE to_entity_type = :entityType AND to_entity_id = :entityId ORDER BY created_at DESC")
    fun getByToEntity(entityType: String, entityId: UUID): Flow<List<EntityRelationEntity>>

    @Insert
    suspend fun insert(entity: EntityRelationEntity)

    @Update
    suspend fun update(entity: EntityRelationEntity)

    @Query("DELETE FROM entity_relations WHERE id = :id")
    suspend fun delete(id: UUID)
}

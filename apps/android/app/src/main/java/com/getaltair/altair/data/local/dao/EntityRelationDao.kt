package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.EntityRelationEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface EntityRelationDao {
    @Query("SELECT * FROM entity_relations WHERE user_id = :userId AND deleted_at IS NULL")
    fun watchAll(userId: String): Flow<List<EntityRelationEntity>>

    @Query("SELECT * FROM entity_relations WHERE id = :id")
    fun watchById(id: String): Flow<EntityRelationEntity?>

    @Query("SELECT * FROM entity_relations WHERE to_entity_id = :targetId AND relation_type = 'note_link' AND deleted_at IS NULL")
    fun watchBacklinksForNote(targetId: String): Flow<List<EntityRelationEntity>>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: EntityRelationEntity)

    @Delete
    suspend fun delete(entity: EntityRelationEntity)
}

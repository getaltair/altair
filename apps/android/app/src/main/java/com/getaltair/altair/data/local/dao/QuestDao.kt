package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.QuestEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface QuestDao {
    @Query("SELECT * FROM quests WHERE user_id = :userId AND deleted_at IS NULL")
    fun watchAll(userId: String): Flow<List<QuestEntity>>

    @Query("SELECT * FROM quests WHERE id = :id")
    fun watchById(id: String): Flow<QuestEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: QuestEntity)

    @Delete
    suspend fun delete(entity: QuestEntity)
}

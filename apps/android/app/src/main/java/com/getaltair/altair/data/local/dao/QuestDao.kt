package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Update
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.QuestEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface QuestDao {

    @Query("SELECT * FROM guidance_quests WHERE id = :id")
    fun getById(id: UUID): Flow<QuestEntity?>

    @Query("SELECT * FROM guidance_quests WHERE epic_id = :epicId ORDER BY created_at DESC")
    fun getByEpicId(epicId: UUID): Flow<List<QuestEntity>>

    @Query("SELECT * FROM guidance_quests WHERE user_id = :userId ORDER BY created_at DESC")
    fun getByUserId(userId: UUID): Flow<List<QuestEntity>>

    @Query("SELECT * FROM guidance_quests WHERE status = :status ORDER BY created_at DESC")
    fun getByStatus(status: String): Flow<List<QuestEntity>>

    @Query("SELECT * FROM guidance_quests WHERE due_date = date('now', 'localtime') ORDER BY priority DESC")
    fun getDueToday(): Flow<List<QuestEntity>>

    @Query("SELECT * FROM guidance_quests WHERE due_date <= :date ORDER BY due_date ASC")
    fun getDueBefore(date: String): Flow<List<QuestEntity>>

    @Insert
    suspend fun insert(entity: QuestEntity)

    @Update
    suspend fun update(entity: QuestEntity)

    @Query("DELETE FROM guidance_quests WHERE id = :id")
    suspend fun delete(id: UUID)

    @Upsert
    suspend fun upsert(entity: QuestEntity)
}

package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Update
import com.getaltair.altair.data.local.entity.FocusSessionEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface FocusSessionDao {

    @Query("SELECT * FROM guidance_focus_sessions WHERE quest_id = :questId ORDER BY started_at DESC")
    fun getByQuestId(questId: UUID): Flow<List<FocusSessionEntity>>

    @Query("SELECT * FROM guidance_focus_sessions WHERE user_id = :userId ORDER BY started_at DESC")
    fun getByUserId(userId: UUID): Flow<List<FocusSessionEntity>>

    @Insert
    suspend fun insert(entity: FocusSessionEntity)

    @Update
    suspend fun update(entity: FocusSessionEntity)
}

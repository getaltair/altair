package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.DailyCheckinEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface DailyCheckinDao {
    @Query("SELECT * FROM daily_checkins WHERE user_id = :userId AND deleted_at IS NULL")
    fun watchAll(userId: String): Flow<List<DailyCheckinEntity>>

    @Query("SELECT * FROM daily_checkins WHERE id = :id")
    fun watchById(id: String): Flow<DailyCheckinEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: DailyCheckinEntity)

    @Delete
    suspend fun delete(entity: DailyCheckinEntity)
}

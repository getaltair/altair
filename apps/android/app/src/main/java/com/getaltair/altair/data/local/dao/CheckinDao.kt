package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Update
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.CheckinEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface CheckinDao {

    @Query("SELECT * FROM guidance_daily_checkins WHERE user_id = :userId AND date = :date")
    fun getByUserAndDate(userId: UUID, date: String): Flow<CheckinEntity?>

    @Query("SELECT * FROM guidance_daily_checkins WHERE user_id = :userId AND date >= :start AND date <= :end ORDER BY date ASC")
    fun getByUserInRange(userId: UUID, start: String, end: String): Flow<List<CheckinEntity>>

    @Insert
    suspend fun insert(entity: CheckinEntity)

    @Update
    suspend fun update(entity: CheckinEntity)

    @Upsert
    suspend fun upsert(entity: CheckinEntity)
}

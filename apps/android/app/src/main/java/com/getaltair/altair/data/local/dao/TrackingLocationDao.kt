package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.TrackingLocationEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface TrackingLocationDao {
    @Query("SELECT * FROM tracking_locations WHERE household_id = :householdId AND deleted_at IS NULL")
    fun watchAll(householdId: String): Flow<List<TrackingLocationEntity>>

    @Query("SELECT * FROM tracking_locations WHERE id = :id")
    fun watchById(id: String): Flow<TrackingLocationEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: TrackingLocationEntity)

    @Delete
    suspend fun delete(entity: TrackingLocationEntity)
}

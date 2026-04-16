package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.TrackingItemEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface TrackingItemDao {
    @Query("SELECT * FROM tracking_items WHERE household_id = :householdId AND deleted_at IS NULL")
    fun watchAll(householdId: String): Flow<List<TrackingItemEntity>>

    @Query("SELECT * FROM tracking_items WHERE id = :id")
    fun watchById(id: String): Flow<TrackingItemEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: TrackingItemEntity)

    @Delete
    suspend fun delete(entity: TrackingItemEntity)
}

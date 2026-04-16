package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.TrackingItemEventEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface TrackingItemEventDao {
    // Append-only: filter by item_id (no deleted_at column)
    @Query("SELECT * FROM tracking_item_events WHERE item_id = :itemId ORDER BY occurred_at DESC")
    fun watchAll(itemId: String): Flow<List<TrackingItemEventEntity>>

    @Query("SELECT * FROM tracking_item_events WHERE id = :id")
    fun watchById(id: String): Flow<TrackingItemEventEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: TrackingItemEventEntity)
}

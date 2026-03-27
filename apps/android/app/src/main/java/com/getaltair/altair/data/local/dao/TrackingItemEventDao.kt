package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.TrackingItemEventEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface TrackingItemEventDao {

    @androidx.room.Query("SELECT * FROM tracking_item_events WHERE item_id = :itemId ORDER BY created_at DESC")
    fun getByItemId(itemId: UUID): Flow<List<TrackingItemEventEntity>>

    @Insert
    suspend fun insert(entity: TrackingItemEventEntity)

    @Upsert
    suspend fun upsert(entity: TrackingItemEventEntity)
}

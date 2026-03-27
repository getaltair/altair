package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.TrackingLocationEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface TrackingLocationDao {

    @Query("SELECT * FROM tracking_locations WHERE household_id = :householdId ORDER BY name ASC")
    fun getByHousehold(householdId: UUID): Flow<List<TrackingLocationEntity>>

    @Query("SELECT * FROM tracking_locations WHERE id = :id")
    fun getById(id: UUID): Flow<TrackingLocationEntity?>

    @Insert
    suspend fun insert(entity: TrackingLocationEntity)

    @Upsert
    suspend fun upsert(entity: TrackingLocationEntity)

    @Delete
    suspend fun delete(entity: TrackingLocationEntity)
}

package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Update
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.TrackingItemEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface TrackingItemDao {

    @Query("SELECT * FROM tracking_items WHERE id = :id")
    fun getById(id: UUID): Flow<TrackingItemEntity?>

    @Query("SELECT * FROM tracking_items WHERE household_id = :householdId ORDER BY name ASC")
    fun getByHousehold(householdId: UUID): Flow<List<TrackingItemEntity>>

    @Query("SELECT * FROM tracking_items WHERE user_id = :userId ORDER BY name ASC")
    fun getByUserId(userId: UUID): Flow<List<TrackingItemEntity>>

    @Query("SELECT * FROM tracking_items WHERE user_id = :userId AND name LIKE '%' || :query || '%' ORDER BY name ASC")
    fun searchByName(userId: UUID, query: String): Flow<List<TrackingItemEntity>>

    @Query("SELECT * FROM tracking_items WHERE user_id = :userId AND barcode = :barcode LIMIT 1")
    fun getByBarcode(userId: UUID, barcode: String): Flow<TrackingItemEntity?>

    @Insert
    suspend fun insert(entity: TrackingItemEntity)

    @Update
    suspend fun update(entity: TrackingItemEntity)

    @Upsert
    suspend fun upsert(entity: TrackingItemEntity)

    @Delete
    suspend fun delete(entity: TrackingItemEntity)
}

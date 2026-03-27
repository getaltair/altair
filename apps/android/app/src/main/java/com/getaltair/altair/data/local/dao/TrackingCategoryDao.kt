package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.TrackingCategoryEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface TrackingCategoryDao {

    @Query("SELECT * FROM tracking_categories WHERE household_id = :householdId ORDER BY name ASC")
    fun getByHousehold(householdId: UUID): Flow<List<TrackingCategoryEntity>>

    @Query("SELECT * FROM tracking_categories WHERE id = :id")
    fun getById(id: UUID): Flow<TrackingCategoryEntity?>

    @Insert
    suspend fun insert(entity: TrackingCategoryEntity)

    @Upsert
    suspend fun upsert(entity: TrackingCategoryEntity)

    @Delete
    suspend fun delete(entity: TrackingCategoryEntity)
}

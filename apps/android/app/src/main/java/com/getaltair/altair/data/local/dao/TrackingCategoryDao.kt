package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.TrackingCategoryEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface TrackingCategoryDao {
    @Query("SELECT * FROM tracking_categories WHERE household_id = :householdId AND deleted_at IS NULL")
    fun watchAll(householdId: String): Flow<List<TrackingCategoryEntity>>

    @Query("SELECT * FROM tracking_categories WHERE id = :id")
    fun watchById(id: String): Flow<TrackingCategoryEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: TrackingCategoryEntity)

    @Delete
    suspend fun delete(entity: TrackingCategoryEntity)
}

package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.HouseholdEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface HouseholdDao {
    @Query("SELECT * FROM households WHERE owner_id = :userId AND deleted_at IS NULL")
    fun watchAll(userId: String): Flow<List<HouseholdEntity>>

    @Query("SELECT * FROM households WHERE id = :id")
    fun watchById(id: String): Flow<HouseholdEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: HouseholdEntity)

    @Delete
    suspend fun delete(entity: HouseholdEntity)
}

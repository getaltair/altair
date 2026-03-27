package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Update
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.HouseholdEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface HouseholdDao {

    @Query("SELECT * FROM households WHERE id = :id")
    fun getById(id: UUID): Flow<HouseholdEntity?>

    @Query("SELECT * FROM households WHERE created_by = :userId ORDER BY created_at DESC")
    fun getByUserId(userId: UUID): Flow<List<HouseholdEntity>>

    @Insert
    suspend fun insert(entity: HouseholdEntity)

    @Update
    suspend fun update(entity: HouseholdEntity)

    @Upsert
    suspend fun upsert(entity: HouseholdEntity)

    @Query("DELETE FROM households WHERE id = :id")
    suspend fun delete(id: UUID)
}

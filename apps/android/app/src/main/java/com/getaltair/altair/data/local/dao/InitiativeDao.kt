package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Update
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.InitiativeEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface InitiativeDao {

    @Query("SELECT * FROM initiatives WHERE id = :id")
    fun getById(id: UUID): Flow<InitiativeEntity?>

    @Query("SELECT * FROM initiatives WHERE user_id = :userId ORDER BY created_at DESC")
    fun getByUserId(userId: UUID): Flow<List<InitiativeEntity>>

    @Query("SELECT * FROM initiatives WHERE household_id = :householdId ORDER BY created_at DESC")
    fun getByHouseholdId(householdId: UUID): Flow<List<InitiativeEntity>>

    @Insert
    suspend fun insert(entity: InitiativeEntity)

    @Update
    suspend fun update(entity: InitiativeEntity)

    @Query("DELETE FROM initiatives WHERE id = :id")
    suspend fun delete(id: UUID)

    @Upsert
    suspend fun upsert(entity: InitiativeEntity)
}

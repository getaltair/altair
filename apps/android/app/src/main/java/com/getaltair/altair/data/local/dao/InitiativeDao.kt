package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.InitiativeEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface InitiativeDao {
    @Query("SELECT * FROM initiatives WHERE user_id = :userId AND deleted_at IS NULL")
    fun watchAll(userId: String): Flow<List<InitiativeEntity>>

    @Query("SELECT * FROM initiatives WHERE id = :id")
    fun watchById(id: String): Flow<InitiativeEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: InitiativeEntity)

    @Delete
    suspend fun delete(entity: InitiativeEntity)
}

package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.EpicEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface EpicDao {
    @Query("SELECT * FROM epics WHERE user_id = :userId AND deleted_at IS NULL")
    fun watchAll(userId: String): Flow<List<EpicEntity>>

    @Query("SELECT * FROM epics WHERE id = :id")
    fun watchById(id: String): Flow<EpicEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: EpicEntity)

    @Delete
    suspend fun delete(entity: EpicEntity)
}

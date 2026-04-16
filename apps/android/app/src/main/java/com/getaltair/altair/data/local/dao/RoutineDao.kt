package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.RoutineEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface RoutineDao {
    @Query("SELECT * FROM routines WHERE user_id = :userId AND deleted_at IS NULL")
    fun watchAll(userId: String): Flow<List<RoutineEntity>>

    @Query("SELECT * FROM routines WHERE id = :id")
    fun watchById(id: String): Flow<RoutineEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: RoutineEntity)

    @Delete
    suspend fun delete(entity: RoutineEntity)
}

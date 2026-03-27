package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Update
import com.getaltair.altair.data.local.entity.RoutineEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface RoutineDao {

    @Query("SELECT * FROM guidance_routines WHERE id = :id")
    fun getById(id: UUID): Flow<RoutineEntity?>

    @Query("SELECT * FROM guidance_routines WHERE user_id = :userId ORDER BY created_at DESC")
    fun getByUserId(userId: UUID): Flow<List<RoutineEntity>>

    @Query("SELECT * FROM guidance_routines WHERE user_id = :userId AND status = 'active' ORDER BY created_at DESC")
    fun getActiveByUserId(userId: UUID): Flow<List<RoutineEntity>>

    @Insert
    suspend fun insert(entity: RoutineEntity)

    @Update
    suspend fun update(entity: RoutineEntity)

    @Query("DELETE FROM guidance_routines WHERE id = :id")
    suspend fun delete(id: UUID)
}

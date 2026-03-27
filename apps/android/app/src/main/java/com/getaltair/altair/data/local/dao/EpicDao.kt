package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Update
import com.getaltair.altair.data.local.entity.EpicEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface EpicDao {

    @Query("SELECT * FROM guidance_epics WHERE id = :id")
    fun getById(id: UUID): Flow<EpicEntity?>

    @Query("SELECT * FROM guidance_epics WHERE initiative_id = :initiativeId ORDER BY created_at DESC")
    fun getByInitiativeId(initiativeId: UUID): Flow<List<EpicEntity>>

    @Query("SELECT * FROM guidance_epics WHERE user_id = :userId ORDER BY created_at DESC")
    fun getByUserId(userId: UUID): Flow<List<EpicEntity>>

    @Insert
    suspend fun insert(entity: EpicEntity)

    @Update
    suspend fun update(entity: EpicEntity)

    @Query("DELETE FROM guidance_epics WHERE id = :id")
    suspend fun delete(id: UUID)
}

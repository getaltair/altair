package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.Query
import com.getaltair.altair.data.local.entity.TagEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface TagDao {

    @Query("SELECT * FROM tags WHERE user_id = :userId ORDER BY name ASC")
    fun getByUserId(userId: UUID): Flow<List<TagEntity>>

    @Query("SELECT * FROM tags WHERE household_id = :householdId ORDER BY name ASC")
    fun getByHouseholdId(householdId: UUID): Flow<List<TagEntity>>

    @Insert
    suspend fun insert(entity: TagEntity)

    @Query("DELETE FROM tags WHERE id = :id")
    suspend fun delete(id: UUID)
}

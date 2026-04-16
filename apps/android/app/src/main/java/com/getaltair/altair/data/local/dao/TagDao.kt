package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.TagEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface TagDao {
    @Query("SELECT * FROM tags WHERE user_id = :userId")
    fun watchAll(userId: String): Flow<List<TagEntity>>

    @Query("SELECT * FROM tags WHERE id = :id")
    fun watchById(id: String): Flow<TagEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: TagEntity)

    @Delete
    suspend fun delete(entity: TagEntity)
}

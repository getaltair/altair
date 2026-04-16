package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.UserEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface UserDao {
    @Query("SELECT * FROM users WHERE id = :userId AND deleted_at IS NULL")
    fun watchAll(userId: String): Flow<List<UserEntity>>

    @Query("SELECT * FROM users WHERE id = :id")
    fun watchById(id: String): Flow<UserEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: UserEntity)

    @Delete
    suspend fun delete(entity: UserEntity)
}

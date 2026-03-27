package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Query
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.UserEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface UserDao {

    @Query("SELECT * FROM users WHERE id = :id")
    fun getById(id: UUID): Flow<UserEntity?>

    @Query("SELECT * FROM users ORDER BY display_name ASC")
    fun getAll(): Flow<List<UserEntity>>

    @Upsert
    suspend fun upsert(user: UserEntity)
}

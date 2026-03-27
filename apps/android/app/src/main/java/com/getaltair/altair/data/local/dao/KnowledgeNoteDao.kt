package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Update
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.KnowledgeNoteEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface KnowledgeNoteDao {

    @Query("SELECT * FROM knowledge_notes WHERE id = :id")
    fun getById(id: UUID): Flow<KnowledgeNoteEntity?>

    @Query("SELECT * FROM knowledge_notes WHERE user_id = :userId ORDER BY updated_at DESC")
    fun getByUserId(userId: UUID): Flow<List<KnowledgeNoteEntity>>

    @Query("SELECT * FROM knowledge_notes WHERE household_id = :householdId ORDER BY updated_at DESC")
    fun getByHousehold(householdId: UUID): Flow<List<KnowledgeNoteEntity>>

    @Query("SELECT * FROM knowledge_notes WHERE user_id = :userId AND title LIKE '%' || :query || '%' ORDER BY updated_at DESC")
    fun searchByTitle(userId: UUID, query: String): Flow<List<KnowledgeNoteEntity>>

    @Query("SELECT * FROM knowledge_notes WHERE user_id = :userId AND is_pinned = 1 ORDER BY updated_at DESC")
    fun getPinned(userId: UUID): Flow<List<KnowledgeNoteEntity>>

    @Insert
    suspend fun insert(entity: KnowledgeNoteEntity)

    @Update
    suspend fun update(entity: KnowledgeNoteEntity)

    @Upsert
    suspend fun upsert(entity: KnowledgeNoteEntity)

    @Query("DELETE FROM knowledge_notes WHERE id = :id")
    suspend fun delete(id: UUID)
}

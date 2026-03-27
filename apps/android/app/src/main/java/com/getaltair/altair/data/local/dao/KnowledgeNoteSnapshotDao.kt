package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.KnowledgeNoteSnapshotEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface KnowledgeNoteSnapshotDao {

    @Query("SELECT * FROM knowledge_note_snapshots WHERE note_id = :noteId ORDER BY created_at DESC")
    fun getByNoteId(noteId: UUID): Flow<List<KnowledgeNoteSnapshotEntity>>

    @Insert
    suspend fun insert(entity: KnowledgeNoteSnapshotEntity)

    @Upsert
    suspend fun upsert(entity: KnowledgeNoteSnapshotEntity)
}

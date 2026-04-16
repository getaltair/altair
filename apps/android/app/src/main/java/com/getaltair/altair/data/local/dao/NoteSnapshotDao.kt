package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.NoteSnapshotEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface NoteSnapshotDao {
    // Append-only: filter by note_id (no deleted_at column)
    @Query("SELECT * FROM note_snapshots WHERE note_id = :noteId ORDER BY created_at DESC")
    fun watchAll(noteId: String): Flow<List<NoteSnapshotEntity>>

    @Query("SELECT * FROM note_snapshots WHERE id = :id")
    fun watchById(id: String): Flow<NoteSnapshotEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: NoteSnapshotEntity)
}

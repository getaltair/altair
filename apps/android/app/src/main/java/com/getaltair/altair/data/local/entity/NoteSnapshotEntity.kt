package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "note_snapshots")
data class NoteSnapshotEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "note_id") val noteId: String,
    @ColumnInfo(name = "content") val content: String?,
    @ColumnInfo(name = "captured_at") val capturedAt: String?,
    @ColumnInfo(name = "created_at") val createdAt: String,
)

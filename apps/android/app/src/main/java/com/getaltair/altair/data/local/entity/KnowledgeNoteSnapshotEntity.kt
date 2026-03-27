package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.ForeignKey
import androidx.room.Index
import androidx.room.PrimaryKey
import java.util.UUID

@Entity(
    tableName = "knowledge_note_snapshots",
    foreignKeys = [
        ForeignKey(
            entity = KnowledgeNoteEntity::class,
            parentColumns = ["id"],
            childColumns = ["note_id"],
            onDelete = ForeignKey.CASCADE,
        ),
    ],
    indices = [
        Index(value = ["note_id"]),
    ],
)
data class KnowledgeNoteSnapshotEntity(
    @PrimaryKey
    val id: UUID,

    @ColumnInfo(name = "note_id")
    val noteId: UUID,

    @ColumnInfo(name = "content")
    val content: String,

    @ColumnInfo(name = "created_at")
    val createdAt: Long,

    @ColumnInfo(name = "created_by_process")
    val createdByProcess: String?,
)

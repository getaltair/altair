package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.KnowledgeNoteSnapshotEntity
import com.getaltair.altair.domain.entity.KnowledgeNoteSnapshot
import java.time.Instant

fun KnowledgeNoteSnapshotEntity.toDomain(): KnowledgeNoteSnapshot = KnowledgeNoteSnapshot(
    id = id,
    noteId = noteId,
    content = content,
    createdAt = Instant.ofEpochMilli(createdAt),
    createdByProcess = createdByProcess,
)

fun KnowledgeNoteSnapshot.toEntity(): KnowledgeNoteSnapshotEntity = KnowledgeNoteSnapshotEntity(
    id = id,
    noteId = noteId,
    content = content,
    createdAt = createdAt.toEpochMilli(),
    createdByProcess = createdByProcess,
)

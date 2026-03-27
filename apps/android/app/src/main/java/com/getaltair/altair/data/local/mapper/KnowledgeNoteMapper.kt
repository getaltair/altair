package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.KnowledgeNoteEntity
import com.getaltair.altair.domain.entity.ContentType
import com.getaltair.altair.domain.entity.KnowledgeNote
import java.time.Instant

fun KnowledgeNoteEntity.toDomain(): KnowledgeNote = KnowledgeNote(
    id = id,
    userId = userId,
    householdId = householdId,
    initiativeId = initiativeId,
    title = title,
    content = content,
    contentType = ContentType.fromString(contentType),
    isPinned = isPinned == 1,
    createdAt = Instant.ofEpochMilli(createdAt),
    updatedAt = Instant.ofEpochMilli(updatedAt),
)

fun KnowledgeNote.toEntity(): KnowledgeNoteEntity = KnowledgeNoteEntity(
    id = id,
    userId = userId,
    householdId = householdId,
    initiativeId = initiativeId,
    title = title,
    content = content,
    contentType = contentType.value,
    isPinned = if (isPinned) 1 else 0,
    createdAt = createdAt.toEpochMilli(),
    updatedAt = updatedAt.toEpochMilli(),
)

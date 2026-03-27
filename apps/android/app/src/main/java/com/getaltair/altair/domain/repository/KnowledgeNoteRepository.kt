package com.getaltair.altair.domain.repository

import com.getaltair.altair.domain.entity.KnowledgeNote
import java.util.UUID
import kotlinx.coroutines.flow.Flow

interface KnowledgeNoteRepository {
    fun getAll(): Flow<List<KnowledgeNote>>
    fun getById(id: UUID): Flow<KnowledgeNote?>
    fun search(query: String): Flow<List<KnowledgeNote>>
    fun getPinned(): Flow<List<KnowledgeNote>>
    suspend fun create(note: KnowledgeNote)
    suspend fun update(note: KnowledgeNote)
    suspend fun togglePin(id: UUID)
    suspend fun delete(id: UUID)
}

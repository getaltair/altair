package com.getaltair.altair.data.repository

import com.getaltair.altair.data.local.dao.KnowledgeNoteDao
import com.getaltair.altair.data.local.mapper.toDomain
import com.getaltair.altair.data.local.mapper.toEntity
import com.getaltair.altair.domain.entity.KnowledgeNote
import com.getaltair.altair.domain.repository.KnowledgeNoteRepository
import com.getaltair.altair.util.mapToDomain
import java.time.Instant
import java.util.UUID
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.firstOrNull
import kotlinx.coroutines.flow.map

class KnowledgeNoteRepositoryImpl(
    private val knowledgeNoteDao: KnowledgeNoteDao,
    private val userId: () -> UUID,
) : KnowledgeNoteRepository {

    override fun getAll(): Flow<List<KnowledgeNote>> =
        knowledgeNoteDao.getByUserId(userId()).mapToDomain { it.toDomain() }

    override fun getById(id: UUID): Flow<KnowledgeNote?> =
        knowledgeNoteDao.getById(id).map { it?.toDomain() }

    override fun search(query: String): Flow<List<KnowledgeNote>> =
        knowledgeNoteDao.searchByTitle(userId(), query).mapToDomain { it.toDomain() }

    override fun getPinned(): Flow<List<KnowledgeNote>> =
        knowledgeNoteDao.getPinned(userId()).mapToDomain { it.toDomain() }

    override suspend fun create(note: KnowledgeNote) {
        knowledgeNoteDao.insert(note.copy(userId = userId()).toEntity())
    }

    override suspend fun update(note: KnowledgeNote) {
        knowledgeNoteDao.update(note.toEntity())
    }

    override suspend fun togglePin(id: UUID) {
        val entity = knowledgeNoteDao.getById(id).firstOrNull()
            ?: throw IllegalStateException("KnowledgeNote $id not found")
        val note = entity.toDomain()
        val now = Instant.now()
        val toggled = note.copy(
            isPinned = !note.isPinned,
            updatedAt = now,
        )
        knowledgeNoteDao.upsert(toggled.toEntity())
    }

    override suspend fun delete(id: UUID) {
        knowledgeNoteDao.delete(id)
    }
}

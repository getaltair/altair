package com.getaltair.altair.data.repository

import com.getaltair.altair.data.local.dao.QuestDao
import com.getaltair.altair.data.local.mapper.toDomain
import com.getaltair.altair.data.local.mapper.toEntity
import com.getaltair.altair.domain.entity.Quest
import com.getaltair.altair.domain.entity.QuestStatus
import com.getaltair.altair.domain.repository.QuestRepository
import java.time.Instant
import java.time.LocalDate
import java.util.UUID
import com.getaltair.altair.util.mapToDomain
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.firstOrNull
import kotlinx.coroutines.flow.map

class QuestRepositoryImpl(
    private val questDao: QuestDao,
    private val userId: () -> UUID,
) : QuestRepository {

    override fun getAll(): Flow<List<Quest>> =
        questDao.getByUserId(userId()).mapToDomain { it.toDomain() }

    override fun getById(id: UUID): Flow<Quest?> =
        questDao.getById(id).map { it?.toDomain() }

    override fun getDueToday(): Flow<List<Quest>> =
        questDao.getDueToday(LocalDate.now().toString()).mapToDomain { it.toDomain() }

    override fun getByEpic(epicId: UUID): Flow<List<Quest>> =
        questDao.getByEpicId(epicId).mapToDomain { it.toDomain() }

    override suspend fun create(quest: Quest) {
        questDao.insert(quest.toEntity())
    }

    override suspend fun update(quest: Quest) {
        questDao.update(quest.toEntity())
    }

    override suspend fun complete(id: UUID) {
        val entity = questDao.getById(id).firstOrNull()
            ?: throw IllegalStateException("Quest $id not found")
        val quest = entity.toDomain()
        val now = Instant.now()
        val completed = quest.copy(
            status = QuestStatus.COMPLETED,
            completedAt = now,
            updatedAt = now,
        )
        questDao.update(completed.toEntity())
    }

    override suspend fun delete(id: UUID) {
        questDao.delete(id)
    }
}

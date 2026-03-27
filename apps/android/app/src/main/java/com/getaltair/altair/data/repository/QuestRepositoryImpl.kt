package com.getaltair.altair.data.repository

import com.getaltair.altair.data.local.dao.QuestDao
import com.getaltair.altair.data.local.mapper.toDomain
import com.getaltair.altair.data.local.mapper.toEntity
import com.getaltair.altair.domain.entity.Quest
import com.getaltair.altair.domain.repository.QuestRepository
import java.time.Instant
import java.util.UUID
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.firstOrNull
import kotlinx.coroutines.flow.map

class QuestRepositoryImpl(
    private val questDao: QuestDao,
    private val userId: () -> UUID,
) : QuestRepository {

    override fun getAll(): Flow<List<Quest>> =
        questDao.getByUserId(userId()).map { entities ->
            entities.map { it.toDomain() }
        }

    override fun getById(id: UUID): Flow<Quest?> =
        questDao.getById(id).map { it?.toDomain() }

    override fun getDueToday(): Flow<List<Quest>> =
        questDao.getDueToday().map { entities ->
            entities.map { it.toDomain() }
        }

    override fun getByEpic(epicId: UUID): Flow<List<Quest>> =
        questDao.getByEpicId(epicId).map { entities ->
            entities.map { it.toDomain() }
        }

    override suspend fun create(quest: Quest) {
        questDao.insert(quest.toEntity())
    }

    override suspend fun update(quest: Quest) {
        questDao.update(quest.toEntity())
    }

    override suspend fun complete(id: UUID) {
        val quest = questDao.getById(id).map { it?.toDomain() }.firstOrNull()
        if (quest != null) {
            val completed = quest.copy(
                status = "completed",
                completedAt = Instant.now(),
            )
            questDao.update(completed.toEntity())
        }
    }

    override suspend fun delete(id: UUID) {
        questDao.delete(id)
    }
}

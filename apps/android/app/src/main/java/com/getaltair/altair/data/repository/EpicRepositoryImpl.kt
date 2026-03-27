package com.getaltair.altair.data.repository

import com.getaltair.altair.data.local.dao.EpicDao
import com.getaltair.altair.data.local.mapper.toDomain
import com.getaltair.altair.data.local.mapper.toEntity
import com.getaltair.altair.domain.entity.Epic
import com.getaltair.altair.domain.repository.EpicRepository
import java.util.UUID
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class EpicRepositoryImpl(
    private val epicDao: EpicDao,
) : EpicRepository {

    override fun getByInitiative(initiativeId: UUID): Flow<List<Epic>> =
        epicDao.getByInitiativeId(initiativeId).map { entities ->
            entities.map { it.toDomain() }
        }

    override fun getById(id: UUID): Flow<Epic?> =
        epicDao.getById(id).map { it?.toDomain() }

    override suspend fun create(epic: Epic) {
        epicDao.insert(epic.toEntity())
    }

    override suspend fun update(epic: Epic) {
        epicDao.update(epic.toEntity())
    }
}

package com.getaltair.altair.data.repository

import com.getaltair.altair.data.local.dao.InitiativeDao
import com.getaltair.altair.data.local.mapper.toDomain
import com.getaltair.altair.data.local.mapper.toEntity
import com.getaltair.altair.domain.entity.Initiative
import com.getaltair.altair.domain.repository.InitiativeRepository
import java.util.UUID
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class InitiativeRepositoryImpl(
    private val initiativeDao: InitiativeDao,
    private val userId: () -> UUID,
) : InitiativeRepository {

    override fun getAll(): Flow<List<Initiative>> =
        initiativeDao.getByUserId(userId()).map { entities ->
            entities.map { it.toDomain() }
        }

    override fun getById(id: UUID): Flow<Initiative?> =
        initiativeDao.getById(id).map { it?.toDomain() }

    override suspend fun create(initiative: Initiative) {
        initiativeDao.insert(initiative.toEntity())
    }

    override suspend fun update(initiative: Initiative) {
        initiativeDao.update(initiative.toEntity())
    }

    override suspend fun delete(id: UUID) {
        initiativeDao.delete(id)
    }
}

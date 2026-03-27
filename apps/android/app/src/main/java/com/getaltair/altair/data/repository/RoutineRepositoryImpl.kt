package com.getaltair.altair.data.repository

import com.getaltair.altair.data.local.dao.RoutineDao
import com.getaltair.altair.data.local.mapper.toDomain
import com.getaltair.altair.data.local.mapper.toEntity
import com.getaltair.altair.domain.entity.Routine
import com.getaltair.altair.domain.repository.RoutineRepository
import java.util.UUID
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class RoutineRepositoryImpl(
    private val routineDao: RoutineDao,
    private val userId: () -> UUID,
) : RoutineRepository {

    override fun getAll(): Flow<List<Routine>> =
        routineDao.getByUserId(userId()).map { entities ->
            entities.map { it.toDomain() }
        }

    override fun getActive(): Flow<List<Routine>> =
        routineDao.getActiveByUserId(userId()).map { entities ->
            entities.map { it.toDomain() }
        }

    override fun getById(id: UUID): Flow<Routine?> =
        routineDao.getById(id).map { it?.toDomain() }

    override suspend fun create(routine: Routine) {
        routineDao.insert(routine.toEntity())
    }

    override suspend fun update(routine: Routine) {
        routineDao.update(routine.toEntity())
    }
}

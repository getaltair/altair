package com.getaltair.altair.domain.repository

import com.getaltair.altair.domain.entity.Routine
import java.util.UUID
import kotlinx.coroutines.flow.Flow

interface RoutineRepository {
    fun getAll(): Flow<List<Routine>>
    fun getActive(): Flow<List<Routine>>
    fun getById(id: UUID): Flow<Routine?>
    suspend fun create(routine: Routine)
    suspend fun update(routine: Routine)
}

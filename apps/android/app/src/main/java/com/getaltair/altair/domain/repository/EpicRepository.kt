package com.getaltair.altair.domain.repository

import com.getaltair.altair.domain.entity.Epic
import java.util.UUID
import kotlinx.coroutines.flow.Flow

interface EpicRepository {
    fun getByInitiative(initiativeId: UUID): Flow<List<Epic>>
    fun getById(id: UUID): Flow<Epic?>
    suspend fun create(epic: Epic)
    suspend fun update(epic: Epic)
}

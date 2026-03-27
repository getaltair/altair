package com.getaltair.altair.domain.repository

import com.getaltair.altair.domain.entity.Initiative
import java.util.UUID
import kotlinx.coroutines.flow.Flow

interface InitiativeRepository {
    fun getAll(): Flow<List<Initiative>>
    fun getById(id: UUID): Flow<Initiative?>
    suspend fun create(initiative: Initiative)
    suspend fun update(initiative: Initiative)
    suspend fun delete(id: UUID)
}

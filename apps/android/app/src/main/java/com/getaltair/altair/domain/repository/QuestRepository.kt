package com.getaltair.altair.domain.repository

import com.getaltair.altair.domain.entity.Quest
import java.util.UUID
import kotlinx.coroutines.flow.Flow

interface QuestRepository {
    fun getAll(): Flow<List<Quest>>
    fun getById(id: UUID): Flow<Quest?>
    fun getDueToday(): Flow<List<Quest>>
    fun getByEpic(epicId: UUID): Flow<List<Quest>>
    suspend fun create(quest: Quest)
    suspend fun update(quest: Quest)
    suspend fun complete(id: UUID)
    suspend fun delete(id: UUID)
}

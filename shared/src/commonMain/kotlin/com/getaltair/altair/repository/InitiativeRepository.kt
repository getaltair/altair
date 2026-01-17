package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.system.Initiative
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.InitiativeStatus
import kotlinx.coroutines.flow.Flow

/**
 * Repository for Initiative entities.
 *
 * Initiatives are cross-cutting organizational units (projects or areas)
 * that group related work across all modules.
 */
interface InitiativeRepository : Repository<Initiative, DomainError> {
    /**
     * Finds all initiatives with the specified status.
     *
     * @param status The status to filter by
     * @return A Flow emitting initiatives matching the status
     */
    fun findByStatus(status: InitiativeStatus): Flow<List<Initiative>>

    /**
     * Searches initiatives by name (case-insensitive partial match).
     *
     * @param query The search query to match against initiative names
     * @return Either an error on failure, or matching initiatives
     */
    suspend fun searchByName(query: String): Either<DomainError, List<Initiative>>

    /**
     * Counts entities linked to this initiative across all modules.
     *
     * This includes Quests, Epics, Notes, and Items associated with the initiative.
     *
     * @param id The ULID of the initiative
     * @return Either an error on failure, or the count of linked entities
     */
    suspend fun countLinkedEntities(id: Ulid): Either<DomainError, Int>
}

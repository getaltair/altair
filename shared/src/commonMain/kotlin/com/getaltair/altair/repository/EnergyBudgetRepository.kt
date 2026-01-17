package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.guidance.EnergyBudget
import kotlinx.coroutines.flow.Flow
import kotlinx.datetime.LocalDate

/**
 * Repository for EnergyBudget entities.
 *
 * EnergyBudget tracks how much energy a user has available for a given day
 * and how much has been spent on completed Quests.
 */
interface EnergyBudgetRepository : Repository<EnergyBudget, DomainError> {
    /**
     * Finds or creates an energy budget for a specific date.
     *
     * If no budget exists for the date, creates one with default values.
     *
     * @param date The date to get the budget for
     * @return Either an error on failure, or the budget for the date
     */
    suspend fun findOrCreateByDate(date: LocalDate): Either<DomainError, EnergyBudget>

    /**
     * Finds the energy budget for today.
     *
     * Convenience method for the common case.
     *
     * @return Either an error on failure, or today's budget
     */
    suspend fun findToday(): Either<DomainError, EnergyBudget>

    /**
     * Finds energy budgets for a date range.
     *
     * @param startDate The start of the range (inclusive)
     * @param endDate The end of the range (inclusive)
     * @return A Flow emitting budgets in the date range
     */
    fun findByDateRange(
        startDate: LocalDate,
        endDate: LocalDate,
    ): Flow<List<EnergyBudget>>

    /**
     * Adds spent energy to a budget for a specific date.
     *
     * This is called when a quest is completed to track energy consumption.
     *
     * @param date The date to update
     * @param energyToAdd The amount of energy to add to spent total
     * @return Either an error on failure, or the updated budget
     */
    suspend fun addSpentEnergy(
        date: LocalDate,
        energyToAdd: Int,
    ): Either<DomainError, EnergyBudget>

    /**
     * Updates the total budget for a specific date.
     *
     * @param date The date to update
     * @param newBudget The new total budget value
     * @return Either an error on failure, or the updated budget
     */
    suspend fun updateTotalBudget(
        date: LocalDate,
        newBudget: Int,
    ): Either<DomainError, EnergyBudget>
}

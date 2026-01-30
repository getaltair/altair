package com.getaltair.altair.shared.database.repository

import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.database.AltairDatabase
import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.guidance.Checkpoint
import com.getaltair.altair.shared.domain.guidance.EnergyBudget
import com.getaltair.altair.shared.domain.guidance.Quest
import com.getaltair.altair.shared.repository.QuestRepository
import kotlinx.datetime.Clock
import kotlinx.datetime.LocalDate
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime

/**
 * SQLite implementation of QuestRepository for mobile platforms.
 *
 * Maps between SQLDelight generated Quest table and domain Quest entities,
 * with WIP=1 enforcement and energy budget tracking.
 */
class SQLiteQuestRepository(database: AltairDatabase) : SQLiteRepository(database), QuestRepository {

    private val queries = database.questQueries
    private val checkpointQueries = database.checkpointQueries
    private val energyQueries = database.energy_budgetQueries

    override suspend fun getById(id: Ulid): AltairResult<Quest> = dbOperation {
        val result = queries.selectById(id.value).executeAsOneOrNull()
        result?.toDomain() ?: throw NoSuchElementException("Quest not found: ${id.value}")
    }.mapLeft { error ->
        if (error is AltairError.StorageError.DatabaseError &&
            error.message.contains("Quest not found")) {
            AltairError.NotFoundError.QuestNotFound(id.value)
        } else {
            error
        }
    }

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Quest>> = dbOperation {
        queries.selectByUserId(userId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getByStatus(userId: Ulid, status: QuestStatus): AltairResult<List<Quest>> = dbOperation {
        queries.selectByUserIdAndStatus(userId.value, status.name).executeAsList().map { it.toDomain() }
    }

    override suspend fun getByEpic(epicId: Ulid): AltairResult<List<Quest>> = dbOperation {
        queries.selectByEpicId(epicId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getActiveQuest(userId: Ulid): AltairResult<Quest?> = dbOperation {
        queries.selectActiveQuestByUserId(userId.value).executeAsOneOrNull()?.toDomain()
    }

    override suspend fun getTodayQuests(userId: Ulid, date: LocalDate): AltairResult<List<Quest>> = dbOperation {
        // For MVP, return all non-deleted quests for the user
        // TODO: Add date-based filtering when quest scheduling is implemented
        queries.selectByUserId(userId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun create(quest: Quest): AltairResult<Quest> = dbOperation {
        queries.insert(
            id = quest.id.value,
            user_id = quest.userId.value,
            title = quest.title,
            description = quest.description,
            energy_cost = quest.energyCost.toLong(),
            status = quest.status.name,
            epic_id = quest.epicId?.value,
            routine_id = quest.routineId?.value,
            created_at = quest.createdAt.toLong(),
            updated_at = quest.updatedAt.toLong(),
            started_at = quest.startedAt.toLongOrNull(),
            completed_at = quest.completedAt.toLongOrNull(),
            deleted_at = quest.deletedAt.toLongOrNull()
        )
        quest
    }

    override suspend fun update(quest: Quest): AltairResult<Quest> = dbOperation {
        queries.update(
            title = quest.title,
            description = quest.description,
            energy_cost = quest.energyCost.toLong(),
            status = quest.status.name,
            epic_id = quest.epicId?.value,
            routine_id = quest.routineId?.value,
            updated_at = quest.updatedAt.toLong(),
            started_at = quest.startedAt.toLongOrNull(),
            completed_at = quest.completedAt.toLongOrNull(),
            id = quest.id.value
        )
        quest
    }

    override suspend fun start(id: Ulid): AltairResult<Quest> = dbOperation {
        // Get the quest first
        val quest = queries.selectById(id.value).executeAsOneOrNull()?.toDomain()
            ?: throw NoSuchElementException("Quest not found: ${id.value}")

        // Check WIP=1 constraint
        val activeQuest = queries.selectActiveQuestByUserId(quest.userId.value).executeAsOneOrNull()
        if (activeQuest != null && activeQuest.id != id.value) {
            return AltairError.ConflictError.WipLimitExceeded(current = 1, limit = 1).left()
        }

        // Update to ACTIVE status
        val now = Clock.System.now()
        val updatedQuest = quest.copy(
            status = QuestStatus.ACTIVE,
            startedAt = now,
            updatedAt = now
        )
        update(updatedQuest).getOrNull()!!
    }

    override suspend fun complete(id: Ulid): AltairResult<Quest> = dbOperation {
        val quest = queries.selectById(id.value).executeAsOneOrNull()?.toDomain()
            ?: throw NoSuchElementException("Quest not found: ${id.value}")

        val now = Clock.System.now()
        val updatedQuest = quest.copy(
            status = QuestStatus.COMPLETED,
            completedAt = now,
            updatedAt = now
        )
        update(updatedQuest).getOrNull()!!
    }

    override suspend fun abandon(id: Ulid): AltairResult<Quest> = dbOperation {
        val quest = queries.selectById(id.value).executeAsOneOrNull()?.toDomain()
            ?: throw NoSuchElementException("Quest not found: ${id.value}")

        val now = Clock.System.now()
        val updatedQuest = quest.copy(
            status = QuestStatus.ABANDONED,
            updatedAt = now
        )
        update(updatedQuest).getOrNull()!!
    }

    override suspend fun backlog(id: Ulid): AltairResult<Quest> = dbOperation {
        val quest = queries.selectById(id.value).executeAsOneOrNull()?.toDomain()
            ?: throw NoSuchElementException("Quest not found: ${id.value}")

        val now = Clock.System.now()
        val updatedQuest = quest.copy(
            status = QuestStatus.BACKLOG,
            updatedAt = now
        )
        update(updatedQuest).getOrNull()!!
    }

    override suspend fun getCheckpoints(questId: Ulid): AltairResult<List<Checkpoint>> = dbOperation {
        checkpointQueries.selectByQuestId(questId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun addCheckpoint(checkpoint: Checkpoint): AltairResult<Checkpoint> = dbOperation {
        checkpointQueries.insert(
            id = checkpoint.id.value,
            quest_id = checkpoint.questId.value,
            title = checkpoint.title,
            completed = checkpoint.completed.toLong(),
            order_ = checkpoint.order.toLong(),
            completed_at = checkpoint.completedAt.toLongOrNull()
        )
        checkpoint
    }

    override suspend fun updateCheckpoint(checkpoint: Checkpoint): AltairResult<Checkpoint> = dbOperation {
        checkpointQueries.update(
            title = checkpoint.title,
            completed = checkpoint.completed.toLong(),
            order_ = checkpoint.order.toLong(),
            completed_at = checkpoint.completedAt.toLongOrNull(),
            id = checkpoint.id.value
        )
        checkpoint
    }

    override suspend fun deleteCheckpoint(id: Ulid): AltairResult<Unit> = dbOperation {
        checkpointQueries.delete(id.value)
    }

    override suspend fun reorderCheckpoints(questId: Ulid, order: List<Ulid>): AltairResult<Unit> = dbOperation {
        order.forEachIndexed { index, checkpointId ->
            checkpointQueries.updateOrder(
                order_ = index.toLong(),
                id = checkpointId.value
            )
        }
    }

    override suspend fun getEnergyBudget(userId: Ulid, date: LocalDate): AltairResult<EnergyBudget> = dbOperation {
        val result = energyQueries.selectByUserIdAndDate(
            userId.value,
            date.toString()
        ).executeAsOneOrNull()

        result?.toDomain() ?: EnergyBudget(
            userId = userId,
            date = date,
            budget = 5, // Default budget
            spent = 0
        )
    }

    override suspend fun setDailyBudget(userId: Ulid, date: LocalDate, budget: Int): AltairResult<EnergyBudget> = dbOperation {
        require(budget in 1..10) { "Budget must be in range 1-10, got $budget" }

        // Get existing or create new
        val existing = energyQueries.selectByUserIdAndDate(
            userId.value,
            date.toString()
        ).executeAsOneOrNull()

        if (existing != null) {
            energyQueries.updateBudget(
                budget = budget.toLong(),
                user_id = userId.value,
                date = date.toString()
            )
        } else {
            energyQueries.insert(
                user_id = userId.value,
                date = date.toString(),
                budget = budget.toLong(),
                spent = 0L
            )
        }

        getEnergyBudget(userId, date).getOrNull()!!
    }

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> = dbOperation {
        val now = Clock.System.now()
        queries.delete(
            deleted_at = now.toLong(),
            updated_at = now.toLong(),
            id = id.value
        )
    }

    override suspend fun restore(id: Ulid): AltairResult<Unit> = dbOperation {
        // SQLDelight doesn't have a restore query, so we need to use update
        // This is a simplified implementation - in production you'd add a restore query
        val quest = queries.selectById(id.value).executeAsOneOrNull()?.toDomain()
            ?: throw NoSuchElementException("Quest not found: ${id.value}")

        val restored = quest.copy(
            deletedAt = null,
            updatedAt = Clock.System.now()
        )
        update(restored)
        Unit
    }

    // Mapper extension functions
    private fun com.getaltair.altair.shared.database.Quest.toDomain(): Quest = Quest(
        id = id.toUlid(),
        userId = user_id.toUlid(),
        title = title,
        description = description,
        energyCost = energy_cost.toInt(),
        status = status.toQuestStatus(),
        epicId = epic_id?.toUlid(),
        routineId = routine_id?.toUlid(),
        createdAt = created_at.toInstant(),
        updatedAt = updated_at.toInstant(),
        startedAt = started_at.toInstantOrNull(),
        completedAt = completed_at.toInstantOrNull(),
        deletedAt = deleted_at.toInstantOrNull()
    )

    private fun com.getaltair.altair.shared.database.Checkpoint.toDomain(): Checkpoint = Checkpoint(
        id = id.toUlid(),
        questId = quest_id.toUlid(),
        title = title,
        completed = completed.toBoolean(),
        order = order_.toInt(),
        completedAt = completed_at.toInstantOrNull()
    )

    private fun com.getaltair.altair.shared.database.Energy_budget.toDomain(): EnergyBudget = EnergyBudget(
        userId = user_id.toUlid(),
        date = LocalDate.parse(date),
        budget = budget.toInt(),
        spent = spent.toInt()
    )
}

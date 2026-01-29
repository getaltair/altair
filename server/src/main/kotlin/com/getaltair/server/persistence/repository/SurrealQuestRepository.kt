package com.getaltair.server.persistence.repository

import arrow.core.Either
import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.guidance.Checkpoint
import com.getaltair.altair.shared.domain.guidance.EnergyBudget
import com.getaltair.altair.shared.domain.guidance.Quest
import com.getaltair.altair.shared.repository.QuestRepository
import com.getaltair.server.auth.AuthContext
import com.getaltair.server.persistence.SurrealDbClient
import kotlinx.datetime.DatePeriod
import kotlinx.datetime.LocalDate
import kotlinx.datetime.plus

/**
 * SurrealDB implementation of [QuestRepository].
 *
 * Provides complete CRUD operations for Quests with WIP=1 enforcement,
 * checkpoint management, and energy budget tracking. All queries are
 * automatically scoped to the authenticated user.
 *
 * ## WIP=1 Enforcement
 *
 * The [start] method uses a SurrealDB transaction to atomically check
 * for existing active quests before allowing a new quest to become active.
 * This ensures the WIP=1 constraint is enforced even under concurrent access.
 *
 * ## Energy Budget
 *
 * Daily energy budgets are stored in a separate `energy_budget` table with
 * a composite key of (user_id, date). The [getEnergyBudget] method returns
 * a default budget if none exists for the requested date.
 *
 * @param db The SurrealDB client for database operations
 * @param auth The authentication context providing current user ID
 */
class SurrealQuestRepository(
    db: SurrealDbClient,
    auth: AuthContext
) : BaseSurrealRepository<Quest>(db, auth, "quest", Quest::class), QuestRepository {

    override fun notFoundError(id: Ulid): AltairError =
        AltairError.NotFoundError.QuestNotFound(id.toString())

    // ========== Core CRUD ==========

    override suspend fun getById(id: Ulid): AltairResult<Quest> =
        findById(id)

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Quest>> =
        findAllForUser()

    override suspend fun create(quest: Quest): AltairResult<Quest> {
        val sql = """
            CREATE quest:${'$'}id CONTENT {
                user_id: ${'$'}userId,
                title: ${'$'}title,
                description: ${'$'}description,
                energy_cost: ${'$'}energyCost,
                status: ${'$'}status,
                epic_id: ${'$'}epicId,
                routine_id: ${'$'}routineId,
                created_at: ${'$'}now,
                updated_at: ${'$'}now,
                started_at: ${'$'}startedAt,
                completed_at: ${'$'}completedAt,
                deleted_at: NONE
            }
        """.trimIndent()

        return executeQueryOne(
            sql,
            mapOf(
                "id" to quest.id.toString(),
                "title" to quest.title,
                "description" to quest.description,
                "energyCost" to quest.energyCost,
                "status" to quest.status.name,
                "epicId" to quest.epicId?.toString(),
                "routineId" to quest.routineId?.toString(),
                "now" to now().toString(),
                "startedAt" to quest.startedAt?.toString(),
                "completedAt" to quest.completedAt?.toString()
            )
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create quest").left()
        }
    }

    override suspend fun update(quest: Quest): AltairResult<Quest> {
        val sql = """
            UPDATE quest:${'$'}id SET
                title = ${'$'}title,
                description = ${'$'}description,
                energy_cost = ${'$'}energyCost,
                status = ${'$'}status,
                epic_id = ${'$'}epicId,
                routine_id = ${'$'}routineId,
                updated_at = ${'$'}now,
                started_at = ${'$'}startedAt,
                completed_at = ${'$'}completedAt
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
        """.trimIndent()

        return executeQueryOne(
            sql,
            mapOf(
                "id" to quest.id.toString(),
                "title" to quest.title,
                "description" to quest.description,
                "energyCost" to quest.energyCost,
                "status" to quest.status.name,
                "epicId" to quest.epicId?.toString(),
                "routineId" to quest.routineId?.toString(),
                "now" to now().toString(),
                "startedAt" to quest.startedAt?.toString(),
                "completedAt" to quest.completedAt?.toString()
            )
        ).flatMap { result ->
            result?.right() ?: notFoundError(quest.id).left()
        }
    }

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> =
        softDeleteEntity(id)

    override suspend fun restore(id: Ulid): AltairResult<Unit> =
        restoreEntity(id)

    // ========== Queries ==========

    override suspend fun getByStatus(userId: Ulid, status: QuestStatus): AltairResult<List<Quest>> =
        findWhere("status = \$status", mapOf("status" to status.name))

    override suspend fun getByEpic(epicId: Ulid): AltairResult<List<Quest>> =
        findWhere("epic_id = \$epicId", mapOf("epicId" to "epic:${epicId}"))

    override suspend fun getActiveQuest(userId: Ulid): AltairResult<Quest?> =
        findOneWhere("status = 'ACTIVE'")

    override suspend fun getTodayQuests(userId: Ulid, date: LocalDate): AltairResult<List<Quest>> {
        // Quests scheduled for a specific date are determined by:
        // - Created on that date, OR
        // - Started on that date, OR
        // - Have a scheduled_date field (future enhancement)
        // For now, return quests that were active/completed on the given date
        val sql = """
            SELECT * FROM quest
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
            AND (
                (status = 'ACTIVE') OR
                (status = 'COMPLETED' AND completed_at >= ${'$'}dayStart AND completed_at < ${'$'}dayEnd) OR
                (started_at >= ${'$'}dayStart AND started_at < ${'$'}dayEnd)
            )
            ORDER BY created_at DESC
        """.trimIndent()

        val dayStart = "${date}T00:00:00Z"
        val dayEnd = "${date.plus(kotlinx.datetime.DatePeriod(days = 1))}T00:00:00Z"

        return executeQuery(sql, mapOf("dayStart" to dayStart, "dayEnd" to dayEnd))
    }

    // ========== Status Transitions ==========

    override suspend fun start(id: Ulid): AltairResult<Quest> {
        // WIP=1 enforcement via atomic transaction
        val sql = """
            BEGIN TRANSACTION;
            LET ${'$'}active = (SELECT * FROM quest WHERE user_id = ${'$'}userId AND status = 'ACTIVE' AND deleted_at IS NONE);
            IF array::len(${'$'}active) > 0 THEN
                RETURN { error: 'WIP_LIMIT_EXCEEDED' };
            END;
            UPDATE quest:${'$'}id SET
                status = 'ACTIVE',
                started_at = time::now(),
                updated_at = time::now()
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE;
            COMMIT TRANSACTION;
            RETURN (SELECT * FROM quest:${'$'}id);
        """.trimIndent()

        return db.queryOne(sql, params("id" to id.toString()), Any::class).flatMap { result ->
            // Check for WIP limit error in result
            when {
                result == null -> notFoundError(id).left()
                result is Map<*, *> && result["error"] == "WIP_LIMIT_EXCEEDED" ->
                    AltairError.ConflictError.WipLimitExceeded(current = 1, limit = 1).left()
                result is Quest -> result.right()
                else -> {
                    // Try to fetch the updated quest directly
                    findById(id)
                }
            }
        }
    }

    override suspend fun complete(id: Ulid): AltairResult<Quest> {
        val sql = """
            UPDATE quest:${'$'}id SET
                status = 'COMPLETED',
                completed_at = time::now(),
                updated_at = time::now()
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
        """.trimIndent()

        return executeQueryOne(sql, mapOf("id" to id.toString()))
            .flatMap { result ->
                result?.right() ?: notFoundError(id).left()
            }
    }

    override suspend fun abandon(id: Ulid): AltairResult<Quest> {
        val sql = """
            UPDATE quest:${'$'}id SET
                status = 'ABANDONED',
                updated_at = time::now()
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
        """.trimIndent()

        return executeQueryOne(sql, mapOf("id" to id.toString()))
            .flatMap { result ->
                result?.right() ?: notFoundError(id).left()
            }
    }

    override suspend fun backlog(id: Ulid): AltairResult<Quest> {
        val sql = """
            UPDATE quest:${'$'}id SET
                status = 'BACKLOG',
                updated_at = time::now()
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
        """.trimIndent()

        return executeQueryOne(sql, mapOf("id" to id.toString()))
            .flatMap { result ->
                result?.right() ?: notFoundError(id).left()
            }
    }

    // ========== Checkpoints ==========

    override suspend fun getCheckpoints(questId: Ulid): AltairResult<List<Checkpoint>> {
        val sql = """
            SELECT * FROM checkpoint
            WHERE quest_id = ${'$'}questId
            ORDER BY order ASC
        """.trimIndent()

        return db.query(sql, mapOf("questId" to "quest:${questId}"), Checkpoint::class)
    }

    override suspend fun addCheckpoint(checkpoint: Checkpoint): AltairResult<Checkpoint> {
        val sql = """
            CREATE checkpoint:${'$'}id CONTENT {
                quest_id: ${'$'}questId,
                title: ${'$'}title,
                completed: ${'$'}completed,
                order: ${'$'}order,
                completed_at: ${'$'}completedAt
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to checkpoint.id.toString(),
                "questId" to "quest:${checkpoint.questId}",
                "title" to checkpoint.title,
                "completed" to checkpoint.completed,
                "order" to checkpoint.order,
                "completedAt" to checkpoint.completedAt?.toString()
            ),
            Checkpoint::class
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create checkpoint").left()
        }
    }

    override suspend fun updateCheckpoint(checkpoint: Checkpoint): AltairResult<Checkpoint> {
        val sql = """
            UPDATE checkpoint:${'$'}id SET
                title = ${'$'}title,
                completed = ${'$'}completed,
                order = ${'$'}order,
                completed_at = ${'$'}completedAt
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to checkpoint.id.toString(),
                "title" to checkpoint.title,
                "completed" to checkpoint.completed,
                "order" to checkpoint.order,
                "completedAt" to checkpoint.completedAt?.toString()
            ),
            Checkpoint::class
        ).flatMap { result ->
            result?.right() ?: AltairError.NotFoundError.QuestNotFound(checkpoint.id.toString()).left()
        }
    }

    override suspend fun deleteCheckpoint(id: Ulid): AltairResult<Unit> =
        db.delete("checkpoint", id.toString())

    override suspend fun reorderCheckpoints(questId: Ulid, order: List<Ulid>): AltairResult<Unit> {
        // Update each checkpoint's order based on position in the list
        order.forEachIndexed { index, checkpointId ->
            val sql = """
                UPDATE checkpoint:${'$'}id SET order = ${'$'}order
                WHERE quest_id = ${'$'}questId
            """.trimIndent()

            val result = db.query(
                sql,
                mapOf(
                    "id" to checkpointId.toString(),
                    "questId" to "quest:${questId}",
                    "order" to index
                ),
                Checkpoint::class
            )

            // If any update fails, return the error
            if (result.isLeft()) {
                return result.map { }
            }
        }

        return Unit.right()
    }

    // ========== Energy Budget ==========

    override suspend fun getEnergyBudget(userId: Ulid, date: LocalDate): AltairResult<EnergyBudget> {
        val sql = """
            SELECT * FROM energy_budget
            WHERE user_id = ${'$'}userId AND date = ${'$'}date
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf("userId" to currentUserId.toString(), "date" to date.toString()),
            EnergyBudget::class
        ).flatMap { result ->
            // Return existing budget or create a default one
            result?.right() ?: EnergyBudget(
                userId = currentUserId,
                date = date,
                budget = 5,  // Default budget
                spent = 0
            ).right()
        }
    }

    override suspend fun setDailyBudget(userId: Ulid, date: LocalDate, budget: Int): AltairResult<EnergyBudget> {
        // Upsert the energy budget for the given date
        val sql = """
            UPSERT energy_budget:[${'$'}userId, ${'$'}date] CONTENT {
                user_id: ${'$'}userId,
                date: ${'$'}date,
                budget: ${'$'}budget,
                spent: (SELECT VALUE spent FROM energy_budget:[${'$'}userId, ${'$'}date])[0] OR 0
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "userId" to currentUserId.toString(),
                "date" to date.toString(),
                "budget" to budget
            ),
            EnergyBudget::class
        ).flatMap { result ->
            result?.right() ?: EnergyBudget(
                userId = currentUserId,
                date = date,
                budget = budget,
                spent = 0
            ).right()
        }
    }
}

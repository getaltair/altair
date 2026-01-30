package com.getaltair.altair.persistence.repository

import arrow.core.Either
import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.persistence.DesktopSurrealDbClient
import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.guidance.Checkpoint
import com.getaltair.altair.shared.domain.guidance.EnergyBudget
import com.getaltair.altair.shared.domain.guidance.Quest
import com.getaltair.altair.shared.repository.QuestRepository
import kotlinx.datetime.DatePeriod
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate
import kotlinx.datetime.plus

/**
 * Desktop implementation of [QuestRepository] for single-user local database.
 *
 * Provides complete CRUD operations for Quests with WIP=1 enforcement,
 * checkpoint management, and energy budget tracking. Unlike the server version,
 * this implementation does not require AuthContext as it operates in a
 * single-user environment.
 *
 * ## WIP=1 Enforcement
 *
 * The [start] method uses a SurrealDB transaction to atomically check
 * for existing active quests before allowing a new quest to become active.
 *
 * ## Energy Budget
 *
 * Daily energy budgets are stored in a separate `energy_budget` table with
 * a date as the key. The [getEnergyBudget] method returns a default budget
 * if none exists for the requested date.
 *
 * @param db The desktop SurrealDB client for database operations
 */
class DesktopQuestRepository(
    private val db: DesktopSurrealDbClient
) : QuestRepository {

    private fun now(): Instant = Instant.fromEpochMilliseconds(System.currentTimeMillis())

    private fun notFoundError(id: Ulid): AltairError =
        AltairError.NotFoundError.QuestNotFound(id.toString())

    // ========== Core CRUD ==========

    override suspend fun getById(id: Ulid): AltairResult<Quest> {
        val sql = """
            SELECT * FROM quest:${'$'}id WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(sql, mapOf("id" to id.toString()), Quest::class)
            .flatMap { result ->
                result?.right() ?: notFoundError(id).left()
            }
    }

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Quest>> {
        val sql = """
            SELECT * FROM quest WHERE deleted_at IS NONE ORDER BY created_at DESC
        """.trimIndent()

        return db.query(sql, emptyMap(), Quest::class)
    }

    override suspend fun create(quest: Quest): AltairResult<Quest> {
        val sql = """
            CREATE quest:${'$'}id CONTENT {
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

        return db.queryOne(
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
            ),
            Quest::class
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
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(
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
            ),
            Quest::class
        ).flatMap { result ->
            result?.right() ?: notFoundError(quest.id).left()
        }
    }

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> {
        val sql = """
            UPDATE quest:${'$'}id SET deleted_at = ${'$'}now WHERE deleted_at IS NONE
        """.trimIndent()

        return db.query(sql, mapOf("id" to id.toString(), "now" to now().toString()), Quest::class)
            .flatMap { results ->
                if (results.isNotEmpty()) Unit.right()
                else notFoundError(id).left()
            }
    }

    override suspend fun restore(id: Ulid): AltairResult<Unit> {
        val sql = """
            UPDATE quest:${'$'}id SET deleted_at = NONE WHERE deleted_at IS NOT NONE
        """.trimIndent()

        return db.query(sql, mapOf("id" to id.toString()), Quest::class)
            .flatMap { results ->
                if (results.isNotEmpty()) Unit.right()
                else notFoundError(id).left()
            }
    }

    // ========== Queries ==========

    override suspend fun getByStatus(userId: Ulid, status: QuestStatus): AltairResult<List<Quest>> {
        val sql = """
            SELECT * FROM quest WHERE status = ${'$'}status AND deleted_at IS NONE
        """.trimIndent()

        return db.query(sql, mapOf("status" to status.name), Quest::class)
    }

    override suspend fun getByEpic(epicId: Ulid): AltairResult<List<Quest>> {
        val sql = """
            SELECT * FROM quest WHERE epic_id = ${'$'}epicId AND deleted_at IS NONE
        """.trimIndent()

        return db.query(sql, mapOf("epicId" to "epic:${epicId}"), Quest::class)
    }

    override suspend fun getActiveQuest(userId: Ulid): AltairResult<Quest?> {
        val sql = """
            SELECT * FROM quest WHERE status = 'ACTIVE' AND deleted_at IS NONE LIMIT 1
        """.trimIndent()

        return db.queryOne(sql, emptyMap(), Quest::class)
            .map { it } // Returns nullable Quest
    }

    override suspend fun getTodayQuests(userId: Ulid, date: LocalDate): AltairResult<List<Quest>> {
        val sql = """
            SELECT * FROM quest
            WHERE deleted_at IS NONE
            AND (
                (status = 'ACTIVE') OR
                (status = 'COMPLETED' AND completed_at >= ${'$'}dayStart AND completed_at < ${'$'}dayEnd) OR
                (started_at >= ${'$'}dayStart AND started_at < ${'$'}dayEnd)
            )
            ORDER BY created_at DESC
        """.trimIndent()

        val dayStart = "${date}T00:00:00Z"
        val dayEnd = "${date.plus(DatePeriod(days = 1))}T00:00:00Z"

        return db.query(sql, mapOf("dayStart" to dayStart, "dayEnd" to dayEnd), Quest::class)
    }

    // ========== Status Transitions ==========

    override suspend fun start(id: Ulid): AltairResult<Quest> {
        // WIP=1 enforcement via atomic transaction
        val sql = """
            BEGIN TRANSACTION;
            LET ${'$'}active = (SELECT * FROM quest WHERE status = 'ACTIVE' AND deleted_at IS NONE);
            IF array::len(${'$'}active) > 0 THEN
                RETURN { error: 'WIP_LIMIT_EXCEEDED' };
            END;
            UPDATE quest:${'$'}id SET
                status = 'ACTIVE',
                started_at = time::now(),
                updated_at = time::now()
            WHERE deleted_at IS NONE;
            COMMIT TRANSACTION;
            RETURN (SELECT * FROM quest:${'$'}id);
        """.trimIndent()

        return db.queryOne(sql, mapOf("id" to id.toString()), Any::class).flatMap { result ->
            when {
                result == null -> notFoundError(id).left()
                result is Map<*, *> && result["error"] == "WIP_LIMIT_EXCEEDED" ->
                    AltairError.ConflictError.WipLimitExceeded(current = 1, limit = 1).left()
                result is Quest -> result.right()
                else -> getById(id) // Fetch the updated quest
            }
        }
    }

    override suspend fun complete(id: Ulid): AltairResult<Quest> {
        val sql = """
            UPDATE quest:${'$'}id SET
                status = 'COMPLETED',
                completed_at = time::now(),
                updated_at = time::now()
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(sql, mapOf("id" to id.toString()), Quest::class)
            .flatMap { result ->
                result?.right() ?: notFoundError(id).left()
            }
    }

    override suspend fun abandon(id: Ulid): AltairResult<Quest> {
        val sql = """
            UPDATE quest:${'$'}id SET
                status = 'ABANDONED',
                updated_at = time::now()
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(sql, mapOf("id" to id.toString()), Quest::class)
            .flatMap { result ->
                result?.right() ?: notFoundError(id).left()
            }
    }

    override suspend fun backlog(id: Ulid): AltairResult<Quest> {
        val sql = """
            UPDATE quest:${'$'}id SET
                status = 'BACKLOG',
                updated_at = time::now()
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(sql, mapOf("id" to id.toString()), Quest::class)
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

    override suspend fun deleteCheckpoint(id: Ulid): AltairResult<Unit> {
        val sql = """
            DELETE checkpoint:${'$'}id
        """.trimIndent()

        return db.query(sql, mapOf("id" to id.toString()), Checkpoint::class)
            .map { Unit }
    }

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
            WHERE date = ${'$'}date
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf("date" to date.toString()),
            EnergyBudget::class
        ).flatMap { result ->
            // Return existing budget or create a default one
            result?.right() ?: EnergyBudget(
                userId = userId,
                date = date,
                budget = 5,  // Default budget
                spent = 0
            ).right()
        }
    }

    override suspend fun setDailyBudget(userId: Ulid, date: LocalDate, budget: Int): AltairResult<EnergyBudget> {
        // Upsert the energy budget for the given date
        val sql = """
            UPSERT energy_budget:${'$'}date CONTENT {
                date: ${'$'}date,
                budget: ${'$'}budget,
                spent: (SELECT VALUE spent FROM energy_budget:${'$'}date)[0] OR 0
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "date" to date.toString(),
                "budget" to budget
            ),
            EnergyBudget::class
        ).flatMap { result ->
            result?.right() ?: EnergyBudget(
                userId = userId,
                date = date,
                budget = budget,
                spent = 0
            ).right()
        }
    }
}

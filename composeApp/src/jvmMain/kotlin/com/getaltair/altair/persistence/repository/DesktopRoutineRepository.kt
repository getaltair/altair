package com.getaltair.altair.persistence.repository

import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.persistence.DesktopSurrealDbClient
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.system.Routine
import com.getaltair.altair.shared.repository.RoutineRepository
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalTime

/**
 * Desktop implementation of the Routine repository.
 *
 * Routines are recurring templates that spawn Quest instances on a schedule.
 * This repository supports the scheduler's needs with due-date queries and
 * next-due timestamp updates.
 */
class DesktopRoutineRepository(
    db: DesktopSurrealDbClient
) : BaseDesktopRepository<Routine>(db, "routine", Routine::class), RoutineRepository {

    override fun notFoundError(id: Ulid): AltairError =
        AltairError.NotFoundError.RoutineNotFound(id.toString())

    override suspend fun getById(id: Ulid): AltairResult<Routine> = findById(id)

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Routine>> = findAll()

    override suspend fun getActiveForUser(userId: Ulid): AltairResult<List<Routine>> =
        findWhere("active = true")

    override suspend fun getDue(before: Instant): AltairResult<List<Routine>> {
        val sql = """
            SELECT * FROM routine
            WHERE active = true
              AND deleted_at IS NONE
              AND next_due IS NOT NONE
              AND next_due <= ${'$'}before
            ORDER BY next_due ASC
        """.trimIndent()

        return db.query(sql, mapOf("before" to before.toString()), entityClass)
    }

    override suspend fun create(routine: Routine): AltairResult<Routine> {
        val sql = """
            CREATE routine:${'$'}id CONTENT {
                user_id: ${'$'}userId,
                name: ${'$'}name,
                description: ${'$'}description,
                schedule: ${'$'}schedule,
                time_of_day: ${'$'}timeOfDay,
                energy_cost: ${'$'}energyCost,
                initiative_id: ${'$'}initiativeId,
                active: ${'$'}active,
                next_due: ${'$'}nextDue,
                created_at: ${'$'}now,
                updated_at: ${'$'}now,
                deleted_at: NONE
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to routine.id.toString(),
                "userId" to routine.userId.toString(),
                "name" to routine.name,
                "description" to routine.description,
                "schedule" to routine.schedule.toSerializedString(),
                "timeOfDay" to routine.timeOfDay?.toString(),
                "energyCost" to routine.energyCost,
                "initiativeId" to routine.initiativeId?.toString(),
                "active" to routine.active,
                "nextDue" to routine.nextDue?.toString(),
                "now" to now().toString()
            ),
            entityClass
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create routine").left()
        }
    }

    override suspend fun update(routine: Routine): AltairResult<Routine> {
        val sql = """
            UPDATE routine:${'$'}id SET
                name = ${'$'}name,
                description = ${'$'}description,
                schedule = ${'$'}schedule,
                time_of_day = ${'$'}timeOfDay,
                energy_cost = ${'$'}energyCost,
                initiative_id = ${'$'}initiativeId,
                active = ${'$'}active,
                next_due = ${'$'}nextDue,
                updated_at = ${'$'}now
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to routine.id.toString(),
                "name" to routine.name,
                "description" to routine.description,
                "schedule" to routine.schedule.toSerializedString(),
                "timeOfDay" to routine.timeOfDay?.toString(),
                "energyCost" to routine.energyCost,
                "initiativeId" to routine.initiativeId?.toString(),
                "active" to routine.active,
                "nextDue" to routine.nextDue?.toString(),
                "now" to now().toString()
            ),
            entityClass
        ).flatMap { result ->
            result?.right() ?: notFoundError(routine.id).left()
        }
    }

    override suspend fun updateNextDue(id: Ulid, nextDue: Instant): AltairResult<Unit> {
        val sql = """
            UPDATE routine:${'$'}id SET
                next_due = ${'$'}nextDue,
                updated_at = ${'$'}now
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.query(
            sql,
            mapOf(
                "id" to id.toString(),
                "nextDue" to nextDue.toString(),
                "now" to now().toString()
            ),
            entityClass
        ).flatMap { results ->
            if (results.isNotEmpty()) {
                Unit.right()
            } else {
                notFoundError(id).left()
            }
        }
    }

    override suspend fun setActive(id: Ulid, active: Boolean): AltairResult<Unit> {
        val sql = """
            UPDATE routine:${'$'}id SET
                active = ${'$'}active,
                updated_at = ${'$'}now
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.query(
            sql,
            mapOf(
                "id" to id.toString(),
                "active" to active,
                "now" to now().toString()
            ),
            entityClass
        ).flatMap { results ->
            if (results.isNotEmpty()) {
                Unit.right()
            } else {
                notFoundError(id).left()
            }
        }
    }

    // softDelete inherited from BaseDesktopRepository
}

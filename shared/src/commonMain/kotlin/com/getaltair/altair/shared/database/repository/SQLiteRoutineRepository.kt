package com.getaltair.altair.shared.database.repository

import com.getaltair.altair.shared.database.AltairDatabase
import com.getaltair.altair.shared.domain.common.Schedule
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.system.Routine
import com.getaltair.altair.shared.repository.RoutineRepository
import kotlinx.datetime.Clock
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalTime

/**
 * SQLite implementation of RoutineRepository for mobile platforms.
 *
 * Stub implementation - full functionality to be implemented in later phases.
 */
class SQLiteRoutineRepository(database: AltairDatabase) : SQLiteRepository(database), RoutineRepository {

    private val queries = database.routineQueries

    override suspend fun getById(id: Ulid): AltairResult<Routine> = dbOperation {
        val result = queries.selectById(id.value).executeAsOneOrNull()
        result?.toDomain() ?: throw NoSuchElementException("Routine not found: ${id.value}")
    }.mapLeft { error ->
        if (error is AltairError.StorageError.DatabaseError &&
            error.message.contains("Routine not found")) {
            AltairError.NotFoundError.RoutineNotFound(id.value)
        } else {
            error
        }
    }

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Routine>> = dbOperation {
        queries.selectByUserId(userId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getActiveForUser(userId: Ulid): AltairResult<List<Routine>> = dbOperation {
        queries.selectActive(userId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getDue(before: Instant): AltairResult<List<Routine>> = dbOperation {
        // Note: selectDue requires user_id, but interface doesn't provide it
        // This is a stub - proper implementation would need interface change
        emptyList()
    }

    override suspend fun create(routine: Routine): AltairResult<Routine> = dbOperation {
        queries.insert(
            id = routine.id.value,
            user_id = routine.userId.value,
            name = routine.name,
            description = routine.description,
            schedule = routine.schedule.name,
            time_of_day = routine.timeOfDay?.toString(),
            energy_cost = routine.energyCost.toLong(),
            initiative_id = routine.initiativeId?.value,
            active = routine.active.toLong(),
            next_due = routine.nextDue?.toLongOrNull(),
            created_at = routine.createdAt.toLong(),
            updated_at = routine.updatedAt.toLong(),
            deleted_at = routine.deletedAt.toLongOrNull()
        )
        routine
    }

    override suspend fun update(routine: Routine): AltairResult<Routine> = dbOperation {
        queries.update(
            name = routine.name,
            description = routine.description,
            schedule = routine.schedule.name,
            time_of_day = routine.timeOfDay?.toString(),
            energy_cost = routine.energyCost.toLong(),
            initiative_id = routine.initiativeId?.value,
            active = routine.active.toLong(),
            updated_at = routine.updatedAt.toLong(),
            id = routine.id.value
        )
        routine
    }

    override suspend fun updateNextDue(id: Ulid, nextDue: Instant): AltairResult<Unit> = dbOperation {
        val now = Clock.System.now()
        queries.updateNextDue(
            next_due = nextDue.toLong(),
            updated_at = now.toLong(),
            id = id.value
        )
    }

    override suspend fun setActive(id: Ulid, active: Boolean): AltairResult<Unit> = dbOperation {
        val now = Clock.System.now()
        queries.toggleActive(
            active = active.toLong(),
            updated_at = now.toLong(),
            id = id.value
        )
    }

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> = dbOperation {
        val now = Clock.System.now()
        queries.softDelete(
            deleted_at = now.toLong(),
            updated_at = now.toLong(),
            id = id.value
        )
    }

    // Mapper extension function
    private fun com.getaltair.altair.shared.database.Routine.toDomain(): Routine = Routine(
        id = id.toUlid(),
        userId = user_id.toUlid(),
        name = name,
        description = description,
        schedule = Schedule.valueOf(schedule),
        timeOfDay = time_of_day?.let { LocalTime.parse(it) },
        energyCost = energy_cost.toInt(),
        initiativeId = initiative_id?.toUlid(),
        active = active.toBoolean(),
        nextDue = next_due?.toInstantOrNull(),
        createdAt = created_at.toInstant(),
        updatedAt = updated_at.toInstant(),
        deletedAt = deleted_at.toInstantOrNull()
    )
}

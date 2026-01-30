package com.getaltair.altair.shared.database.repository

import com.getaltair.altair.shared.database.AltairDatabase
import com.getaltair.altair.shared.domain.common.InitiativeStatus
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.system.Initiative
import com.getaltair.altair.shared.repository.InitiativeRepository
import kotlinx.datetime.Clock
import kotlinx.datetime.LocalDate

/**
 * SQLite implementation of InitiativeRepository for mobile platforms.
 *
 * Stub implementation - full functionality to be implemented in later phases.
 */
class SQLiteInitiativeRepository(database: AltairDatabase) : SQLiteRepository(database), InitiativeRepository {

    private val queries = database.initiativeQueries

    override suspend fun getById(id: Ulid): AltairResult<Initiative> = dbOperation {
        val result = queries.selectById(id.value).executeAsOneOrNull()
        result?.toDomain() ?: throw NoSuchElementException("Initiative not found: ${id.value}")
    }.mapLeft { error ->
        if (error is AltairError.StorageError.DatabaseError &&
            error.message.contains("Initiative not found")) {
            AltairError.NotFoundError.InitiativeNotFound(id.value)
        } else {
            error
        }
    }

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Initiative>> = dbOperation {
        queries.selectByUserId(userId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getChildren(parentId: Ulid): AltairResult<List<Initiative>> = dbOperation {
        queries.selectByParentId(parentId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getFocused(userId: Ulid): AltairResult<Initiative?> = dbOperation {
        queries.selectFocused(userId.value).executeAsOneOrNull()?.toDomain()
    }

    override suspend fun create(initiative: Initiative): AltairResult<Initiative> = dbOperation {
        queries.insert(
            id = initiative.id.value,
            user_id = initiative.userId.value,
            name = initiative.name,
            description = initiative.description,
            parent_id = initiative.parentId?.value,
            ongoing = initiative.ongoing.toLong(),
            target_date = initiative.targetDate?.toString(),
            status = initiative.status.name,
            focused = initiative.focused.toLong(),
            created_at = initiative.createdAt.toLong(),
            updated_at = initiative.updatedAt.toLong(),
            deleted_at = initiative.deletedAt.toLongOrNull()
        )
        initiative
    }

    override suspend fun update(initiative: Initiative): AltairResult<Initiative> = dbOperation {
        queries.update(
            name = initiative.name,
            description = initiative.description,
            parent_id = initiative.parentId?.value,
            ongoing = initiative.ongoing.toLong(),
            target_date = initiative.targetDate?.toString(),
            status = initiative.status.name,
            focused = initiative.focused.toLong(),
            updated_at = initiative.updatedAt.toLong(),
            id = initiative.id.value
        )
        initiative
    }

    override suspend fun setFocused(userId: Ulid, initiativeId: Ulid?): AltairResult<Unit> = dbOperation {
        val now = Clock.System.now()

        // Unfocus all initiatives for this user
        queries.clearFocus(
            updated_at = now.toLong(),
            user_id = userId.value
        )

        // Focus the specified initiative if provided
        if (initiativeId != null) {
            queries.updateFocus(
                focused = 1L,
                updated_at = now.toLong(),
                id = initiativeId.value
            )
        }
    }

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> = dbOperation {
        val now = Clock.System.now()
        queries.softDelete(
            deleted_at = now.toLong(),
            updated_at = now.toLong(),
            id = id.value
        )
    }

    override suspend fun restore(id: Ulid): AltairResult<Unit> = dbOperation {
        // Restore by clearing deleted_at - need to fetch and update
        val initiative = queries.selectById(id.value).executeAsOneOrNull()?.toDomain()
            ?: throw NoSuchElementException("Initiative not found: ${id.value}")

        val restored = initiative.copy(
            deletedAt = null,
            updatedAt = Clock.System.now()
        )
        update(restored)
        Unit
    }

    // Mapper extension function
    private fun com.getaltair.altair.shared.database.Initiative.toDomain(): Initiative = Initiative(
        id = id.toUlid(),
        userId = user_id.toUlid(),
        name = name,
        description = description,
        parentId = parent_id?.toUlid(),
        ongoing = ongoing.toBoolean(),
        targetDate = target_date?.let { LocalDate.parse(it) },
        status = InitiativeStatus.valueOf(status),
        focused = focused.toBoolean(),
        createdAt = created_at.toInstant(),
        updatedAt = updated_at.toInstant(),
        deletedAt = deleted_at.toInstantOrNull()
    )
}

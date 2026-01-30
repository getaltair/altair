package com.getaltair.altair.shared.database.repository

import com.getaltair.altair.shared.database.AltairDatabase
import com.getaltair.altair.shared.domain.common.CaptureSource
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.guidance.Quest
import com.getaltair.altair.shared.domain.knowledge.Note
import com.getaltair.altair.shared.domain.system.InboxItem
import com.getaltair.altair.shared.domain.system.SourceDocument
import com.getaltair.altair.shared.domain.tracking.Item
import com.getaltair.altair.shared.repository.InboxRepository
import kotlinx.datetime.Clock

/**
 * SQLite implementation of InboxRepository for mobile platforms.
 *
 * Stub implementation - full functionality to be implemented in later phases.
 */
class SQLiteInboxRepository(database: AltairDatabase) : SQLiteRepository(database), InboxRepository {

    private val queries = database.inbox_itemQueries

    override suspend fun getById(id: Ulid): AltairResult<InboxItem> = dbOperation {
        val result = queries.selectById(id.value).executeAsOneOrNull()
        result?.toDomain() ?: throw NoSuchElementException("InboxItem not found: ${id.value}")
    }.mapLeft { error ->
        if (error is AltairError.StorageError.DatabaseError &&
            error.message.contains("InboxItem not found")) {
            // Note: Using ItemNotFound as there's no InboxItemNotFound error type
            AltairError.NotFoundError.ItemNotFound(id.value)
        } else {
            error
        }
    }

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<InboxItem>> = dbOperation {
        queries.selectByUserId(userId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun create(item: InboxItem): AltairResult<InboxItem> = dbOperation {
        queries.insert(
            id = item.id.value,
            user_id = item.userId.value,
            content = item.content,
            source = item.source.name,
            attachment_ids = item.attachmentIds.serializeUlids(),
            created_at = item.createdAt.toLong(),
            deleted_at = item.deletedAt.toLongOrNull()
        )
        item
    }

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> = dbOperation {
        val now = Clock.System.now()
        queries.softDelete(
            deleted_at = now.toLong(),
            id = id.value
        )
    }

    override suspend fun triageToQuest(itemId: Ulid, quest: Quest): AltairResult<Ulid> = dbOperation {
        // TODO: Implement triage logic with transaction
        // For now, stub implementation
        softDelete(itemId)
        quest.id
    }

    override suspend fun triageToNote(itemId: Ulid, note: Note): AltairResult<Ulid> = dbOperation {
        // TODO: Implement triage logic with transaction
        // For now, stub implementation
        softDelete(itemId)
        note.id
    }

    override suspend fun triageToItem(itemId: Ulid, item: Item): AltairResult<Ulid> = dbOperation {
        // TODO: Implement triage logic with transaction
        // For now, stub implementation
        softDelete(itemId)
        item.id
    }

    override suspend fun triageToSourceDocument(itemId: Ulid, sourceDoc: SourceDocument): AltairResult<Ulid> = dbOperation {
        // TODO: Implement triage logic with transaction
        // For now, stub implementation
        softDelete(itemId)
        sourceDoc.id
    }

    // Mapper extension function
    private fun com.getaltair.altair.shared.database.Inbox_item.toDomain(): InboxItem = InboxItem(
        id = id.toUlid(),
        userId = user_id.toUlid(),
        content = content,
        source = CaptureSource.valueOf(source),
        attachmentIds = attachment_ids.deserializeUlidList(),
        createdAt = created_at.toInstant(),
        deletedAt = deleted_at.toInstantOrNull()
    )
}

package com.getaltair.altair.persistence.repository

import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.persistence.DesktopSurrealDbClient
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.guidance.Quest
import com.getaltair.altair.shared.domain.knowledge.Note
import com.getaltair.altair.shared.domain.system.InboxItem
import com.getaltair.altair.shared.domain.system.SourceDocument
import com.getaltair.altair.shared.domain.tracking.Item
import com.getaltair.altair.shared.repository.InboxRepository
import kotlinx.datetime.Instant

/**
 * Desktop implementation of the Inbox repository for single-user local database.
 *
 * The Universal Inbox is a type-agnostic capture system that holds unprocessed
 * items until they are triaged into their proper domains. Triage operations
 * are atomic: they create the target entity and soft-delete the inbox item
 * within a single transaction.
 *
 * Unlike the server version, this does not require AuthContext as it operates
 * in a single-user environment.
 *
 * @param db The desktop SurrealDB client for database operations
 */
class DesktopInboxRepository(
    private val db: DesktopSurrealDbClient
) : InboxRepository {

    private fun now(): Instant = Instant.fromEpochMilliseconds(System.currentTimeMillis())

    private fun notFoundError(id: Ulid): AltairError =
        AltairError.NotFoundError.ItemNotFound(id.toString()) // Reusing ItemNotFound for inbox items

    override suspend fun getById(id: Ulid): AltairResult<InboxItem> {
        val sql = """
            SELECT * FROM inbox_item:${'$'}id WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(sql, mapOf("id" to id.toString()), InboxItem::class)
            .flatMap { result ->
                result?.right() ?: notFoundError(id).left()
            }
    }

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<InboxItem>> {
        val sql = """
            SELECT * FROM inbox_item WHERE deleted_at IS NONE ORDER BY created_at DESC
        """.trimIndent()

        return db.query(sql, emptyMap(), InboxItem::class)
    }

    override suspend fun create(item: InboxItem): AltairResult<InboxItem> {
        val sql = """
            CREATE inbox_item:${'$'}id CONTENT {
                content: ${'$'}content,
                source: ${'$'}source,
                attachment_ids: ${'$'}attachmentIds,
                created_at: ${'$'}now,
                deleted_at: NONE
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to item.id.toString(),
                "content" to item.content,
                "source" to item.source.name,
                "attachmentIds" to item.attachmentIds.map { it.toString() },
                "now" to now().toString()
            ),
            InboxItem::class
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create inbox item").left()
        }
    }

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> {
        val sql = """
            UPDATE inbox_item:${'$'}id SET deleted_at = ${'$'}now WHERE deleted_at IS NONE
        """.trimIndent()

        return db.query(sql, mapOf("id" to id.toString(), "now" to now().toString()), InboxItem::class)
            .flatMap { results ->
                if (results.isNotEmpty()) Unit.right()
                else notFoundError(id).left()
            }
    }

    override suspend fun triageToQuest(itemId: Ulid, quest: Quest): AltairResult<Ulid> {
        val sql = """
            BEGIN TRANSACTION;
            LET ${'$'}item = (SELECT * FROM inbox_item WHERE id = inbox_item:${'$'}itemId AND deleted_at IS NONE);
            IF array::len(${'$'}item) == 0 THEN
                RETURN { error: 'NOT_FOUND' };
            END;
            CREATE quest:${'$'}questId CONTENT {
                title: ${'$'}title,
                description: ${'$'}description,
                energy_cost: ${'$'}energyCost,
                status: ${'$'}status,
                epic_id: ${'$'}epicId,
                routine_id: ${'$'}routineId,
                created_at: ${'$'}now,
                updated_at: ${'$'}now,
                started_at: NONE,
                completed_at: NONE,
                deleted_at: NONE
            };
            UPDATE inbox_item:${'$'}itemId SET deleted_at = ${'$'}now;
            COMMIT TRANSACTION;
            RETURN ${'$'}questId;
        """.trimIndent()

        return executeTransactionalTriage(
            sql,
            mapOf(
                "itemId" to itemId.toString(),
                "questId" to quest.id.toString(),
                "title" to quest.title,
                "description" to quest.description,
                "energyCost" to quest.energyCost,
                "status" to quest.status.name,
                "epicId" to quest.epicId?.toString(),
                "routineId" to quest.routineId?.toString(),
                "now" to now().toString()
            ),
            itemId,
            quest.id
        )
    }

    override suspend fun triageToNote(itemId: Ulid, note: Note): AltairResult<Ulid> {
        val sql = """
            BEGIN TRANSACTION;
            LET ${'$'}item = (SELECT * FROM inbox_item WHERE id = inbox_item:${'$'}itemId AND deleted_at IS NONE);
            IF array::len(${'$'}item) == 0 THEN
                RETURN { error: 'NOT_FOUND' };
            END;
            CREATE note:${'$'}noteId CONTENT {
                title: ${'$'}title,
                content: ${'$'}content,
                folder_id: ${'$'}folderId,
                initiative_id: ${'$'}initiativeId,
                embedding: ${'$'}embedding,
                created_at: ${'$'}now,
                updated_at: ${'$'}now,
                deleted_at: NONE
            };
            UPDATE inbox_item:${'$'}itemId SET deleted_at = ${'$'}now;
            COMMIT TRANSACTION;
            RETURN ${'$'}noteId;
        """.trimIndent()

        return executeTransactionalTriage(
            sql,
            mapOf(
                "itemId" to itemId.toString(),
                "noteId" to note.id.toString(),
                "title" to note.title,
                "content" to note.content,
                "folderId" to note.folderId?.toString(),
                "initiativeId" to note.initiativeId?.toString(),
                "embedding" to note.embedding,
                "now" to now().toString()
            ),
            itemId,
            note.id
        )
    }

    override suspend fun triageToItem(itemId: Ulid, item: Item): AltairResult<Ulid> {
        val sql = """
            BEGIN TRANSACTION;
            LET ${'$'}inboxItem = (SELECT * FROM inbox_item WHERE id = inbox_item:${'$'}itemId AND deleted_at IS NONE);
            IF array::len(${'$'}inboxItem) == 0 THEN
                RETURN { error: 'NOT_FOUND' };
            END;
            CREATE item:${'$'}newItemId CONTENT {
                name: ${'$'}name,
                description: ${'$'}description,
                quantity: ${'$'}quantity,
                template_id: ${'$'}templateId,
                location_id: ${'$'}locationId,
                container_id: ${'$'}containerId,
                initiative_id: ${'$'}initiativeId,
                image_key: ${'$'}imageKey,
                created_at: ${'$'}now,
                updated_at: ${'$'}now,
                deleted_at: NONE
            };
            UPDATE inbox_item:${'$'}itemId SET deleted_at = ${'$'}now;
            COMMIT TRANSACTION;
            RETURN ${'$'}newItemId;
        """.trimIndent()

        return executeTransactionalTriage(
            sql,
            mapOf(
                "itemId" to itemId.toString(),
                "newItemId" to item.id.toString(),
                "name" to item.name,
                "description" to item.description,
                "quantity" to item.quantity,
                "templateId" to item.templateId?.toString(),
                "locationId" to item.locationId?.toString(),
                "containerId" to item.containerId?.toString(),
                "initiativeId" to item.initiativeId?.toString(),
                "imageKey" to item.imageKey,
                "now" to now().toString()
            ),
            itemId,
            item.id
        )
    }

    override suspend fun triageToSourceDocument(itemId: Ulid, sourceDoc: SourceDocument): AltairResult<Ulid> {
        val sql = """
            BEGIN TRANSACTION;
            LET ${'$'}item = (SELECT * FROM inbox_item WHERE id = inbox_item:${'$'}itemId AND deleted_at IS NONE);
            IF array::len(${'$'}item) == 0 THEN
                RETURN { error: 'NOT_FOUND' };
            END;
            CREATE source_document:${'$'}docId CONTENT {
                title: ${'$'}title,
                source_type: ${'$'}sourceType,
                source_path: ${'$'}sourcePath,
                mime_type: ${'$'}mimeType,
                content_hash: ${'$'}contentHash,
                extracted_text: ${'$'}extractedText,
                embedding: ${'$'}embedding,
                status: ${'$'}status,
                error_message: ${'$'}errorMessage,
                initiative_id: ${'$'}initiativeId,
                watched_folder_id: ${'$'}watchedFolderId,
                last_synced_at: ${'$'}lastSyncedAt,
                created_at: ${'$'}now,
                updated_at: ${'$'}now,
                deleted_at: NONE
            };
            UPDATE inbox_item:${'$'}itemId SET deleted_at = ${'$'}now;
            COMMIT TRANSACTION;
            RETURN ${'$'}docId;
        """.trimIndent()

        return executeTransactionalTriage(
            sql,
            mapOf(
                "itemId" to itemId.toString(),
                "docId" to sourceDoc.id.toString(),
                "title" to sourceDoc.title,
                "sourceType" to sourceDoc.sourceType.name,
                "sourcePath" to sourceDoc.sourcePath,
                "mimeType" to sourceDoc.mimeType,
                "contentHash" to sourceDoc.contentHash,
                "extractedText" to sourceDoc.extractedText,
                "embedding" to sourceDoc.embedding,
                "status" to sourceDoc.status.name,
                "errorMessage" to sourceDoc.errorMessage,
                "initiativeId" to sourceDoc.initiativeId?.toString(),
                "watchedFolderId" to sourceDoc.watchedFolderId?.toString(),
                "lastSyncedAt" to sourceDoc.lastSyncedAt?.toString(),
                "now" to now().toString()
            ),
            itemId,
            sourceDoc.id
        )
    }

    /**
     * Executes a transactional triage operation.
     *
     * SurrealDB transactions either complete fully or roll back on error.
     * This helper handles the common pattern of checking for NOT_FOUND
     * error indicator and returning the created entity's ID.
     */
    private suspend fun executeTransactionalTriage(
        sql: String,
        params: Map<String, Any?>,
        itemId: Ulid,
        targetId: Ulid
    ): AltairResult<Ulid> {
        return db.queryOne(sql, params, Any::class).flatMap { result ->
            when {
                result == null -> notFoundError(itemId).left()
                result is Map<*, *> && result["error"] == "NOT_FOUND" -> notFoundError(itemId).left()
                else -> targetId.right() // Transaction succeeded
            }
        }.mapLeft { error ->
            // Check if this is a not-found error from our transaction
            if (error is AltairError.StorageError.DatabaseError && error.message.contains("NOT_FOUND")) {
                notFoundError(itemId)
            } else {
                error
            }
        }
    }
}

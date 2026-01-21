@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.knowledge.Folder
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.FolderNode
import com.getaltair.altair.repository.FolderRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlin.time.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

class SurrealFolderRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : FolderRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<DomainError, Folder> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM folder WHERE id = folder:\$id AND user_id = user:\$userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).bind()
            parseFolder(result) ?: raise(DomainError.NotFoundError("Folder", id.value))
        }

    override suspend fun save(entity: Folder): Either<DomainError, Folder> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .executeBind(
                        """
                        UPDATE folder:${'$'}id SET
                            name = ${'$'}name,
                            parent_id = ${'$'}parentId,
                            sort_order = ${'$'}sortOrder,
                            updated_at = time::now()
                        WHERE user_id = user:${'$'}userId;
                        """.trimIndent(),
                        mapOf(
                            "id" to entity.id.value,
                            "name" to entity.name,
                            "parentId" to entity.parentId?.let { "folder:${it.value}" },
                            "sortOrder" to entity.sortOrder,
                            "userId" to userId.value,
                        ),
                    ).bind()
            } else {
                db
                    .executeBind(
                        """
                        CREATE folder:${'$'}id CONTENT {
                            user_id: user:${'$'}userId,
                            name: ${'$'}name,
                            parent_id: ${'$'}parentId,
                            sort_order: ${'$'}sortOrder
                        };
                        """.trimIndent(),
                        mapOf(
                            "id" to entity.id.value,
                            "userId" to userId.value,
                            "name" to entity.name,
                            "parentId" to entity.parentId?.let { "folder:${it.value}" },
                            "sortOrder" to entity.sortOrder,
                        ),
                    ).bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE folder:\$id SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:\$userId;",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).bind()
        }

    override fun findAll(): Flow<List<Folder>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM folder WHERE user_id = user:\$userId AND deleted_at IS NONE ORDER BY sort_order",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseFolders(it) }))
        }

    override fun findRoots(): Flow<List<Folder>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM folder WHERE user_id = user:\$userId AND parent_id IS NONE AND deleted_at IS NONE ORDER BY sort_order",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseFolders(it) }))
        }

    override fun findByParent(parentId: Ulid): Flow<List<Folder>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM folder WHERE user_id = user:\$userId AND parent_id = folder:\$parentId AND deleted_at IS NONE ORDER BY sort_order",
                    mapOf("userId" to userId.value, "parentId" to parentId.value),
                )
            emit(result.fold({ emptyList() }, { parseFolders(it) }))
        }

    override fun findTree(): Flow<List<FolderNode>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM folder WHERE user_id = user:\$userId AND deleted_at IS NONE ORDER BY sort_order",
                    mapOf("userId" to userId.value),
                )
            val folders = result.fold({ emptyList() }, { parseFolders(it) })
            emit(buildTree(folders))
        }

    override suspend fun getPath(id: Ulid): Either<DomainError, List<Folder>> =
        either {
            val path = mutableListOf<Folder>()
            var current: Folder? = findById(id).bind()
            while (current != null) {
                path.add(0, current)
                current = current.parentId?.let { findById(it).getOrNull() }
            }
            path
        }

    override suspend fun move(
        id: Ulid,
        newParentId: Ulid?,
    ): Either<DomainError, Folder> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE folder:\$id SET parent_id = \$parentId, updated_at = time::now() WHERE user_id = user:\$userId;",
                    mapOf("id" to id.value, "parentId" to newParentId?.let { "folder:${it.value}" }, "userId" to userId.value),
                ).bind()
            findById(id).bind()
        }

    override suspend fun reorder(
        parentId: Ulid?,
        orderedIds: List<Ulid>,
    ): Either<DomainError, Unit> =
        either {
            orderedIds.forEachIndexed { index, id ->
                db
                    .executeBind(
                        "UPDATE folder:\$id SET sort_order = \$sortOrder, updated_at = time::now() WHERE user_id = user:\$userId;",
                        mapOf("id" to id.value, "sortOrder" to index, "userId" to userId.value),
                    ).bind()
            }
        }

    private fun buildTree(folders: List<Folder>): List<FolderNode> {
        val byParent = folders.groupBy { it.parentId }

        fun buildNodes(parentId: Ulid?): List<FolderNode> =
            byParent[parentId]?.map { folder ->
                FolderNode(folder, buildNodes(folder.id))
            } ?: emptyList()
        return buildNodes(null)
    }

    private fun parseFolder(result: String): Folder? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToFolder(it) }
        } catch (e: Exception) {
            null
        }

    private fun parseFolders(result: String): List<Folder> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToFolder(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
            emptyList()
        }

    private fun mapToFolder(obj: kotlinx.serialization.json.JsonObject): Folder {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return Folder(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            name = obj["name"]?.jsonPrimitive?.content ?: "",
            parentId =
                obj["parent_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            sortOrder = obj["sort_order"]?.jsonPrimitive?.content?.toIntOrNull() ?: 0,
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
            deletedAt = obj["deleted_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
        )
    }

    private fun parseInstant(value: String?): Instant =
        value?.let {
            try {
                Instant.parse(it)
            } catch (e: Exception) {
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}

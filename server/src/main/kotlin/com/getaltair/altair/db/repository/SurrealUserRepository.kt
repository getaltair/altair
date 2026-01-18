@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.ensure
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.UserError
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import com.getaltair.altair.repository.UserRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory

/**
 * SurrealDB implementation of UserRepository.
 *
 * Note: UserRepository is NOT user-scoped because it's used for admin operations
 * and user lookup during authentication.
 */
class SurrealUserRepository(
    private val db: SurrealDbClient,
) : UserRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<UserError, User> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM user WHERE id = user:${id.value} AND deleted_at IS NONE",
                    ).mapLeft { UserError.NotFound(id) }
                    .bind()

            parseUser(result) ?: raise(UserError.NotFound(id))
        }

    override suspend fun findByEmail(email: String): Either<UserError, User> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM user WHERE email = '${email.replace("'", "''")}' AND deleted_at IS NONE",
                    ).mapLeft { UserError.EmailNotFound }
                    .bind()

            parseUser(result) ?: raise(UserError.EmailNotFound)
        }

    override suspend fun create(user: User): Either<UserError, User> =
        either {
            // Check if email already exists
            val existing = findByEmail(user.email)
            ensure(existing.isLeft()) {
                UserError.EmailAlreadyExists
            }

            db
                .execute(
                    """
                    CREATE user:${user.id.value} CONTENT {
                        email: '${user.email.replace("'", "''")}',
                        display_name: '${user.displayName.replace("'", "''")}',
                        role: '${user.role.name.lowercase()}',
                        status: '${user.status.name.lowercase()}',
                        storage_used_bytes: ${user.storageUsedBytes},
                        storage_quota_bytes: ${user.storageQuotaBytes}
                    };
                    """.trimIndent(),
                ).mapLeft { UserError.NotFound(user.id) }
                .bind()

            findById(user.id).bind()
        }

    override suspend fun update(user: User): Either<UserError, User> =
        either {
            // Verify user exists
            findById(user.id).bind()

            db
                .execute(
                    """
                    UPDATE user:${user.id.value} SET
                        email = '${user.email.replace("'", "''")}',
                        display_name = '${user.displayName.replace("'", "''")}',
                        role = '${user.role.name.lowercase()}',
                        status = '${user.status.name.lowercase()}',
                        storage_used_bytes = ${user.storageUsedBytes},
                        storage_quota_bytes = ${user.storageQuotaBytes},
                        updated_at = time::now();
                    """.trimIndent(),
                ).mapLeft { UserError.NotFound(user.id) }
                .bind()

            findById(user.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<UserError, Unit> =
        either {
            findById(id).bind()

            db
                .execute(
                    """
                    UPDATE user:${id.value} SET
                        deleted_at = time::now(),
                        updated_at = time::now();
                    """.trimIndent(),
                ).mapLeft { UserError.NotFound(id) }
                .bind()
        }

    override fun findAll(): Flow<List<User>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM user WHERE deleted_at IS NONE ORDER BY created_at DESC",
                )
            emit(result.fold({ emptyList() }, { parseUsers(it) }))
        }

    override fun findByRole(role: UserRole): Flow<List<User>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM user WHERE role = '${role.name.lowercase()}' AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseUsers(it) }))
        }

    override fun findByStatus(status: UserStatus): Flow<List<User>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM user WHERE status = '${status.name.lowercase()}' AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseUsers(it) }))
        }

    override suspend fun updateStorageUsed(
        id: Ulid,
        bytesUsed: Long,
    ): Either<UserError, User> =
        either {
            val user = findById(id).bind()

            ensure(bytesUsed <= user.storageQuotaBytes) {
                UserError.StorageQuotaExceeded(bytesUsed, user.storageQuotaBytes)
            }

            db
                .execute(
                    """
                    UPDATE user:${id.value} SET
                        storage_used_bytes = $bytesUsed,
                        updated_at = time::now();
                    """.trimIndent(),
                ).mapLeft { UserError.NotFound(id) }
                .bind()

            findById(id).bind()
        }

    override suspend fun isEmailAvailable(email: String): Either<UserError, Boolean> =
        either {
            findByEmail(email).fold(
                ifLeft = { true },
                ifRight = { false },
            )
        }

    override suspend fun countActive(): Either<UserError, Int> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT count() FROM user WHERE status = 'active' AND deleted_at IS NONE GROUP ALL",
                    ).mapLeft { UserError.NotFound(Ulid.generate()) }
                    .bind()

            parseCount(result)
        }

    private fun parseUser(result: String): User? {
        return try {
            val array = json.parseToJsonElement(result).jsonArray
            val obj = array.firstOrNull()?.jsonObject ?: return null
            mapToUser(obj)
        } catch (e: Exception) {
            logger.warn("Failed to parse user: ${e.message}", e)
            null
        }
    }

    private fun parseUsers(result: String): List<User> =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            array.mapNotNull { element ->
                try {
                    mapToUser(element.jsonObject)
                } catch (e: Exception) {
                    logger.warn("Failed to parse user element: ${e.message}", e)
                    null
                }
            }
        } catch (e: Exception) {
            logger.warn("Failed to parse users array: ${e.message}", e)
            emptyList()
        }

    private fun mapToUser(obj: kotlinx.serialization.json.JsonObject): User {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException("Missing id")
        return User(
            id = Ulid(id),
            email = obj["email"]?.jsonPrimitive?.content ?: throw IllegalStateException("Missing email"),
            displayName =
                obj["display_name"]?.jsonPrimitive?.content ?: throw IllegalStateException("Missing display_name"),
            role = UserRole.valueOf(obj["role"]?.jsonPrimitive?.content?.uppercase() ?: "MEMBER"),
            status = UserStatus.valueOf(obj["status"]?.jsonPrimitive?.content?.uppercase() ?: "ACTIVE"),
            storageUsedBytes = obj["storage_used_bytes"]?.jsonPrimitive?.content?.toLongOrNull() ?: 0L,
            storageQuotaBytes =
                obj["storage_quota_bytes"]?.jsonPrimitive?.content?.toLongOrNull() ?: 10_737_418_240L,
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
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST

    private fun parseCount(result: String): Int =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            val obj = array.firstOrNull()?.jsonObject
            obj
                ?.get("count")
                ?.jsonPrimitive
                ?.content
                ?.toIntOrNull() ?: 0
        } catch (e: Exception) {
            logger.warn("Failed to parse count: ${e.message}", e)
            0
        }

    companion object {
        private val logger = LoggerFactory.getLogger(SurrealUserRepository::class.java)
    }
}

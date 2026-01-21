@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.ensure
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.UserError
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.model.system.UserWithCredentials
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import com.getaltair.altair.repository.UserRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory
import kotlin.time.Instant

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
                    .queryBind(
                        "SELECT * FROM user WHERE id = user:${'$'}id AND deleted_at IS NONE",
                        mapOf("id" to id.value),
                    ).mapLeft { error ->
                        logger.warn("Database error in findById for ${id.value}: ERROR_PLACEHOLDER (converting to NotFound)")
                        UserError.NotFound(id)
                    }.bind()

            parseUser(result) ?: raise(UserError.NotFound(id))
        }

    override suspend fun findByEmail(email: String): Either<UserError, User> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM user WHERE email = ${'$'}email AND deleted_at IS NONE",
                        mapOf("email" to email),
                    ).mapLeft { error ->
                        logger.warn("Database error in findByEmail for $email: ERROR_PLACEHOLDER (converting to EmailNotFound)")
                        UserError.EmailNotFound
                    }.bind()

            parseUser(result) ?: raise(UserError.EmailNotFound)
        }

    override suspend fun findByEmailWithCredentials(email: String): Either<UserError, UserWithCredentials> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM user WHERE email = ${'$'}email AND deleted_at IS NONE",
                        mapOf("email" to email),
                    ).mapLeft { error ->
                        logger.warn("Database error in findByEmailWithCredentials for $email: ERROR_PLACEHOLDER (converting to EmailNotFound)")
                        UserError.EmailNotFound
                    }.bind()

            parseUserWithCredentials(result) ?: raise(UserError.EmailNotFound)
        }

    override suspend fun findByIdWithCredentials(id: Ulid): Either<UserError, UserWithCredentials> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM user WHERE id = user:${'$'}id AND deleted_at IS NONE",
                        mapOf("id" to id.value),
                    ).mapLeft { error ->
                        logger.warn("Database error in findByIdWithCredentials for ${id.value}: ERROR_PLACEHOLDER (converting to NotFound)")
                        UserError.NotFound(id)
                    }.bind()

            parseUserWithCredentials(result) ?: raise(UserError.NotFound(id))
        }

    override suspend fun create(user: User): Either<UserError, User> =
        either {
            // Check if email already exists
            val existing = findByEmail(user.email)
            ensure(existing.isLeft()) {
                UserError.EmailAlreadyExists
            }

            db
                .executeBind(
                    """
                    CREATE user:${user.id.value} CONTENT {
                        email: ${'$'}email,
                        display_name: ${'$'}displayName,
                        role: ${'$'}role,
                        status: ${'$'}status,
                        storage_used_bytes: ${'$'}storageUsed,
                        storage_quota_bytes: ${'$'}storageQuota
                    };
                    """.trimIndent(),
                    mapOf(
                        "email" to user.email,
                        "displayName" to user.displayName,
                        "role" to user.role.name.lowercase(),
                        "status" to user.status.name.lowercase(),
                        "storageUsed" to user.storageUsedBytes,
                        "storageQuota" to user.storageQuotaBytes,
                    ),
                ).mapLeft { error ->
                    logger.warn("Database error creating user ${user.id.value}: ERROR_PLACEHOLDER (converting to NotFound)")
                    UserError.NotFound(user.id)
                }.bind()

            findById(user.id).bind()
        }

    override suspend fun createWithPassword(
        user: User,
        passwordHash: String,
    ): Either<UserError, User> =
        either {
            // Check if email already exists
            val existing = findByEmail(user.email)
            ensure(existing.isLeft()) {
                UserError.EmailAlreadyExists
            }

            db
                .executeBind(
                    """
                    CREATE user:${user.id.value} CONTENT {
                        email: ${'$'}email,
                        display_name: ${'$'}displayName,
                        role: ${'$'}role,
                        status: ${'$'}status,
                        storage_used_bytes: ${'$'}storageUsed,
                        storage_quota_bytes: ${'$'}storageQuota,
                        password_hash: ${'$'}passwordHash
                    };
                    """.trimIndent(),
                    mapOf(
                        "email" to user.email,
                        "displayName" to user.displayName,
                        "role" to user.role.name.lowercase(),
                        "status" to user.status.name.lowercase(),
                        "storageUsed" to user.storageUsedBytes,
                        "storageQuota" to user.storageQuotaBytes,
                        "passwordHash" to passwordHash,
                    ),
                ).mapLeft { error ->
                    logger.warn("Database error creating user with password ${user.id.value}: ERROR_PLACEHOLDER (converting to NotFound)")
                    UserError.NotFound(user.id)
                }.bind()

            findById(user.id).bind()
        }

    override suspend fun updatePassword(
        id: Ulid,
        passwordHash: String,
    ): Either<UserError, Unit> =
        either {
            findById(id).bind()

            db
                .executeBind(
                    """
                    UPDATE user:${id.value} SET
                        password_hash = ${'$'}passwordHash,
                        updated_at = time::now();
                    """.trimIndent(),
                    mapOf("passwordHash" to passwordHash),
                ).mapLeft { error ->
                    logger.warn("Database error in updatePassword for ${id.value}: ERROR_PLACEHOLDER (converting to NotFound)")
                    UserError.NotFound(id)
                }.bind()
        }

    override suspend fun update(user: User): Either<UserError, User> =
        either {
            // Verify user exists
            findById(user.id).bind()

            val result =
                db
                    .queryBind(
                        """
                        UPDATE user:${user.id.value} SET
                            email = ${'$'}email,
                            display_name = ${'$'}displayName,
                            role = ${'$'}role,
                            status = ${'$'}status,
                            storage_used_bytes = ${'$'}storageUsed,
                            storage_quota_bytes = ${'$'}storageQuota,
                            updated_at = time::now()
                        RETURN AFTER;
                        """.trimIndent(),
                        mapOf(
                            "email" to user.email,
                            "displayName" to user.displayName,
                            "role" to user.role.name.lowercase(),
                            "status" to user.status.name.lowercase(),
                            "storageUsed" to user.storageUsedBytes,
                            "storageQuota" to user.storageQuotaBytes,
                        ),
                    ).mapLeft { error ->
                        logger.warn("Database error updating user ${user.id.value}: ERROR_PLACEHOLDER (converting to NotFound)")
                        UserError.NotFound(user.id)
                    }.bind()

            parseUser(result) ?: raise(UserError.NotFound(user.id))
        }

    override suspend fun delete(id: Ulid): Either<UserError, Unit> =
        either {
            findById(id).bind()

            db
                .executeBind(
                    """
                    UPDATE user:${id.value} SET
                        deleted_at = time::now(),
                        updated_at = time::now();
                    """.trimIndent(),
                    emptyMap(),
                ).mapLeft { error ->
                    logger.warn("Database error in delete for ${id.value}: ERROR_PLACEHOLDER (converting to NotFound)")
                    UserError.NotFound(id)
                }.bind()
        }

    override fun findAll(): Flow<List<User>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM user WHERE deleted_at IS NONE ORDER BY created_at DESC",
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error: ERROR_PLACEHOLDER")

                            is DomainError.UnexpectedError -> logger.warn("Database error: ERROR_PLACEHOLDER")

                            is DomainError.NotFoundError -> logger.warn("Database error: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error: ${error.field} - ERROR_PLACEHOLDER")

                            is DomainError.UnauthorizedError -> logger.warn("Database error: ERROR_PLACEHOLDER")

                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseUsers(it) },
                ),
            )
        }

    override fun findByRole(role: UserRole): Flow<List<User>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM user WHERE role = ${'$'}role AND deleted_at IS NONE",
                    mapOf("role" to role.name.lowercase()),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error: ERROR_PLACEHOLDER")

                            is DomainError.UnexpectedError -> logger.warn("Database error: ERROR_PLACEHOLDER")

                            is DomainError.NotFoundError -> logger.warn("Database error: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error: ${error.field} - ERROR_PLACEHOLDER")

                            is DomainError.UnauthorizedError -> logger.warn("Database error: ERROR_PLACEHOLDER")

                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseUsers(it) },
                ),
            )
        }

    override fun findByStatus(status: UserStatus): Flow<List<User>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM user WHERE status = ${'$'}status AND deleted_at IS NONE",
                    mapOf("status" to status.name.lowercase()),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error: ERROR_PLACEHOLDER")

                            is DomainError.UnexpectedError -> logger.warn("Database error: ERROR_PLACEHOLDER")

                            is DomainError.NotFoundError -> logger.warn("Database error: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error: ${error.field} - ERROR_PLACEHOLDER")

                            is DomainError.UnauthorizedError -> logger.warn("Database error: ERROR_PLACEHOLDER")

                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseUsers(it) },
                ),
            )
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

            val result =
                db
                    .queryBind(
                        """
                        UPDATE user:${id.value} SET
                            storage_used_bytes = ${'$'}bytesUsed,
                            updated_at = time::now()
                        RETURN AFTER;
                        """.trimIndent(),
                        mapOf("bytesUsed" to bytesUsed),
                    ).mapLeft { error ->
                        logger.warn("Database error in updateStorageUsed for ${id.value}: ERROR_PLACEHOLDER (converting to NotFound)")
                        UserError.NotFound(id)
                    }.bind()

            parseUser(result) ?: raise(UserError.NotFound(id))
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
                    ).mapLeft { error ->
                        logger.warn("Database error in countActive: ERROR_PLACEHOLDER (converting to NotFound)")
                        UserError.NotFound(Ulid.generate())
                    }.bind()

            parseCount(result)
        }

    private fun parseUser(result: String): User? {
        return try {
            val array = json.parseToJsonElement(result).jsonArray
            val obj = array.firstOrNull()?.jsonObject ?: return null
            mapToUser(obj)
        } catch (e: SerializationException) {
            logger.warn("Failed to parse user: ${e.message}", e)
            null
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse user: ${e.message}", e)
            null
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse user: ${e.message}", e)
            null
        }
    }

    @Suppress("ReturnCount") // Parsing requires multiple validation points
    private fun parseUserWithCredentials(result: String): UserWithCredentials? =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            val obj = array.firstOrNull()?.jsonObject ?: return null
            val user = mapToUser(obj)
            val passwordHash =
                obj["password_hash"]?.jsonPrimitive?.content
                    ?: return null // User has no password set
            UserWithCredentials(user, passwordHash)
        } catch (e: SerializationException) {
            logger.warn("Failed to parse user with credentials: ${e.message}", e)
            null
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse user with credentials: ${e.message}", e)
            null
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse user with credentials: ${e.message}", e)
            null
        }

    private fun parseUsers(result: String): List<User> =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            array.mapNotNull { element ->
                try {
                    mapToUser(element.jsonObject)
                } catch (e: SerializationException) {
                    logger.warn("Failed to parse user element: ${e.message}", e)
                    null
                } catch (e: IllegalStateException) {
                    logger.warn("Failed to parse user element: ${e.message}", e)
                    null
                } catch (e: IllegalArgumentException) {
                    logger.warn("Failed to parse user element: ${e.message}", e)
                    null
                }
            }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse users array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse users array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalArgumentException) {
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
            } catch (e: SerializationException) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            } catch (e: IllegalStateException) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            } catch (e: IllegalArgumentException) {
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
        } catch (e: SerializationException) {
            logger.warn("Failed to parse count: ${e.message}", e)
            0
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse count: ${e.message}", e)
            0
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse count: ${e.message}", e)
            0
        }

    companion object {
        private val logger = LoggerFactory.getLogger(SurrealUserRepository::class.java)
    }
}

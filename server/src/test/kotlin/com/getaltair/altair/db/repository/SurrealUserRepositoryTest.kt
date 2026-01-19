package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbTestContainer
import com.getaltair.altair.domain.UserError
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.AfterAll
import org.junit.jupiter.api.BeforeAll
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestInstance
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertIs
import kotlin.test.assertTrue
import kotlin.time.Clock

@Testcontainers
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class SurrealUserRepositoryTest {
    private lateinit var dbClient: SurrealDbClient
    private lateinit var repository: SurrealUserRepository

    @BeforeAll
    fun setupContainer() {
        container.start()
        runBlocking {
            val config = container.createNetworkConfig()
            dbClient = SurrealDbClient(config)
            dbClient.connect().getOrNull()

            // Run migrations
            val migrationRunner = MigrationRunner(dbClient)
            migrationRunner.runMigrations()
        }
    }

    @AfterAll
    fun tearDown() {
        runBlocking {
            dbClient.close()
        }
        container.stop()
    }

    @BeforeEach
    fun setup() {
        repository = SurrealUserRepository(dbClient)
        // Clean up users before each test
        runBlocking {
            dbClient.execute("DELETE user;")
        }
    }

    @Test
    fun `create creates new user`(): Unit =
        runBlocking {
            val user = createTestUser()

            val result = repository.create(user)

            assertTrue(result.isRight())
            result.onRight { saved ->
                assertEquals(user.id, saved.id)
                assertEquals(user.email, saved.email)
                assertEquals(user.displayName, saved.displayName)
            }
        }

    @Test
    fun `create rejects duplicate email`(): Unit =
        runBlocking {
            val user1 = createTestUser(email = "duplicate@test.com")
            repository.create(user1)

            val user2 = createTestUser(email = "duplicate@test.com")
            val result = repository.create(user2)

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<UserError.EmailAlreadyExists>(error)
            }
        }

    @Test
    fun `findByEmail returns user when exists`(): Unit =
        runBlocking {
            val user = createTestUser(email = "find@test.com")
            repository.create(user)

            val result = repository.findByEmail("find@test.com")

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(user.email, found.email)
            }
        }

    @Test
    fun `findByEmail returns error when not found`(): Unit =
        runBlocking {
            val result = repository.findByEmail("nonexistent@test.com")

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<UserError.EmailNotFound>(error)
            }
        }

    @Test
    fun `isEmailAvailable returns true for unused email`(): Unit =
        runBlocking {
            val result = repository.isEmailAvailable("available@test.com")

            assertTrue(result.isRight())
            result.onRight { available ->
                assertTrue(available)
            }
        }

    @Test
    fun `isEmailAvailable returns false for used email`(): Unit =
        runBlocking {
            val user = createTestUser(email = "taken@test.com")
            repository.create(user)

            val result = repository.isEmailAvailable("taken@test.com")

            assertTrue(result.isRight())
            result.onRight { available ->
                assertFalse(available)
            }
        }

    @Test
    fun `updateStorageUsed enforces storage quota`(): Unit =
        runBlocking {
            val smallQuota = 1000L // 1KB quota
            val user = createTestUser(storageQuotaBytes = smallQuota)
            repository.create(user)

            // Try to update storage used beyond quota
            val result = repository.updateStorageUsed(user.id, smallQuota + 1)

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<UserError.StorageQuotaExceeded>(error)
                assertEquals(smallQuota + 1, error.currentUsage)
                assertEquals(smallQuota, error.quota)
            }
        }

    @Test
    fun `updateStorageUsed succeeds within quota`(): Unit =
        runBlocking {
            val quota = 10_000L
            val user = createTestUser(storageQuotaBytes = quota)
            repository.create(user)

            val result = repository.updateStorageUsed(user.id, quota - 1000)

            assertTrue(result.isRight())
            result.onRight { updated ->
                assertEquals(quota - 1000, updated.storageUsedBytes)
            }
        }

    @Test
    fun `updateStorageUsed succeeds at exactly quota`(): Unit =
        runBlocking {
            val quota = 10_000L
            val user = createTestUser(storageQuotaBytes = quota)
            repository.create(user)

            val result = repository.updateStorageUsed(user.id, quota)

            assertTrue(result.isRight())
            result.onRight { updated ->
                assertEquals(quota, updated.storageUsedBytes)
            }
        }

    @Test
    fun `findByRole returns only matching users`(): Unit =
        runBlocking {
            val admin = createTestUser(email = "admin@test.com", role = UserRole.ADMIN)
            val member = createTestUser(email = "member@test.com", role = UserRole.MEMBER)
            repository.create(admin)
            repository.create(member)

            val admins = repository.findByRole(UserRole.ADMIN).first()

            assertEquals(1, admins.size)
            assertEquals(UserRole.ADMIN, admins.first().role)
        }

    @Test
    fun `findByStatus returns only matching users`(): Unit =
        runBlocking {
            val active = createTestUser(email = "active@test.com", status = UserStatus.ACTIVE)
            val suspended = createTestUser(email = "suspended@test.com", status = UserStatus.DISABLED)
            repository.create(active)
            repository.create(suspended)

            val activeUsers = repository.findByStatus(UserStatus.ACTIVE).first()

            assertEquals(1, activeUsers.size)
            assertEquals(UserStatus.ACTIVE, activeUsers.first().status)
        }

    @Test
    fun `delete soft deletes user`(): Unit =
        runBlocking {
            val user = createTestUser()
            repository.create(user)

            val deleteResult = repository.delete(user.id)
            assertTrue(deleteResult.isRight())

            val findResult = repository.findById(user.id)
            assertTrue(findResult.isLeft())
        }

    @Test
    fun `countActive returns correct count`(): Unit =
        runBlocking {
            repository.create(createTestUser(email = "active1@test.com", status = UserStatus.ACTIVE))
            repository.create(createTestUser(email = "active2@test.com", status = UserStatus.ACTIVE))
            repository.create(createTestUser(email = "suspended@test.com", status = UserStatus.DISABLED))

            val result = repository.countActive()

            assertTrue(result.isRight())
            result.onRight { count ->
                assertEquals(2, count)
            }
        }

    private fun createTestUser(
        email: String = "test@test.com",
        role: UserRole = UserRole.MEMBER,
        status: UserStatus = UserStatus.ACTIVE,
        storageQuotaBytes: Long = DEFAULT_QUOTA_BYTES,
    ): User {
        val now = Clock.System.now()
        return User(
            id = Ulid.generate(),
            email = email,
            displayName = "Test User",
            role = role,
            status = status,
            storageUsedBytes = 0L,
            storageQuotaBytes = storageQuotaBytes,
            createdAt = now,
            updatedAt = now,
            deletedAt = null,
        )
    }

    companion object {
        @Container
        val container = SurrealDbTestContainer()

        private const val DEFAULT_QUOTA_BYTES = 10_737_418_240L // 10GB
    }
}

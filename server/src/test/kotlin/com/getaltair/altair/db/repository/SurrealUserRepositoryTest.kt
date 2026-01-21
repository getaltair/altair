package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbContainerExtension
import com.getaltair.altair.domain.UserError
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.booleans.shouldBeFalse
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf
import kotlinx.coroutines.flow.first
import kotlin.time.Clock

/**
 * Tests for SurrealUserRepository using Testcontainers.
 *
 * Verifies:
 * - CRUD operations (create, findByEmail, findById, update, delete)
 * - Email uniqueness constraints
 * - Storage quota enforcement
 * - Query operations (findByRole, findByStatus, countActive)
 * - Status transitions (ACTIVE ↔ DISABLED)
 */
class SurrealUserRepositoryTest :
    BehaviorSpec({
        lateinit var dbClient: SurrealDbClient
        lateinit var repository: SurrealUserRepository

        beforeSpec {
            val config = SurrealDbContainerExtension.createNetworkConfig()
            dbClient = SurrealDbClient(config)
            dbClient.connect().getOrNull()

            // Run migrations
            val migrationRunner = MigrationRunner(dbClient)
            migrationRunner.runMigrations()
        }

        afterSpec {
            dbClient.close()
        }

        beforeEach {
            repository = SurrealUserRepository(dbClient)
            // Clean up users before each test
            dbClient.execute("DELETE user;")
        }

        given("CRUD operations") {
            `when`("creating a new user") {
                then("creates the user successfully") {
                    val user = createTestUser()

                    val result = repository.create(user)

                    result.shouldBeRight()
                    val saved = result.getOrNull()
                    saved?.id shouldBe user.id
                    saved?.email shouldBe user.email
                    saved?.displayName shouldBe user.displayName
                }
            }

            `when`("finding a user by email") {
                then("returns the user when exists") {
                    val user = createTestUser(email = "find@test.com")
                    repository.create(user)

                    val result = repository.findByEmail("find@test.com")

                    result.shouldBeRight()
                    val found = result.getOrNull()
                    found?.email shouldBe user.email
                }

                then("returns error when not found") {
                    val result = repository.findByEmail("nonexistent@test.com")

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<UserError.EmailNotFound>()
                }
            }

            `when`("updating a user") {
                then("persists changes") {
                    val user = createTestUser(email = "original@test.com", status = UserStatus.ACTIVE)
                    repository.create(user)

                    val updatedUser =
                        user.copy(
                            email = "updated@test.com",
                            displayName = "Updated Name",
                            status = UserStatus.DISABLED,
                        )
                    val result = repository.update(updatedUser)

                    result.shouldBeRight()
                    val updated = result.getOrNull()
                    updated?.email shouldBe "updated@test.com"
                    updated?.displayName shouldBe "Updated Name"
                    updated?.status shouldBe UserStatus.DISABLED
                }

                then("returns NotFound for non-existent user") {
                    val nonExistentUser = createTestUser()

                    val result = repository.update(nonExistentUser)

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<UserError.NotFound>()
                }
            }

            `when`("deleting a user") {
                then("soft deletes the user") {
                    val user = createTestUser()
                    repository.create(user)

                    val deleteResult = repository.delete(user.id)
                    deleteResult.shouldBeRight()

                    val findResult = repository.findById(user.id)
                    findResult.shouldBeLeft()
                }
            }
        }

        given("email uniqueness") {
            `when`("creating a user with duplicate email") {
                then("rejects the creation") {
                    val user1 = createTestUser(email = "duplicate@test.com")
                    repository.create(user1)

                    val user2 = createTestUser(email = "duplicate@test.com")
                    val result = repository.create(user2)

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<UserError.EmailAlreadyExists>()
                }
            }

            `when`("checking email availability") {
                then("returns true for unused email") {
                    val result = repository.isEmailAvailable("available@test.com")

                    result.shouldBeRight()
                    val available = result.getOrNull() ?: false
                    available.shouldBeTrue()
                }

                then("returns false for used email") {
                    val user = createTestUser(email = "taken@test.com")
                    repository.create(user)

                    val result = repository.isEmailAvailable("taken@test.com")

                    result.shouldBeRight()
                    val available = result.getOrNull() ?: true
                    available.shouldBeFalse()
                }
            }
        }

        given("storage quota enforcement") {
            `when`("updating storage beyond quota") {
                then("rejects the update") {
                    val smallQuota = 1000L // 1KB quota
                    val user = createTestUser(storageQuotaBytes = smallQuota)
                    repository.create(user)

                    val result = repository.updateStorageUsed(user.id, smallQuota + 1)

                    result.shouldBeLeft()
                    val error = result.leftOrNull()
                    error.shouldBeInstanceOf<UserError.StorageQuotaExceeded>()
                    (error as UserError.StorageQuotaExceeded).currentUsage shouldBe smallQuota + 1
                    error.quota shouldBe smallQuota
                }
            }

            `when`("updating storage within quota") {
                then("succeeds") {
                    val quota = 10_000L
                    val user = createTestUser(storageQuotaBytes = quota)
                    repository.create(user)

                    val result = repository.updateStorageUsed(user.id, quota - 1000)

                    result.shouldBeRight()
                    val updated = result.getOrNull()
                    updated?.storageUsedBytes shouldBe quota - 1000
                }
            }

            `when`("updating storage to exactly quota") {
                then("succeeds") {
                    val quota = 10_000L
                    val user = createTestUser(storageQuotaBytes = quota)
                    repository.create(user)

                    val result = repository.updateStorageUsed(user.id, quota)

                    result.shouldBeRight()
                    val updated = result.getOrNull()
                    updated?.storageUsedBytes shouldBe quota
                }
            }
        }

        given("query operations") {
            `when`("finding users by role") {
                then("returns only matching users") {
                    val admin = createTestUser(email = "admin@test.com", role = UserRole.ADMIN)
                    val member = createTestUser(email = "member@test.com", role = UserRole.MEMBER)
                    repository.create(admin)
                    repository.create(member)

                    val admins = repository.findByRole(UserRole.ADMIN).first()

                    admins shouldHaveSize 1
                    admins.first().role shouldBe UserRole.ADMIN
                }
            }

            `when`("finding users by status") {
                then("returns only matching users") {
                    val active = createTestUser(email = "active@test.com", status = UserStatus.ACTIVE)
                    val suspended = createTestUser(email = "suspended@test.com", status = UserStatus.DISABLED)
                    repository.create(active)
                    repository.create(suspended)

                    val activeUsers = repository.findByStatus(UserStatus.ACTIVE).first()

                    activeUsers shouldHaveSize 1
                    activeUsers.first().status shouldBe UserStatus.ACTIVE
                }
            }

            `when`("counting active users") {
                then("returns correct count") {
                    repository.create(createTestUser(email = "active1@test.com", status = UserStatus.ACTIVE))
                    repository.create(createTestUser(email = "active2@test.com", status = UserStatus.ACTIVE))
                    repository.create(createTestUser(email = "suspended@test.com", status = UserStatus.DISABLED))

                    val result = repository.countActive()

                    result.shouldBeRight()
                    val count = result.getOrNull() ?: 0
                    count shouldBe 2
                }
            }
        }

        given("status transitions") {
            `when`("changing status from ACTIVE to DISABLED") {
                then("persists the change") {
                    val user = createTestUser(email = "suspend@test.com", status = UserStatus.ACTIVE)
                    repository.create(user)

                    val updatedUser = user.copy(status = UserStatus.DISABLED)
                    val result = repository.update(updatedUser)

                    result.shouldBeRight()
                    val updated = result.getOrNull()
                    updated?.status shouldBe UserStatus.DISABLED

                    // Verify by fetching again
                    val fetchResult = repository.findById(user.id)
                    fetchResult.shouldBeRight()
                    val fetched = fetchResult.getOrNull()
                    fetched?.status shouldBe UserStatus.DISABLED
                }
            }

            `when`("changing status from DISABLED to ACTIVE") {
                then("persists the change") {
                    val user = createTestUser(email = "restore@test.com", status = UserStatus.DISABLED)
                    repository.create(user)

                    val updatedUser = user.copy(status = UserStatus.ACTIVE)
                    val result = repository.update(updatedUser)

                    result.shouldBeRight()
                    val updated = result.getOrNull()
                    updated?.status shouldBe UserStatus.ACTIVE

                    // Verify by fetching again
                    val fetchResult = repository.findById(user.id)
                    fetchResult.shouldBeRight()
                    val fetched = fetchResult.getOrNull()
                    fetched?.status shouldBe UserStatus.ACTIVE
                }
            }
        }
    }) {
    companion object {
        private const val DEFAULT_QUOTA_BYTES = 10_737_418_240L // 10GB

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
    }
}

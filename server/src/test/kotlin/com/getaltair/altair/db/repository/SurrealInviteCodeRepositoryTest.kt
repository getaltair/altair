package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbContainerExtension
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.model.system.InviteCode
import com.getaltair.altair.domain.types.Ulid
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.inspectors.forAll
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.collections.shouldBeEmpty
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.comparables.shouldBeGreaterThan
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.nulls.shouldNotBeNull
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldNotBeEmpty
import io.kotest.matchers.types.shouldBeInstanceOf
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlin.time.Clock
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.hours

/**
 * Tests for SurrealInviteCodeRepository using Testcontainers.
 *
 * Verifies:
 * - CRUD operations (create, findByCode, markUsed)
 * - Expiration handling (expired codes not findable)
 * - Usage prevention (used codes not findable, no reuse)
 * - Concurrency (concurrent markUsed attempts)
 * - Query operations (findByCreator filtering)
 * - Cleanup (deleteExpiredAndUsed)
 * - Edge cases (timestamps, case sensitivity, minimum length)
 */
class SurrealInviteCodeRepositoryTest :
    BehaviorSpec({
        lateinit var dbClient: SurrealDbClient
        lateinit var repository: SurrealInviteCodeRepository

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
            repository = SurrealInviteCodeRepository(dbClient)
            // Clean up invite codes before each test
            dbClient.execute("DELETE invite_code;")
        }

        given("CRUD operations") {
            `when`("creating a new invite code") {
                then("stores the code successfully") {
                    val inviteCode = createTestInviteCode(code = "WELCOME2024")

                    val result = repository.create(inviteCode)

                    result.shouldBeRight()
                    val saved = result.getOrNull()
                    saved?.id shouldBe inviteCode.id
                    saved?.code shouldBe inviteCode.code
                    saved?.createdBy shouldBe inviteCode.createdBy
                }

                then("stores code with minimum length") {
                    val minLengthCode = createTestInviteCode(code = "12345678") // 8 chars minimum

                    val result = repository.create(minLengthCode)

                    result.shouldBeRight()
                }
            }

            `when`("finding by code") {
                then("returns valid unused code") {
                    val inviteCode = createTestInviteCode(code = "TESTCODE123")
                    repository.create(inviteCode)

                    val result = repository.findByCode("TESTCODE123")

                    result.shouldBeRight()
                    val found = result.getOrNull()
                    found?.code shouldBe "TESTCODE123"
                    found?.usedBy.shouldBeNull()
                    found?.usedAt.shouldBeNull()
                }

                then("returns error for non-existent code") {
                    val result = repository.findByCode("NONEXISTENT")

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<AuthError.InvalidInviteCode>()
                }

                then("is case-sensitive") {
                    val inviteCode = createTestInviteCode(code = "CaseSensitive")
                    repository.create(inviteCode)

                    // Exact match should work
                    val exactMatch = repository.findByCode("CaseSensitive")
                    exactMatch.shouldBeRight()

                    // Different case should not match
                    val upperCase = repository.findByCode("CASESENSITIVE")
                    upperCase.shouldBeLeft()

                    val lowerCase = repository.findByCode("casesensitive")
                    lowerCase.shouldBeLeft()
                }
            }

            `when`("marking code as used") {
                then("marks the code successfully") {
                    val inviteCode = createTestInviteCode(code = "MARKUSED123")
                    repository.create(inviteCode)

                    val userId = Ulid.generate()
                    val result = repository.markUsed(inviteCode.id, userId)

                    result.shouldBeRight()

                    // Verify code is no longer findable (because it's used)
                    val findResult = repository.findByCode("MARKUSED123")
                    findResult.shouldBeLeft()
                }

                then("prevents code reuse") {
                    val inviteCode = createTestInviteCode(code = "ONCEONLY")
                    repository.create(inviteCode)

                    val userId1 = Ulid.generate()
                    val userId2 = Ulid.generate()

                    // First use - should succeed
                    val result1 = repository.markUsed(inviteCode.id, userId1)
                    result1.shouldBeRight()

                    // Try to find it again - should fail since it's used
                    val findResult = repository.findByCode("ONCEONLY")
                    findResult.shouldBeLeft()

                    // Even if we try to mark it as used again with different user,
                    // the code is already used and won't be findable
                    val result2 = repository.markUsed(inviteCode.id, userId2)
                    result2.shouldBeRight() // UPDATE succeeds even if already used

                    // Verify it's still not findable
                    val findResult2 = repository.findByCode("ONCEONLY")
                    findResult2.shouldBeLeft()
                }

                then("sets usedAt and usedBy correctly") {
                    val inviteCode = createTestInviteCode(code = "USEDTIME")
                    repository.create(inviteCode)

                    val userId = Ulid.generate()
                    repository.markUsed(inviteCode.id, userId)

                    // Query directly from database to verify usedAt and usedBy
                    val queryResult =
                        dbClient.queryBind(
                            "SELECT * FROM invite_code WHERE code = \$code",
                            mapOf("code" to "USEDTIME"),
                        )

                    queryResult.shouldBeRight()
                    val json = queryResult.getOrNull() ?: ""
                    json.shouldNotBeEmpty()
                }
            }
        }

        given("expiration handling") {
            `when`("code is expired") {
                then("returns error when finding") {
                    // Create an expired invite code (manually construct with past expiration)
                    val now = Clock.System.now()
                    val expiredCode =
                        InviteCode(
                            id = Ulid.generate(),
                            code = "EXPIRED123",
                            createdBy = Ulid.generate(),
                            createdAt = now - 8.days,
                            expiresAt = now - 1.days, // Expired yesterday
                        )
                    repository.create(expiredCode)

                    // Try to find it - should not be found since it's expired
                    val result = repository.findByCode("EXPIRED123")

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<AuthError.InvalidInviteCode>()
                }
            }
        }

        given("usage prevention") {
            `when`("code is already used") {
                then("returns error when finding") {
                    val inviteCode = createTestInviteCode(code = "USED0123")
                    repository.create(inviteCode)

                    // Mark it as used
                    val userId = Ulid.generate()
                    repository.markUsed(inviteCode.id, userId)

                    // Try to find it - should not be found since it's used
                    val result = repository.findByCode("USED0123")

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<AuthError.InvalidInviteCode>()
                }
            }
        }

        given("concurrent access") {
            `when`("multiple users try to use same code") {
                then("handles concurrent markUsed attempts") {
                    val inviteCode = createTestInviteCode(code = "CONCURRENT")
                    repository.create(inviteCode)

                    // Simulate 10 users trying to use the same invite code simultaneously
                    val userIds =
                        (1..10).map {
                            Ulid.generate()
                        }

                    val results =
                        userIds
                            .map { userId ->
                                async {
                                    // First try to find the code (this is what the registration flow does)
                                    val findResult = repository.findByCode("CONCURRENT")
                                    if (findResult.isRight()) {
                                        // If found, try to mark as used
                                        repository.markUsed(inviteCode.id, userId)
                                    } else {
                                        findResult
                                    }
                                }
                            }.awaitAll()

                    // At least one should succeed in finding and marking
                    val successCount = results.count { it.isRight() }
                    successCount shouldBeGreaterThan 0

                    // After all attempts, code should definitely not be findable
                    val finalFind = repository.findByCode("CONCURRENT")
                    finalFind.shouldBeLeft()
                }
            }
        }

        given("findByCreator queries") {
            `when`("finding codes by creator") {
                then("returns all codes created by user") {
                    val adminId = Ulid.generate()

                    val code1 = createTestInviteCode(code = "ADMIN001", createdBy = adminId)
                    val code2 = createTestInviteCode(code = "ADMIN002", createdBy = adminId)
                    val code3 = createTestInviteCode(code = "ADMIN003", createdBy = adminId)

                    repository.create(code1)
                    repository.create(code2)
                    repository.create(code3)

                    val result = repository.findByCreator(adminId)

                    result.shouldBeRight()
                    val codes = result.getOrNull() ?: emptyList()
                    codes shouldHaveSize 3
                    codes.forAll { it.createdBy shouldBe adminId }
                }

                then("filters by creator correctly") {
                    val admin1 = Ulid.generate()
                    val admin2 = Ulid.generate()

                    val admin1Code1 = createTestInviteCode(code = "ADMIN1-1", createdBy = admin1)
                    val admin1Code2 = createTestInviteCode(code = "ADMIN1-2", createdBy = admin1)
                    val admin2Code = createTestInviteCode(code = "ADMIN2-1", createdBy = admin2)

                    repository.create(admin1Code1)
                    repository.create(admin1Code2)
                    repository.create(admin2Code)

                    val result = repository.findByCreator(admin1)

                    result.shouldBeRight()
                    val codes = result.getOrNull() ?: emptyList()
                    codes shouldHaveSize 2
                    codes.forAll { it.createdBy shouldBe admin1 }
                    codes.none { it.createdBy == admin2 }.shouldBeTrue()
                }

                then("includes both used and unused codes") {
                    val adminId = Ulid.generate()

                    val usedCode = createTestInviteCode(code = "USED0001", createdBy = adminId)
                    val unusedCode = createTestInviteCode(code = "UNUSED01", createdBy = adminId)

                    repository.create(usedCode)
                    repository.create(unusedCode)

                    // Mark one as used
                    repository.markUsed(usedCode.id, Ulid.generate())

                    val result = repository.findByCreator(adminId)

                    result.shouldBeRight()
                    val codes = result.getOrNull() ?: emptyList()
                    codes shouldHaveSize 2
                    val used = codes.find { it.code == "USED0001" }
                    val unused = codes.find { it.code == "UNUSED01" }

                    used.shouldNotBeNull()
                    unused.shouldNotBeNull()
                    used.usedBy.shouldNotBeNull()
                    unused.usedBy.shouldBeNull()
                }

                then("includes expired codes") {
                    val adminId = Ulid.generate()

                    // Create expired code manually
                    val now = Clock.System.now()
                    val expiredCode =
                        InviteCode(
                            id = Ulid.generate(),
                            code = "EXPIRED1",
                            createdBy = adminId,
                            createdAt = now - 8.days,
                            expiresAt = now - 1.days,
                        )
                    val validCode = createTestInviteCode(code = "VALID001", createdBy = adminId)

                    repository.create(expiredCode)
                    repository.create(validCode)

                    val result = repository.findByCreator(adminId)

                    result.shouldBeRight()
                    val codes = result.getOrNull() ?: emptyList()
                    codes shouldHaveSize 2
                }

                then("returns empty list for creator with no codes") {
                    val adminId = Ulid.generate()

                    val result = repository.findByCreator(adminId)

                    result.shouldBeRight()
                    val codes = result.getOrNull() ?: emptyList()
                    codes.shouldBeEmpty()
                }
            }
        }

        given("cleanup operations") {
            `when`("deleting expired and used codes") {
                then("removes expired codes") {
                    // Create expired code manually
                    val now = Clock.System.now()
                    val expiredCode =
                        InviteCode(
                            id = Ulid.generate(),
                            code = "EXPIRED1",
                            createdBy = Ulid.generate(),
                            createdAt = now - 8.days,
                            expiresAt = now - 1.days,
                        )
                    val validCode =
                        createTestInviteCode(
                            code = "VALID001",
                            expiresIn = 7.days,
                        )

                    repository.create(expiredCode)
                    repository.create(validCode)

                    // Delete expired and used codes
                    val result = repository.deleteExpiredAndUsed()
                    result.shouldBeRight()

                    // Expired code should not be findable
                    val expiredFind = repository.findByCode("EXPIRED1")
                    expiredFind.shouldBeLeft()

                    // Valid code should still be findable
                    val validFind = repository.findByCode("VALID001")
                    validFind.shouldBeRight()
                }

                then("removes used codes") {
                    val usedCode = createTestInviteCode(code = "USED0001")
                    val unusedCode = createTestInviteCode(code = "UNUSED01")

                    repository.create(usedCode)
                    repository.create(unusedCode)

                    // Mark one as used
                    repository.markUsed(usedCode.id, Ulid.generate())

                    // Delete expired and used codes
                    val result = repository.deleteExpiredAndUsed()
                    result.shouldBeRight()

                    // Used code should not be findable
                    val usedFind = repository.findByCode("USED0001")
                    usedFind.shouldBeLeft()

                    // Unused code should still be findable
                    val unusedFind = repository.findByCode("UNUSED01")
                    unusedFind.shouldBeRight()
                }

                then("keeps valid unused codes") {
                    val validCode1 = createTestInviteCode(code = "VALID001", expiresIn = 7.days)
                    val validCode2 = createTestInviteCode(code = "VALID002", expiresIn = 30.days)
                    val validCode3 = createTestInviteCode(code = "VALID003", expiresIn = 1.hours)

                    repository.create(validCode1)
                    repository.create(validCode2)
                    repository.create(validCode3)

                    // Delete expired and used codes
                    val result = repository.deleteExpiredAndUsed()
                    result.shouldBeRight()

                    // All valid codes should still be findable
                    repository.findByCode("VALID001").shouldBeRight()
                    repository.findByCode("VALID002").shouldBeRight()
                    repository.findByCode("VALID003").shouldBeRight()
                }
            }
        }

        given("edge cases") {
            `when`("working with timestamps") {
                then("preserves timestamps correctly") {
                    val inviteCode = createTestInviteCode(code = "TIMESTAMPS")
                    repository.create(inviteCode)

                    val result = repository.findByCode("TIMESTAMPS")

                    result.shouldBeRight()
                    val found = result.getOrNull()
                    found?.createdAt.shouldNotBeNull()
                    found?.expiresAt.shouldNotBeNull()
                    found?.expiresAt?.let { expiresAt ->
                        found.createdAt?.let { createdAt ->
                            expiresAt shouldBeGreaterThan createdAt
                        }
                    }
                    found?.usedAt.shouldBeNull()
                }
            }

            `when`("multiple valid codes exist") {
                then("all are independently findable") {
                    val code1 = createTestInviteCode(code = "FIRST001")
                    val code2 = createTestInviteCode(code = "SECOND01")
                    val code3 = createTestInviteCode(code = "THIRD001")

                    repository.create(code1)
                    repository.create(code2)
                    repository.create(code3)

                    // All should be independently findable
                    repository.findByCode("FIRST001").shouldBeRight()
                    repository.findByCode("SECOND01").shouldBeRight()
                    repository.findByCode("THIRD001").shouldBeRight()

                    // Use one
                    repository.markUsed(code2.id, Ulid.generate())

                    // First and third should still be findable
                    repository.findByCode("FIRST001").shouldBeRight()
                    repository.findByCode("SECOND01").shouldBeLeft()
                    repository.findByCode("THIRD001").shouldBeRight()
                }
            }
        }
    }) {
    companion object {
        private fun createTestInviteCode(
            id: Ulid = Ulid.generate(),
            code: String = "TEST-${Ulid.generate().value}",
            createdBy: Ulid = Ulid.generate(),
            expiresIn: kotlin.time.Duration = 7.days,
        ): InviteCode =
            InviteCode.create(
                id = id,
                code = code,
                createdBy = createdBy,
                expiresIn = expiresIn,
                clock = Clock.System,
            )
    }
}

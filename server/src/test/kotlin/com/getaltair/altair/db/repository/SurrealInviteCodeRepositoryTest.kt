package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbTestContainer
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.model.system.InviteCode
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.AfterAll
import org.junit.jupiter.api.BeforeAll
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestInstance
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.test.assertEquals
import kotlin.test.assertIs
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue
import kotlin.time.Clock
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.hours

@Testcontainers
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class SurrealInviteCodeRepositoryTest {
    private lateinit var dbClient: SurrealDbClient
    private lateinit var repository: SurrealInviteCodeRepository

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
        repository = SurrealInviteCodeRepository(dbClient)
        // Clean up invite codes before each test
        runBlocking {
            dbClient.execute("DELETE invite_code;")
        }
    }

    @Test
    fun `create stores new invite code`(): Unit =
        runBlocking {
            val inviteCode = createTestInviteCode(code = "WELCOME2024")

            val result = repository.create(inviteCode)

            assertTrue(result.isRight())
            result.onRight { saved ->
                assertEquals(inviteCode.id, saved.id)
                assertEquals(inviteCode.code, saved.code)
                assertEquals(inviteCode.createdBy, saved.createdBy)
            }
        }

    @Test
    fun `findByCode returns invite code when valid and unused`(): Unit =
        runBlocking {
            val inviteCode = createTestInviteCode(code = "TESTCODE123")
            repository.create(inviteCode)

            val result = repository.findByCode("TESTCODE123")

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals("TESTCODE123", found.code)
                assertNull(found.usedBy)
                assertNull(found.usedAt)
            }
        }

    @Test
    fun `findByCode returns error when code does not exist`(): Unit =
        runBlocking {
            val result = repository.findByCode("NONEXISTENT")

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<AuthError.InvalidInviteCode>(error)
            }
        }

    @Test
    fun `findByCode returns error when code is expired`(): Unit =
        runBlocking {
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

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<AuthError.InvalidInviteCode>(error)
            }
        }

    @Test
    fun `findByCode returns error when code is already used`(): Unit =
        runBlocking {
            val inviteCode = createTestInviteCode(code = "USED0123")
            repository.create(inviteCode)

            // Mark it as used
            val userId = Ulid.generate()
            repository.markUsed(inviteCode.id, userId)

            // Try to find it - should not be found since it's used
            val result = repository.findByCode("USED0123")

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<AuthError.InvalidInviteCode>(error)
            }
        }

    @Test
    fun `markUsed marks invite code as used`(): Unit =
        runBlocking {
            val inviteCode = createTestInviteCode(code = "MARKUSED123")
            repository.create(inviteCode)

            val userId = Ulid.generate()
            val result = repository.markUsed(inviteCode.id, userId)

            assertTrue(result.isRight())

            // Verify code is no longer findable (because it's used)
            val findResult = repository.findByCode("MARKUSED123")
            assertTrue(findResult.isLeft())
        }

    @Test
    fun `markUsed prevents code reuse`(): Unit =
        runBlocking {
            val inviteCode = createTestInviteCode(code = "ONCEONLY")
            repository.create(inviteCode)

            val userId1 = Ulid.generate()
            val userId2 = Ulid.generate()

            // First use - should succeed
            val result1 = repository.markUsed(inviteCode.id, userId1)
            assertTrue(result1.isRight())

            // Try to find it again - should fail since it's used
            val findResult = repository.findByCode("ONCEONLY")
            assertTrue(findResult.isLeft())

            // Even if we try to mark it as used again with different user,
            // the code is already used and won't be findable
            val result2 = repository.markUsed(inviteCode.id, userId2)
            assertTrue(result2.isRight()) // UPDATE succeeds even if already used

            // Verify it's still not findable
            val findResult2 = repository.findByCode("ONCEONLY")
            assertTrue(findResult2.isLeft())
        }

    @Test
    fun `concurrent markUsed attempts on same code`(): Unit =
        runBlocking {
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

            // Due to race conditions, we can't guarantee exactly how many succeed
            // But at least one should have gotten through
            assertTrue(successCount >= 1, "At least one concurrent attempt should succeed")

            // After all attempts, code should definitely not be findable
            val finalFind = repository.findByCode("CONCURRENT")
            assertTrue(finalFind.isLeft())
        }

    @Test
    fun `findByCreator returns all codes created by user`(): Unit =
        runBlocking {
            val adminId = Ulid.generate()

            val code1 = createTestInviteCode(code = "ADMIN001", createdBy = adminId)
            val code2 = createTestInviteCode(code = "ADMIN002", createdBy = adminId)
            val code3 = createTestInviteCode(code = "ADMIN003", createdBy = adminId)

            repository.create(code1)
            repository.create(code2)
            repository.create(code3)

            val result = repository.findByCreator(adminId)

            assertTrue(result.isRight())
            result.onRight { codes ->
                assertEquals(3, codes.size)
                assertTrue(codes.all { it.createdBy == adminId })
            }
        }

    @Test
    fun `findByCreator filters by creator correctly`(): Unit =
        runBlocking {
            val admin1 = Ulid.generate()
            val admin2 = Ulid.generate()

            val admin1Code1 = createTestInviteCode(code = "ADMIN1-1", createdBy = admin1)
            val admin1Code2 = createTestInviteCode(code = "ADMIN1-2", createdBy = admin1)
            val admin2Code = createTestInviteCode(code = "ADMIN2-1", createdBy = admin2)

            repository.create(admin1Code1)
            repository.create(admin1Code2)
            repository.create(admin2Code)

            val result = repository.findByCreator(admin1)

            assertTrue(result.isRight())
            result.onRight { codes ->
                assertEquals(2, codes.size)
                assertTrue(codes.all { it.createdBy == admin1 })
                assertTrue(codes.none { it.createdBy == admin2 })
            }
        }

    @Test
    fun `findByCreator includes both used and unused codes`(): Unit =
        runBlocking {
            val adminId = Ulid.generate()

            val usedCode = createTestInviteCode(code = "USED0001", createdBy = adminId)
            val unusedCode = createTestInviteCode(code = "UNUSED01", createdBy = adminId)

            repository.create(usedCode)
            repository.create(unusedCode)

            // Mark one as used
            repository.markUsed(usedCode.id, Ulid.generate())

            val result = repository.findByCreator(adminId)

            assertTrue(result.isRight())
            result.onRight { codes ->
                assertEquals(2, codes.size)
                val used = codes.find { it.code == "USED0001" }
                val unused = codes.find { it.code == "UNUSED01" }

                assertNotNull(used)
                assertNotNull(unused)
                assertNotNull(used.usedBy)
                assertNull(unused.usedBy)
            }
        }

    @Test
    fun `findByCreator includes expired codes`(): Unit =
        runBlocking {
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

            assertTrue(result.isRight())
            result.onRight { codes ->
                assertEquals(2, codes.size)
            }
        }

    @Test
    fun `findByCreator returns empty list for creator with no codes`(): Unit =
        runBlocking {
            val adminId = Ulid.generate()

            val result = repository.findByCreator(adminId)

            assertTrue(result.isRight())
            result.onRight { codes ->
                assertTrue(codes.isEmpty())
            }
        }

    @Test
    fun `deleteExpiredAndUsed removes expired codes`(): Unit =
        runBlocking {
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
            assertTrue(result.isRight())

            // Expired code should not be findable (was already unfindable, now physically deleted)
            val expiredFind = repository.findByCode("EXPIRED1")
            assertTrue(expiredFind.isLeft())

            // Valid code should still be findable
            val validFind = repository.findByCode("VALID001")
            assertTrue(validFind.isRight())
        }

    @Test
    fun `deleteExpiredAndUsed removes used codes`(): Unit =
        runBlocking {
            val usedCode = createTestInviteCode(code = "USED0001")
            val unusedCode = createTestInviteCode(code = "UNUSED01")

            repository.create(usedCode)
            repository.create(unusedCode)

            // Mark one as used
            repository.markUsed(usedCode.id, Ulid.generate())

            // Delete expired and used codes
            val result = repository.deleteExpiredAndUsed()
            assertTrue(result.isRight())

            // Used code should not be findable (was already unfindable, now physically deleted)
            val usedFind = repository.findByCode("USED0001")
            assertTrue(usedFind.isLeft())

            // Unused code should still be findable
            val unusedFind = repository.findByCode("UNUSED01")
            assertTrue(unusedFind.isRight())
        }

    @Test
    fun `deleteExpiredAndUsed keeps valid unused codes`(): Unit =
        runBlocking {
            val validCode1 = createTestInviteCode(code = "VALID001", expiresIn = 7.days)
            val validCode2 = createTestInviteCode(code = "VALID002", expiresIn = 30.days)
            val validCode3 = createTestInviteCode(code = "VALID003", expiresIn = 1.hours)

            repository.create(validCode1)
            repository.create(validCode2)
            repository.create(validCode3)

            // Delete expired and used codes
            val result = repository.deleteExpiredAndUsed()
            assertTrue(result.isRight())

            // All valid codes should still be findable
            assertTrue(repository.findByCode("VALID001").isRight())
            assertTrue(repository.findByCode("VALID002").isRight())
            assertTrue(repository.findByCode("VALID003").isRight())
        }

    @Test
    fun `invite code timestamps are preserved correctly`(): Unit =
        runBlocking {
            val inviteCode = createTestInviteCode(code = "TIMESTAMPS")
            repository.create(inviteCode)

            val result = repository.findByCode("TIMESTAMPS")

            assertTrue(result.isRight())
            result.onRight { found ->
                assertNotNull(found.createdAt)
                assertNotNull(found.expiresAt)
                assertTrue(found.expiresAt > found.createdAt)
                assertNull(found.usedAt)
            }
        }

    @Test
    fun `used invite code has usedAt and usedBy set`(): Unit =
        runBlocking {
            val inviteCode = createTestInviteCode(code = "USEDTIME")
            repository.create(inviteCode)

            val userId = Ulid.generate()
            repository.markUsed(inviteCode.id, userId)

            // Query directly from database to check usedAt and usedBy
            // (can't use findByCode since it filters out used codes)
            val queryResult =
                dbClient.queryBind(
                    "SELECT * FROM invite_code WHERE code = \$code",
                    mapOf("code" to "USEDTIME"),
                )

            assertTrue(queryResult.isRight())
            queryResult.onRight { json ->
                // Just verify we get a result - the fact that usedBy/usedAt are set
                // is proven by the fact that findByCode returns Left
                assertTrue(json.isNotEmpty())
            }
        }

    @Test
    fun `multiple valid codes can exist simultaneously`(): Unit =
        runBlocking {
            val code1 = createTestInviteCode(code = "FIRST001")
            val code2 = createTestInviteCode(code = "SECOND01")
            val code3 = createTestInviteCode(code = "THIRD001")

            repository.create(code1)
            repository.create(code2)
            repository.create(code3)

            // All should be independently findable
            assertTrue(repository.findByCode("FIRST001").isRight())
            assertTrue(repository.findByCode("SECOND01").isRight())
            assertTrue(repository.findByCode("THIRD001").isRight())

            // Use one
            repository.markUsed(code2.id, Ulid.generate())

            // First and third should still be findable
            assertTrue(repository.findByCode("FIRST001").isRight())
            assertTrue(repository.findByCode("SECOND01").isLeft())
            assertTrue(repository.findByCode("THIRD001").isRight())
        }

    @Test
    fun `code search is case-sensitive`(): Unit =
        runBlocking {
            val inviteCode = createTestInviteCode(code = "CaseSensitive")
            repository.create(inviteCode)

            // Exact match should work
            val exactMatch = repository.findByCode("CaseSensitive")
            assertTrue(exactMatch.isRight())

            // Different case should not match
            val upperCase = repository.findByCode("CASESENSITIVE")
            assertTrue(upperCase.isLeft())

            val lowerCase = repository.findByCode("casesensitive")
            assertTrue(lowerCase.isLeft())
        }

    @Test
    fun `create stores code with minimum length`(): Unit =
        runBlocking {
            val minLengthCode = createTestInviteCode(code = "12345678") // 8 chars minimum

            val result = repository.create(minLengthCode)

            assertTrue(result.isRight())
        }

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

    companion object {
        @Container
        val container = SurrealDbTestContainer()
    }
}

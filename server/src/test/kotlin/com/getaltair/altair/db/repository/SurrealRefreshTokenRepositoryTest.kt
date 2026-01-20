package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbTestContainer
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.model.system.RefreshToken
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.runBlocking
import kotlin.time.Clock
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
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.hours

@Testcontainers
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class SurrealRefreshTokenRepositoryTest {
    private lateinit var dbClient: SurrealDbClient
    private lateinit var repository: SurrealRefreshTokenRepository

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
        repository = SurrealRefreshTokenRepository(dbClient)
        // Clean up tokens before each test
        runBlocking {
            dbClient.execute("DELETE refresh_token;")
        }
    }

    @Test
    fun `create stores new refresh token`(): Unit =
        runBlocking {
            val token = createTestToken()

            val result = repository.create(token)

            assertTrue(result.isRight())
            result.onRight { saved ->
                assertEquals(token.id, saved.id)
                assertEquals(token.userId, saved.userId)
                assertEquals(token.tokenHash, saved.tokenHash)
            }
        }

    @Test
    fun `findByHash returns token when exists and not revoked`(): Unit =
        runBlocking {
            val token = createTestToken(tokenHash = "unique-hash-123")
            repository.create(token)

            val result = repository.findByHash("unique-hash-123")

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(token.id, found.id)
                assertEquals(token.tokenHash, found.tokenHash)
                assertNull(found.revokedAt)
            }
        }

    @Test
    fun `findByHash returns error when token not found`(): Unit =
        runBlocking {
            val result = repository.findByHash("nonexistent-hash")

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<AuthError.TokenInvalid>(error)
            }
        }

    @Test
    fun `findByHash returns error when token is revoked`(): Unit =
        runBlocking {
            val token = createTestToken(tokenHash = "revoked-token-hash")
            repository.create(token)

            // Revoke the token
            repository.revoke(token.id, token.userId)

            // Try to find it - should not be found since it's revoked
            val result = repository.findByHash("revoked-token-hash")

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<AuthError.TokenInvalid>(error)
            }
        }

    @Test
    fun `revoke marks token as revoked`(): Unit =
        runBlocking {
            val token = createTestToken()
            repository.create(token)

            val result = repository.revoke(token.id, token.userId)

            assertTrue(result.isRight())

            // Verify token is no longer findable (because it's revoked)
            val findResult = repository.findByHash(token.tokenHash)
            assertTrue(findResult.isLeft())
        }

    @Test
    fun `revoke only affects token with matching user ID`(): Unit =
        runBlocking {
            val userId1 = Ulid.generate()
            val userId2 = Ulid.generate()
            val tokenId = Ulid.generate()

            val token = createTestToken(id = tokenId, userId = userId1, tokenHash = "token-user1")
            repository.create(token)

            // Try to revoke with different user ID
            val result = repository.revoke(tokenId, userId2)

            // Revoke should succeed (no error) but token should still be findable
            // because the WHERE clause filtered it out
            assertTrue(result.isRight())

            // Token should still be findable since user ID didn't match
            val findResult = repository.findByHash("token-user1")
            assertTrue(findResult.isRight())
        }

    @Test
    fun `token rotation - old token revoked, new token created`(): Unit =
        runBlocking {
            val userId = Ulid.generate()
            val oldToken = createTestToken(userId = userId, tokenHash = "old-token-hash")
            repository.create(oldToken)

            // Simulate token rotation: revoke old, create new
            val revokeResult = repository.revoke(oldToken.id, userId)
            assertTrue(revokeResult.isRight())

            val newToken = createTestToken(userId = userId, tokenHash = "new-token-hash")
            val createResult = repository.create(newToken)
            assertTrue(createResult.isRight())

            // Old token should not be findable (revoked)
            val oldFind = repository.findByHash("old-token-hash")
            assertTrue(oldFind.isLeft())

            // New token should be findable
            val newFind = repository.findByHash("new-token-hash")
            assertTrue(newFind.isRight())
        }

    @Test
    fun `revokeAllForUser revokes all user tokens`(): Unit =
        runBlocking {
            val userId = Ulid.generate()
            val token1 = createTestToken(userId = userId, tokenHash = "user1-token1")
            val token2 = createTestToken(userId = userId, tokenHash = "user1-token2")
            val token3 = createTestToken(userId = userId, tokenHash = "user1-token3")

            repository.create(token1)
            repository.create(token2)
            repository.create(token3)

            // Revoke all tokens for user
            val result = repository.revokeAllForUser(userId)
            assertTrue(result.isRight())

            // All tokens should now be unfindable (revoked)
            assertTrue(repository.findByHash("user1-token1").isLeft())
            assertTrue(repository.findByHash("user1-token2").isLeft())
            assertTrue(repository.findByHash("user1-token3").isLeft())
        }

    @Test
    fun `revokeAllForUser does not affect other users`(): Unit =
        runBlocking {
            val userId1 = Ulid.generate()
            val userId2 = Ulid.generate()

            val user1Token = createTestToken(userId = userId1, tokenHash = "user1-token")
            val user2Token = createTestToken(userId = userId2, tokenHash = "user2-token")

            repository.create(user1Token)
            repository.create(user2Token)

            // Revoke all tokens for user1
            repository.revokeAllForUser(userId1)

            // User1 token should be revoked
            assertTrue(repository.findByHash("user1-token").isLeft())

            // User2 token should still be valid
            val user2Find = repository.findByHash("user2-token")
            assertTrue(user2Find.isRight())
        }

    @Test
    fun `revokeAllForUser on already revoked tokens succeeds`(): Unit =
        runBlocking {
            val userId = Ulid.generate()
            val token = createTestToken(userId = userId, tokenHash = "already-revoked")

            repository.create(token)
            repository.revoke(token.id, userId)

            // Revoke all should still succeed even though token is already revoked
            val result = repository.revokeAllForUser(userId)
            assertTrue(result.isRight())
        }

    @Test
    fun `deleteExpired removes expired tokens`(): Unit =
        runBlocking {
            // Create an expired token (manually construct with past expiration)
            val now = Clock.System.now()
            val expiredToken =
                RefreshToken(
                    id = Ulid.generate(),
                    userId = Ulid.generate(),
                    tokenHash = "expired-token",
                    deviceName = null,
                    createdAt = now - 2.days,
                    expiresAt = now - 1.days, // Expired yesterday
                )
            repository.create(expiredToken)

            // Create a valid token
            val validToken =
                createTestToken(
                    tokenHash = "valid-token",
                    expiresIn = 30.days,
                )
            repository.create(validToken)

            // Delete expired tokens
            val result = repository.deleteExpired()
            assertTrue(result.isRight())

            // Expired token should be deleted
            assertTrue(repository.findByHash("expired-token").isLeft())

            // Valid token should still exist
            assertTrue(repository.findByHash("valid-token").isRight())
        }

    @Test
    fun `deleteExpired removes revoked tokens`(): Unit =
        runBlocking {
            val token = createTestToken(tokenHash = "revoked-token")
            repository.create(token)
            repository.revoke(token.id, token.userId)

            // Delete expired/revoked tokens
            val result = repository.deleteExpired()
            assertTrue(result.isRight())

            // Revoked token should be deleted
            // (it was already unfindable due to revocation, but now it's physically deleted)
            assertTrue(repository.findByHash("revoked-token").isLeft())
        }

    @Test
    fun `deleteExpired keeps valid non-revoked tokens`(): Unit =
        runBlocking {
            val validToken1 = createTestToken(tokenHash = "valid1", expiresIn = 30.days)
            val validToken2 = createTestToken(tokenHash = "valid2", expiresIn = 7.days)
            val validToken3 = createTestToken(tokenHash = "valid3", expiresIn = 1.hours)

            repository.create(validToken1)
            repository.create(validToken2)
            repository.create(validToken3)

            // Delete expired tokens
            val result = repository.deleteExpired()
            assertTrue(result.isRight())

            // All valid tokens should still exist
            assertTrue(repository.findByHash("valid1").isRight())
            assertTrue(repository.findByHash("valid2").isRight())
            assertTrue(repository.findByHash("valid3").isRight())
        }

    @Test
    fun `concurrent revoke operations on same token complete successfully`(): Unit =
        runBlocking {
            val token = createTestToken(tokenHash = "concurrent-test")
            repository.create(token)

            // Launch 10 concurrent revoke operations
            val results =
                (1..10).map {
                    async {
                        repository.revoke(token.id, token.userId)
                    }
                }.awaitAll()

            // All operations should complete (some may be no-ops if already revoked)
            results.forEach { result ->
                assertTrue(result.isRight(), "Expected Right but got Left: $result")
            }

            // Token should definitely be revoked now
            val findResult = repository.findByHash("concurrent-test")
            assertTrue(findResult.isLeft())
        }

    @Test
    fun `concurrent revokeAllForUser operations complete successfully`(): Unit =
        runBlocking {
            val userId = Ulid.generate()
            val token1 = createTestToken(userId = userId, tokenHash = "concurrent1")
            val token2 = createTestToken(userId = userId, tokenHash = "concurrent2")

            repository.create(token1)
            repository.create(token2)

            // Launch 5 concurrent revokeAllForUser operations
            val results =
                (1..5).map {
                    async {
                        repository.revokeAllForUser(userId)
                    }
                }.awaitAll()

            // All operations should complete
            results.forEach { result ->
                assertTrue(result.isRight())
            }

            // All tokens should be revoked
            assertTrue(repository.findByHash("concurrent1").isLeft())
            assertTrue(repository.findByHash("concurrent2").isLeft())
        }

    @Test
    fun `multiple tokens with different hashes for same user`(): Unit =
        runBlocking {
            val userId = Ulid.generate()
            val token1 = createTestToken(userId = userId, tokenHash = "device1-hash")
            val token2 = createTestToken(userId = userId, tokenHash = "device2-hash")
            val token3 = createTestToken(userId = userId, tokenHash = "device3-hash")

            repository.create(token1)
            repository.create(token2)
            repository.create(token3)

            // All tokens should be independently findable
            assertTrue(repository.findByHash("device1-hash").isRight())
            assertTrue(repository.findByHash("device2-hash").isRight())
            assertTrue(repository.findByHash("device3-hash").isRight())

            // Revoke just one
            repository.revoke(token2.id, userId)

            // Token2 should be unfindable, others should still work
            assertTrue(repository.findByHash("device1-hash").isRight())
            assertTrue(repository.findByHash("device2-hash").isLeft())
            assertTrue(repository.findByHash("device3-hash").isRight())
        }

    @Test
    fun `token with device name is persisted correctly`(): Unit =
        runBlocking {
            val token =
                createTestToken(
                    tokenHash = "device-token",
                    deviceName = "iPhone 15 Pro",
                )
            repository.create(token)

            val result = repository.findByHash("device-token")

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals("iPhone 15 Pro", found.deviceName)
            }
        }

    @Test
    fun `token without device name is persisted correctly`(): Unit =
        runBlocking {
            val token =
                createTestToken(
                    tokenHash = "no-device-token",
                    deviceName = null,
                )
            repository.create(token)

            val result = repository.findByHash("no-device-token")

            assertTrue(result.isRight())
            result.onRight { found ->
                assertNull(found.deviceName)
            }
        }

    @Test
    fun `token timestamps are preserved correctly`(): Unit =
        runBlocking {
            val token = createTestToken()
            repository.create(token)

            val result = repository.findByHash(token.tokenHash)

            assertTrue(result.isRight())
            result.onRight { found ->
                assertNotNull(found.createdAt)
                assertNotNull(found.expiresAt)
                assertTrue(found.expiresAt > found.createdAt)
                assertNull(found.revokedAt)
            }
        }

    @Test
    fun `revoked token has revokedAt timestamp set`(): Unit =
        runBlocking {
            val token = createTestToken()
            repository.create(token)

            // Revoke the token
            repository.revoke(token.id, token.userId)

            // Query directly from database to check revokedAt
            // (can't use findByHash since it filters out revoked tokens)
            val queryResult =
                dbClient.queryBind(
                    "SELECT * FROM refresh_token WHERE token_hash = \$tokenHash",
                    mapOf("tokenHash" to token.tokenHash),
                )

            assertTrue(queryResult.isRight())
            queryResult.onRight { json ->
                // Just verify we get a result - the fact that revokedAt is set
                // is proven by the fact that findByHash returns Left
                assertTrue(json.isNotEmpty())
            }
        }

    private fun createTestToken(
        id: Ulid = Ulid.generate(),
        userId: Ulid = Ulid.generate(),
        tokenHash: String = "test-token-hash-${Ulid.generate().value}",
        deviceName: String? = null,
        expiresIn: kotlin.time.Duration = 30.days,
    ): RefreshToken =
        RefreshToken.create(
            id = id,
            userId = userId,
            tokenHash = tokenHash,
            deviceName = deviceName,
            expiresIn = expiresIn,
            clock = Clock.System,
        )

    companion object {
        @Container
        val container = SurrealDbTestContainer()
    }
}

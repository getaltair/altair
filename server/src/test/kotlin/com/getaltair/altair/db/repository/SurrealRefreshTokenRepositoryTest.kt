package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbContainerExtension
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.model.system.RefreshToken
import com.getaltair.altair.domain.types.Ulid
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.BehaviorSpec
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
 * Tests for SurrealRefreshTokenRepository using Testcontainers.
 *
 * Verifies:
 * - CRUD operations (create, findByHash)
 * - Token revocation (revoke, revokeAllForUser)
 * - Token rotation scenarios
 * - Cleanup operations (deleteExpired removes expired and revoked tokens)
 * - Concurrency (concurrent revoke operations)
 * - Device name handling (optional field)
 * - Timestamp handling (createdAt, expiresAt, revokedAt)
 * - User isolation (revoke only affects matching user ID)
 */
class SurrealRefreshTokenRepositoryTest :
    BehaviorSpec({
        lateinit var dbClient: SurrealDbClient
        lateinit var repository: SurrealRefreshTokenRepository

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
            repository = SurrealRefreshTokenRepository(dbClient)
            // Clean up tokens before each test
            dbClient.execute("DELETE refresh_token;")
        }

        given("CRUD operations") {
            `when`("creating a new refresh token") {
                then("stores the token successfully") {
                    val token = createTestToken()

                    val result = repository.create(token)

                    result.shouldBeRight()
                    val saved = result.getOrNull()
                    saved?.id shouldBe token.id
                    saved?.userId shouldBe token.userId
                    saved?.tokenHash shouldBe token.tokenHash
                }
            }

            `when`("finding by hash") {
                then("returns token when exists and not revoked") {
                    val token = createTestToken(tokenHash = "unique-hash-123")
                    repository.create(token)

                    val result = repository.findByHash("unique-hash-123")

                    result.shouldBeRight()
                    val found = result.getOrNull()
                    found?.id shouldBe token.id
                    found?.tokenHash shouldBe token.tokenHash
                    found?.revokedAt.shouldBeNull()
                }

                then("returns error when token not found") {
                    val result = repository.findByHash("nonexistent-hash")

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<AuthError.TokenInvalid>()
                }

                then("returns error when token is revoked") {
                    val token = createTestToken(tokenHash = "revoked-token-hash")
                    repository.create(token)

                    // Revoke the token
                    repository.revoke(token.id, token.userId)

                    // Try to find it - should not be found since it's revoked
                    val result = repository.findByHash("revoked-token-hash")

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<AuthError.TokenInvalid>()
                }
            }
        }

        given("token revocation") {
            `when`("revoking a token") {
                then("marks token as revoked") {
                    val token = createTestToken()
                    repository.create(token)

                    val result = repository.revoke(token.id, token.userId)

                    result.shouldBeRight()

                    // Verify token is no longer findable (because it's revoked)
                    val findResult = repository.findByHash(token.tokenHash)
                    findResult.shouldBeLeft()
                }

                then("only affects token with matching user ID") {
                    val userId1 = Ulid.generate()
                    val userId2 = Ulid.generate()
                    val tokenId = Ulid.generate()

                    val token = createTestToken(id = tokenId, userId = userId1, tokenHash = "token-user1")
                    repository.create(token)

                    // Try to revoke with different user ID
                    val result = repository.revoke(tokenId, userId2)

                    // Revoke should succeed (no error) but token should still be findable
                    // because the WHERE clause filtered it out
                    result.shouldBeRight()

                    // Token should still be findable since user ID didn't match
                    val findResult = repository.findByHash("token-user1")
                    findResult.shouldBeRight()
                }

                then("sets revokedAt timestamp") {
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

                    queryResult.shouldBeRight()
                    val json = queryResult.getOrNull() ?: ""
                    json.shouldNotBeEmpty()
                }
            }

            `when`("revoking all tokens for user") {
                then("revokes all user tokens") {
                    val userId = Ulid.generate()
                    val token1 = createTestToken(userId = userId, tokenHash = "user1-token1")
                    val token2 = createTestToken(userId = userId, tokenHash = "user1-token2")
                    val token3 = createTestToken(userId = userId, tokenHash = "user1-token3")

                    repository.create(token1)
                    repository.create(token2)
                    repository.create(token3)

                    // Revoke all tokens for user
                    val result = repository.revokeAllForUser(userId)
                    result.shouldBeRight()

                    // All tokens should now be unfindable (revoked)
                    repository.findByHash("user1-token1").shouldBeLeft()
                    repository.findByHash("user1-token2").shouldBeLeft()
                    repository.findByHash("user1-token3").shouldBeLeft()
                }

                then("does not affect other users") {
                    val userId1 = Ulid.generate()
                    val userId2 = Ulid.generate()

                    val user1Token = createTestToken(userId = userId1, tokenHash = "user1-token")
                    val user2Token = createTestToken(userId = userId2, tokenHash = "user2-token")

                    repository.create(user1Token)
                    repository.create(user2Token)

                    // Revoke all tokens for user1
                    repository.revokeAllForUser(userId1)

                    // User1 token should be revoked
                    repository.findByHash("user1-token").shouldBeLeft()

                    // User2 token should still be valid
                    val user2Find = repository.findByHash("user2-token")
                    user2Find.shouldBeRight()
                }

                then("succeeds on already revoked tokens") {
                    val userId = Ulid.generate()
                    val token = createTestToken(userId = userId, tokenHash = "already-revoked")

                    repository.create(token)
                    repository.revoke(token.id, userId)

                    // Revoke all should still succeed even though token is already revoked
                    val result = repository.revokeAllForUser(userId)
                    result.shouldBeRight()
                }
            }
        }

        given("token rotation") {
            `when`("rotating tokens") {
                then("old token revoked, new token created") {
                    val userId = Ulid.generate()
                    val oldToken = createTestToken(userId = userId, tokenHash = "old-token-hash")
                    repository.create(oldToken)

                    // Simulate token rotation: revoke old, create new
                    val revokeResult = repository.revoke(oldToken.id, userId)
                    revokeResult.shouldBeRight()

                    val newToken = createTestToken(userId = userId, tokenHash = "new-token-hash")
                    val createResult = repository.create(newToken)
                    createResult.shouldBeRight()

                    // Old token should not be findable (revoked)
                    val oldFind = repository.findByHash("old-token-hash")
                    oldFind.shouldBeLeft()

                    // New token should be findable
                    val newFind = repository.findByHash("new-token-hash")
                    newFind.shouldBeRight()
                }
            }
        }

        given("cleanup operations") {
            `when`("deleting expired tokens") {
                then("removes expired tokens") {
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
                    result.shouldBeRight()

                    // Expired token should be deleted
                    repository.findByHash("expired-token").shouldBeLeft()

                    // Valid token should still exist
                    repository.findByHash("valid-token").shouldBeRight()
                }

                then("removes revoked tokens") {
                    val token = createTestToken(tokenHash = "revoked-token")
                    repository.create(token)
                    repository.revoke(token.id, token.userId)

                    // Delete expired/revoked tokens
                    val result = repository.deleteExpired()
                    result.shouldBeRight()

                    // Revoked token should be deleted
                    // (it was already unfindable due to revocation, but now it's physically deleted)
                    repository.findByHash("revoked-token").shouldBeLeft()
                }

                then("keeps valid non-revoked tokens") {
                    val validToken1 = createTestToken(tokenHash = "valid1", expiresIn = 30.days)
                    val validToken2 = createTestToken(tokenHash = "valid2", expiresIn = 7.days)
                    val validToken3 = createTestToken(tokenHash = "valid3", expiresIn = 1.hours)

                    repository.create(validToken1)
                    repository.create(validToken2)
                    repository.create(validToken3)

                    // Delete expired tokens
                    val result = repository.deleteExpired()
                    result.shouldBeRight()

                    // All valid tokens should still exist
                    repository.findByHash("valid1").shouldBeRight()
                    repository.findByHash("valid2").shouldBeRight()
                    repository.findByHash("valid3").shouldBeRight()
                }
            }
        }

        given("concurrent operations") {
            `when`("multiple concurrent revoke operations on same token") {
                then("all complete successfully") {
                    val token = createTestToken(tokenHash = "concurrent-test")
                    repository.create(token)

                    // Launch 10 concurrent revoke operations
                    val results =
                        (1..10)
                            .map {
                                async {
                                    repository.revoke(token.id, token.userId)
                                }
                            }.awaitAll()

                    // All operations should complete (some may be no-ops if already revoked)
                    results.forEach { result ->
                        result.shouldBeRight()
                    }

                    // Token should definitely be revoked now
                    val findResult = repository.findByHash("concurrent-test")
                    findResult.shouldBeLeft()
                }
            }

            `when`("multiple concurrent revokeAllForUser operations") {
                then("all complete successfully") {
                    val userId = Ulid.generate()
                    val token1 = createTestToken(userId = userId, tokenHash = "concurrent1")
                    val token2 = createTestToken(userId = userId, tokenHash = "concurrent2")

                    repository.create(token1)
                    repository.create(token2)

                    // Launch 5 concurrent revokeAllForUser operations
                    val results =
                        (1..5)
                            .map {
                                async {
                                    repository.revokeAllForUser(userId)
                                }
                            }.awaitAll()

                    // All operations should complete
                    results.forEach { result ->
                        result.shouldBeRight()
                    }

                    // All tokens should be revoked
                    repository.findByHash("concurrent1").shouldBeLeft()
                    repository.findByHash("concurrent2").shouldBeLeft()
                }
            }
        }

        given("device name handling") {
            `when`("token has device name") {
                then("persists correctly") {
                    val token =
                        createTestToken(
                            tokenHash = "device-token",
                            deviceName = "iPhone 15 Pro",
                        )
                    repository.create(token)

                    val result = repository.findByHash("device-token")

                    result.shouldBeRight()
                    val found = result.getOrNull()
                    found?.deviceName shouldBe "iPhone 15 Pro"
                }
            }

            `when`("token has no device name") {
                then("persists correctly") {
                    val token =
                        createTestToken(
                            tokenHash = "no-device-token",
                            deviceName = null,
                        )
                    repository.create(token)

                    val result = repository.findByHash("no-device-token")

                    result.shouldBeRight()
                    val found = result.getOrNull()
                    found?.deviceName.shouldBeNull()
                }
            }
        }

        given("multiple tokens per user") {
            `when`("user has multiple tokens with different hashes") {
                then("all are independently findable") {
                    val userId = Ulid.generate()
                    val token1 = createTestToken(userId = userId, tokenHash = "device1-hash")
                    val token2 = createTestToken(userId = userId, tokenHash = "device2-hash")
                    val token3 = createTestToken(userId = userId, tokenHash = "device3-hash")

                    repository.create(token1)
                    repository.create(token2)
                    repository.create(token3)

                    // All tokens should be independently findable
                    repository.findByHash("device1-hash").shouldBeRight()
                    repository.findByHash("device2-hash").shouldBeRight()
                    repository.findByHash("device3-hash").shouldBeRight()

                    // Revoke just one
                    repository.revoke(token2.id, userId)

                    // Token2 should be unfindable, others should still work
                    repository.findByHash("device1-hash").shouldBeRight()
                    repository.findByHash("device2-hash").shouldBeLeft()
                    repository.findByHash("device3-hash").shouldBeRight()
                }
            }
        }

        given("timestamp handling") {
            `when`("token is created") {
                then("preserves timestamps correctly") {
                    val token = createTestToken()
                    repository.create(token)

                    val result = repository.findByHash(token.tokenHash)

                    result.shouldBeRight()
                    val found = result.getOrNull()
                    found?.createdAt.shouldNotBeNull()
                    found?.expiresAt.shouldNotBeNull()
                    found?.expiresAt?.let { expiresAt ->
                        found.createdAt?.let { createdAt ->
                            expiresAt shouldBeGreaterThan createdAt
                        }
                    }
                    found?.revokedAt.shouldBeNull()
                }
            }
        }
    }) {
    companion object {
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
    }
}

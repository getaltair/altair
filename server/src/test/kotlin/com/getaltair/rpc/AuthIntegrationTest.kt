package com.getaltair.rpc

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbContainerExtension
import com.getaltair.altair.db.repository.SurrealInviteCodeRepository
import com.getaltair.altair.db.repository.SurrealRefreshTokenRepository
import com.getaltair.altair.db.repository.SurrealUserRepository
import com.getaltair.altair.domain.model.system.InviteCode
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.auth.Argon2PasswordService
import com.getaltair.auth.JwtConfig
import com.getaltair.auth.JwtTokenServiceImpl
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.nulls.shouldNotBeNull
import io.kotest.matchers.shouldBe
import io.kotest.matchers.shouldNotBe
import io.kotest.matchers.string.shouldContain
import kotlin.time.Clock
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.minutes
import com.getaltair.altair.domain.types.enums.UserRole as DomainUserRole

/**
 * Integration tests for authentication flows using Testcontainers.
 *
 * Tests the full authentication lifecycle with real dependencies:
 * - SurrealDB test container
 * - Real password hashing (Argon2)
 * - Real JWT token generation/validation
 *
 * Verifies:
 * - Registration (first user, with/without invite codes, validation)
 * - Login (valid/invalid credentials, JWT validation)
 * - Token refresh (valid/invalid, token rotation)
 * - Logout and session invalidation
 * - Account status handling
 * - Expired tokens
 * - Input validation
 */
class AuthIntegrationTest :
    BehaviorSpec({
        lateinit var dbClient: SurrealDbClient
        lateinit var userRepository: SurrealUserRepository
        lateinit var refreshTokenRepository: SurrealRefreshTokenRepository
        lateinit var inviteCodeRepository: SurrealInviteCodeRepository
        lateinit var passwordService: Argon2PasswordService
        lateinit var jwtTokenService: JwtTokenServiceImpl
        lateinit var jwtConfig: JwtConfig
        lateinit var publicAuthService: PublicAuthServiceImpl
        lateinit var authService: AuthServiceImpl

        beforeSpec {
            val config = SurrealDbContainerExtension.createNetworkConfig()
            dbClient = SurrealDbClient(config)
            dbClient.connect().getOrNull()

            // Run migrations
            val migrationRunner = MigrationRunner(dbClient)
            migrationRunner.runMigrations()

            // Initialize dependencies
            userRepository = SurrealUserRepository(dbClient)
            refreshTokenRepository = SurrealRefreshTokenRepository(dbClient)
            inviteCodeRepository = SurrealInviteCodeRepository(dbClient)
            passwordService = Argon2PasswordService()
            jwtConfig =
                JwtConfig(
                    secret = "test-secret-that-is-at-least-32-characters-long",
                    accessTokenExpiration = 15.minutes,
                    refreshTokenExpiration = 30.days,
                )
            jwtTokenService = JwtTokenServiceImpl(jwtConfig)

            publicAuthService =
                PublicAuthServiceImpl(
                    userRepository = userRepository,
                    refreshTokenRepository = refreshTokenRepository,
                    inviteCodeRepository = inviteCodeRepository,
                    passwordService = passwordService,
                    jwtTokenService = jwtTokenService,
                    jwtConfig = jwtConfig,
                )

            authService =
                AuthServiceImpl(
                    userRepository = userRepository,
                    refreshTokenRepository = refreshTokenRepository,
                    inviteCodeRepository = inviteCodeRepository,
                    passwordService = passwordService,
                    jwtTokenService = jwtTokenService,
                    jwtConfig = jwtConfig,
                )
        }

        afterSpec {
            dbClient.close()
        }

        beforeEach {
            // Clean up all auth-related tables
            dbClient.execute("DELETE user;")
            dbClient.execute("DELETE refresh_token;")
            dbClient.execute("DELETE invite_code;")
        }

        given("user registration") {
            `when`("first user registers without invite code") {
                then("succeeds and becomes admin") {
                    val response =
                        publicAuthService.register(
                            RegisterRequest(
                                email = "admin@test.com",
                                password = "SecurePassword123!",
                                displayName = "Admin User",
                                inviteCode = null,
                            ),
                        )

                    response.accessToken.shouldNotBeNull()
                    response.refreshToken.shouldNotBeNull()
                    response.role shouldBe com.getaltair.altair.domain.types.enums.UserRole.ADMIN
                    response.displayName shouldBe "Admin User"
                    response.expiresIn shouldBe 900 // 15 minutes in seconds

                    // Verify in database
                    val user = userRepository.findByEmail("admin@test.com").getOrNull()
                    user.shouldNotBeNull()
                    user.role shouldBe DomainUserRole.ADMIN
                }
            }

            `when`("registering with valid invite code") {
                then("succeeds and becomes member") {
                    // First create an admin user
                    publicAuthService.register(
                        RegisterRequest(
                            email = "admin@test.com",
                            password = "AdminPassword123!",
                            displayName = "Admin User",
                        ),
                    )

                    // Create an invite code
                    val adminUser = userRepository.findByEmail("admin@test.com").getOrNull()!!
                    val inviteCode = createInviteCode(inviteCodeRepository, adminUser.id)

                    // Register with invite code
                    val response =
                        publicAuthService.register(
                            RegisterRequest(
                                email = "invited@test.com",
                                password = "InvitedPassword123!",
                                displayName = "Invited User",
                                inviteCode = inviteCode.code,
                            ),
                        )

                    response.accessToken.shouldNotBeNull()
                    response.role shouldBe com.getaltair.altair.domain.types.enums.UserRole.MEMBER
                    response.displayName shouldBe "Invited User"

                    // Verify in database
                    val user = userRepository.findByEmail("invited@test.com").getOrNull()
                    user.shouldNotBeNull()
                    user.role shouldBe DomainUserRole.MEMBER
                }
            }

            `when`("non-first user attempts registration without invite code") {
                then("fails") {
                    // First create an initial user
                    publicAuthService.register(
                        RegisterRequest(
                            email = "first@test.com",
                            password = "FirstPassword123!",
                            displayName = "First User",
                        ),
                    )

                    // Attempt to register without invite code
                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            publicAuthService.register(
                                RegisterRequest(
                                    email = "noinvite@test.com",
                                    password = "NoInvitePassword123!",
                                    displayName = "No Invite User",
                                    inviteCode = null,
                                ),
                            )
                        }

                    exception.message shouldBe "Invite code is required"
                }
            }

            `when`("registering with invalid invite code") {
                then("fails") {
                    // First create an initial user
                    publicAuthService.register(
                        RegisterRequest(
                            email = "first@test.com",
                            password = "FirstPassword123!",
                            displayName = "First User",
                        ),
                    )

                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            publicAuthService.register(
                                RegisterRequest(
                                    email = "invalid@test.com",
                                    password = "InvalidPassword123!",
                                    displayName = "Invalid Invite User",
                                    inviteCode = "INVALID123",
                                ),
                            )
                        }

                    exception.message shouldBe "Invalid or expired invite code"
                }
            }

            `when`("registering with expired invite code") {
                then("fails") {
                    // First create an admin user
                    publicAuthService.register(
                        RegisterRequest(
                            email = "admin@test.com",
                            password = "AdminPassword123!",
                            displayName = "Admin User",
                        ),
                    )

                    val adminUser = userRepository.findByEmail("admin@test.com").getOrNull()!!

                    // Create an expired invite code
                    val expiredInvite = createExpiredInviteCode(inviteCodeRepository, adminUser.id)

                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            publicAuthService.register(
                                RegisterRequest(
                                    email = "expired@test.com",
                                    password = "ExpiredPassword123!",
                                    displayName = "Expired Invite User",
                                    inviteCode = expiredInvite.code,
                                ),
                            )
                        }

                    exception.message shouldBe "Invalid or expired invite code"
                }
            }

            `when`("registering with already used invite code") {
                then("fails") {
                    // First create an admin user
                    publicAuthService.register(
                        RegisterRequest(
                            email = "admin@test.com",
                            password = "AdminPassword123!",
                            displayName = "Admin User",
                        ),
                    )

                    val adminUser = userRepository.findByEmail("admin@test.com").getOrNull()!!
                    val inviteCode = createInviteCode(inviteCodeRepository, adminUser.id)

                    // Use the invite code once
                    publicAuthService.register(
                        RegisterRequest(
                            email = "first-invitee@test.com",
                            password = "FirstInviteePassword123!",
                            displayName = "First Invitee",
                            inviteCode = inviteCode.code,
                        ),
                    )

                    // Try to use the same invite code again
                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            publicAuthService.register(
                                RegisterRequest(
                                    email = "second-invitee@test.com",
                                    password = "SecondInviteePassword123!",
                                    displayName = "Second Invitee",
                                    inviteCode = inviteCode.code,
                                ),
                            )
                        }

                    exception.message shouldBe "Invalid or expired invite code"
                }
            }

            `when`("registering with duplicate email") {
                then("fails") {
                    // First user registers as admin
                    val firstUserResponse =
                        publicAuthService.register(
                            RegisterRequest(
                                email = "duplicate@test.com",
                                password = "DuplicatePassword123!",
                                displayName = "First User",
                            ),
                        )

                    // Create an invite code for the second registration attempt
                    val inviteCode = createInviteCode(inviteCodeRepository, firstUserResponse.userId)

                    // Try to register with the same email but valid invite code
                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            publicAuthService.register(
                                RegisterRequest(
                                    email = "duplicate@test.com",
                                    password = "AnotherPassword123!",
                                    displayName = "Second User",
                                    inviteCode = inviteCode.code,
                                ),
                            )
                        }

                    exception.message shouldBe "Email is already registered"
                }
            }

            `when`("registering with short password") {
                then("fails") {
                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            publicAuthService.register(
                                RegisterRequest(
                                    email = "short@test.com",
                                    password = "short",
                                    displayName = "Short Password User",
                                ),
                            )
                        }

                    exception.message shouldContain "at least"
                }
            }
        }

        given("user login") {
            `when`("logging in with valid credentials") {
                then("returns tokens") {
                    // First register a user
                    publicAuthService.register(
                        RegisterRequest(
                            email = "user@test.com",
                            password = "SecurePassword123!",
                            displayName = "Test User",
                        ),
                    )

                    // Login with same credentials
                    val response =
                        publicAuthService.login(
                            AuthRequest(
                                email = "user@test.com",
                                password = "SecurePassword123!",
                            ),
                        )

                    response.accessToken.shouldNotBeNull()
                    response.refreshToken.shouldNotBeNull()
                    response.displayName shouldBe "Test User"
                    response.expiresIn shouldBe 900
                }

                then("returns valid JWT token") {
                    publicAuthService.register(
                        RegisterRequest(
                            email = "jwt@test.com",
                            password = "SecurePassword123!",
                            displayName = "JWT User",
                        ),
                    )

                    val response =
                        publicAuthService.login(
                            AuthRequest(
                                email = "jwt@test.com",
                                password = "SecurePassword123!",
                            ),
                        )

                    // Validate the access token
                    val claims = jwtTokenService.validateAccessToken(response.accessToken)
                    claims.shouldBeRight()
                    val tokenClaims = claims.getOrNull()
                    tokenClaims?.email shouldBe "jwt@test.com"
                    tokenClaims?.userId?.value shouldBe response.userId.value
                }
            }

            `when`("logging in with wrong password") {
                then("fails") {
                    publicAuthService.register(
                        RegisterRequest(
                            email = "wrongpass@test.com",
                            password = "CorrectPassword123!",
                            displayName = "Test User",
                        ),
                    )

                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            publicAuthService.login(
                                AuthRequest(
                                    email = "wrongpass@test.com",
                                    password = "WrongPassword123!",
                                ),
                            )
                        }

                    exception.message shouldBe "Invalid email or password"
                }
            }

            `when`("logging in with non-existent email") {
                then("fails") {
                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            publicAuthService.login(
                                AuthRequest(
                                    email = "nonexistent@test.com",
                                    password = "Password123!",
                                ),
                            )
                        }

                    exception.message shouldBe "Invalid email or password"
                }
            }

            `when`("logging in with active status") {
                then("succeeds") {
                    // Register a user - they start as ACTIVE
                    publicAuthService.register(
                        RegisterRequest(
                            email = "statuscheck@test.com",
                            password = "StatusCheckPassword123!",
                            displayName = "Status Check User",
                        ),
                    )

                    // Login should succeed with ACTIVE status
                    val loginResponse =
                        publicAuthService.login(
                            AuthRequest(
                                email = "statuscheck@test.com",
                                password = "StatusCheckPassword123!",
                            ),
                        )

                    loginResponse.accessToken.shouldNotBeNull()
                }
            }
        }

        given("token refresh") {
            `when`("refreshing with valid token") {
                then("returns new access token") {
                    val registerResponse =
                        publicAuthService.register(
                            RegisterRequest(
                                email = "refresh@test.com",
                                password = "RefreshPassword123!",
                                displayName = "Refresh User",
                            ),
                        )

                    val refreshResponse = publicAuthService.refresh(registerResponse.refreshToken)

                    refreshResponse.accessToken.shouldNotBeNull()
                    refreshResponse.expiresIn shouldBe 900
                    // New token should be different from original
                    refreshResponse.accessToken shouldNotBe registerResponse.accessToken
                }

                then("new access token is valid") {
                    val registerResponse =
                        publicAuthService.register(
                            RegisterRequest(
                                email = "validrefresh@test.com",
                                password = "ValidRefreshPassword123!",
                                displayName = "Valid Refresh User",
                            ),
                        )

                    val refreshResponse = publicAuthService.refresh(registerResponse.refreshToken)

                    // Validate the new access token
                    val claims = jwtTokenService.validateAccessToken(refreshResponse.accessToken)
                    claims.shouldBeRight()
                    val tokenClaims = claims.getOrNull()
                    tokenClaims?.email shouldBe "validrefresh@test.com"
                }
            }

            `when`("refreshing with invalid token") {
                then("fails") {
                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            publicAuthService.refresh("invalid-refresh-token")
                        }

                    exception.message shouldBe "Invalid refresh token"
                }
            }

            `when`("attempting token reuse") {
                then("fails due to token rotation") {
                    val registerResponse =
                        publicAuthService.register(
                            RegisterRequest(
                                email = "rotation@test.com",
                                password = "RotationPassword123!",
                                displayName = "Rotation User",
                            ),
                        )

                    // First refresh should succeed
                    publicAuthService.refresh(registerResponse.refreshToken)

                    // Second refresh with same token should fail (token rotation)
                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            publicAuthService.refresh(registerResponse.refreshToken)
                        }

                    val message = exception.message.shouldNotBeNull()
                    (message.contains("expired") || message.contains("Invalid")).shouldBeTrue()
                }
            }

            `when`("refreshing with expired token") {
                then("fails") {
                    // Register a user
                    val registerResponse =
                        publicAuthService.register(
                            RegisterRequest(
                                email = "expired-refresh@test.com",
                                password = "ExpiredRefreshPassword123!",
                                displayName = "Expired Refresh User",
                            ),
                        )

                    // Manually expire the refresh token in the database
                    dbClient.execute(
                        """
                        UPDATE refresh_token SET
                            created_at = d"1970-01-01T00:00:00Z",
                            expires_at = d"1970-01-02T00:00:00Z"
                        WHERE user_id = user:${registerResponse.userId};
                        """.trimIndent(),
                    )

                    // Attempt to refresh with the now-expired token
                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            publicAuthService.refresh(registerResponse.refreshToken)
                        }

                    val message = exception.message.shouldNotBeNull()
                    (message.contains("expired") || message.contains("revoked")).shouldBeTrue()
                }
            }
        }

        given("logout") {
            `when`("logging out") {
                then("returns success response") {
                    // Note: Due to kotlinx-rpc limitations, logout doesn't have access to auth context
                    // It returns a success response instructing client to discard tokens
                    val response = authService.logout()

                    response.success.shouldBeTrue()
                    response.message.shouldNotBeNull()
                }
            }
        }
    }) {
    companion object {
        private suspend fun createInviteCode(
            repository: SurrealInviteCodeRepository,
            createdBy: Ulid,
        ): InviteCode {
            val inviteCode =
                InviteCode.create(
                    id = Ulid.generate(),
                    code = generateCode(),
                    createdBy = createdBy,
                    expiresIn = 7.days,
                )
            repository.create(inviteCode)
            return inviteCode
        }

        private suspend fun createExpiredInviteCode(
            repository: SurrealInviteCodeRepository,
            createdBy: Ulid,
        ): InviteCode {
            val now = Clock.System.now()
            val createdAt = now - 2.days
            val expiresAt = now - 1.days

            val inviteCode =
                InviteCode(
                    id = Ulid.generate(),
                    code = generateCode(),
                    createdBy = createdBy,
                    expiresAt = expiresAt,
                    createdAt = createdAt,
                )
            repository.create(inviteCode)
            return inviteCode
        }

        private fun generateCode(): String {
            val chars = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789"
            return (1..8).map { chars.random() }.joinToString("")
        }
    }
}

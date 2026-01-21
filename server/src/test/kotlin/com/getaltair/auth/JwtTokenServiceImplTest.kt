package com.getaltair.auth

import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.types.Ulid
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.comparables.shouldBeGreaterThan
import io.kotest.matchers.shouldBe
import io.kotest.matchers.shouldNotBe
import io.kotest.matchers.string.shouldNotBeBlank
import io.kotest.matchers.string.shouldNotContain
import io.kotest.matchers.types.shouldBeInstanceOf
import kotlin.time.Duration.Companion.milliseconds

/**
 * Tests for JwtTokenServiceImpl using HMAC-SHA256.
 *
 * Verifies:
 * - Token generation (access and refresh tokens)
 * - Token validation (claims extraction)
 * - Expiration handling
 * - Security (wrong issuer, wrong audience, wrong secret, tampered tokens)
 * - Refresh token generation (uniqueness, URL-safe encoding)
 */
class JwtTokenServiceImplTest :
    DescribeSpec({
        val config =
            JwtConfig(
                secret = "test-secret-that-is-at-least-32-characters-long",
                issuer = "test-issuer",
                audience = "test-audience",
            )
        val tokenService = JwtTokenServiceImpl(config)

        describe("token generation") {
            it("creates valid token pair") {
                val userId = Ulid.generate()
                val email = "test@example.com"
                val role = "member"

                val tokenPair = tokenService.generateTokens(userId, email, role)

                tokenPair.accessToken.shouldNotBeBlank()
                tokenPair.refreshToken.shouldNotBeBlank()
                tokenPair.accessTokenExpiresIn shouldBeGreaterThan 0
            }
        }

        describe("access token validation") {
            it("returns claims for valid token") {
                val userId = Ulid.generate()
                val email = "test@example.com"
                val role = "admin"

                val tokenPair = tokenService.generateTokens(userId, email, role)
                val result = tokenService.validateAccessToken(tokenPair.accessToken)

                result.shouldBeRight()
                val claims = result.getOrNull()
                claims?.userId shouldBe userId
                claims?.email shouldBe email
                claims?.role shouldBe role
            }

            it("returns error for invalid token") {
                val result = tokenService.validateAccessToken("invalid-token")

                result.shouldBeLeft()
                result.leftOrNull().shouldBeInstanceOf<AuthError.TokenInvalid>()
            }
        }

        describe("token expiration") {
            it("returns error for expired token") {
                val shortLivedConfig =
                    JwtConfig(
                        secret = "test-secret-that-is-at-least-32-characters-long",
                        issuer = "test-issuer",
                        audience = "test-audience",
                        accessTokenExpiration = 1.milliseconds,
                    )
                val shortLivedService = JwtTokenServiceImpl(shortLivedConfig)

                val userId = Ulid.generate()
                val (token, _) = shortLivedService.generateAccessToken(userId, "test@example.com", "member")

                // Wait for token to expire
                Thread.sleep(10)

                val result = shortLivedService.validateAccessToken(token)

                result.shouldBeLeft()
                result.leftOrNull().shouldBeInstanceOf<AuthError.TokenExpired>()
            }
        }

        describe("refresh token generation") {
            it("creates unique tokens") {
                val token1 = tokenService.generateRefreshToken()
                val token2 = tokenService.generateRefreshToken()

                token1 shouldNotBe token2
                token1.shouldNotBeBlank()
                token2.shouldNotBeBlank()
            }

            it("creates URL-safe tokens") {
                val token = tokenService.generateRefreshToken()

                // URL-safe Base64 should not contain + or /
                token shouldNotContain "+"
                token shouldNotContain "/"
            }
        }

        describe("security validation") {
            it("rejects token with wrong issuer") {
                val otherConfig =
                    JwtConfig(
                        secret = "test-secret-that-is-at-least-32-characters-long",
                        issuer = "other-issuer",
                        audience = "test-audience",
                    )
                val otherService = JwtTokenServiceImpl(otherConfig)

                val userId = Ulid.generate()
                val tokenPair = otherService.generateTokens(userId, "test@example.com", "member")

                val result = tokenService.validateAccessToken(tokenPair.accessToken)

                result.shouldBeLeft()
                result.leftOrNull().shouldBeInstanceOf<AuthError.TokenInvalid>()
            }

            it("rejects token with wrong audience") {
                val wrongAudienceConfig =
                    JwtConfig(
                        secret = config.secret, // Same secret
                        issuer = config.issuer, // Same issuer
                        audience = "wrong-audience",
                    )
                val wrongAudienceService = JwtTokenServiceImpl(wrongAudienceConfig)

                val userId = Ulid.generate()
                val tokenPair = wrongAudienceService.generateTokens(userId, "test@example.com", "member")

                val result = tokenService.validateAccessToken(tokenPair.accessToken)

                result.shouldBeLeft()
                result.leftOrNull().shouldBeInstanceOf<AuthError.TokenInvalid>()
            }

            it("rejects token signed with different secret") {
                val attackerConfig =
                    JwtConfig(
                        secret = "attacker-secret-that-is-32-characters-long",
                        issuer = config.issuer, // Same issuer
                        audience = config.audience, // Same audience
                    )
                val attackerService = JwtTokenServiceImpl(attackerConfig)

                val userId = Ulid.generate()
                val tokenPair = attackerService.generateTokens(userId, "test@example.com", "admin")

                // Try to validate attacker's token with our service
                val result = tokenService.validateAccessToken(tokenPair.accessToken)

                result.shouldBeLeft()
                result.leftOrNull().shouldBeInstanceOf<AuthError.TokenInvalid>()
            }

            it("rejects tampered token") {
                val userId = Ulid.generate()
                val tokenPair = tokenService.generateTokens(userId, "test@example.com", "member")

                // Tamper with the token (flip a character in the signature)
                val tamperedToken =
                    tokenPair.accessToken.let { token ->
                        val parts = token.split(".")
                        if (parts.size == 3) {
                            val signature = parts[2]
                            val tamperedSignature =
                                if (signature.first() == 'a') {
                                    "b${signature.drop(1)}"
                                } else {
                                    "a${signature.drop(1)}"
                                }
                            "${parts[0]}.${parts[1]}.$tamperedSignature"
                        } else {
                            token
                        }
                    }

                val result = tokenService.validateAccessToken(tamperedToken)

                result.shouldBeLeft()
                result.leftOrNull().shouldBeInstanceOf<AuthError.TokenInvalid>()
            }
        }
    })

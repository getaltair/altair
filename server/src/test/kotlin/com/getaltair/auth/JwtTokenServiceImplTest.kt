package com.getaltair.auth

import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.types.Ulid
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs
import kotlin.test.assertNotEquals
import kotlin.test.assertTrue
import kotlin.time.Duration.Companion.milliseconds

class JwtTokenServiceImplTest {
    private val config =
        JwtConfig(
            secret = "test-secret-that-is-at-least-32-characters-long",
            issuer = "test-issuer",
            audience = "test-audience",
        )
    private val tokenService = JwtTokenServiceImpl(config)

    @Test
    fun `generateTokens creates valid token pair`() {
        val userId = Ulid.generate()
        val email = "test@example.com"
        val role = "member"

        val tokenPair = tokenService.generateTokens(userId, email, role)

        assertTrue(tokenPair.accessToken.isNotBlank())
        assertTrue(tokenPair.refreshToken.isNotBlank())
        assertTrue(tokenPair.accessTokenExpiresIn > 0)
    }

    @Test
    fun `validateAccessToken returns claims for valid token`() {
        val userId = Ulid.generate()
        val email = "test@example.com"
        val role = "admin"

        val tokenPair = tokenService.generateTokens(userId, email, role)
        val result = tokenService.validateAccessToken(tokenPair.accessToken)

        assertTrue(result.isRight())
        result.onRight { claims ->
            assertEquals(userId, claims.userId)
            assertEquals(email, claims.email)
            assertEquals(role, claims.role)
        }
    }

    @Test
    fun `validateAccessToken returns error for invalid token`() {
        val result = tokenService.validateAccessToken("invalid-token")

        assertTrue(result.isLeft())
        result.onLeft { error ->
            assertIs<AuthError.TokenInvalid>(error)
        }
    }

    @Test
    fun `validateAccessToken returns error for expired token`() {
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

        assertTrue(result.isLeft())
        result.onLeft { error ->
            assertIs<AuthError.TokenExpired>(error)
        }
    }

    @Test
    fun `validateAccessToken rejects token with wrong issuer`() {
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

        assertTrue(result.isLeft())
        result.onLeft { error ->
            assertIs<AuthError.TokenInvalid>(error)
        }
    }

    @Test
    fun `generateRefreshToken creates unique tokens`() {
        val token1 = tokenService.generateRefreshToken()
        val token2 = tokenService.generateRefreshToken()

        assertNotEquals(token1, token2)
        assertTrue(token1.isNotBlank())
        assertTrue(token2.isNotBlank())
    }

    @Test
    fun `generateRefreshToken creates URL-safe tokens`() {
        val token = tokenService.generateRefreshToken()

        // URL-safe Base64 should not contain + or /
        assertTrue(!token.contains('+'))
        assertTrue(!token.contains('/'))
    }
}

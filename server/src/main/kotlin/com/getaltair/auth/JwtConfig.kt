package com.getaltair.auth

import kotlin.time.Duration
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.minutes

/**
 * Configuration for JWT token generation and validation.
 *
 * @property secret The HMAC secret for signing tokens. Must be at least 32 characters.
 * @property issuer The token issuer (iss claim)
 * @property audience The token audience (aud claim)
 * @property accessTokenExpiration Duration until access tokens expire
 * @property refreshTokenExpiration Duration until refresh tokens expire
 */
data class JwtConfig(
    val secret: String,
    val issuer: String = "altair-server",
    val audience: String = "altair-client",
    val accessTokenExpiration: Duration = 15.minutes,
    val refreshTokenExpiration: Duration = 30.days,
) {
    init {
        require(secret.length >= MIN_SECRET_LENGTH) {
            "JWT secret must be at least $MIN_SECRET_LENGTH characters"
        }
    }

    companion object {
        private const val MIN_SECRET_LENGTH = 32

        /**
         * Load JWT configuration from environment variables.
         *
         * Required environment variables:
         * - JWT_SECRET: The HMAC secret (at least 32 characters)
         *
         * Optional environment variables:
         * - JWT_ISSUER: Token issuer (default: altair-server)
         * - JWT_AUDIENCE: Token audience (default: altair-client)
         * - JWT_ACCESS_EXPIRY_MINUTES: Access token expiry in minutes (default: 15)
         * - JWT_REFRESH_EXPIRY_DAYS: Refresh token expiry in days (default: 30)
         *
         * @return JwtConfig loaded from environment
         * @throws IllegalStateException if JWT_SECRET is not set or too short
         */
        fun fromEnvironment(): JwtConfig {
            val secret =
                System.getenv("JWT_SECRET")
                    ?: error("JWT_SECRET environment variable is required")

            return JwtConfig(
                secret = secret,
                issuer = System.getenv("JWT_ISSUER") ?: "altair-server",
                audience = System.getenv("JWT_AUDIENCE") ?: "altair-client",
                accessTokenExpiration =
                    System
                        .getenv("JWT_ACCESS_EXPIRY_MINUTES")
                        ?.toLongOrNull()
                        ?.minutes
                        ?: 15.minutes,
                refreshTokenExpiration =
                    System
                        .getenv("JWT_REFRESH_EXPIRY_DAYS")
                        ?.toLongOrNull()
                        ?.days
                        ?: 30.days,
            )
        }
    }
}

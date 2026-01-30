package com.getaltair.server.auth

/**
 * Configuration for authentication and authorization.
 *
 * This class reads from environment variables with sensible defaults for development.
 * In production, all sensitive values should be provided via environment variables.
 *
 * Environment Variables:
 * - JWT_SECRET: Secret key for signing JWT tokens (MUST be set in production)
 * - JWT_ISSUER: The issuer claim for JWT tokens
 * - JWT_AUDIENCE: The audience claim for JWT tokens
 * - INVITE_CODE: Required code for user registration (optional security measure)
 */
data class AuthConfig(
    /**
     * Secret key used to sign and verify JWT tokens.
     * **CRITICAL**: Must be a strong, randomly generated secret in production.
     */
    val jwtSecret: String = System.getenv("JWT_SECRET") ?: "dev-secret-change-in-production",

    /**
     * Issuer claim (iss) for JWT tokens.
     * Identifies the server that issued the token.
     */
    val jwtIssuer: String = System.getenv("JWT_ISSUER") ?: "https://altair.local",

    /**
     * Audience claim (aud) for JWT tokens.
     * Identifies the intended recipients of the token.
     */
    val jwtAudience: String = System.getenv("JWT_AUDIENCE") ?: "altair-api",

    /**
     * JWT realm for HTTP authentication challenges.
     */
    val jwtRealm: String = "Altair",

    /**
     * Access token expiration time in seconds.
     * Default: 15 minutes (900 seconds)
     */
    val accessTokenExpiry: Long = 15 * 60,

    /**
     * Refresh token expiration time in seconds.
     * Default: 7 days (604800 seconds)
     */
    val refreshTokenExpiry: Long = 7 * 24 * 60 * 60,

    /**
     * Invite code required for registration.
     * Optional security measure during beta/private testing.
     */
    val inviteCode: String = System.getenv("INVITE_CODE") ?: "altair-beta-2026"
)

package com.getaltair.altair.rpc

import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.auth.TokenRefreshResponse
import kotlinx.rpc.annotations.Rpc

/**
 * Public RPC service for authentication operations that don't require a valid session.
 *
 * This service handles login, registration, and token refresh - operations that
 * are performed before the user has authenticated or when their access token has
 * expired.
 *
 * This service is exposed on the public `/rpc/auth` endpoint without authentication.
 *
 * ## Error Handling
 *
 * RPC services use exception-based error handling at the transport layer.
 * Callers should wrap RPC calls with Arrow's `Either.catch {}` to convert
 * exceptions to typed errors.
 */
@Rpc
interface PublicAuthService {
    /**
     * Authenticate a user with email and password.
     *
     * @param request Credentials for authentication
     * @return AuthResponse with tokens and user info on success
     * @throws IllegalArgumentException if credentials are invalid
     */
    suspend fun login(request: AuthRequest): AuthResponse

    /**
     * Refresh an expired access token using a refresh token.
     *
     * @param refreshToken Valid refresh token from previous authentication
     * @return TokenRefreshResponse with new access token
     * @throws IllegalArgumentException if refresh token is invalid or expired
     */
    suspend fun refresh(refreshToken: String): TokenRefreshResponse

    /**
     * Register a new user account.
     *
     * The first user registered becomes an admin and doesn't require an invite code.
     * Subsequent users must provide a valid invite code.
     *
     * @param request Registration details including email, password, display name
     * @return AuthResponse with tokens and user info for the new account
     * @throws IllegalArgumentException if email already exists or validation fails
     */
    suspend fun register(request: RegisterRequest): AuthResponse
}

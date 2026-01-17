package com.getaltair.altair.rpc

import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.auth.TokenRefreshResponse
import kotlinx.rpc.annotations.Rpc

/**
 * RPC service for authentication operations.
 *
 * Handles user login, registration, token refresh, and logout.
 * This service does not require authentication middleware for
 * login/register endpoints.
 */
@Rpc
interface AuthService {
    /**
     * Authenticate a user with email and password.
     *
     * @param request Credentials for authentication
     * @return AuthResponse with tokens and user info, or throws on invalid credentials
     */
    suspend fun login(request: AuthRequest): AuthResponse

    /**
     * Refresh an expired access token using a refresh token.
     *
     * @param refreshToken Valid refresh token from previous authentication
     * @return TokenRefreshResponse with new access token
     */
    suspend fun refresh(refreshToken: String): TokenRefreshResponse

    /**
     * Invalidate the current session and revoke tokens.
     *
     * After logout, the refresh token will no longer be valid.
     */
    suspend fun logout()

    /**
     * Register a new user account.
     *
     * @param request Registration details including email, password, display name
     * @return AuthResponse with tokens and user info for the new account
     */
    suspend fun register(request: RegisterRequest): AuthResponse
}

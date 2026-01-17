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
 *
 * ## Error Handling
 *
 * RPC services use exception-based error handling at the transport layer.
 * Callers should wrap RPC calls with Arrow's `Either.catch {}` to convert
 * exceptions to typed errors:
 *
 * ```kotlin
 * suspend fun login(request: AuthRequest): Either<AuthError, AuthResponse> =
 *     Either.catch { authService.login(request) }
 *         .mapLeft { e -> AuthError.fromException(e) }
 * ```
 *
 * This pattern allows the RPC layer to use kotlinx-rpc's native error handling
 * while maintaining the project's `Either<DomainError, T>` convention at the
 * repository/use-case layer.
 */
@Rpc
interface AuthService {
    /**
     * Authenticate a user with email and password.
     *
     * @param request Credentials for authentication
     * @return AuthResponse with tokens and user info on success
     * @throws IllegalArgumentException if credentials are invalid (real implementation)
     */
    suspend fun login(request: AuthRequest): AuthResponse

    /**
     * Refresh an expired access token using a refresh token.
     *
     * @param refreshToken Valid refresh token from previous authentication
     * @return TokenRefreshResponse with new access token
     * @throws IllegalArgumentException if refresh token is invalid or expired (real implementation)
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
     * @throws IllegalArgumentException if email already exists or validation fails (real implementation)
     */
    suspend fun register(request: RegisterRequest): AuthResponse
}

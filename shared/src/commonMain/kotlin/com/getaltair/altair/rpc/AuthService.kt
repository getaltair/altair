package com.getaltair.altair.rpc

import com.getaltair.altair.dto.auth.ChangePasswordRequest
import com.getaltair.altair.dto.auth.InviteCodeResponse
import com.getaltair.altair.dto.auth.SuccessResponse
import kotlinx.rpc.annotations.Rpc

/**
 * RPC service for authenticated user operations.
 *
 * This service handles operations that require a valid session, such as
 * logout, password changes, and session management.
 *
 * This service is exposed on the authenticated `/rpc` endpoint and requires
 * a valid JWT token in the WebSocket connection headers.
 *
 * ## Error Handling
 *
 * RPC services use exception-based error handling at the transport layer.
 * Callers should wrap RPC calls with Arrow's `Either.catch {}` to convert
 * exceptions to typed errors.
 *
 * @see PublicAuthService for login, register, and token refresh operations
 */
@Rpc
interface AuthService {
    /**
     * Invalidate the current session and revoke tokens.
     *
     * After logout, the refresh token will no longer be valid.
     *
     * @return SuccessResponse indicating success
     */
    suspend fun logout(): SuccessResponse

    /**
     * Generate a new invite code (admin only).
     *
     * @return InviteCodeResponse with the generated code and expiration
     * @throws IllegalArgumentException if the user is not an admin
     */
    suspend fun generateInviteCode(): InviteCodeResponse

    /**
     * Change the current user's password.
     *
     * @param request The current and new passwords
     * @return SuccessResponse indicating success or failure
     * @throws IllegalArgumentException if the current password is incorrect
     */
    suspend fun changePassword(request: ChangePasswordRequest): SuccessResponse

    /**
     * Revoke all sessions for the current user.
     *
     * This invalidates all refresh tokens, forcing re-authentication on all devices.
     *
     * @return SuccessResponse with the count of revoked sessions
     */
    suspend fun revokeAllSessions(): SuccessResponse
}

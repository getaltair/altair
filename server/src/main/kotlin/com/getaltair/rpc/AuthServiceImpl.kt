package com.getaltair.rpc

import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.auth.TokenRefreshResponse
import com.getaltair.altair.rpc.AuthService

/**
 * Stub implementation of AuthService for infrastructure validation.
 *
 * Accepts any credentials and returns test tokens.
 * Real implementation will integrate with JWT and user repository in Phase 5+.
 */
class AuthServiceImpl : AuthService {
    override suspend fun login(request: AuthRequest): AuthResponse {
        // Stub: Accept any credentials, return test token
        return AuthResponse(
            accessToken = "test-access-token-${System.currentTimeMillis()}",
            refreshToken = "test-refresh-token-${System.currentTimeMillis()}",
            expiresIn = 3600,
            userId = "01HWTEST000000000000000001",
            displayName = "Test User",
            role = "member",
        )
    }

    override suspend fun refresh(refreshToken: String): TokenRefreshResponse {
        // Stub: Return new access token
        return TokenRefreshResponse(
            accessToken = "test-access-token-refreshed-${System.currentTimeMillis()}",
            expiresIn = 3600,
        )
    }

    override suspend fun logout() {
        // Stub: No-op
    }

    override suspend fun register(request: RegisterRequest): AuthResponse {
        // Stub: Return test response for any registration
        return AuthResponse(
            accessToken = "test-access-token-${System.currentTimeMillis()}",
            refreshToken = "test-refresh-token-${System.currentTimeMillis()}",
            expiresIn = 3600,
            userId = "01HWTEST000000000000000002",
            displayName = request.displayName,
            role = "member",
        )
    }
}

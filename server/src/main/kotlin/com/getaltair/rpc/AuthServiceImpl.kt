package com.getaltair.rpc

import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.auth.TokenRefreshResponse
import com.getaltair.altair.rpc.AuthService
import org.slf4j.LoggerFactory

/**
 * Stub implementation of AuthService for infrastructure validation.
 *
 * Accepts any credentials and returns test tokens.
 * Real implementation will integrate with JWT and user repository in Phase 5+.
 */
class AuthServiceImpl : AuthService {
    private val logger = LoggerFactory.getLogger(AuthServiceImpl::class.java)

    override suspend fun login(request: AuthRequest): AuthResponse {
        logger.warn("STUB: AuthService.login() accepting any credentials for {}", request.email)
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
        logger.warn("STUB: AuthService.refresh() returning new token without validation")
        return TokenRefreshResponse(
            accessToken = "test-access-token-refreshed-${System.currentTimeMillis()}",
            expiresIn = 3600,
        )
    }

    override suspend fun logout() {
        logger.warn("STUB: AuthService.logout() is a no-op")
    }

    override suspend fun register(request: RegisterRequest): AuthResponse {
        logger.warn("STUB: AuthService.register() accepting any registration for {}", request.email)
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

package com.getaltair.altair.service.auth

import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.auth.TokenRefreshResponse
import com.getaltair.altair.rpc.PublicAuthService

/**
 * Fake implementation of PublicAuthService for testing.
 *
 * Allows configuring responses and tracking method calls.
 */
class FakePublicAuthService : PublicAuthService {
    var loginResponse: AuthResponse? = null
    var loginError: Exception? = null
    var loginCallCount = 0
    var lastLoginRequest: AuthRequest? = null

    var refreshResponse: TokenRefreshResponse? = null
    var refreshError: Exception? = null
    var refreshCallCount = 0
    var lastRefreshToken: String? = null
    var onRefresh: (() -> TokenRefreshResponse)? = null

    var registerResponse: AuthResponse? = null
    var registerError: Exception? = null
    var registerCallCount = 0
    var lastRegisterRequest: RegisterRequest? = null

    override suspend fun login(request: AuthRequest): AuthResponse {
        loginCallCount++
        lastLoginRequest = request
        loginError?.let { throw it }
        return loginResponse ?: error("No login response configured")
    }

    override suspend fun refresh(refreshToken: String): TokenRefreshResponse {
        refreshCallCount++
        lastRefreshToken = refreshToken
        refreshError?.let { throw it }
        return onRefresh?.invoke() ?: refreshResponse ?: error("No refresh response configured")
    }

    override suspend fun register(request: RegisterRequest): AuthResponse {
        registerCallCount++
        lastRegisterRequest = request
        registerError?.let { throw it }
        return registerResponse ?: error("No register response configured")
    }

    fun reset() {
        loginResponse = null
        loginError = null
        loginCallCount = 0
        lastLoginRequest = null
        refreshResponse = null
        refreshError = null
        refreshCallCount = 0
        lastRefreshToken = null
        onRefresh = null
        registerResponse = null
        registerError = null
        registerCallCount = 0
        lastRegisterRequest = null
    }
}

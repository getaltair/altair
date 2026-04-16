package com.getaltair.altair.data.repository

import com.getaltair.altair.data.auth.TokenPreferences
import com.getaltair.altair.data.network.AuthApi
import com.getaltair.altair.data.network.LoginRequest
import com.getaltair.altair.data.network.RegisterRequest
import com.getaltair.altair.data.sync.SyncCoordinator
import com.getaltair.altair.domain.repository.AuthRepository

class AuthRepositoryImpl(
    private val authApi: AuthApi,
    private val tokenPreferences: TokenPreferences,
    private val syncCoordinator: SyncCoordinator,
) : AuthRepository {
    override suspend fun login(
        email: String,
        password: String,
    ) {
        val response = authApi.login(LoginRequest(email, password))
        tokenPreferences.accessToken = response.accessToken
        tokenPreferences.refreshToken = response.refreshToken
        syncCoordinator.startSync()
    }

    override suspend fun register(
        email: String,
        password: String,
        displayName: String,
    ) {
        val response = authApi.register(RegisterRequest(email, password, displayName))
        tokenPreferences.accessToken = response.accessToken
        tokenPreferences.refreshToken = response.refreshToken
        syncCoordinator.startSync()
    }

    override suspend fun logout() {
        syncCoordinator.stopSync()
        tokenPreferences.clearTokens()
    }
}

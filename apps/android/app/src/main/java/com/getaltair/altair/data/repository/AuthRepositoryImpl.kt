package com.getaltair.altair.data.repository

import android.util.Log
import com.getaltair.altair.data.auth.TokenPreferences
import com.getaltair.altair.data.network.AuthApi
import com.getaltair.altair.data.network.LoginRequest
import com.getaltair.altair.data.network.RegisterRequest
import com.getaltair.altair.data.sync.SyncCoordinator
import com.getaltair.altair.domain.repository.AuthRepository

private const val TAG = "AuthRepositoryImpl"

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
        try {
            syncCoordinator.stopSync()
        } catch (e: Exception) {
            Log.w(TAG, "stopSync failed during logout", e)
        } finally {
            tokenPreferences.clearTokens()
        }
    }
}

package com.getaltair.altair.data.auth

import com.getaltair.altair.data.network.AuthApi
import com.getaltair.altair.data.network.RefreshRequest
import kotlinx.coroutines.runBlocking
import okhttp3.Authenticator
import okhttp3.Request
import okhttp3.Response
import okhttp3.Route

class AuthAuthenticator(
    private val tokenPreferences: TokenPreferences,
    private val authApi: AuthApi,
) : Authenticator {
    override fun authenticate(
        route: Route?,
        response: Response,
    ): Request? {
        // Prevent infinite retry loop
        if (response.priorResponse?.code == 401) {
            tokenPreferences.clearTokens()
            return null
        }

        val refreshToken =
            tokenPreferences.refreshToken ?: run {
                tokenPreferences.clearTokens()
                return null
            }

        return runBlocking {
            try {
                val newTokens = authApi.refresh(RefreshRequest(refreshToken))
                tokenPreferences.accessToken = newTokens.accessToken
                tokenPreferences.refreshToken = newTokens.refreshToken
                response.request
                    .newBuilder()
                    .header("Authorization", "Bearer ${newTokens.accessToken}")
                    .build()
            } catch (e: Exception) {
                tokenPreferences.clearTokens()
                null
            }
        }
    }
}

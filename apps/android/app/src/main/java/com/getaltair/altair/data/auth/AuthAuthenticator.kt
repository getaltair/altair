package com.getaltair.altair.data.auth

import android.util.Log
import com.getaltair.altair.data.network.AuthApi
import com.getaltair.altair.data.network.RefreshRequest
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.runBlocking
import okhttp3.Authenticator
import okhttp3.Request
import okhttp3.Response
import okhttp3.Route
import retrofit2.HttpException
import java.io.IOException

private const val TAG = "AuthAuthenticator"

// runBlocking is intentional here: OkHttp's Authenticator is synchronous by contract
// (runs on OkHttp's thread pool, never the main thread). This is the standard bridge.
class AuthAuthenticator(
    private val tokenPreferences: TokenPreferences,
    private val authApi: AuthApi,
) : Authenticator {
    override fun authenticate(
        route: Route?,
        response: Response,
    ): Request? {
        // Walk the full prior response chain to prevent infinite retry loops
        var prior = response.priorResponse
        while (prior != null) {
            if (prior.code == 401) {
                tokenPreferences.clearTokens()
                return null
            }
            prior = prior.priorResponse
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
            } catch (e: CancellationException) {
                throw e
            } catch (e: HttpException) {
                Log.w(TAG, "Token refresh rejected (HTTP ${e.code()}), clearing tokens", e)
                tokenPreferences.clearTokens()
                null
            } catch (e: IOException) {
                Log.w(TAG, "Token refresh failed due to network error", e)
                tokenPreferences.clearTokens()
                null
            } catch (e: Exception) {
                Log.w(TAG, "Token refresh failed unexpectedly", e)
                tokenPreferences.clearTokens()
                null
            }
        }
    }
}

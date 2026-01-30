package com.getaltair.altair.auth

import com.getaltair.altair.api.TokenPair
import com.getaltair.altair.api.TokenProvider
import platform.Foundation.NSUserDefaults

/**
 * iOS TokenProvider using NSUserDefaults.
 *
 * Note: For production, consider using Keychain via expect/actual pattern
 * or a KMP keychain library. NSUserDefaults is acceptable for development
 * but tokens should ideally be stored in Keychain for better security.
 *
 * TODO: Migrate to Keychain for production security (see ADR-018)
 */
class IosTokenProvider(
    override val serverUrl: String = "http://localhost:8080"
) : TokenProvider {

    private val defaults = NSUserDefaults.standardUserDefaults

    override suspend fun getTokens(): TokenPair? {
        val accessToken = defaults.stringForKey(KEY_ACCESS_TOKEN)
        val refreshToken = defaults.stringForKey(KEY_REFRESH_TOKEN)
        return if (accessToken != null && refreshToken != null) {
            TokenPair(accessToken, refreshToken)
        } else null
    }

    override suspend fun refresh(): TokenPair? {
        // Return current tokens; Ktor bearer auth handles actual refresh
        return getTokens()
    }

    override suspend fun storeTokens(tokens: TokenPair) {
        defaults.setObject(tokens.accessToken, KEY_ACCESS_TOKEN)
        defaults.setObject(tokens.refreshToken, KEY_REFRESH_TOKEN)
        defaults.synchronize()
    }

    override suspend fun clearTokens() {
        defaults.removeObjectForKey(KEY_ACCESS_TOKEN)
        defaults.removeObjectForKey(KEY_REFRESH_TOKEN)
        defaults.synchronize()
    }

    companion object {
        private const val KEY_ACCESS_TOKEN = "com.getaltair.altair.access_token"
        private const val KEY_REFRESH_TOKEN = "com.getaltair.altair.refresh_token"
    }
}

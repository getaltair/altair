package com.getaltair.altair.auth

import android.content.Context
import android.content.SharedPreferences
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import com.getaltair.altair.api.TokenPair
import com.getaltair.altair.api.TokenProvider
import java.io.File

/**
 * Android TokenProvider using EncryptedSharedPreferences for secure token storage.
 *
 * Uses Android Keystore-backed encryption via MasterKey with AES256_GCM scheme.
 * Both keys and values are encrypted:
 * - Keys: AES256_SIV (deterministic encryption for lookups)
 * - Values: AES256_GCM (authenticated encryption)
 *
 * Requires API 23+ (minSdk 24 satisfies this requirement).
 *
 * @param context Android application context
 * @param serverUrl Base URL for the Altair server (defaults to Android emulator localhost)
 */
class AndroidTokenProvider(
    context: Context,
    override val serverUrl: String = "http://10.0.2.2:8080" // localhost from emulator
) : TokenProvider {

    private val masterKey = MasterKey.Builder(context)
        .setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
        .build()

    private val prefs: SharedPreferences = EncryptedSharedPreferences.create(
        context,
        "altair_auth_prefs",
        masterKey,
        EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
        EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
    )

    override suspend fun getTokens(): TokenPair? {
        val accessToken = prefs.getString(KEY_ACCESS_TOKEN, null)
        val refreshToken = prefs.getString(KEY_REFRESH_TOKEN, null)
        return if (accessToken != null && refreshToken != null) {
            TokenPair(accessToken, refreshToken)
        } else null
    }

    /**
     * Returns current stored tokens.
     *
     * Note: The actual token refresh logic is handled by the Ktor BearerAuth plugin
     * in HttpClientFactory. This method simply returns the currently stored tokens
     * which the plugin will use to make the refresh request.
     */
    override suspend fun refresh(): TokenPair? {
        val refreshToken = prefs.getString(KEY_REFRESH_TOKEN, null) ?: return null
        // The bearer auth plugin handles the actual refresh API call
        return getTokens()
    }

    override suspend fun storeTokens(tokens: TokenPair) {
        prefs.edit()
            .putString(KEY_ACCESS_TOKEN, tokens.accessToken)
            .putString(KEY_REFRESH_TOKEN, tokens.refreshToken)
            .apply()
    }

    override suspend fun clearTokens() {
        prefs.edit()
            .remove(KEY_ACCESS_TOKEN)
            .remove(KEY_REFRESH_TOKEN)
            .apply()
    }

    companion object {
        private const val KEY_ACCESS_TOKEN = "access_token"
        private const val KEY_REFRESH_TOKEN = "refresh_token"
    }
}

package com.getaltair.altair.auth

import com.getaltair.altair.api.TokenPair
import com.getaltair.altair.api.TokenProvider
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import java.io.File

/**
 * Desktop TokenProvider using file-based storage.
 * Stores tokens in user's config directory.
 *
 * Note: For production, consider using a proper secrets management
 * library or OS keychain integration. This implementation stores
 * tokens in a JSON file which should have restricted permissions.
 *
 * TODO: Migrate to OS keychain integration for production security (see ADR-018)
 */
class DesktopTokenProvider(
    override val serverUrl: String = "http://localhost:8080"
) : TokenProvider {

    private val json = Json {
        ignoreUnknownKeys = true
        prettyPrint = true
    }

    private val configDir: File by lazy {
        val userHome = System.getProperty("user.home")
        val configPath = when {
            System.getProperty("os.name").lowercase().contains("mac") ->
                "$userHome/Library/Application Support/Altair"
            System.getProperty("os.name").lowercase().contains("win") ->
                "${System.getenv("APPDATA")}/Altair"
            else -> "$userHome/.config/altair"
        }
        File(configPath).also { it.mkdirs() }
    }

    private val tokenFile: File get() = File(configDir, "tokens.json")

    override suspend fun getTokens(): TokenPair? {
        return try {
            if (tokenFile.exists()) {
                val stored = json.decodeFromString<StoredTokens>(tokenFile.readText())
                TokenPair(stored.accessToken, stored.refreshToken)
            } else null
        } catch (e: Exception) {
            null
        }
    }

    override suspend fun refresh(): TokenPair? {
        // Return current tokens; Ktor bearer auth handles actual refresh
        return getTokens()
    }

    override suspend fun storeTokens(tokens: TokenPair) {
        val stored = StoredTokens(tokens.accessToken, tokens.refreshToken)
        tokenFile.writeText(json.encodeToString(stored))
        // Set restrictive permissions on Unix-like systems
        try {
            tokenFile.setReadable(false, false)
            tokenFile.setReadable(true, true)
            tokenFile.setWritable(false, false)
            tokenFile.setWritable(true, true)
        } catch (_: Exception) {
            // Ignore permission errors on Windows
        }
    }

    override suspend fun clearTokens() {
        if (tokenFile.exists()) {
            tokenFile.delete()
        }
    }

    @Serializable
    private data class StoredTokens(
        val accessToken: String,
        val refreshToken: String
    )
}

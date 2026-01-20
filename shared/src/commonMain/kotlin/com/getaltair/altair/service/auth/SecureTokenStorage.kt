package com.getaltair.altair.service.auth

import com.getaltair.altair.domain.types.Ulid

/**
 * Secure storage interface for authentication tokens.
 *
 * Platform implementations should use the most secure storage mechanism available:
 * - Android: EncryptedSharedPreferences with Android Keystore
 * - iOS: Keychain Services with kSecAttrAccessibleWhenUnlockedThisDeviceOnly
 * - Desktop: Java Keystore or system credential manager
 *
 * All implementations must:
 * - Encrypt tokens at rest
 * - Not back up tokens to cloud services
 * - Clear tokens on app uninstall (where possible)
 * - Handle storage failures gracefully
 */
interface SecureTokenStorage {
    /**
     * Store the access token securely.
     *
     * @param token The JWT access token
     */
    suspend fun saveAccessToken(token: String)

    /**
     * Retrieve the stored access token.
     *
     * @return The access token, or null if not stored
     */
    suspend fun getAccessToken(): String?

    /**
     * Store the refresh token securely.
     *
     * @param token The refresh token
     */
    suspend fun saveRefreshToken(token: String)

    /**
     * Retrieve the stored refresh token.
     *
     * @return The refresh token, or null if not stored
     */
    suspend fun getRefreshToken(): String?

    /**
     * Store the token expiration timestamp.
     *
     * @param expiresAtMillis Epoch milliseconds when the access token expires
     */
    suspend fun saveTokenExpiration(expiresAtMillis: Long)

    /**
     * Retrieve the token expiration timestamp.
     *
     * @return Epoch milliseconds when the token expires, or null if not stored
     */
    suspend fun getTokenExpiration(): Long?

    /**
     * Store the authenticated user's ID.
     *
     * @param userId The user's unique identifier
     */
    suspend fun saveUserId(userId: Ulid)

    /**
     * Retrieve the authenticated user's ID.
     *
     * @return The user's unique identifier, or null if not stored
     */
    suspend fun getUserId(): Ulid?

    /**
     * Clear all stored authentication data.
     *
     * Should be called on logout to ensure no credentials remain.
     */
    suspend fun clear()

    /**
     * Check if any authentication data is stored.
     *
     * @return true if at least a refresh token exists
     */
    suspend fun hasStoredCredentials(): Boolean
}

package com.getaltair.altair.service.auth

/**
 * Interface for platform-specific credential store providers.
 *
 * Each platform implementation uses native credential storage:
 * - macOS: Keychain Services
 * - Windows: Credential Manager (advapi32.dll)
 * - Linux: Secret Service (libsecret / GNOME Keyring / KDE Wallet)
 *
 * All implementations store credentials with:
 * - Service name: "com.getaltair.altair"
 * - Account/label: the key name (e.g., "access_token", "refresh_token")
 */
interface CredentialStoreProvider {
    /**
     * The name of this credential store provider (for logging/debugging).
     */
    val name: String

    /**
     * Check if this credential store is available on the current system.
     *
     * @return true if the native library is loaded and operational
     */
    fun isAvailable(): Boolean

    /**
     * Store a credential in the native credential store.
     *
     * @param key The credential key (e.g., "access_token")
     * @param value The credential value to store
     * @return true if the credential was stored successfully
     */
    fun store(
        key: String,
        value: String,
    ): Boolean

    /**
     * Retrieve a credential from the native credential store.
     *
     * @param key The credential key to look up
     * @return The credential value, or null if not found
     */
    fun retrieve(key: String): String?

    /**
     * Delete a credential from the native credential store.
     *
     * @param key The credential key to delete
     * @return true if the credential was deleted (or didn't exist)
     */
    fun delete(key: String): Boolean

    companion object {
        /**
         * Service name used for all Altair credentials.
         */
        const val SERVICE_NAME = "com.getaltair.altair"
    }
}

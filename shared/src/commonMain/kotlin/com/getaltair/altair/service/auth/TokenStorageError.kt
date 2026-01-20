package com.getaltair.altair.service.auth

/**
 * Represents errors that can occur during token storage operations.
 *
 * These are expected operational failures, not programmer errors.
 */
sealed class TokenStorageError {
    /**
     * Encryption or decryption operation failed.
     *
     * Possible causes:
     * - Key derivation failure
     * - Cipher initialization failure
     * - Data corruption during encryption
     */
    data class EncryptionFailed(
        val message: String,
    ) : TokenStorageError()

    /**
     * Failed to persist data to storage.
     *
     * Possible causes:
     * - Disk full
     * - Permission denied
     * - Backing store unavailable
     */
    data class PersistenceFailed(
        val message: String,
    ) : TokenStorageError()

    /**
     * Invalid data provided or retrieved.
     *
     * Possible causes:
     * - Malformed data
     * - Invalid encoding
     * - Validation failure
     */
    data class ValidationFailed(
        val message: String,
    ) : TokenStorageError()

    /**
     * Unknown or unexpected error occurred.
     */
    data class Unknown(
        val message: String,
    ) : TokenStorageError()
}

package com.getaltair.altair.service.auth

/**
 * Exception thrown when token storage operations fail.
 *
 * This exception indicates a critical failure in storing or retrieving authentication tokens.
 * Common causes:
 * - Encryption/decryption failures
 * - Disk full or permission denied when persisting
 * - Invalid token data
 * - Secure storage unavailable
 */
class TokenStorageException(
    message: String,
    cause: Throwable? = null,
) : Exception(message, cause)

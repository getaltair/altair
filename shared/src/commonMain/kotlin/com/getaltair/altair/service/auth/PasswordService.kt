package com.getaltair.altair.service.auth

/**
 * Service for secure password hashing and verification.
 *
 * Implementations should use a memory-hard password hashing algorithm
 * such as Argon2id as recommended by OWASP.
 *
 * This interface is defined in commonMain to allow for platform-specific
 * implementations (e.g., Argon2 on JVM, native implementations on iOS).
 */
interface PasswordService {
    /**
     * Hash a plaintext password for secure storage.
     *
     * @param password The plaintext password to hash
     * @return The hashed password suitable for storage
     */
    fun hash(password: String): String

    /**
     * Verify a plaintext password against a stored hash.
     *
     * @param password The plaintext password to verify
     * @param hash The stored password hash
     * @return true if the password matches the hash, false otherwise
     */
    fun verify(
        password: String,
        hash: String,
    ): Boolean
}

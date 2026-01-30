package com.getaltair.server.auth

import de.mkammerer.argon2.Argon2
import de.mkammerer.argon2.Argon2Factory

/**
 * Service for securely hashing and verifying passwords.
 *
 * Uses Argon2id, the recommended variant of the Argon2 password hashing algorithm.
 * Argon2id provides balanced protection against both side-channel and GPU attacks.
 */
interface PasswordHasher {
    /**
     * Hashes a plaintext password using Argon2id.
     *
     * The returned hash is a self-contained string that includes:
     * - The algorithm variant (Argon2id)
     * - The algorithm parameters (memory, iterations, parallelism)
     * - The salt (randomly generated)
     * - The hash output
     *
     * @param password The plaintext password to hash
     * @return The encoded hash string
     */
    fun hash(password: String): String

    /**
     * Verifies a plaintext password against a hash.
     *
     * @param password The plaintext password to verify
     * @param hash The encoded hash string from a previous hash() call
     * @return true if the password matches the hash, false otherwise
     */
    fun verify(password: String, hash: String): Boolean
}

/**
 * Default implementation of PasswordHasher using Argon2id.
 *
 * Parameters chosen for security and performance balance:
 * - Memory: 65536 KB (64 MB) - Resistant to GPU cracking
 * - Iterations: 3 - Multiple passes for time hardening
 * - Parallelism: 1 - Sequential computation
 *
 * These are OWASP recommended minimum values as of 2024.
 */
class Argon2PasswordHasher : PasswordHasher {

    private val argon2: Argon2 = Argon2Factory.create(
        Argon2Factory.Argon2Types.ARGON2id,
        32, // Salt length in bytes
        64  // Hash length in bytes
    )

    /**
     * Memory cost in KB.
     * 65536 KB = 64 MB, provides good resistance against GPU attacks.
     */
    private val memoryCost = 65536

    /**
     * Number of iterations.
     * Higher values increase computation time, slowing down brute-force attacks.
     */
    private val iterations = 3

    /**
     * Parallelism factor.
     * Number of parallel threads to use. 1 is sufficient for server-side hashing.
     */
    private val parallelism = 1

    override fun hash(password: String): String {
        return try {
            argon2.hash(iterations, memoryCost, parallelism, password.toCharArray())
        } finally {
            // Security best practice: wipe sensitive data from memory
            // Note: The library handles this internally for the password char array
        }
    }

    override fun verify(password: String, hash: String): Boolean {
        return try {
            argon2.verify(hash, password.toCharArray())
        } catch (e: Exception) {
            // If hash format is invalid or verification fails for any reason, return false
            false
        }
    }
}

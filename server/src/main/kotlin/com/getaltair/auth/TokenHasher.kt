package com.getaltair.auth

import java.security.MessageDigest

/**
 * Utility for hashing refresh tokens with SHA-256.
 *
 * We store the hash of refresh tokens in the database, not the raw token.
 * This provides an additional layer of security in case of database compromise.
 */
object TokenHasher {
    /**
     * Hash a token using SHA-256.
     *
     * @param token The raw token to hash
     * @return The hex-encoded SHA-256 hash
     */
    fun hash(token: String): String {
        val digest = MessageDigest.getInstance("SHA-256")
        val hashBytes = digest.digest(token.toByteArray(Charsets.UTF_8))
        return hashBytes.joinToString("") { "%02x".format(it) }
    }
}

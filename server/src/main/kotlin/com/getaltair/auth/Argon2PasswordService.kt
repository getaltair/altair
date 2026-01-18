package com.getaltair.auth

import com.getaltair.altair.service.auth.PasswordService
import de.mkammerer.argon2.Argon2Factory

/**
 * Argon2id implementation of PasswordService.
 *
 * Uses OWASP-recommended parameters:
 * - Memory: 64 MB (65536 KB)
 * - Iterations: 3
 * - Parallelism: 4
 *
 * Argon2id is recommended over Argon2i/Argon2d as it provides
 * resistance against both side-channel and GPU cracking attacks.
 */
class Argon2PasswordService : PasswordService {
    private val argon2 = Argon2Factory.create(Argon2Factory.Argon2Types.ARGON2id)

    override fun hash(password: String): String =
        argon2.hash(
            ITERATIONS,
            MEMORY_KB,
            PARALLELISM,
            password.toCharArray(),
        )

    override fun verify(
        password: String,
        hash: String,
    ): Boolean = argon2.verify(hash, password.toCharArray())

    companion object {
        private const val ITERATIONS = 3
        private const val MEMORY_KB = 65_536 // 64 MB
        private const val PARALLELISM = 4
    }
}

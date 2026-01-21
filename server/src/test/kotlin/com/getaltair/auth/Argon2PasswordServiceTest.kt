package com.getaltair.auth

import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.booleans.shouldBeFalse
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.shouldNotBe
import io.kotest.matchers.string.shouldStartWith

/**
 * Tests for Argon2PasswordService using Argon2id algorithm.
 *
 * Verifies:
 * - Password hashing produces unique hashes (random salt)
 * - Password verification works correctly
 * - Edge cases (empty passwords, unicode characters)
 * - Hash format compliance (Argon2id)
 */
class Argon2PasswordServiceTest :
    DescribeSpec({
        val passwordService = Argon2PasswordService()

        describe("password hashing") {
            it("produces different output for same password") {
                val password = "testPassword123!"
                val hash1 = passwordService.hash(password)
                val hash2 = passwordService.hash(password)

                // Each hash should be unique due to random salt
                hash1 shouldNotBe hash2
            }

            it("produces argon2id format") {
                val password = "testPassword"
                val hash = passwordService.hash(password)

                hash shouldStartWith "\$argon2id\$"
            }
        }

        describe("password verification") {
            it("returns true for correct password") {
                val password = "correctPassword!"
                val hash = passwordService.hash(password)

                passwordService.verify(password, hash).shouldBeTrue()
            }

            it("returns false for incorrect password") {
                val correctPassword = "correctPassword!"
                val wrongPassword = "wrongPassword!"
                val hash = passwordService.hash(correctPassword)

                passwordService.verify(wrongPassword, hash).shouldBeFalse()
            }

            it("handles empty password") {
                val password = ""
                val hash = passwordService.hash(password)

                passwordService.verify(password, hash).shouldBeTrue()
                passwordService.verify("notEmpty", hash).shouldBeFalse()
            }

            it("handles unicode characters") {
                val password = "пароль密码🔐"
                val hash = passwordService.hash(password)

                passwordService.verify(password, hash).shouldBeTrue()
                passwordService.verify("wrong", hash).shouldBeFalse()
            }
        }
    })

package com.getaltair.auth

import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertNotEquals
import kotlin.test.assertTrue

class Argon2PasswordServiceTest {
    private val passwordService = Argon2PasswordService()

    @Test
    fun `hash produces different output for same password`() {
        val password = "testPassword123!"
        val hash1 = passwordService.hash(password)
        val hash2 = passwordService.hash(password)

        // Each hash should be unique due to random salt
        assertNotEquals(hash1, hash2)
    }

    @Test
    fun `verify returns true for correct password`() {
        val password = "correctPassword!"
        val hash = passwordService.hash(password)

        assertTrue(passwordService.verify(password, hash))
    }

    @Test
    fun `verify returns false for incorrect password`() {
        val correctPassword = "correctPassword!"
        val wrongPassword = "wrongPassword!"
        val hash = passwordService.hash(correctPassword)

        assertFalse(passwordService.verify(wrongPassword, hash))
    }

    @Test
    fun `verify handles empty password`() {
        val password = ""
        val hash = passwordService.hash(password)

        assertTrue(passwordService.verify(password, hash))
        assertFalse(passwordService.verify("notEmpty", hash))
    }

    @Test
    fun `verify handles unicode characters`() {
        val password = "пароль密码🔐"
        val hash = passwordService.hash(password)

        assertTrue(passwordService.verify(password, hash))
        assertFalse(passwordService.verify("wrong", hash))
    }

    @Test
    fun `hash produces argon2id format`() {
        val password = "testPassword"
        val hash = passwordService.hash(password)

        assertTrue(hash.startsWith("\$argon2id\$"))
    }
}

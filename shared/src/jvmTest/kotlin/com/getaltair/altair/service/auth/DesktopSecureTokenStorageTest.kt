package com.getaltair.altair.service.auth

import com.getaltair.altair.domain.types.Ulid
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.booleans.shouldBeFalse
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.nulls.shouldNotBeNull
import io.kotest.matchers.shouldBe
import io.kotest.matchers.shouldNotBe
import io.kotest.matchers.string.shouldMatch
import java.util.prefs.Preferences

/**
 * Tests for DesktopSecureTokenStorage.
 *
 * Verifies:
 * - Token storage and retrieval
 * - Encryption/decryption round-trip
 * - Clear functionality
 * - hasStoredCredentials behavior
 */
class DesktopSecureTokenStorageTest :
    BehaviorSpec({
        lateinit var storage: DesktopSecureTokenStorage
        val testAppName = "AltairTest-${System.nanoTime()}"

        beforeEach {
            storage = DesktopSecureTokenStorage(appName = testAppName)
        }

        afterEach {
            storage.clear()
        }

        given("token storage round-trips") {
            `when`("storing and retrieving access token") {
                then("token is retrieved correctly") {
                    val token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test-access-token"

                    storage.saveAccessToken(token)
                    val retrieved = storage.getAccessToken()

                    retrieved shouldBe token
                }
            }

            `when`("storing and retrieving refresh token") {
                then("token is retrieved correctly") {
                    val token = "refresh-token-abc123-xyz789"

                    storage.saveRefreshToken(token)
                    val retrieved = storage.getRefreshToken()

                    retrieved shouldBe token
                }
            }

            `when`("storing and retrieving token expiration") {
                then("expiration is retrieved correctly") {
                    val expiration = 1_705_500_900_000L

                    storage.saveTokenExpiration(expiration)
                    val retrieved = storage.getTokenExpiration()

                    retrieved shouldBe expiration
                }
            }

            `when`("storing and retrieving user id") {
                then("user id is retrieved correctly") {
                    val userId = Ulid("01HW0ABCD00000000000000001")

                    storage.saveUserId(userId)
                    val retrieved = storage.getUserId()

                    retrieved shouldBe userId
                }
            }
        }

        given("null retrieval") {
            `when`("no access token stored") {
                then("returns null") {
                    storage.getAccessToken().shouldBeNull()
                }
            }

            `when`("no refresh token stored") {
                then("returns null") {
                    storage.getRefreshToken().shouldBeNull()
                }
            }

            `when`("no token expiration stored") {
                then("returns null") {
                    storage.getTokenExpiration().shouldBeNull()
                }
            }

            `when`("no user id stored") {
                then("returns null") {
                    storage.getUserId().shouldBeNull()
                }
            }
        }

        given("clear functionality") {
            `when`("clearing all stored data") {
                then("all values are removed") {
                    storage.saveAccessToken("access-token")
                    storage.saveRefreshToken("refresh-token")
                    storage.saveTokenExpiration(1_705_500_900_000L)
                    storage.saveUserId(Ulid("01HW0ABCD00000000000000001"))

                    storage.clear()

                    storage.getAccessToken().shouldBeNull()
                    storage.getRefreshToken().shouldBeNull()
                    storage.getTokenExpiration().shouldBeNull()
                    storage.getUserId().shouldBeNull()
                }
            }
        }

        given("hasStoredCredentials") {
            `when`("refresh token exists") {
                then("returns true") {
                    storage.saveRefreshToken("refresh-token")

                    storage.hasStoredCredentials().shouldBeTrue()
                }
            }

            `when`("no refresh token exists") {
                then("returns false") {
                    storage.saveAccessToken("access-token")

                    storage.hasStoredCredentials().shouldBeFalse()
                }
            }

            `when`("cleared after storing refresh token") {
                then("returns false") {
                    storage.saveRefreshToken("refresh-token")
                    storage.clear()

                    storage.hasStoredCredentials().shouldBeFalse()
                }
            }
        }

        given("token replacement") {
            `when`("overwriting token") {
                then("replaces previous value") {
                    storage.saveAccessToken("original-token")
                    storage.saveAccessToken("updated-token")

                    storage.getAccessToken() shouldBe "updated-token"
                }
            }
        }

        given("special characters and edge cases") {
            `when`("storing tokens with special characters") {
                then("characters are preserved") {
                    val tokenWithSpecialChars = "token-with-special-chars!@#\$%^&*()_+-=[]{}|;':\",./<>?"

                    storage.saveAccessToken(tokenWithSpecialChars)
                    val retrieved = storage.getAccessToken()

                    retrieved shouldBe tokenWithSpecialChars
                }
            }

            `when`("storing long tokens") {
                then("long tokens are stored correctly") {
                    val longToken = "a".repeat(4096)

                    storage.saveAccessToken(longToken)
                    val retrieved = storage.getAccessToken()

                    retrieved shouldBe longToken
                }
            }

            `when`("storing different ulid values") {
                then("values are stored correctly") {
                    val differentUserId = Ulid("01HW0ABCD99999999999999999")

                    storage.saveUserId(differentUserId)
                    val retrieved = storage.getUserId()

                    retrieved shouldBe differentUserId
                }
            }

            `when`("storing empty string token") {
                then("empty string is stored and retrieved") {
                    val emptyToken = ""

                    storage.saveAccessToken(emptyToken)
                    val retrieved = storage.getAccessToken()

                    retrieved shouldBe emptyToken
                }
            }

            `when`("storing token expiration boundary values") {
                then("boundary values work correctly") {
                    storage.saveTokenExpiration(Long.MAX_VALUE)
                    storage.getTokenExpiration() shouldBe Long.MAX_VALUE

                    storage.saveTokenExpiration(0L)
                    storage.getTokenExpiration() shouldBe 0L
                }
            }
        }

        given("encryption verification") {
            `when`("storing sensitive data") {
                then("data is not stored as plaintext") {
                    val token = "sensitive-access-token-12345"

                    storage.saveAccessToken(token)

                    val prefs = Preferences.userNodeForPackage(DesktopSecureTokenStorage::class.java)
                    val storedValue = prefs.get("access_token", null)

                    storedValue.shouldNotBeNull()
                    storedValue shouldNotBe token
                    storedValue shouldMatch Regex("^[A-Za-z0-9+/=]+$")
                }
            }
        }
    })

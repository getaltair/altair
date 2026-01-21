package com.getaltair.altair.service.auth

import com.getaltair.altair.domain.types.Ulid
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.booleans.shouldBeFalse
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe

/**
 * Tests for IosSecureTokenStorage.
 *
 * These tests must run on an iOS simulator or device because they require:
 * - iOS Keychain Services for secure storage
 *
 * Run with: ./gradlew :shared:iosSimulatorArm64Test
 */
class IosSecureTokenStorageTest :
    BehaviorSpec({
        lateinit var storage: IosSecureTokenStorage

        beforeEach {
            storage = IosSecureTokenStorage(serviceName = "com.getaltair.altair.test")
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
                    val expiration = TEST_EXPIRATION_TIMESTAMP

                    storage.saveTokenExpiration(expiration)
                    val retrieved = storage.getTokenExpiration()

                    retrieved shouldBe expiration
                }
            }

            `when`("storing and retrieving user id") {
                then("user id is retrieved correctly") {
                    val userId = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV")

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
                    storage.saveTokenExpiration(TEST_EXPIRATION_TIMESTAMP)
                    storage.saveUserId(Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV"))

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
                    val longToken = "a".repeat(LONG_TOKEN_LENGTH)

                    storage.saveAccessToken(longToken)
                    val retrieved = storage.getAccessToken()

                    retrieved shouldBe longToken
                }
            }

            `when`("storing different ulid values") {
                then("values are stored correctly") {
                    val userId1 = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV")
                    val userId2 = Ulid("01BRCD4EFGHTSV5SSGHR8H6GBX")

                    storage.saveUserId(userId1)
                    storage.getUserId() shouldBe userId1

                    storage.saveUserId(userId2)
                    storage.getUserId() shouldBe userId2
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
    }) {
    companion object {
        /** Test timestamp representing January 17, 2024 */
        private const val TEST_EXPIRATION_TIMESTAMP = 1_705_500_900_000L

        /** Long token length for stress testing */
        private const val LONG_TOKEN_LENGTH = 4096
    }
}

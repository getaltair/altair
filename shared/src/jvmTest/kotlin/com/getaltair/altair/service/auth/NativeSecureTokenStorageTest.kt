package com.getaltair.altair.service.auth

import com.getaltair.altair.domain.types.Ulid
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.booleans.shouldBeFalse
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.collections.shouldContain
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe

/**
 * Tests for NativeSecureTokenStorage.
 *
 * Uses a fake in-memory credential store provider to test the storage
 * wrapper without requiring actual native credential stores.
 */
class NativeSecureTokenStorageTest :
    BehaviorSpec({
        val fakeProvider = FakeCredentialStoreProvider()
        val storage = NativeSecureTokenStorage(fakeProvider)

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

            `when`("storing different ulid values") {
                then("values are stored correctly") {
                    val differentUserId = Ulid("01HW0ABCD99999999999999999")

                    storage.saveUserId(differentUserId)
                    val retrieved = storage.getUserId()

                    retrieved shouldBe differentUserId
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

        given("provider delegation") {
            `when`("saving access token") {
                then("delegates to provider store method") {
                    storage.saveAccessToken("token")

                    fakeProvider.storeCalled.shouldBeTrue()
                    fakeProvider.lastStoredKey shouldBe "access_token"
                    fakeProvider.lastStoredValue shouldBe "token"
                }
            }

            `when`("retrieving access token") {
                then("delegates to provider retrieve method") {
                    fakeProvider.store("access_token", "my-token")

                    storage.getAccessToken()

                    fakeProvider.retrieveCalled.shouldBeTrue()
                    fakeProvider.lastRetrievedKey shouldBe "access_token"
                }
            }

            `when`("clearing storage") {
                then("delegates to provider delete method for all keys") {
                    storage.clear()

                    fakeProvider.deletedKeys shouldContain "access_token"
                    fakeProvider.deletedKeys shouldContain "refresh_token"
                    fakeProvider.deletedKeys shouldContain "token_expiration"
                    fakeProvider.deletedKeys shouldContain "user_id"
                }
            }
        }
    })

/**
 * Fake in-memory credential store provider for testing.
 */
private class FakeCredentialStoreProvider : CredentialStoreProvider {
    override val name: String = "Fake Provider"

    private val credentials = mutableMapOf<String, String>()

    var storeCalled = false
    var retrieveCalled = false
    var lastStoredKey: String? = null
    var lastStoredValue: String? = null
    var lastRetrievedKey: String? = null
    val deletedKeys = mutableListOf<String>()

    override fun isAvailable(): Boolean = true

    override fun store(
        key: String,
        value: String,
    ): Boolean {
        storeCalled = true
        lastStoredKey = key
        lastStoredValue = value
        credentials[key] = value
        return true
    }

    override fun retrieve(key: String): String? {
        retrieveCalled = true
        lastRetrievedKey = key
        return credentials[key]
    }

    override fun delete(key: String): Boolean {
        deletedKeys.add(key)
        credentials.remove(key)
        return true
    }
}

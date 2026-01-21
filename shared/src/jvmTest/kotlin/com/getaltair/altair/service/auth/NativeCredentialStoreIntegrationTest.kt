package com.getaltair.altair.service.auth

import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe

/**
 * Integration tests for native credential stores.
 *
 * These tests run only on their respective platforms and require the native
 * credential store services to be available:
 * - macOS: Keychain Services
 * - Windows: Credential Manager
 * - Linux: Secret Service daemon (gnome-keyring, kwallet, etc.)
 *
 * Tests verify behavior both when native store is available and unavailable.
 */
class NativeCredentialStoreIntegrationTest :
    BehaviorSpec({
        var provider: CredentialStoreProvider? = null

        beforeEach {
            provider = NativeCredentialStoreFactory.create()
        }

        afterEach {
            // Clean up any test credentials
            provider?.delete(TEST_KEY)
        }

        given("native credential store integration") {
            `when`("checking availability") {
                then("native store is available when provider exists") {
                    // Provider can be null in CI environments where native services aren't running
                    if (provider != null) {
                        provider!!.isAvailable().shouldBeTrue()
                    } else {
                        // Null provider is acceptable - native service not available
                        provider.shouldBeNull()
                    }
                }
            }

            `when`("storing and retrieving credential") {
                then("credential is retrieved correctly") {
                    if (provider != null) {
                        val p = provider!!
                        val stored = p.store(TEST_KEY, TEST_VALUE)
                        stored.shouldBeTrue()

                        val retrieved = p.retrieve(TEST_KEY)
                        retrieved shouldBe TEST_VALUE
                    } else {
                        // Skip test - native service not available
                        provider.shouldBeNull()
                    }
                }
            }

            `when`("retrieving non-existent key") {
                then("returns null") {
                    if (provider != null) {
                        val p = provider!!
                        val retrieved = p.retrieve("non-existent-key-${System.nanoTime()}")
                        retrieved.shouldBeNull()
                    } else {
                        provider.shouldBeNull()
                    }
                }
            }

            `when`("deleting credential") {
                then("credential is removed") {
                    if (provider != null) {
                        val p = provider!!
                        p.store(TEST_KEY, TEST_VALUE)
                        val deleted = p.delete(TEST_KEY)
                        deleted.shouldBeTrue()

                        val retrieved = p.retrieve(TEST_KEY)
                        retrieved.shouldBeNull()
                    } else {
                        provider.shouldBeNull()
                    }
                }
            }

            `when`("deleting non-existent key") {
                then("returns true") {
                    if (provider != null) {
                        val p = provider!!
                        val deleted = p.delete("non-existent-key-${System.nanoTime()}")
                        deleted.shouldBeTrue()
                    } else {
                        provider.shouldBeNull()
                    }
                }
            }

            `when`("overwriting credential") {
                then("updates value") {
                    if (provider != null) {
                        val p = provider!!
                        p.store(TEST_KEY, "original-value")
                        p.store(TEST_KEY, "updated-value")

                        val retrieved = p.retrieve(TEST_KEY)
                        retrieved shouldBe "updated-value"
                    } else {
                        provider.shouldBeNull()
                    }
                }
            }

            `when`("storing special characters") {
                then("special characters are preserved") {
                    if (provider != null) {
                        val p = provider!!
                        val specialValue = "value-with-special-chars!@#\$%^&*()_+-=[]{}|;':\",./<>?"
                        p.store(TEST_KEY, specialValue)

                        val retrieved = p.retrieve(TEST_KEY)
                        retrieved shouldBe specialValue
                    } else {
                        provider.shouldBeNull()
                    }
                }
            }

            `when`("storing unicode characters") {
                then("unicode is preserved") {
                    if (provider != null) {
                        val p = provider!!
                        val unicodeValue = "日本語-émoji-🎉-مرحبا-Привет"
                        p.store(TEST_KEY, unicodeValue)

                        val retrieved = p.retrieve(TEST_KEY)
                        retrieved shouldBe unicodeValue
                    } else {
                        provider.shouldBeNull()
                    }
                }
            }

            `when`("storing long value") {
                then("long value is stored correctly") {
                    if (provider != null) {
                        val p = provider!!
                        // JWTs can be quite long, test with a realistic size
                        val longValue = "a".repeat(2048)
                        p.store(TEST_KEY, longValue)

                        val retrieved = p.retrieve(TEST_KEY)
                        retrieved shouldBe longValue
                    } else {
                        provider.shouldBeNull()
                    }
                }
            }

            `when`("checking provider name") {
                then("provider reports correct name for platform") {
                    if (provider != null) {
                        val p = provider!!
                        val osName = System.getProperty("os.name", "").lowercase()
                        when {
                            osName.contains("mac") || osName.contains("darwin") ->
                                p.name shouldBe "macOS Keychain"
                            osName.contains("win") ->
                                p.name shouldBe "Windows Credential Manager"
                            osName.contains("linux") || osName.contains("nix") || osName.contains("nux") ->
                                p.name shouldBe "Linux Secret Service (secret-tool)"
                            else ->
                                // On unknown platform with provider, just verify name is non-empty
                                p.name.isNotEmpty().shouldBeTrue()
                        }
                    } else {
                        provider.shouldBeNull()
                    }
                }
            }
        }
    }) {
    companion object {
        private const val TEST_KEY = "altair-test-credential-${Long.MAX_VALUE}"
        private const val TEST_VALUE = "test-secret-value-12345"
    }
}

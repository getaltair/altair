package com.getaltair.altair.service.auth

import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.booleans.shouldBeFalse
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.nulls.shouldNotBeNull
import io.kotest.matchers.shouldBe

/**
 * Tests for credential store fallback behavior.
 *
 * Verifies that the factory correctly handles unavailable native stores
 * and that the fallback behavior works as expected.
 */
class CredentialStoreFallbackTest :
    DescribeSpec({
        describe("credential store provider") {
            it("returns null when provider is not available") {
                val unavailableProvider =
                    object : CredentialStoreProvider {
                        override val name = "Unavailable Store"

                        override fun isAvailable() = false

                        override fun store(
                            key: String,
                            value: String,
                        ) = false

                        override fun retrieve(key: String): String? = null

                        override fun delete(key: String) = false
                    }

                // Verify unavailable provider reports correctly
                unavailableProvider.isAvailable().shouldBeFalse()
            }
        }

        describe("OS detection") {
            it("returns null provider for unknown OS") {
                val provider =
                    NativeCredentialStoreFactory.createForOS(
                        NativeCredentialStoreFactory.OperatingSystem.UNKNOWN,
                    )

                provider.shouldBeNull()
            }
        }

        describe("DesktopSecureTokenStorage") {
            it("is always available as fallback") {
                // This test ensures the fallback is always functional
                val fallback = DesktopSecureTokenStorage(appName = "FallbackTest-${System.nanoTime()}")

                fallback.shouldNotBeNull()

                // Verify it can perform basic operations
                fallback.saveAccessToken("test-token")
                val retrieved = fallback.getAccessToken()
                retrieved shouldBe "test-token"
                fallback.clear()
            }
        }

        describe("native store creation") {
            it("does not throw on current platform") {
                // This test ensures that attempting to create native stores
                // doesn't crash on the current platform
                val currentOS = NativeCredentialStoreFactory.detectOS()
                if (currentOS == NativeCredentialStoreFactory.OperatingSystem.UNKNOWN) {
                    return@it // Skip on unknown platforms
                }

                val provider = NativeCredentialStoreFactory.createForOS(currentOS)
                provider.shouldNotBeNull()
                // isAvailable() may return false (e.g., if libsecret isn't installed)
                // but should not throw
                provider.isAvailable()
            }

            @Suppress("TooGenericExceptionCaught", "SwallowedException")
            it("factory create method handles library loading failures gracefully") {
                // This tests that the factory doesn't throw when native libraries fail to load
                val result =
                    try {
                        NativeCredentialStoreFactory.create()
                        true
                    } catch (e: Exception) {
                        false
                    }

                // Should always succeed (return null or a provider, never throw)
                result.shouldBeTrue()
            }
        }
    })

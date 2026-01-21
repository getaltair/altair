package com.getaltair.altair.service.auth

import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.nulls.shouldNotBeNull
import io.kotest.matchers.shouldBe

/**
 * Tests for NativeCredentialStoreFactory.
 *
 * Tests platform detection and provider creation logic.
 */
class NativeCredentialStoreFactoryTest :
    BehaviorSpec({
        given("platform detection") {
            `when`("detecting current OS") {
                then("returns correct OS for current platform") {
                    val os = NativeCredentialStoreFactory.detectOS()

                    // Should return something other than UNKNOWN on supported platforms
                    val osName = System.getProperty("os.name", "").lowercase()
                    when {
                        osName.contains("mac") || osName.contains("darwin") ->
                            os shouldBe NativeCredentialStoreFactory.OperatingSystem.MACOS
                        osName.contains("win") ->
                            os shouldBe NativeCredentialStoreFactory.OperatingSystem.WINDOWS
                        osName.contains("linux") || osName.contains("nix") || osName.contains("nux") ->
                            os shouldBe NativeCredentialStoreFactory.OperatingSystem.LINUX
                    }
                }
            }
        }

        given("provider creation") {
            `when`("creating provider for current OS") {
                then("returns correct provider type") {
                    // Only test the current platform to avoid JNA library loading errors
                    val currentOS = NativeCredentialStoreFactory.detectOS()
                    if (currentOS == NativeCredentialStoreFactory.OperatingSystem.UNKNOWN) {
                        return@then // Skip on unknown platforms
                    }

                    val provider = NativeCredentialStoreFactory.createForOS(currentOS)
                    provider.shouldNotBeNull()

                    when (currentOS) {
                        NativeCredentialStoreFactory.OperatingSystem.MACOS ->
                            provider.name shouldBe "macOS Keychain"
                        NativeCredentialStoreFactory.OperatingSystem.WINDOWS ->
                            provider.name shouldBe "Windows Credential Manager"
                        NativeCredentialStoreFactory.OperatingSystem.LINUX ->
                            provider.name shouldBe "Linux Secret Service (secret-tool)"
                        NativeCredentialStoreFactory.OperatingSystem.UNKNOWN ->
                            { /* Skip */ }
                    }
                }
            }

            `when`("creating provider for unknown OS") {
                then("returns null") {
                    val provider =
                        NativeCredentialStoreFactory.createForOS(
                            NativeCredentialStoreFactory.OperatingSystem.UNKNOWN,
                        )
                    provider.shouldBeNull()
                }
            }

            `when`("creating provider without specifying OS") {
                then("returns provider or null based on availability") {
                    val result = NativeCredentialStoreFactory.create()

                    // The result depends on whether native libraries are available
                    // We just verify it doesn't throw
                    if (result != null) {
                        result.isAvailable().shouldBeTrue()
                    }
                }
            }
        }
    })

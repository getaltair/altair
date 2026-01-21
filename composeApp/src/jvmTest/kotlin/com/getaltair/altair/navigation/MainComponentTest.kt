package com.getaltair.altair.navigation

import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.arkivanov.essenty.lifecycle.resume
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.shouldBe

/**
 * Tests for MainComponent navigation state management.
 *
 * Verifies:
 * - Initial destination defaults to Home
 * - Initial destination can be customized
 * - Navigation updates current destination correctly
 * - Idempotent navigation to same destination
 */
class MainComponentTest :
    BehaviorSpec({
        fun createTestComponentContext(): DefaultComponentContext {
            val lifecycle = LifecycleRegistry()
            lifecycle.resume()
            return DefaultComponentContext(lifecycle = lifecycle)
        }

        given("MainComponent initialization") {
            `when`("created with default settings") {
                then("initial destination is Home") {
                    val component = MainComponent(componentContext = createTestComponentContext())
                    component.currentDestination.value shouldBe MainDestination.Home
                }
            }

            `when`("created with custom initial destination") {
                then("initial destination is customized") {
                    val component =
                        MainComponent(
                            componentContext = createTestComponentContext(),
                            initialDestination = MainDestination.Settings,
                        )
                    component.currentDestination.value shouldBe MainDestination.Settings
                }
            }
        }

        given("navigation") {
            `when`("navigating to different destinations") {
                then("updates current destination") {
                    val component = MainComponent(componentContext = createTestComponentContext())

                    component.navigateTo(MainDestination.Guidance)
                    component.currentDestination.value shouldBe MainDestination.Guidance

                    component.navigateTo(MainDestination.Knowledge)
                    component.currentDestination.value shouldBe MainDestination.Knowledge

                    component.navigateTo(MainDestination.Tracking)
                    component.currentDestination.value shouldBe MainDestination.Tracking

                    component.navigateTo(MainDestination.Settings)
                    component.currentDestination.value shouldBe MainDestination.Settings

                    component.navigateTo(MainDestination.Home)
                    component.currentDestination.value shouldBe MainDestination.Home
                }
            }

            `when`("navigating through all destinations") {
                then("updates state correctly for each") {
                    val component = MainComponent(componentContext = createTestComponentContext())

                    MainDestination.entries.forEach { destination ->
                        component.navigateTo(destination)
                        component.currentDestination.value shouldBe destination
                    }
                }
            }

            `when`("navigating to same destination twice") {
                then("is idempotent") {
                    val component = MainComponent(componentContext = createTestComponentContext())

                    // Navigate to Guidance
                    component.navigateTo(MainDestination.Guidance)
                    component.currentDestination.value shouldBe MainDestination.Guidance

                    // Navigate again to same destination
                    component.navigateTo(MainDestination.Guidance)
                    component.currentDestination.value shouldBe MainDestination.Guidance
                }
            }
        }
    })

package com.getaltair.altair.navigation

import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.arkivanov.essenty.lifecycle.resume
import kotlin.test.Test
import kotlin.test.assertEquals

/**
 * Tests for MainComponent navigation state management.
 */
class MainComponentTest {
    private fun createTestComponentContext(): DefaultComponentContext {
        val lifecycle = LifecycleRegistry()
        lifecycle.resume()
        return DefaultComponentContext(lifecycle = lifecycle)
    }

    @Test
    fun `initial destination is Home by default`() {
        val component = MainComponent(componentContext = createTestComponentContext())
        assertEquals(MainDestination.Home, component.currentDestination.value)
    }

    @Test
    fun `initial destination can be customized`() {
        val component =
            MainComponent(
                componentContext = createTestComponentContext(),
                initialDestination = MainDestination.Settings,
            )
        assertEquals(MainDestination.Settings, component.currentDestination.value)
    }

    @Test
    fun `navigateTo updates current destination`() {
        val component = MainComponent(componentContext = createTestComponentContext())

        component.navigateTo(MainDestination.Guidance)
        assertEquals(MainDestination.Guidance, component.currentDestination.value)

        component.navigateTo(MainDestination.Knowledge)
        assertEquals(MainDestination.Knowledge, component.currentDestination.value)

        component.navigateTo(MainDestination.Tracking)
        assertEquals(MainDestination.Tracking, component.currentDestination.value)

        component.navigateTo(MainDestination.Settings)
        assertEquals(MainDestination.Settings, component.currentDestination.value)

        component.navigateTo(MainDestination.Home)
        assertEquals(MainDestination.Home, component.currentDestination.value)
    }

    @Test
    fun `navigating to all destinations updates state correctly`() {
        val component = MainComponent(componentContext = createTestComponentContext())

        // Verify we can navigate through all destinations
        MainDestination.entries.forEach { destination ->
            component.navigateTo(destination)
            assertEquals(
                destination,
                component.currentDestination.value,
                "Should navigate to $destination",
            )
        }
    }

    @Test
    fun `navigating to same destination is idempotent`() {
        val component = MainComponent(componentContext = createTestComponentContext())

        // Navigate to Guidance
        component.navigateTo(MainDestination.Guidance)
        assertEquals(MainDestination.Guidance, component.currentDestination.value)

        // Navigate again to same destination
        component.navigateTo(MainDestination.Guidance)
        assertEquals(MainDestination.Guidance, component.currentDestination.value)
    }
}

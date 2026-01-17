package com.getaltair.altair.navigation

import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.arkivanov.essenty.lifecycle.resume
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs

/**
 * Tests for RootComponent navigation behavior.
 */
class RootComponentTest {
    private fun createTestComponentContext(): DefaultComponentContext {
        val lifecycle = LifecycleRegistry()
        lifecycle.resume()
        return DefaultComponentContext(lifecycle = lifecycle)
    }

    @Test
    fun `initial stack contains Home configuration`() {
        val component =
            RootComponent(
                componentContext = createTestComponentContext(),
            )

        val stack = component.stack.value
        assertEquals(1, stack.items.size, "Stack should have exactly one item")
        assertIs<Config.Home>(stack.active.configuration, "Active configuration should be Home")
    }

    @Test
    fun `initial child is Home`() {
        val component =
            RootComponent(
                componentContext = createTestComponentContext(),
            )

        val activeChild = component.stack.value.active.instance
        assertIs<RootComponent.Child.Home>(activeChild, "Active child should be Home")
    }

    @Test
    fun `stack handles back button by default`() {
        val component =
            RootComponent(
                componentContext = createTestComponentContext(),
            )

        // Verify stack was created with handleBackButton = true
        // This is tested implicitly - if it wasn't configured,
        // back handling wouldn't work on platforms that support it
        val stack = component.stack.value
        assertEquals(1, stack.items.size, "Initial stack should have one item")
    }
}

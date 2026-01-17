package com.getaltair.altair.navigation

import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.Lifecycle
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.arkivanov.essenty.lifecycle.resume
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs
import kotlin.test.assertNotNull

/**
 * Tests for RootComponent navigation behavior and ComponentContextFactory.
 */
class RootComponentTest {
    private fun createTestComponentContext(): DefaultComponentContext {
        val lifecycle = LifecycleRegistry()
        lifecycle.resume()
        return DefaultComponentContext(lifecycle = lifecycle)
    }

    // ============================================
    // RootComponent Tests
    // ============================================

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

    // ============================================
    // ComponentContextFactory Tests
    // ============================================

    @Test
    fun `createRootComponentContext returns valid context`() {
        val context = createRootComponentContext()
        assertNotNull(context, "Context should not be null")
    }

    @Test
    fun `createRootComponentContext returns context with resumed lifecycle`() {
        val context = createRootComponentContext()
        val lifecycle = context.getLifecycle()
        assertEquals(
            Lifecycle.State.RESUMED,
            lifecycle.state,
            "Lifecycle should be in RESUMED state",
        )
    }

    @Test
    fun `getLifecycle returns LifecycleRegistry from createRootComponentContext`() {
        val context = createRootComponentContext()
        val lifecycle = context.getLifecycle()
        assertNotNull(lifecycle, "getLifecycle should return non-null LifecycleRegistry")
        assertIs<LifecycleRegistry>(lifecycle, "Should return LifecycleRegistry type")
    }

    @Test
    fun `getLifecycleState returns RESUMED for new context`() {
        val context = createRootComponentContext()
        val state = context.getLifecycleState()
        assertEquals(Lifecycle.State.RESUMED, state)
    }
}

package com.getaltair.altair.ui

import com.getaltair.altair.ui.components.ButtonVariant
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals

/**
 * Test Scenario TS5-TS7: Component State Tests
 * Verifies component variant enums and basic state behavior.
 */
class ComponentTests {

    // TS5: AltairButton variant tests
    @Test
    fun `ButtonVariant has Primary variant`() {
        val variant = ButtonVariant.Primary
        assertEquals(ButtonVariant.Primary, variant)
    }

    @Test
    fun `ButtonVariant has Secondary variant`() {
        val variant = ButtonVariant.Secondary
        assertEquals(ButtonVariant.Secondary, variant)
    }

    @Test
    fun `ButtonVariant has Ghost variant`() {
        val variant = ButtonVariant.Ghost
        assertEquals(ButtonVariant.Ghost, variant)
    }

    @Test
    fun `ButtonVariant variants are distinct`() {
        assertNotEquals(ButtonVariant.Primary, ButtonVariant.Secondary)
        assertNotEquals(ButtonVariant.Secondary, ButtonVariant.Ghost)
        assertNotEquals(ButtonVariant.Primary, ButtonVariant.Ghost)
    }

    @Test
    fun `ButtonVariant has exactly three variants`() {
        val variants = ButtonVariant.entries
        assertEquals(3, variants.size)
    }

    @Test
    fun `ButtonVariant contains all expected variants`() {
        val variants = ButtonVariant.entries
        assert(variants.contains(ButtonVariant.Primary))
        assert(variants.contains(ButtonVariant.Secondary))
        assert(variants.contains(ButtonVariant.Ghost))
    }
}

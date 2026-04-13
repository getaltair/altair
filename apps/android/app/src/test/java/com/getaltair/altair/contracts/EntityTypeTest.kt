package com.getaltair.altair.contracts

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class EntityTypeTest {
    @Test
    fun fromValue_guidanceEpic_returnsGuidanceEpic() {
        val result = EntityType.fromValue("guidance_epic")
        assertEquals(EntityType.GUIDANCE_EPIC, result)
    }

    @Test
    fun fromValue_unknownType_returnsNull() {
        val result = EntityType.fromValue("unknown_fake_type")
        assertNull(result)
    }

    @Test
    fun entries_hasSize18() {
        assertEquals(18, EntityType.entries.size)
    }

    @Test
    fun allEntries_haveNonBlankValues() {
        EntityType.entries.forEach { entry ->
            assertTrue("Entry ${entry.name} has blank value", entry.value.isNotBlank())
        }
    }

    @Test
    fun fromValue_relatedTo_returnsRelatedTo() {
        val result = RelationType.fromValue("related_to")
        assertEquals(RelationType.RELATED_TO, result)
    }
}

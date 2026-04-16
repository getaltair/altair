package com.getaltair.altair.data.local.entity

import android.content.Context
import androidx.room.Room
import androidx.sqlite.db.SimpleSQLiteQuery
import androidx.test.core.app.ApplicationProvider
import com.getaltair.altair.data.local.AltairDatabase
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

/**
 * Verifies Room entity table schemas via PRAGMA table_info.
 *
 * NOTE: @ColumnInfo has AnnotationRetention.BINARY, which is NOT accessible via Java or
 * Kotlin reflection at runtime. The correct approach is to create an in-memory Room database
 * and query the actual SQLite schema, which Room derives from the @ColumnInfo annotations
 * at compile/build time via KSP.
 */
@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34], application = android.app.Application::class)
class RoomEntityColumnParityTest {
    private lateinit var db: AltairDatabase

    @Before
    fun setup() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        db = Room.inMemoryDatabaseBuilder(context, AltairDatabase::class.java)
            .allowMainThreadQueries()
            .build()
    }

    @After
    fun teardown() {
        db.close()
    }

    private fun columnsOf(tableName: String): Set<String> {
        val result = mutableSetOf<String>()
        // db.query() uses Room's SupportSQLiteQuery abstraction (works with Room 2.7+)
        val cursor = db.query(SimpleSQLiteQuery("PRAGMA table_info($tableName)"))
        cursor.use {
            val nameIndex = cursor.getColumnIndexOrThrow("name")
            while (cursor.moveToNext()) {
                result.add(cursor.getString(nameIndex))
            }
        }
        return result
    }

    @Test
    fun `QuestEntity has all required columns`() {
        val cols = columnsOf("quests")
        val required =
            setOf(
                "id",
                "title",
                "description",
                "status",
                "priority",
                "due_date",
                "epic_id",
                "initiative_id",
                "routine_id",
                "user_id",
                "created_at",
                "updated_at",
                "deleted_at",
            )
        val missing = required - cols
        assertTrue("QuestEntity is missing columns: $missing. Found: $cols", missing.isEmpty())
    }

    @Test
    fun `QuestEntity has no unexpected columns`() {
        val cols = columnsOf("quests")
        val allowed =
            setOf(
                "id",
                "title",
                "description",
                "status",
                "priority",
                "due_date",
                "epic_id",
                "initiative_id",
                "routine_id",
                "user_id",
                "created_at",
                "updated_at",
                "deleted_at",
            )
        val unexpected = cols - allowed
        assertTrue("QuestEntity has unexpected columns: $unexpected", unexpected.isEmpty())
    }

    @Test
    fun `EntityRelationEntity has to_entity_id column (not target_id)`() {
        val cols = columnsOf("entity_relations")
        assertTrue(
            "EntityRelationEntity must have to_entity_id column. Found: $cols",
            "to_entity_id" in cols,
        )
    }

    @Test
    fun `EntityRelationEntity has required columns`() {
        val cols = columnsOf("entity_relations")
        val required =
            setOf(
                "from_entity_type",
                "from_entity_id",
                "to_entity_type",
                "to_entity_id",
                "relation_type",
                "deleted_at",
            )
        val missing = required - cols
        assertTrue("EntityRelationEntity is missing columns: $missing. Found: $cols", missing.isEmpty())
    }

    @Test
    fun `TrackingItemEntity has household_id, quantity, barcode columns`() {
        val cols = columnsOf("tracking_items")
        val required = setOf("household_id", "quantity", "barcode")
        val missing = required - cols
        assertTrue("TrackingItemEntity is missing columns: $missing. Found: $cols", missing.isEmpty())
    }
}

package com.getaltair.altair.data.local.dao

import android.content.Context
import androidx.arch.core.executor.testing.InstantTaskExecutorRule
import androidx.room.Room
import androidx.test.core.app.ApplicationProvider
import app.cash.turbine.test
import com.getaltair.altair.data.local.AltairDatabase
import com.getaltair.altair.data.local.entity.TrackingItemEntity
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34], application = android.app.Application::class)
class TrackingItemDaoTest {
    @get:Rule
    val instantTaskExecutorRule = InstantTaskExecutorRule()

    private lateinit var db: AltairDatabase
    private lateinit var trackingItemDao: TrackingItemDao

    @Before
    fun setup() {
        val context: Context = ApplicationProvider.getApplicationContext()
        db =
            Room
                .inMemoryDatabaseBuilder(context, AltairDatabase::class.java)
                .allowMainThreadQueries()
                .build()
        trackingItemDao = db.trackingItemDao()
    }

    @After
    fun teardown() {
        db.close()
    }

    /**
     * Verifies consumption quantity invariant at DAO level:
     * After upserting an item with quantity 10.0, then upserting again with quantity 7.0
     * (simulating a consumption event of 3 units), watchById returns the updated value 7.0.
     */
    @Test
    fun trackingItemDao_quantityAfterConsumption_reflectsReducedAmount() =
        runTest {
            val item = makeTrackingItem("item-1", quantity = 10.0)
            trackingItemDao.upsert(item)

            // Simulate consumption: new quantity = original - consumed (10 - 3 = 7)
            val afterConsumption = item.copy(quantity = 7.0, updatedAt = "2026-01-02T00:00:00Z")
            trackingItemDao.upsert(afterConsumption)

            trackingItemDao.watchById("item-1").test {
                val result = awaitItem()
                assertEquals(7.0, result?.quantity)
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * Verifies that successive consumption upserts produce correct final quantity:
     * start 20.0, consume 5.0 → 15.0, consume 8.0 → 7.0.
     */
    @Test
    fun trackingItemDao_multipleConsumptionUpserts_quantityIsCorrect() =
        runTest {
            val item = makeTrackingItem("item-1", quantity = 20.0)
            trackingItemDao.upsert(item)

            trackingItemDao.upsert(item.copy(quantity = 15.0, updatedAt = "2026-01-02T00:00:00Z"))
            trackingItemDao.upsert(item.copy(quantity = 7.0, updatedAt = "2026-01-03T00:00:00Z"))

            trackingItemDao.watchById("item-1").test {
                val result = awaitItem()
                assertEquals(7.0, result?.quantity)
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * Verifies quantity is correctly reflected in watchAll for the household after consumption.
     */
    @Test
    fun trackingItemDao_watchAll_reflectsQuantityAfterConsumption() =
        runTest {
            trackingItemDao.upsert(makeTrackingItem("item-1", quantity = 5.0))

            val consumed = makeTrackingItem("item-1", quantity = 2.0).copy(updatedAt = "2026-01-02T00:00:00Z")
            trackingItemDao.upsert(consumed)

            trackingItemDao.watchAll("hh-1").test {
                val items = awaitItem()
                assertEquals(1, items.size)
                assertEquals(2.0, items[0].quantity, 0.001)
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * Verifies that watchById returns null for a non-existent item id.
     */
    @Test
    fun trackingItemDao_watchById_returnsNullForMissingItem() =
        runTest {
            trackingItemDao.watchById("nonexistent").test {
                val result = awaitItem()
                assertNull(result)
                cancelAndIgnoreRemainingEvents()
            }
        }

    // ─── Helpers ──────────────────────────────────────────────────────────────

    private fun makeTrackingItem(
        id: String,
        quantity: Double,
    ) = TrackingItemEntity(
        id = id,
        name = "Item $id",
        description = null,
        quantity = quantity,
        barcode = null,
        locationId = null,
        categoryId = null,
        userId = "user-1",
        householdId = "hh-1",
        initiativeId = null,
        expiresAt = null,
        createdAt = "2026-01-01T00:00:00Z",
        updatedAt = "2026-01-01T00:00:00Z",
        deletedAt = null,
    )
}

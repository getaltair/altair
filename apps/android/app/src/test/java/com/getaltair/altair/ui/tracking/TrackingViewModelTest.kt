package com.getaltair.altair.ui.tracking

import app.cash.turbine.test
import com.getaltair.altair.data.local.dao.ShoppingListDao
import com.getaltair.altair.data.local.dao.ShoppingListItemDao
import com.getaltair.altair.data.local.dao.TrackingCategoryDao
import com.getaltair.altair.data.local.dao.TrackingItemDao
import com.getaltair.altair.data.local.dao.TrackingItemEventDao
import com.getaltair.altair.data.local.dao.TrackingLocationDao
import com.getaltair.altair.data.local.entity.TrackingItemEntity
import com.powersync.PowerSyncDatabase
import io.mockk.coVerify
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.test.UnconfinedTestDispatcher
import kotlinx.coroutines.test.advanceUntilIdle
import kotlinx.coroutines.test.resetMain
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.test.setMain
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.Assertions.assertInstanceOf
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test

/**
 * Unit tests for [TrackingViewModel], covering FA-008:
 * consumption validation blocks writes that exceed current item quantity.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class TrackingViewModelTest {
    private val testDispatcher = UnconfinedTestDispatcher()

    private lateinit var trackingItemDao: TrackingItemDao
    private lateinit var trackingItemEventDao: TrackingItemEventDao
    private lateinit var trackingLocationDao: TrackingLocationDao
    private lateinit var trackingCategoryDao: TrackingCategoryDao
    private lateinit var shoppingListDao: ShoppingListDao
    private lateinit var shoppingListItemDao: ShoppingListItemDao
    private lateinit var db: PowerSyncDatabase

    // Mutable backing flow so tests can control items.value
    private lateinit var itemsFlow: MutableStateFlow<List<TrackingItemEntity>>

    private lateinit var viewModel: TrackingViewModel

    @BeforeEach
    fun setUp() {
        Dispatchers.setMain(testDispatcher)

        trackingItemDao = mockk(relaxed = true)
        trackingItemEventDao = mockk(relaxed = true)
        trackingLocationDao = mockk(relaxed = true)
        trackingCategoryDao = mockk(relaxed = true)
        shoppingListDao = mockk(relaxed = true)
        shoppingListItemDao = mockk(relaxed = true)
        db = mockk(relaxed = true)

        itemsFlow = MutableStateFlow(emptyList())
        every { trackingItemDao.watchAll(any()) } returns itemsFlow
        every { trackingLocationDao.watchAll(any()) } returns flowOf(emptyList())
        every { trackingCategoryDao.watchAll(any()) } returns flowOf(emptyList())
        every { shoppingListDao.watchAll(any()) } returns flowOf(emptyList())

        viewModel =
            TrackingViewModel(
                trackingItemDao = trackingItemDao,
                trackingItemEventDao = trackingItemEventDao,
                trackingLocationDao = trackingLocationDao,
                trackingCategoryDao = trackingCategoryDao,
                shoppingListDao = shoppingListDao,
                shoppingListItemDao = shoppingListItemDao,
                db = db,
            )
    }

    @AfterEach
    fun tearDown() {
        Dispatchers.resetMain()
    }

    /**
     * FA-008: Attempting to log consumption exceeding current quantity is rejected.
     * db.execute() must NOT be called; an Error uiState must be emitted.
     */
    @Test
    fun `consumptionValidation_blocksExcess - rejects amount greater than current quantity`() =
        runTest {
            val item = makeItem("item-1", quantity = 5.0)
            itemsFlow.value = listOf(item)
            advanceUntilIdle()

            viewModel.uiState.test {
                // Skip initial Idle
                awaitItem()

                viewModel.logConsumption(itemId = "item-1", amount = 10.0)
                advanceUntilIdle()

                val state = awaitItem()
                assertInstanceOf(TrackingUiState.Error::class.java, state)
                assertNotNull((state as TrackingUiState.Error).message)

                cancelAndIgnoreRemainingEvents()
            }

            coVerify(exactly = 0) { db.execute(any(), any()) }
        }

    /**
     * FA-008: Attempting to log consumption of zero or negative is rejected.
     */
    @Test
    fun `consumptionValidation_blocksZeroOrNegative - rejects non-positive amount`() =
        runTest {
            val item = makeItem("item-1", quantity = 5.0)
            itemsFlow.value = listOf(item)
            advanceUntilIdle()

            viewModel.uiState.test {
                awaitItem() // skip Idle

                viewModel.logConsumption(itemId = "item-1", amount = 0.0)
                advanceUntilIdle()

                val state = awaitItem()
                assertInstanceOf(TrackingUiState.Error::class.java, state)

                cancelAndIgnoreRemainingEvents()
            }

            coVerify(exactly = 0) { db.execute(any(), any()) }
        }

    /**
     * FA-008: A valid consumption (amount <= currentQuantity) proceeds and calls db.execute().
     */
    @Test
    fun `consumptionValidation_allowsValid - amount within quantity triggers db execute`() =
        runTest {
            val item = makeItem("item-1", quantity = 5.0)
            itemsFlow.value = listOf(item)
            advanceUntilIdle()

            viewModel.logConsumption(itemId = "item-1", amount = 3.0)
            advanceUntilIdle()

            coVerify(atLeast = 1) { db.execute(any(), any()) }
        }

    /**
     * FA-008: Consuming the exact current quantity (boundary) is valid.
     */
    @Test
    fun `consumptionValidation_allowsExactQuantity - consuming all stock is valid`() =
        runTest {
            val item = makeItem("item-1", quantity = 5.0)
            itemsFlow.value = listOf(item)
            advanceUntilIdle()

            viewModel.logConsumption(itemId = "item-1", amount = 5.0)
            advanceUntilIdle()

            coVerify(atLeast = 1) { db.execute(any(), any()) }
        }

    /**
     * Consuming from a non-existent item emits an Error state.
     */
    @Test
    fun `consumptionValidation_itemNotFound - emits error when item does not exist`() =
        runTest {
            itemsFlow.value = emptyList()
            advanceUntilIdle()

            viewModel.uiState.test {
                awaitItem() // skip Idle

                viewModel.logConsumption(itemId = "nonexistent", amount = 1.0)
                advanceUntilIdle()

                val state = awaitItem()
                assertInstanceOf(TrackingUiState.Error::class.java, state)

                cancelAndIgnoreRemainingEvents()
            }
        }

    // ─── Helpers ──────────────────────────────────────────────────────────────

    private fun makeItem(
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

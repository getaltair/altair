package com.getaltair.altair.ui.sync

import app.cash.turbine.test
import com.powersync.PowerSyncDatabase
import com.powersync.sync.SyncStatusData
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.test.UnconfinedTestDispatcher
import kotlinx.coroutines.test.resetMain
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.test.setMain
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test

/**
 * Unit tests for [SyncStatusViewModel].
 * Verifies that [SyncStatusViewModel.isPending] correctly reflects the PowerSync status.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class SyncStatusViewModelTest {
    private val testDispatcher = UnconfinedTestDispatcher()

    private lateinit var db: PowerSyncDatabase
    private lateinit var syncFlow: MutableSharedFlow<SyncStatusData>

    @BeforeEach
    fun setUp() {
        Dispatchers.setMain(testDispatcher)

        db = mockk(relaxed = true)
        syncFlow = MutableSharedFlow(replay = 1)
        every { db.currentStatus } returns
            mockk(relaxed = true) {
                every { asFlow() } returns syncFlow
            }
    }

    @AfterEach
    fun tearDown() {
        Dispatchers.resetMain()
    }

    @Test
    fun `isPending_initialValueIsFalse - StateFlow starts as false before any emission`() {
        val viewModel = SyncStatusViewModel(db = db)

        assertFalse(
            viewModel.isPending.value,
            "isPending must default to false before any status is emitted",
        )
    }

    @Test
    fun `isPending_emitsTrue_whenUploading - uploading=true causes isPending to emit true`() =
        runTest {
            val status =
                mockk<SyncStatusData>(relaxed = true) {
                    every { uploading } returns true
                    every { connected } returns true
                }
            syncFlow.tryEmit(status)

            val viewModel = SyncStatusViewModel(db = db)

            viewModel.isPending.test {
                assertTrue(
                    awaitItem(),
                    "isPending must be true when uploading=true",
                )
                cancelAndIgnoreRemainingEvents()
            }
        }

    @Test
    fun `isPending_emitsTrue_whenNotConnected - connected=false causes isPending to emit true`() =
        runTest {
            val status =
                mockk<SyncStatusData>(relaxed = true) {
                    every { uploading } returns false
                    every { connected } returns false
                }
            syncFlow.tryEmit(status)

            val viewModel = SyncStatusViewModel(db = db)

            viewModel.isPending.test {
                assertTrue(
                    awaitItem(),
                    "isPending must be true when connected=false",
                )
                cancelAndIgnoreRemainingEvents()
            }
        }

    @Test
    fun `isPending_emitsFalse_whenIdleAndConnected - uploading=false and connected=true yields false`() =
        runTest {
            val status =
                mockk<SyncStatusData>(relaxed = true) {
                    every { uploading } returns false
                    every { connected } returns true
                }
            syncFlow.tryEmit(status)

            val viewModel = SyncStatusViewModel(db = db)

            viewModel.isPending.test {
                assertFalse(
                    awaitItem(),
                    "isPending must be false when uploading=false and connected=true",
                )
                cancelAndIgnoreRemainingEvents()
            }
        }
}

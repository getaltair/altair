package com.getaltair.altair.data.sync

import android.content.Context
import androidx.work.WorkManager
import com.powersync.PowerSyncDatabase
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkStatic
import io.mockk.unmockkAll
import io.mockk.verify
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test

/**
 * Unit tests for [SyncCoordinator] debounce behaviour introduced in S010.
 *
 * The debounce guard is: if `System.currentTimeMillis() - lastSyncTime < SYNC_DEBOUNCE_WINDOW_MS`,
 * skip enqueuing work. `SYNC_DEBOUNCE_WINDOW_MS` is 5 * 60 * 1_000 = 300_000 ms.
 *
 * WorkManager is a static singleton; we use [mockkStatic] to intercept
 * `WorkManager.getInstance(context)` without touching the real Android runtime.
 */
class SyncCoordinatorTest {
    private lateinit var context: Context
    private lateinit var workManager: WorkManager
    private lateinit var coordinator: SyncCoordinator

    @BeforeEach
    fun setUp() {
        context = mockk(relaxed = true)
        workManager = mockk(relaxed = true)

        mockkStatic(WorkManager::class)
        mockkStatic(System::class)

        every { WorkManager.getInstance(any()) } returns workManager
    }

    @AfterEach
    fun tearDown() {
        unmockkAll()
    }

    // ─── Helpers ──────────────────────────────────────────────────────────────

    private fun newCoordinator(): SyncCoordinator {
        val db = mockk<PowerSyncDatabase>(relaxed = true)
        val connector = mockk<AltairPowerSyncConnector>(relaxed = true)
        return SyncCoordinator(db, connector)
    }

    // ─── Tests ────────────────────────────────────────────────────────────────

    /**
     * S010: On a fresh [SyncCoordinator] instance `lastSyncTime` is 0.
     * `System.currentTimeMillis() - 0` is always >= SYNC_DEBOUNCE_WINDOW_MS,
     * so the guard passes and work is enqueued exactly once.
     */
    @Test
    fun `enqueueExpedited_enqueuesWork_whenLastSyncTimeIsZero`() {
        // currentTimeMillis returns a realistic epoch value (well beyond 300 000 ms)
        every { System.currentTimeMillis() } returns 1_700_000_000_000L

        coordinator = newCoordinator()
        coordinator.enqueueExpedited(context)

        verify(exactly = 1) {
            workManager.enqueueUniqueWork(eq<String>("sync_expedited"), any(), any())
        }
    }

    /**
     * S010: A second call within the debounce window (last sync was 1 second ago)
     * must be a no-op — work must only be enqueued once in total.
     */
    @Test
    fun `enqueueExpedited_isNoOp_whenCalledWithin5Minutes`() {
        val baseTime = 1_700_000_000_000L
        // First call sets lastSyncTime = baseTime
        every { System.currentTimeMillis() } returns baseTime
        coordinator = newCoordinator()
        coordinator.enqueueExpedited(context)

        // Second call is 1 000 ms later — inside the 300 000 ms window
        every { System.currentTimeMillis() } returns baseTime + 1_000L
        coordinator.enqueueExpedited(context)

        verify(exactly = 1) {
            workManager.enqueueUniqueWork(eq<String>("sync_expedited"), any(), any())
        }
    }

    /**
     * S010: After the debounce window has elapsed a subsequent call must enqueue
     * work again, resulting in exactly two enqueue operations in total.
     */
    @Test
    fun `enqueueExpedited_enqueuesAgain_afterWindowExpires`() {
        val baseTime = 1_700_000_000_000L
        // First call
        every { System.currentTimeMillis() } returns baseTime
        coordinator = newCoordinator()
        coordinator.enqueueExpedited(context)

        // Second call is exactly 300 000 ms later — window has expired
        every { System.currentTimeMillis() } returns baseTime + 300_000L
        coordinator.enqueueExpedited(context)

        verify(exactly = 2) {
            workManager.enqueueUniqueWork(eq<String>("sync_expedited"), any(), any())
        }
    }
}

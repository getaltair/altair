package com.getaltair.altair.data.sync

import android.content.Context
import androidx.work.ExistingWorkPolicy
import androidx.work.OneTimeWorkRequest
import androidx.work.WorkManager
import com.powersync.PowerSyncDatabase
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkStatic
import io.mockk.unmockkAll
import io.mockk.verify
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test

/**
 * Unit tests for [SyncCoordinator] debounce behaviour introduced in S010.
 *
 * The debounce guard is: if `clock() - lastSyncTime < SYNC_DEBOUNCE_WINDOW_MS`,
 * skip enqueuing work. `SYNC_DEBOUNCE_WINDOW_MS` is 5 * 60 * 1_000 = 300_000 ms.
 *
 * WorkManager is a static singleton; we use [mockkStatic] to intercept
 * `WorkManager.getInstance(context)` without touching the real Android runtime.
 * The clock is injected directly — no System class mocking needed.
 */
class SyncCoordinatorTest {
    private lateinit var context: Context
    private lateinit var workManager: WorkManager

    @BeforeEach
    fun setUp() {
        context = mockk(relaxed = true)
        workManager = mockk(relaxed = true)

        mockkStatic(WorkManager::class)
        every { WorkManager.getInstance(any()) } returns workManager
    }

    @AfterEach
    fun tearDown() {
        unmockkAll()
    }

    // ─── Helpers ──────────────────────────────────────────────────────────────

    private fun newCoordinator(clock: () -> Long): SyncCoordinator {
        val db = mockk<PowerSyncDatabase>(relaxed = true)
        val connector = mockk<AltairPowerSyncConnector>(relaxed = true)
        return SyncCoordinator(db, connector, clock)
    }

    // ─── Tests ────────────────────────────────────────────────────────────────

    /**
     * S010: On a fresh [SyncCoordinator] instance `lastSyncTime` is 0.
     * `clock() - 0` is always >= SYNC_DEBOUNCE_WINDOW_MS,
     * so the guard passes and work is enqueued exactly once.
     */
    @Test
    fun `enqueueExpedited_enqueuesWork_whenLastSyncTimeIsZero`() {
        val coordinator = newCoordinator { 1_700_000_000_000L }
        coordinator.enqueueExpedited(context)

        verify(exactly = 1) {
            workManager.enqueueUniqueWork(eq<String>("sync_expedited"), any<ExistingWorkPolicy>(), any<OneTimeWorkRequest>())
        }
    }

    /**
     * S010: A second call within the debounce window (last sync was 1 second ago)
     * must be a no-op — work must only be enqueued once in total.
     */
    @Test
    fun `enqueueExpedited_isNoOp_whenCalledWithin5Minutes`() {
        val baseTime = 1_700_000_000_000L
        var currentTime = baseTime

        val coordinator = newCoordinator { currentTime }
        coordinator.enqueueExpedited(context) // sets lastSyncTime = baseTime

        currentTime = baseTime + 1_000L // 1s later — inside 300s window
        coordinator.enqueueExpedited(context)

        verify(exactly = 1) {
            workManager.enqueueUniqueWork(eq<String>("sync_expedited"), any<ExistingWorkPolicy>(), any<OneTimeWorkRequest>())
        }
    }

    /**
     * S010: After the debounce window has elapsed a subsequent call must enqueue
     * work again, resulting in exactly two enqueue operations in total.
     */
    @Test
    fun `enqueueExpedited_enqueuesAgain_afterWindowExpires`() {
        val baseTime = 1_700_000_000_000L
        var currentTime = baseTime

        val coordinator = newCoordinator { currentTime }
        coordinator.enqueueExpedited(context) // first call

        currentTime = baseTime + 300_000L // exactly 300s — window expired
        coordinator.enqueueExpedited(context)

        verify(exactly = 2) {
            workManager.enqueueUniqueWork(eq<String>("sync_expedited"), any<ExistingWorkPolicy>(), any<OneTimeWorkRequest>())
        }
    }
}

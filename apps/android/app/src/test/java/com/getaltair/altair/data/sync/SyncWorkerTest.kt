package com.getaltair.altair.data.sync

import androidx.test.core.app.ApplicationProvider
import io.mockk.coEvery
import io.mockk.coVerify
import io.mockk.mockk
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.runBlocking
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertSame
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.koin.core.context.startKoin
import org.koin.core.context.stopKoin
import org.koin.dsl.module
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config
import java.io.IOException

/**
 * Unit tests for [SyncWorker].
 *
 * Verifies that [SyncWorker.doWork] correctly maps exceptions from
 * [SyncCoordinator.triggerSync] to WorkManager [Result] values, and that
 * [CancellationException] is never swallowed so structured concurrency is preserved.
 *
 * [SyncCoordinator] is provided via Koin; each test installs a fresh module
 * and tears it down to prevent cross-test contamination.
 */
@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34], application = android.app.Application::class)
class SyncWorkerTest {
    private lateinit var syncCoordinator: SyncCoordinator

    @Before
    fun setUp() {
        syncCoordinator = mockk(relaxed = true)
        startKoin {
            modules(
                module {
                    single<SyncCoordinator> { syncCoordinator }
                },
            )
        }
    }

    @After
    fun tearDown() {
        stopKoin()
    }

    /**
     * When [SyncCoordinator.triggerSync] throws [IOException], [SyncWorker.doWork]
     * must return [androidx.work.ListenableWorker.Result.retry] so WorkManager
     * will schedule a retry for the transient network error.
     */
    @Test
    fun `doWork_ioException_returnsRetry`() {
        coEvery { syncCoordinator.triggerSync() } throws IOException("network unavailable")

        val worker = buildWorker()
        val result = runBlocking { worker.doWork() }

        assertEquals(
            androidx.work.ListenableWorker.Result
                .retry(),
            result,
        )
        coVerify(exactly = 1) { syncCoordinator.triggerSync() }
    }

    /**
     * When [SyncCoordinator.triggerSync] throws a non-transient [RuntimeException],
     * [SyncWorker.doWork] must return [androidx.work.ListenableWorker.Result.failure]
     * so WorkManager does not keep retrying a permanently broken operation.
     */
    @Test
    fun `doWork_genericException_returnsFailure`() {
        coEvery { syncCoordinator.triggerSync() } throws RuntimeException("permanent error")

        val worker = buildWorker()
        val result = runBlocking { worker.doWork() }

        assertEquals(
            androidx.work.ListenableWorker.Result
                .failure(),
            result,
        )
        coVerify(exactly = 1) { syncCoordinator.triggerSync() }
    }

    /**
     * When [SyncCoordinator.triggerSync] throws [CancellationException], [SyncWorker.doWork]
     * must rethrow it unchanged so structured concurrency is not broken.
     * The exception must NOT be swallowed or wrapped.
     */
    @Test
    fun `doWork_cancellationException_rethrows`() {
        val cancellation = CancellationException("cancelled")
        coEvery { syncCoordinator.triggerSync() } throws cancellation

        val worker = buildWorker()
        var thrown: CancellationException? = null
        try {
            runBlocking { worker.doWork() }
        } catch (e: CancellationException) {
            thrown = e
        }

        assertSame(
            "CancellationException must propagate as-is, not be wrapped or swallowed",
            cancellation,
            thrown,
        )
    }

    // ─── Helpers ────────────────────────────────────────────────────────────────

    private fun buildWorker(): SyncWorker {
        val context = ApplicationProvider.getApplicationContext<android.app.Application>()
        return SyncWorker(
            context,
            mockk(relaxed = true),
        )
    }
}

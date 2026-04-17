package com.getaltair.altair.service

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Intent
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.Robolectric
import org.robolectric.RobolectricTestRunner
import org.robolectric.RuntimeEnvironment
import org.robolectric.Shadows.shadowOf
import org.robolectric.annotation.Config
import org.robolectric.shadows.ShadowLooper

/**
 * Robolectric unit tests for [FocusTimerService].
 *
 * Covers FA-040 / FA-041:
 *  - Foreground service starts with an ongoing notification when onStartCommand is called.
 *  - The 1-second tick loop is posted to the main handler.
 *  - When the end time is in the past the service posts a completion notification and stops itself.
 *  - onDestroy removes all handler callbacks so no further work is scheduled.
 *
 * Uses @Config(sdk = [33]) to stay on the pre-UPSIDE_DOWN_CAKE code path where startForeground
 * does not require FOREGROUND_SERVICE_TYPE_SPECIAL_USE.
 */
@RunWith(RobolectricTestRunner::class)
@Config(sdk = [33])
class FocusTimerServiceTest {
    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    private fun buildController(endTimeEpochMs: Long) = Robolectric.buildService(FocusTimerService::class.java, buildIntent(endTimeEpochMs))

    private fun buildIntent(endTimeEpochMs: Long): Intent =
        Intent(RuntimeEnvironment.getApplication(), FocusTimerService::class.java).apply {
            putExtra(FocusTimerService.EXTRA_END_TIME_EPOCH_MS, endTimeEpochMs)
        }

    /** Creates the "FOCUS_TIMER_CHANNEL" notification channel so Robolectric does not drop notifs. */
    private fun createNotificationChannel() {
        val nm =
            RuntimeEnvironment
                .getApplication()
                .getSystemService(NotificationManager::class.java)
        nm.createNotificationChannel(
            NotificationChannel(
                "FOCUS_TIMER_CHANNEL",
                "Focus Timer",
                NotificationManager.IMPORTANCE_LOW,
            ),
        )
    }

    // -----------------------------------------------------------------------
    // Test 1: onStartCommand calls startForeground
    // -----------------------------------------------------------------------

    /**
     * After onStartCommand the service must be in foreground state and the notification
     * posted to the system notification manager must be non-null.
     *
     * Verifies: FA-040 — ongoing timer notification is shown when session starts.
     */
    @Test
    fun `onStartCommand_callsStartForeground`() {
        createNotificationChannel()
        val endTime = System.currentTimeMillis() + 60_000L

        val controller = buildController(endTime)
        val service = controller.create().startCommand(0, 1).get()
        val shadowService = shadowOf(service)

        // startForeground was called — Robolectric records the notification
        val foregroundNotif: Notification? = shadowService.lastForegroundNotification
        assertNotNull("startForeground must have been called with a notification", foregroundNotif)
        assertTrue(
            "Service must be in foreground state",
            shadowService.isForegroundStopped.not(),
        )

        controller.destroy()
    }

    // -----------------------------------------------------------------------
    // Test 2: tickRunnable is posted to the handler after onStartCommand
    // -----------------------------------------------------------------------

    /**
     * After onStartCommand a tick runnable is pending on the main looper.
     * Running one task advances the loop and posts the 1-second delayed callback.
     *
     * Verifies the tick scheduling loop is wired up correctly.
     */
    @Test
    fun `tickRunnable_reschedulesEvery1s`() {
        createNotificationChannel()
        val endTime = System.currentTimeMillis() + 60_000L

        val controller = buildController(endTime)
        controller.create().startCommand(0, 1)

        val mainLooper = ShadowLooper.getShadowMainLooper()

        // After onStartCommand the immediate post() should be pending.
        assertFalse(
            "Main looper must have a pending runnable after onStartCommand",
            mainLooper.isIdle,
        )

        // Drain the immediate post — this executes tickRunnable once (remaining > 0 path),
        // which then calls postDelayed(this, 1000).
        mainLooper.runOneTask()

        // The postDelayed(1000ms) should now be pending on the looper.
        assertFalse(
            "A 1-second delayed runnable must be re-posted after the first tick",
            mainLooper.isIdle,
        )

        controller.destroy()
    }

    // -----------------------------------------------------------------------
    // Test 3: elapsed end time → completion notification + stopSelf
    // -----------------------------------------------------------------------

    /**
     * When endTimeEpochMs is already in the past onTimerFinished() is called from the first tick,
     * which posts a completion notification and calls stopSelf().
     *
     * Verifies: FA-041 — completion notification is shown when the timer expires.
     */
    @Test
    fun `onTimerFinished_postsCompletionNotification_andStopsSelf`() {
        createNotificationChannel()
        // End time 1 second in the past — timer has already elapsed.
        val endTime = System.currentTimeMillis() - 1_000L

        val controller = buildController(endTime)
        val service = controller.create().startCommand(0, 1).get()
        val shadowService = shadowOf(service)

        // Drain the main looper so tickRunnable runs (remaining <= 0 path).
        ShadowLooper.runMainLooperOneTask()

        // Check that stopSelf was called.
        assertTrue("Service must have called stopSelf() when timer expired", shadowService.isStoppedBySelf)

        // The completion notification (COMPLETION_NOTIF_ID = 1002) must be posted.
        val nm =
            shadowOf(
                RuntimeEnvironment
                    .getApplication()
                    .getSystemService(NotificationManager::class.java),
            )
        val notifs = nm.allNotifications
        val completionNotifId = 1002 // NOTIF_ID(1001) + 1
        val hasCompletion = notifs.any { (id, _) -> id == completionNotifId }
        assertTrue(
            "A completion notification (id=$completionNotifId) must be posted when timer expires",
            hasCompletion,
        )
    }

    // -----------------------------------------------------------------------
    // Test 4: onDestroy removes handler callbacks
    // -----------------------------------------------------------------------

    /**
     * After onDestroy() the main looper must have no pending runnables from this service,
     * so no further notifications are posted.
     *
     * Verifies: handler.removeCallbacks is called in onDestroy so the tick loop is cancelled.
     */
    @Test
    fun `onDestroy_removesCallbacks`() {
        createNotificationChannel()
        val endTime = System.currentTimeMillis() + 60_000L

        val controller = buildController(endTime)
        controller.create().startCommand(0, 1)

        val mainLooper = ShadowLooper.getShadowMainLooper()
        assertFalse("Looper must have pending tasks after startCommand", mainLooper.isIdle)

        // Destroy the service — onDestroy calls handler.removeCallbacks(tickRunnable).
        controller.destroy()

        // After destroy the looper must be idle — no further ticks scheduled.
        assertTrue(
            "Main looper must be idle after onDestroy (all callbacks removed)",
            mainLooper.isIdle,
        )
    }
}

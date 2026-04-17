package com.getaltair.altair.service

import android.app.Notification
import android.app.ServiceInfo
import android.content.Intent
import android.os.Build
import android.os.Handler
import android.os.Looper
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import androidx.lifecycle.LifecycleService

class FocusTimerService : LifecycleService() {
    companion object {
        const val EXTRA_END_TIME_EPOCH_MS = "end_time_epoch_ms"
        private const val NOTIF_ID = 1001
        private const val COMPLETION_NOTIF_ID = NOTIF_ID + 1
    }

    private val handler = Handler(Looper.getMainLooper())
    private var endTimeEpochMs: Long = 0L

    override fun onStartCommand(
        intent: Intent?,
        flags: Int,
        startId: Int,
    ): Int {
        super.onStartCommand(intent, flags, startId)
        endTimeEpochMs = intent?.getLongExtra(EXTRA_END_TIME_EPOCH_MS, 0L) ?: 0L
        val remaining = endTimeEpochMs - System.currentTimeMillis()
        val notification = buildNotification(remaining)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
            startForeground(NOTIF_ID, notification, ServiceInfo.FOREGROUND_SERVICE_TYPE_SPECIAL_USE)
        } else {
            startForeground(NOTIF_ID, notification)
        }
        handler.removeCallbacks(tickRunnable)
        handler.post(tickRunnable)
        return START_NOT_STICKY
    }

    private val tickRunnable: Runnable =
        object : Runnable {
            override fun run() {
                val remaining = endTimeEpochMs - System.currentTimeMillis()
                if (remaining <= 0) {
                    onTimerFinished()
                } else {
                    NotificationManagerCompat
                        .from(this@FocusTimerService)
                        .notify(NOTIF_ID, buildNotification(remaining))
                    handler.postDelayed(this, 1_000L)
                }
            }
        }

    private fun onTimerFinished() {
        NotificationManagerCompat.from(this).cancel(NOTIF_ID)
        val completionNotif =
            NotificationCompat
                .Builder(this, "FOCUS_TIMER_CHANNEL")
                .setSmallIcon(android.R.drawable.ic_dialog_info)
                .setContentTitle("Focus session complete")
                .setAutoCancel(true)
                .build()
        NotificationManagerCompat.from(this).notify(COMPLETION_NOTIF_ID, completionNotif)
        stopSelf()
    }

    override fun onDestroy() {
        handler.removeCallbacks(tickRunnable)
        NotificationManagerCompat.from(this).cancel(NOTIF_ID)
        super.onDestroy()
    }

    private fun buildNotification(remainingMs: Long): Notification {
        val minutes = (remainingMs / 1000) / 60
        val seconds = (remainingMs / 1000) % 60
        val timeStr = "%02d:%02d".format(minutes, seconds)
        return NotificationCompat
            .Builder(this, "FOCUS_TIMER_CHANNEL")
            .setSmallIcon(android.R.drawable.ic_dialog_info)
            .setContentTitle("Focus session")
            .setContentText(timeStr)
            .setPriority(NotificationCompat.PRIORITY_LOW)
            .setOngoing(true)
            .setOnlyAlertOnce(true)
            .build()
    }
}

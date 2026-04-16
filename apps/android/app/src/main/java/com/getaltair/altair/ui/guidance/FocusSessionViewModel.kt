package com.getaltair.altair.ui.guidance

import android.os.CountDownTimer
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter
import java.util.UUID

private const val DEFAULT_DURATION_MS = 25 * 60 * 1000L

class FocusSessionViewModel(
    private val db: PowerSyncDatabase,
) : ViewModel() {
    private val _remainingMs = MutableStateFlow(DEFAULT_DURATION_MS)
    val remainingMs: StateFlow<Long> = _remainingMs

    private val _isRunning = MutableStateFlow(false)
    val isRunning: StateFlow<Boolean> = _isRunning

    private val _isFinished = MutableStateFlow(false)
    val isFinished: StateFlow<Boolean> = _isFinished

    private var questId: String? = null
    private var userId: String? = null
    private var startedAt: String? = null
    private var timer: CountDownTimer? = null

    fun init(qId: String) {
        questId = qId
    }

    fun start() {
        if (_isRunning.value) return
        val now = nowIso()
        startedAt = now

        // Resolve userId lazily from DB
        viewModelScope.launch {
            val rows =
                db.getAll<String>(
                    sql = "SELECT id FROM users WHERE deleted_at IS NULL LIMIT 1",
                    parameters = emptyList(),
                ) { cursor -> cursor.getString(0) ?: "" }
            userId = rows.firstOrNull()?.ifEmpty { null }
        }

        _isRunning.value = true
        timer =
            object : CountDownTimer(_remainingMs.value, 1_000L) {
                override fun onTick(millisUntilFinished: Long) {
                    _remainingMs.value = millisUntilFinished
                }

                override fun onFinish() {
                    _remainingMs.value = 0
                    _isRunning.value = false
                    _isFinished.value = true
                    recordSession()
                }
            }.start()
    }

    fun end() {
        timer?.cancel()
        timer = null
        _isRunning.value = false
        if (startedAt != null) {
            recordSession()
        }
    }

    private fun recordSession() {
        val uid = userId ?: return
        val started = startedAt ?: return
        val ended = nowIso()
        val durationMinutes =
            ((DEFAULT_DURATION_MS - _remainingMs.value) / 60_000)
                .toInt()
                .coerceAtLeast(1)
        viewModelScope.launch {
            val id = UUID.randomUUID().toString()
            db.execute(
                """INSERT OR IGNORE INTO focus_sessions
                   (id, quest_id, started_at, ended_at, duration_minutes, user_id, created_at, updated_at, deleted_at)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, NULL)""",
                listOf(id, questId, started, ended, durationMinutes, uid, started, ended),
            )
        }
    }

    override fun onCleared() {
        timer?.cancel()
        super.onCleared()
    }

    private fun nowIso(): String = LocalDateTime.now().format(DateTimeFormatter.ISO_LOCAL_DATE_TIME)
}

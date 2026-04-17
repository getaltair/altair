package com.getaltair.altair.ui.guidance

import android.os.CountDownTimer
import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import kotlinx.datetime.Clock
import java.util.UUID

private const val TAG = "FocusSessionViewModel"
private const val DEFAULT_DURATION_MS = 25 * 60 * 1000L

@OptIn(ExperimentalCoroutinesApi::class)
class FocusSessionViewModel(
    private val db: PowerSyncDatabase,
) : ViewModel() {
    private val _remainingMs = MutableStateFlow(DEFAULT_DURATION_MS)
    val remainingMs: StateFlow<Long> = _remainingMs

    private val _isRunning = MutableStateFlow(false)
    val isRunning: StateFlow<Boolean> = _isRunning

    private val _isFinished = MutableStateFlow(false)
    val isFinished: StateFlow<Boolean> = _isFinished

    private val _error = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = _error

    private var questId: String? = null
    private var startedAt: String? = null
    private var timer: CountDownTimer? = null

    // Eagerly fetch userId so it's available when the timer fires
    private val currentUserId: StateFlow<String?> =
        db
            .watch<String?>(
                sql = "SELECT id FROM users WHERE deleted_at IS NULL LIMIT 1",
                parameters = emptyList(),
            ) { cursor -> cursor.getString(0) }
            .map { it.firstOrNull() }
            .stateIn(viewModelScope, SharingStarted.Eagerly, null)

    fun init(qId: String) {
        questId = qId
    }

    fun start() {
        if (_isRunning.value) return
        val uid = currentUserId.value
        if (uid == null) {
            _error.value = "User not available — cannot start session"
            return
        }
        val now = Clock.System.now().toString()
        startedAt = now
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
                    recordSession(uid)
                }
            }.start()
    }

    fun end() {
        timer?.cancel()
        timer = null
        _isRunning.value = false
        val uid = currentUserId.value
        if (startedAt != null && uid != null) {
            recordSession(uid)
        }
    }

    private fun recordSession(uid: String) {
        val started = startedAt ?: return
        val ended = Clock.System.now().toString()
        val durationMinutes =
            ((DEFAULT_DURATION_MS - _remainingMs.value) / 60_000)
                .toInt()
                .coerceAtLeast(1)
        viewModelScope.launch {
            try {
                val id = UUID.randomUUID().toString()
                db.execute(
                    """INSERT OR IGNORE INTO focus_sessions
                   (id, quest_id, started_at, ended_at, duration_minutes, user_id, created_at, updated_at, deleted_at)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, NULL)""",
                    listOf(id, questId, started, ended, durationMinutes, uid, started, ended),
                )
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to record focus session", e)
                _error.value = e.message ?: "Failed to save session"
            }
        }
    }

    override fun onCleared() {
        if (_isRunning.value) end()
        super.onCleared()
    }
}

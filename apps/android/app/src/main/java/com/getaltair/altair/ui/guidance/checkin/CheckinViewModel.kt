package com.getaltair.altair.ui.guidance.checkin

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.domain.entity.DailyCheckin
import com.getaltair.altair.domain.repository.CheckinRepository
import java.time.Instant
import java.time.LocalDate
import java.util.UUID
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class CheckinViewModel(
    private val savedStateHandle: SavedStateHandle,
    private val checkinRepository: CheckinRepository,
) : ViewModel() {

    private val _uiState = MutableStateFlow<CheckinUiState>(CheckinUiState.Loading)
    val uiState: StateFlow<CheckinUiState> = _uiState.asStateFlow()

    init {
        loadCheckin()
    }

    private fun loadCheckin() {
        viewModelScope.launch {
            checkinRepository.getForToday(PLACEHOLDER_USER_ID).collect { existing ->
                if (existing != null) {
                    _uiState.value = CheckinUiState.Ready(
                        energyLevel = existing.energyLevel,
                        mood = existing.mood ?: "",
                        notes = existing.notes ?: "",
                        isSaved = true,
                    )
                } else {
                    _uiState.value = CheckinUiState.Ready(
                        energyLevel = savedStateHandle.get<Int>(KEY_ENERGY),
                        mood = savedStateHandle.get<String>(KEY_MOOD) ?: "",
                        notes = savedStateHandle.get<String>(KEY_NOTES) ?: "",
                    )
                }
            }
        }
    }

    fun setEnergyLevel(level: Int) {
        savedStateHandle[KEY_ENERGY] = level
        updateReady { it.copy(energyLevel = level) }
    }

    fun setMood(mood: String) {
        savedStateHandle[KEY_MOOD] = mood
        updateReady { it.copy(mood = mood) }
    }

    fun setNotes(notes: String) {
        savedStateHandle[KEY_NOTES] = notes
        updateReady { it.copy(notes = notes) }
    }

    fun save() {
        val state = _uiState.value as? CheckinUiState.Ready ?: return
        updateReady { it.copy(isSaving = true) }
        viewModelScope.launch {
            try {
                val checkin = DailyCheckin(
                    id = UUID.randomUUID(),
                    userId = PLACEHOLDER_USER_ID,
                    date = LocalDate.now(),
                    energyLevel = state.energyLevel,
                    mood = state.mood.ifBlank { null },
                    notes = state.notes.ifBlank { null },
                    createdAt = Instant.now(),
                )
                checkinRepository.save(checkin)
                updateReady { it.copy(isSaving = false, isSaved = true) }
            } catch (e: Exception) {
                _uiState.value = CheckinUiState.Error(e.message ?: "Failed to save")
            }
        }
    }

    private fun updateReady(transform: (CheckinUiState.Ready) -> CheckinUiState.Ready) {
        val current = _uiState.value as? CheckinUiState.Ready ?: return
        _uiState.value = transform(current)
    }

    companion object {
        val PLACEHOLDER_USER_ID: UUID =
            UUID.fromString("00000000-0000-0000-0000-000000000001")
        private const val KEY_ENERGY = "checkin_energy"
        private const val KEY_MOOD = "checkin_mood"
        private const val KEY_NOTES = "checkin_notes"
    }
}

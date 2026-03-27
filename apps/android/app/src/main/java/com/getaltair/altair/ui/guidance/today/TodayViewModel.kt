package com.getaltair.altair.ui.guidance.today

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.domain.repository.CheckinRepository
import com.getaltair.altair.domain.repository.QuestRepository
import com.getaltair.altair.domain.repository.RoutineRepository
import java.util.UUID
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch

class TodayViewModel(
    private val questRepository: QuestRepository,
    private val routineRepository: RoutineRepository,
    private val checkinRepository: CheckinRepository,
    private val userIdProvider: () -> UUID,
) : ViewModel() {

    private val _uiState = MutableStateFlow<TodayUiState>(TodayUiState.Loading)
    val uiState: StateFlow<TodayUiState> = _uiState.asStateFlow()

    init {
        loadData()
    }

    private fun loadData() {
        viewModelScope.launch {
            combine(
                questRepository.getDueToday(),
                routineRepository.getActive(),
                checkinRepository.getForToday(userIdProvider()),
            ) { quests, routines, checkin ->
                TodayUiState.Success(quests, routines, checkin)
            }.catch { e ->
                _uiState.value = TodayUiState.Error(e.message ?: "Unknown error")
            }.collect { state ->
                _uiState.value = state
            }
        }
    }

    fun completeQuest(id: UUID) {
        viewModelScope.launch {
            try {
                questRepository.complete(id)
            } catch (e: Exception) {
                _uiState.value =
                    TodayUiState.Error(e.message ?: "Failed to complete quest")
            }
        }
    }

}

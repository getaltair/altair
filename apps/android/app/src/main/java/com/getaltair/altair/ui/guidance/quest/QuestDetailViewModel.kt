package com.getaltair.altair.ui.guidance.quest

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.domain.repository.QuestRepository
import com.getaltair.altair.navigation.Screen
import java.util.UUID
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.launch

class QuestDetailViewModel(
    savedStateHandle: SavedStateHandle,
    private val questRepository: QuestRepository,
) : ViewModel() {

    private val questId: UUID = UUID.fromString(
        savedStateHandle.get<String>(Screen.QuestDetail.ARG_ID)!!,
    )

    private val _uiState = MutableStateFlow<QuestDetailUiState>(QuestDetailUiState.Loading)
    val uiState: StateFlow<QuestDetailUiState> = _uiState.asStateFlow()

    init {
        loadQuest()
    }

    private fun loadQuest() {
        viewModelScope.launch {
            questRepository.getById(questId)
                .catch { e ->
                    _uiState.value =
                        QuestDetailUiState.Error(e.message ?: "Unknown error")
                }
                .collect { quest ->
                    _uiState.value = if (quest != null) {
                        QuestDetailUiState.Success(quest)
                    } else {
                        QuestDetailUiState.Error("Quest not found")
                    }
                }
        }
    }

    fun completeQuest() {
        viewModelScope.launch { questRepository.complete(questId) }
    }
}

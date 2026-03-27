package com.getaltair.altair.ui.guidance.initiative

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.domain.repository.EpicRepository
import com.getaltair.altair.domain.repository.InitiativeRepository
import com.getaltair.altair.navigation.Screen
import java.util.UUID
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.launch

class InitiativeDetailViewModel(
    savedStateHandle: SavedStateHandle,
    private val initiativeRepository: InitiativeRepository,
    private val epicRepository: EpicRepository,
) : ViewModel() {

    private val initiativeId: UUID = UUID.fromString(
        savedStateHandle.get<String>(Screen.InitiativeDetail.ARG_ID)!!,
    )

    private val _uiState =
        MutableStateFlow<InitiativeDetailUiState>(InitiativeDetailUiState.Loading)
    val uiState: StateFlow<InitiativeDetailUiState> = _uiState.asStateFlow()

    init {
        loadDetail()
    }

    private fun loadDetail() {
        viewModelScope.launch {
            combine(
                initiativeRepository.getById(initiativeId),
                epicRepository.getByInitiative(initiativeId),
            ) { initiative, epics ->
                if (initiative != null) {
                    InitiativeDetailUiState.Success(initiative, epics)
                } else {
                    InitiativeDetailUiState.Error("Initiative not found")
                }
            }.catch { e ->
                _uiState.value =
                    InitiativeDetailUiState.Error(e.message ?: "Unknown error")
            }.collect { state ->
                _uiState.value = state
            }
        }
    }
}

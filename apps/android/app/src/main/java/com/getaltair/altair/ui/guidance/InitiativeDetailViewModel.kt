package com.getaltair.altair.ui.guidance

import android.util.Log
import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.EpicDao
import com.getaltair.altair.data.local.dao.InitiativeDao
import com.getaltair.altair.data.local.entity.EpicEntity
import com.getaltair.altair.data.local.entity.InitiativeEntity
import com.getaltair.altair.ui.UiState
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn

private const val TAG = "InitiativeDetailViewModel"

class InitiativeDetailViewModel(
    savedStateHandle: SavedStateHandle,
    private val initiativeDao: InitiativeDao,
    private val epicDao: EpicDao,
) : ViewModel() {
    private val initiativeId: String =
        savedStateHandle["id"] ?: run {
            Log.e(TAG, "Missing nav arg: id")
            ""
        }

    val initiative: StateFlow<UiState<InitiativeEntity>> =
        initiativeDao
            .watchById(initiativeId)
            .map<InitiativeEntity?, UiState<InitiativeEntity>> { UiState.Success(it) }
            .catch { emit(UiState.Error(it.message ?: "Unknown error")) }
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), UiState.Loading)

    val epics: StateFlow<UiState<List<EpicEntity>>> =
        epicDao
            .watchByInitiativeId(initiativeId)
            .map<List<EpicEntity>, UiState<List<EpicEntity>>> { UiState.Success(it) }
            .catch { emit(UiState.Error(it.message ?: "Unknown error")) }
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), UiState.Loading)
}

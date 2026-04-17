package com.getaltair.altair.ui.guidance

import android.util.Log
import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.EpicDao
import com.getaltair.altair.data.local.dao.InitiativeDao
import com.getaltair.altair.data.local.entity.EpicEntity
import com.getaltair.altair.data.local.entity.InitiativeEntity
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
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

    val initiative: StateFlow<InitiativeEntity?> =
        initiativeDao
            .watchById(initiativeId)
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)

    val epics: StateFlow<List<EpicEntity>> =
        epicDao
            .watchByInitiativeId(initiativeId)
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())
}

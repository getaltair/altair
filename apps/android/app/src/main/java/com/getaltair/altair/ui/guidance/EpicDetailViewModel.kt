package com.getaltair.altair.ui.guidance

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.EpicDao
import com.getaltair.altair.data.local.dao.QuestDao
import com.getaltair.altair.data.local.entity.EpicEntity
import com.getaltair.altair.data.local.entity.QuestEntity
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.stateIn

class EpicDetailViewModel(
    savedStateHandle: SavedStateHandle,
    private val epicDao: EpicDao,
    private val questDao: QuestDao,
    private val db: PowerSyncDatabase,
) : ViewModel() {
    private val epicId: String = checkNotNull(savedStateHandle["id"])

    val epic: StateFlow<EpicEntity?> =
        epicDao
            .watchById(epicId)
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)

    val quests: StateFlow<List<QuestEntity>> =
        questDao
            .watchByEpicId(epicId)
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())
}

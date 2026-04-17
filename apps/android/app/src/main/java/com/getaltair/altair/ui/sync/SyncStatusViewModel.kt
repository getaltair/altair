package com.getaltair.altair.ui.sync

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn

class SyncStatusViewModel(
    private val db: PowerSyncDatabase,
) : ViewModel() {
    val isPending: StateFlow<Boolean> =
        db.currentStatus
            .asFlow()
            .map { status -> status.uploading || !status.connected }
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), true)
}

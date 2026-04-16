package com.getaltair.altair.di

import com.getaltair.altair.ui.auth.AuthViewModel
import com.getaltair.altair.ui.guidance.EpicDetailViewModel
import com.getaltair.altair.ui.guidance.FocusSessionViewModel
import com.getaltair.altair.ui.guidance.GuidanceViewModel
import com.getaltair.altair.ui.guidance.InitiativeDetailViewModel
import com.getaltair.altair.ui.guidance.QuestDetailViewModel
import com.getaltair.altair.ui.knowledge.KnowledgeViewModel
import com.getaltair.altair.ui.knowledge.NoteDetailViewModel
import com.getaltair.altair.ui.settings.SettingsViewModel
import com.getaltair.altair.ui.sync.SyncStatusViewModel
import com.getaltair.altair.ui.today.TodayViewModel
import com.getaltair.altair.ui.tracking.TrackingViewModel
import org.koin.core.module.dsl.viewModelOf
import org.koin.dsl.module

val viewModelModule =
    module {
        viewModelOf(::AuthViewModel)
        viewModelOf(::SettingsViewModel)
        viewModelOf(::TrackingViewModel)
        viewModelOf(::TodayViewModel)
        viewModelOf(::GuidanceViewModel)
        viewModelOf(::QuestDetailViewModel)
        viewModelOf(::InitiativeDetailViewModel)
        viewModelOf(::EpicDetailViewModel)
        viewModelOf(::FocusSessionViewModel)
        viewModelOf(::KnowledgeViewModel)
        viewModelOf(::NoteDetailViewModel)
        viewModelOf(::SyncStatusViewModel)
    }
